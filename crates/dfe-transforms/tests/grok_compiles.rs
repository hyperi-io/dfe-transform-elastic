// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every grok literal in the generated transforms must compile.
//!
//! A pattern the regex engine rejects does not error at the call site: it is
//! swapped for a never-matching one and an error is logged, which under test
//! goes nowhere. The processor then quietly extracts nothing, and because the
//! header grok is usually the one that fails, every field downstream of it
//! disappears with it -- cisco_asa scored 0 of 512 events that way, on one
//! duplicated capture name.
//!
//! Nothing else catches this. The corpus notices the damage but not the cause,
//! and a source with no corpus capture notices neither.

use std::fs;
use std::path::{Path, PathBuf};

use dfe_runtime::codegen_api::grok_to_regex_typed;

/// A grok literal, and where it was written.
struct Site {
    file: PathBuf,
    line: usize,
    pattern: String,
}

/// Undo Rust's string-literal escaping, so the pattern reads as the compiler
/// would hand it to the macro.
fn unescape(literal: &str) -> String {
    let mut out = String::with_capacity(literal.len());
    let mut chars = literal.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// Read the string literal that opens at `bytes[start]`, honouring escapes.
fn read_literal(bytes: &[u8], start: usize) -> Option<(String, usize)> {
    // rustfmt wraps a long invocation, so the literal may begin on the next
    // line. Anything else between the paren and the quote is not a literal.
    let mut start = start;
    while bytes.get(start).is_some_and(u8::is_ascii_whitespace) {
        start += 1;
    }
    if bytes.get(start) != Some(&b'"') {
        return None;
    }
    let mut i = start + 1;
    let mut raw = String::new();
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                raw.push('\\');
                i += 1;
                if i < bytes.len() {
                    raw.push(bytes[i] as char);
                    i += 1;
                }
            }
            b'"' => return Some((raw, i + 1)),
            b => {
                raw.push(b as char);
                i += 1;
            }
        }
    }
    None
}

/// Every `cached_grok!` / `cached_grok_mapped!` literal in one file, and the
/// count of invocations whose literal could not be read -- a scan that skips
/// what it cannot parse would report a clean tree it never looked at.
fn sites_in(path: &Path) -> (Vec<Site>, usize) {
    let Ok(text) = fs::read_to_string(path) else {
        return (Vec::new(), 0);
    };
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut unread = 0;
    for marker in ["cached_grok!(", "cached_grok_mapped!("] {
        let mut from = 0;
        while let Some(offset) = text[from..].find(marker) {
            let open = from + offset + marker.len();
            if let Some((raw, end)) = read_literal(bytes, open) {
                found.push(Site {
                    file: path.to_path_buf(),
                    line: text[..open].lines().count(),
                    pattern: unescape(&raw),
                });
                from = end;
            } else {
                unread += 1;
                from = open;
            }
        }
    }
    (found, unread)
}

/// Walk the generated transforms.
fn every_site() -> (Vec<Site>, usize) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![root];
    let mut found = Vec::new();
    let mut unread = 0;
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let (sites, missed) = sites_in(&path);
                found.extend(sites);
                unread += missed;
            }
        }
    }
    (found, unread)
}

#[test]
fn every_grok_literal_compiles() {
    let (sites, unread) = every_site();
    assert_eq!(
        unread, 0,
        "{unread} grok invocation(s) do not open with a string literal, so the \
         scan cannot see them"
    );

    // Either engine will do -- the fast one for nearly all of them, the
    // backtracking one for the handful that use look-around. What is not
    // acceptable is a pattern neither can compile.
    let mut broken = Vec::new();
    for site in &sites {
        let (expanded, _, _) = grok_to_regex_typed(&site.pattern);
        let fast = regex::Regex::new(&expanded);
        if let Err(err) = &fast {
            if fancy_regex::Regex::new(&expanded).is_ok() {
                continue;
            }
            let file = site
                .file
                .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                .unwrap_or(&site.file);
            broken.push(format!(
                "{}:{}\n    error: {}",
                file.display(),
                site.line,
                // The reason is the last line; the lines above it are the
                // pattern with a caret under the offending byte.
                err.to_string()
                    .lines()
                    .next_back()
                    .unwrap_or_default()
                    .trim(),
            ));
        }
    }

    assert!(
        broken.is_empty(),
        "{} of {} grok literals do not compile, so each matches nothing:\n  {}",
        broken.len(),
        sites.len(),
        broken.join("\n  ")
    );
}
