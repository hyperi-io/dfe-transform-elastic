// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::format_push_string,
    reason = "a test asserts by panicking, and this one builds its report as a string"
)]

//! The dispatch order of the shape ladders, pinned to a committed lock.
//!
//! Order is BEHAVIOUR. The first shape whose trigger fires claims the script,
//! so a shape inserted at the wrong position silently takes scripts belonging
//! to a later one -- a defect that has cost over a hundred parity events more
//! than once, and one that reads as an unrelated source regressing.
//!
//! Position in the file carries that order today, which makes it invisible in
//! review and a conflict for anyone else editing the same file. The lock makes
//! it a diff: adding or moving a shape changes `shapes.lock`, so the change is
//! reviewable on its own, and two people doing it at once get a MERGE CONFLICT
//! rather than a silent reorder.
//!
//! A dispatch site either constructs its variant inline
//! (`shapes.push(KnownShape::Foo(...))`) or hands back a value a `parse_*`
//! call already built (`shapes.push(shape)`). The second form names no
//! variant at its own site, so it is resolved by finding the ONE `let
//! Some(shape) = parse_*(...)` binding that fed it, then reading the ONE
//! variant `parse_*`'s own body constructs -- whatever combinator it used to
//! wrap it. A site the scanner cannot place either way is a scanner gap, not
//! a skip: it panics rather than silently dropping a shape out of the lock.
//!
//! Update deliberately, never by reflex:
//!
//! ```text
//! DFE_UPDATE_SHAPE_LOCK=1 cargo test -p dfe-runtime --test shape_order
//! ```

use std::path::{Path, PathBuf};

/// A ladder: the file it lives in, its dispatch functions in call order, the
/// text that opens a pushed/returned shape, and the enum's own `Name::` prefix.
///
/// EVERY function the ladder falls through to has to be listed, in order. The
/// params ladder runs `params_shape` -> `params_shape_tail` ->
/// `params_shape_rest`, and while the last was missing its shapes were
/// dispatched but unpinned -- the lock read as a guard over the whole ladder
/// and covered two thirds of it.
struct Ladder {
    name: &'static str,
    source: &'static str,
    functions: &'static [&'static str],
    push_prefix: &'static str,
    variant_prefix: &'static str,
}

const LADDERS: &[Ladder] = &[
    Ladder {
        name: "known_shapes",
        source: "src/painless_common.rs",
        functions: &["known_shapes"],
        push_prefix: "shapes.push(",
        variant_prefix: "KnownShape::",
    },
    Ladder {
        name: "params_shape",
        source: "src/painless_params.rs",
        functions: &["params_shape", "params_shape_tail", "params_shape_rest"],
        push_prefix: "return Some(",
        variant_prefix: "ParamsShape::",
    },
];

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_source(relative: &str) -> String {
    std::fs::read_to_string(crate_root().join(relative))
        .unwrap_or_else(|_| panic!("cannot read {relative}"))
}

/// The identifier `prefix` opens, when `text` starts with it exactly -- used
/// at a dispatch site, where the pushed expression must begin right where the
/// push left off.
fn variant_leading<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(prefix)?;
    let end = rest
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    (end > 0).then(|| &rest[..end])
}

/// The identifier after `prefix`'s first occurrence ANYWHERE in `text` --
/// used to read a helper's own construction, which sits after whatever
/// combinator (`Some(`, `.then(|| `, ...) the helper wrapped it in.
fn variant_anywhere<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let at = text.find(prefix)?;
    variant_leading(&text[at..], prefix)
}

/// The body of `fn <name>(...) { ... }`, braces included.
///
/// Found by locating the signature, then the next line that is a bare `}` --
/// rustfmt always closes a top-level item that way, which needs no
/// brace-depth count and so cannot be desynced by a `'{'` or `"...{"` inside
/// the body, both of which recur in this file's own Painless-matching code.
fn function_body<'a>(text: &'a str, name: &str) -> &'a str {
    let needle = format!("fn {name}(");
    let mut at = None;
    let mut hits = 0u32;
    let mut from = 0usize;
    while let Some(rel) = text[from..].find(needle.as_str()) {
        let candidate = from + rel;
        let line_start = text[..candidate].rfind('\n').map_or(0, |n| n + 1);
        if !text[line_start..candidate].trim_start().starts_with("//") {
            at = Some(candidate);
            hits += 1;
        }
        from = candidate + needle.len();
    }
    assert!(
        hits == 1,
        "expected exactly one definition of `fn {name}(`, found {hits} -- \
         the bare-dispatch resolver cannot tell which one runs"
    );
    let at = at.expect("hits == 1, checked above");

    let open = text[at..]
        .find('{')
        .map_or_else(|| panic!("no body opens for fn {name}"), |rel| at + rel);
    let close = text[open..].find("\n}").map_or_else(
        || panic!("no top-level close found for fn {name}"),
        |rel| open + rel + 2,
    );
    &text[open..close]
}

