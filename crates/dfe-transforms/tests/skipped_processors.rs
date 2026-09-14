// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How many processors never run because their `if` condition did not
//! transpile.
//!
//! An Elastic ingest processor can carry an `if`. Where the generator cannot
//! express that condition in Rust it emits the processor's body inside
//! `if false` behind a `// SKIPPED:` marker, so the processor is present and
//! unreachable. That is the deliberate safe direction -- running it would
//! apply the processor where the pipeline says not to, and one that then reads
//! an absent field aborts the whole event through `?`, losing every field
//! rather than one (`codegen/processor.rs::wrap_conditional`).
//!
//! Safe is not free. A skipped `set` leaves a field unwritten and a skipped
//! `drop` keeps an event Elastic discards, so this is a defect count:
//!
//! 1. The number is STATED rather than rediscovered.
//! 2. It cannot grow. A new untranspilable condition fails this test.
//!
//! The counts live in `tests/ratchets.json` and this test writes them under
//! `DFE_UPDATE_RATCHETS=1`, so teaching the transpiler a condition lowers them
//! without anyone retyping a number. `scripts/skipped_debt.py` ranks what is
//! left by the parity debt of the source it sits in.
//!
//! **This file measured a different marker until 2026-09-03 and had read ZERO
//! since 2026-08-19.** It counted `// TODO: conditional: `, left where a guard
//! was dropped and the body ran UNGUARDED. `8fd4e550` regenerated the six
//! sources it ceilinged and the new codegen stopped emitting that marker
//! entirely -- so both tests passed on an empty scan, and the ceilings said
//! nothing. The unguarded class is now closed by construction; this counts its
//! successor. Hence `FILES_SCANNED`: a scan that finds nothing must fail
//! rather than pass.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use dfe_runtime::testutil::ratchets::Ratchets;

/// The marker the generator leaves where a condition defeated the transpiler.
const MARKER: &str = "// SKIPPED: condition not transpiled: ";

/// The walk has to actually reach the generated tree.
///
/// Not a property of the code under test -- a property of this test. Reading
/// zero markers is indistinguishable from scanning zero files, and that is
/// exactly how the previous version of this file passed for a fortnight while
/// measuring nothing.
///
/// Was 2,500 against 2,863 files, 653 of which no `mod.rs` declared and
/// nothing compiled. The walk sees the disk, so the floor follows the 2,210
/// the crate actually builds, at the same ratio.
const FILES_SCANNED: usize = 1_900;

/// Where the ratchets live, relative to this crate.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

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

/// Skipped processors per integration directory, the total, and the file count
/// the walk covered.
fn survey() -> (BTreeMap<String, usize>, usize, usize) {
    let root = source_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    let scanned = files.len();

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

    (per_source, total, scanned)
}

#[test]
fn skipped_processors_do_not_increase() {
    let (per_source, total, scanned) = survey();

    for (source, count) in &per_source {
        println!("{count:5}  {source}");
    }
    println!("{total} processor(s) skipped over {scanned} generated files");

    assert!(
        scanned >= FILES_SCANNED,
        "the walk covered {scanned} files, under the {FILES_SCANNED} floor -- \
         it is not reading the generated tree, so every count below is a zero \
         that means nothing"
    );
    let mut ratchets = Ratchets::load(&workspace_root());
    ratchets.check(
        "skipped_processors",
        total,
        "A condition the transpiler used to read no longer transpiles, so the \
         processor behind it stopped running.",
    );
    // Per source as well as in total, so a fix in one cannot be cancelled out
    // by a regression in another and still pass.
    ratchets.check_table(
        "skipped_per_source",
        &per_source,
        "Skipped processors rose in this source.",
    );
    ratchets.save();
}

/// A skipped `set` leaves one field unwritten. A skipped DROP keeps an event
/// Elastic throws away, so every field of an event that should not exist
/// reaches the sink.
///
/// The mirror of what this file used to count: an unguarded drop discarded
/// everything, and this keeps everything. Both come from a condition the
/// generator could not express.
#[test]
fn no_more_drops_are_skipped_than_today() {
    /// Lines to read past the marker before deciding the block is not a drop.
    const WINDOW: usize = 8;

    let root = source_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    let scanned = files.len();

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

    for site in &offenders {
        println!("drop never taken: {site}");
    }

    assert!(
        scanned >= FILES_SCANNED,
        "the walk covered {scanned} files, under the {FILES_SCANNED} floor"
    );
    let mut ratchets = Ratchets::load(&workspace_root());
    ratchets.check(
        "skipped_drops",
        offenders.len(),
        &format!("An event the pipeline discards is being kept: {offenders:?}"),
    );
    ratchets.save();
}
