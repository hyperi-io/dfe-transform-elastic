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
//! Update deliberately, never by reflex:
//!
//! ```text
//! DFE_UPDATE_SHAPE_LOCK=1 cargo test -p dfe-runtime --test shape_order
//! ```

use std::path::{Path, PathBuf};

/// A ladder: the file it lives in, and the text that marks a dispatch site.
struct Ladder {
    name: &'static str,
    source: &'static str,
    markers: &'static [&'static str],
}

const LADDERS: &[Ladder] = &[
    Ladder {
        name: "known_shapes",
        source: "src/painless_common.rs",
        markers: &["shapes.push(KnownShape::"],
    },
    Ladder {
        name: "params_shape",
        source: "src/painless_params.rs",
        markers: &["return Some(ParamsShape::", "Some(ParamsShape::"],
    },
];

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The variant named at a dispatch site, or `None` when the line is not one.
fn variant(line: &str, marker: &str) -> Option<String> {
    let rest = line.trim().strip_prefix(marker).or_else(|| {
        let at = line.find(marker)?;
        Some(&line[at + marker.len()..])
    })?;
    let end = rest
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(rest.len());
    (end > 0).then(|| rest[..end].to_string())
}

/// Every dispatch site in every ladder, in the order the source reads them.
fn observed() -> String {
    let mut out = String::new();
    for ladder in LADDERS {
        out.push_str(&format!("[{}]\n", ladder.name));
        let text = std::fs::read_to_string(crate_root().join(ladder.source))
            .unwrap_or_else(|_| panic!("cannot read {}", ladder.source));

        let mut position = 0;
        for line in text.lines() {
            // A doc comment quoting a marker is prose, not a dispatch site.
            if line.trim_start().starts_with("//") {
                continue;
            }
            for marker in ladder.markers {
                if let Some(name) = variant(line, marker) {
                    position += 1;
                    out.push_str(&format!("{position:3} {name}\n"));
                    break;
                }
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