/// The one variant a helper function's own body constructs, read off its
/// text rather than its call site -- a bare local at the call site names no
/// variant, so the only way to know what it dispatches is to read the parser
/// that just ran.
fn resolve_variant(ladder: &Ladder, source_text: &str, function: &str) -> String {
    let body = function_body(source_text, function);
    let mut found: Option<String> = None;
    for line in body.lines() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        let Some(name) = variant_anywhere(line, ladder.variant_prefix) else {
            continue;
        };
        match &found {
            None => found = Some(name.to_string()),
            Some(existing) if existing == name => {}
            Some(existing) => panic!(
                "{function}: constructs both `{existing}` and `{name}` -- a bare dispatch \
                 site cannot be resolved to one variant. Teach the scanner which one runs."
            ),
        }
    }
    found.unwrap_or_else(|| {
        panic!(
            "{function}: constructs no `{}` -- teach the scanner its shape, or the lock \
             silently drops it.",
            ladder.variant_prefix
        )
    })
}

/// The variant a bare local resolves to: the nearest `let Some(<var>) =
/// <call>(...)` above byte offset `push_at` in `body`, then whatever variant
/// that call's own function constructs.
fn resolve_bare(
    ladder: &Ladder,
    source_text: &str,
    body: &str,
    push_at: usize,
    var: &str,
    function: &str,
) -> String {
    let binder = format!("let Some({var}) = ");
    let bind_at = body[..push_at].rfind(binder.as_str()).unwrap_or_else(|| {
        panic!(
            "{function}: bare dispatch of `{var}` has no `{binder}` binding above it in \
             this function -- teach the scanner this shape, or it silently drops out of \
             the lock."
        )
    });
    let after = &body[bind_at + binder.len()..];
    let call_end = after
        .find('(')
        .unwrap_or_else(|| panic!("{function}: binding for `{var}` is not a call: {after:?}"));
    let call_name = after[..call_end]
        .trim()
        .rsplit("::")
        .next()
        .unwrap_or_default();
    assert!(
        !call_name.is_empty(),
        "{function}: cannot read a function name out of the `{var}` binding"
    );
    resolve_variant(ladder, source_text, call_name)
}

/// Every dispatch site in one function's body, resolved to its variant name,
/// in the order the source reads them.
fn dispatch_sites(ladder: &Ladder, source_text: &str, function: &str) -> Vec<String> {
    let body = function_body(source_text, function);
    let mut out = Vec::new();
    let mut line_start = 0usize;
    loop {
        let line_end = body[line_start..]
            .find('\n')
            .map_or(body.len(), |rel| line_start + rel);
        let line = &body[line_start..line_end];

        if !line.trim_start().starts_with("//")
            && let Some(rel) = line.find(ladder.push_prefix)
        {
            let at = line_start + rel + ladder.push_prefix.len();
            let after = body[at..].trim_start();

            if let Some(name) = variant_leading(after, ladder.variant_prefix) {
                out.push(name.to_string());
            } else {
                let end = after
                    .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .unwrap_or(after.len());
                let var = &after[..end];
                let closes = after[end..].trim_start();
                assert!(
                    !var.is_empty() && closes.starts_with(')'),
                    "{function}: dispatch site matches neither `{}<Variant>` nor a single \
                     bare local -- teach the scanner this form, or it silently drops out of \
                     the lock: {line:?}",
                    ladder.variant_prefix
                );
                out.push(resolve_bare(ladder, source_text, body, at, var, function));
            }
        }

        if line_end >= body.len() {
            break;
        }
        line_start = line_end + 1;
    }
    out
}

/// Every dispatch site in every ladder, in the order the source reads them.
fn observed() -> String {
    let mut out = String::new();
    for ladder in LADDERS {
        out.push_str(&format!("[{}]\n", ladder.name));
        let text = read_source(ladder.source);

        let mut position = 0;
        for function in ladder.functions {
            for name in dispatch_sites(ladder, &text, function) {
                position += 1;
                out.push_str(&format!("{position:3} {name}\n"));
            }
        }
        out.push('\n');
    }
    out
}

#[test]
fn the_dispatch_order_matches_its_lock() {
    let lock = crate_root().join("shapes.lock");
    let current = observed();

    if std::env::var_os("DFE_UPDATE_SHAPE_LOCK").is_some() {
        std::fs::write(&lock, &current).expect("write shapes.lock");
        println!("wrote {}", lock.display());
        return;
    }

    let recorded = std::fs::read_to_string(&lock).unwrap_or_default();
    if recorded == current {
        return;
    }

    let recorded_lines: Vec<&str> = recorded.lines().collect();
    let current_lines: Vec<&str> = current.lines().collect();
    let first = recorded_lines
        .iter()
        .zip(&current_lines)
        .position(|(a, b)| a != b)
        .unwrap_or(recorded_lines.len().min(current_lines.len()));

    panic!(
        "the shape ladders no longer match shapes.lock, first difference at line {}:\n\
         lock    : {:?}\n\
         source  : {:?}\n\n\
         Order is behaviour: a shape placed too early takes scripts that belong to a \
         later one. Run the whole compat corpus, confirm no source regressed, then \
         update the lock with DFE_UPDATE_SHAPE_LOCK=1.",
        first + 1,
        recorded_lines.get(first),
        current_lines.get(first),
    );
}

#[test]
fn every_ladder_source_exists() {
    for ladder in LADDERS {
        let path = crate_root().join(ladder.source);
        assert!(
            Path::new(&path).is_file(),
            "{} moved; the lock would silently stop covering it",
            ladder.source
        );
    }
}
