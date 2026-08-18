// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How many processors run without the guard their pipeline gave them.
//!
//! An Elastic ingest processor can carry an `if` condition. Where that
//! condition is not expressed in Rust, the processor's ACTION is still here
//! and the condition sits above it in a comment, so the processor runs on
//! every event instead of the ones it was meant for.
//!
//! This is a defect count, not a style check. It exists for two reasons:
//!
//! 1. The number is STATED rather than rediscovered.
//! 2. It cannot grow. A new unguarded processor fails this test.
//!
//! Lower the floor as they are fixed. Never raise it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The marker left where a guard should be.
const MARKER: &str = "// TODO: conditional: ";

/// Total unguarded processors as measured today. Only ever goes down.
const CEILING: usize = 673;

/// Per-source ceilings, so a fix in one source cannot be cancelled out by a
/// regression in another and still pass the total.
const PER_SOURCE: &[(&str, usize)] = &[
    ("cisco_ios", 43),
    ("cisco_meraki", 151),
    ("cisco_nexus", 49),
    ("fortinet", 239),
    ("o365", 159),
    ("panw", 32),
];

fn source_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Unguarded processors per integration directory, plus the total.
fn survey() -> (BTreeMap<String, usize>, usize) {
    let root = source_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);

    let mut per_source: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0;

    for path in files {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let count = text.matches(MARKER).count();
        if count == 0 {
            continue;
        }
        total += count;

        // `<root>/filebeat/<source>/<file>.rs` -- the integration directory.
        let source = path
            .parent()
            .and_then(Path::file_name)
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        *per_source.entry(source).or_default() += count;
    }

    (per_source, total)
}

#[test]
fn unguarded_processors_do_not_increase() {
    let (per_source, total) = survey();

    for (source, count) in &per_source {
        println!("{count:5}  {source}");
    }
    println!("{total} processor(s) run without their pipeline's guard");

    assert!(
        total <= CEILING,
        "unguarded processors rose from {CEILING} to {total} -- a processor was added \
         without expressing its `if` condition"
    );

    for (source, ceiling) in PER_SOURCE {
        let count = per_source.get(*source).copied().unwrap_or(0);
        assert!(
            count <= *ceiling,
            "{source}: unguarded processors rose from {ceiling} to {count}"
        );
    }

    // A source not in the table must have none, or the table is stale.
    for (source, count) in &per_source {
        assert!(
            PER_SOURCE.iter().any(|(name, _)| name == source),
            "{source} has {count} unguarded processor(s) and no ceiling -- add one"
        );
    }
}

/// An unguarded `set` writes a field that should not be there. An unguarded
/// DROP discards the event, and every processor after it never runs -- which
/// is how every azure event ended up on the floor. That class is at zero and
/// must stay there.
#[test]
fn no_unguarded_processor_drops_the_event() {
    /// Lines to read past the marker before deciding the block is not a drop.
    const WINDOW: usize = 8;

    let root = source_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);

    let mut offenders = Vec::new();
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.contains(MARKER) {
                continue;
            }
            let end = (i + 1 + WINDOW).min(lines.len());
            if lines[i + 1..end]
                .iter()
                .any(|l| l.contains("TransformResult::Drop"))
            {
                offenders.push(format!("{}:{}", path.display(), i + 1));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "an unguarded processor drops the event, so nothing after it runs: {offenders:?}"
    );
}
