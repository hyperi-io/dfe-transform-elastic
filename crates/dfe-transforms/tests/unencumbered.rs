// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every licence-clean sample log reaches its transform and produces fields.
//!
//! `tests/fixtures/unencumbered/` carries vendor samples from CC0, MIT and
//! Apache-2.0 projects -- see the PROVENANCE.md beside them. They have no
//! expected output, so this is not a parity test: it asserts the transform
//! consumes each shape and emits something, which is what makes them usable as
//! input to `scripts/compat.py`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use dfe_runtime::event::Event;
use dfe_runtime::transform::Transform;
use dfe_transforms::filebeat;
use serde_json::{Value, json};

const SAMPLES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/unencumbered"
);

/// Sample directories with no transform, and none planned: they are here
/// because the upstream corpus carries them, not because a source needs them.
///
/// Declared rather than skipped. `transform_for` used to spell these in an arm
/// directly above its `_` catch-all, which made the arm dead and let ANY
/// unmapped directory pass as though it were deliberate -- the opposite of
/// what the comment on it said.
const UNMAPPED: &[&str] = &["cisco_asa", "cisco_umbrella"];

/// The transform each sample directory feeds. A directory with no entry here
/// and no place on [`UNMAPPED`] is a sample nothing consumes, which
/// `every_sample_directory_is_mapped_or_declared_unmapped` fails on.
fn transform_for(source: &str) -> Option<&'static dyn Transform> {
    Some(match source {
        "azure" => &filebeat::azure_activitylogs::default::Default,
        "cisco_ios" => &filebeat::cisco_ios::default::Default,
        "cisco_meraki" => &filebeat::cisco_meraki::default::Default,
        "cisco_nexus" => &filebeat::cisco_nexus::default::Default,
        "crowdstrike" => &filebeat::crowdstrike::default::Default,
        "fortinet" => &filebeat::fortinet::default::Default,
        "o365" => &filebeat::o365::default::Default,
        "okta" => &filebeat::okta::default::Default,
        "panw" => &filebeat::panw::default::Default,
        _ => return None,
    })
}

/// Every sample, guarded against finding none.
///
/// All three tests here loop over this and assert inside the loop, so an empty
/// walk passes every one of them -- and `read_dir` returning `Err` is the
/// ordinary way that happens. The guard lives HERE rather than in each test,
/// because two of the three did not have it and the third did.
fn sample_files() -> Vec<(String, PathBuf)> {
    let root = Path::new(SAMPLES);
    let mut out = Vec::new();
    let Ok(sources) = std::fs::read_dir(root) else {
        panic!("no sample directory at {SAMPLES}");
    };
    for source in sources.flatten().filter(|e| e.path().is_dir()) {
        let name = source.file_name().to_string_lossy().into_owned();
        let Ok(files) = std::fs::read_dir(source.path()) else {
            continue;
        };
        for file in files.flatten().filter(|e| e.path().is_file()) {
            out.push((name.clone(), file.path()));
        }
    }
    out.sort();
    assert!(!out.is_empty(), "no samples under {SAMPLES}");
    out
}

/// Wrap a raw line the way a Beats input would: the vendor payload in `message`.
fn wrap(line: &str) -> Event {
    Event::new(json!({ "message": line }))
}

#[test]
fn every_sample_feeds_a_transform_and_yields_fields() {
    let files = sample_files();
    let mut total = 0usize;
    for (source, path) in &files {
        let Some(transform) = transform_for(source) else {
            continue;
        };
        let text = std::fs::read_to_string(path).unwrap_or_default();
        // A few hundred lines is enough to exercise the shapes; the whole file
        // is for volume work, not for a unit-test-speed check.
        let lines: Vec<&str> = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .take(200)
            .collect();
        assert!(!lines.is_empty(), "{} is empty", path.display());

        let mut enriched = 0;
        for line in &lines {
            let mut event = wrap(line);
            if transform.transform(&mut event).is_err() {
                continue;
            }
            // `message` alone means nothing was extracted.
            if event
                .as_value()
                .as_object()
                .is_some_and(|m| m.keys().any(|k| k != "message"))
            {
                enriched += 1;
            }
        }

        println!(
            "[{source}] {enriched}/{} produced fields ({})",
            lines.len(),
            path.file_name().unwrap_or_default().to_string_lossy(),
        );
        assert!(
            enriched > 0,
            "{} produced no fields on any of {} lines",
            path.display(),
            lines.len()
        );
        total += lines.len();
    }
    assert!(total > 0, "no sample reached a transform");
}

/// No transform may panic on this material, whatever its shape.
#[test]
fn no_sample_panics_a_transform() {
    for (source, path) in sample_files() {
        let Some(transform) = transform_for(&source) else {
            continue;
        };
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        for line in text.lines().filter(|l| !l.trim().is_empty()).take(200) {
            let mut event = wrap(line);
            let _ = transform.transform(&mut event);
            // Already-JSON samples arrive as the document, not wrapped.
            if let Ok(value) = serde_json::from_str::<Value>(line) {
                let mut event = Event::new(value);
                let _ = transform.transform(&mut event);
            }
        }
    }
}

/// A sample nobody consumes is dead weight in the tree, and one that USED to
/// be consumed is a transform that quietly stopped being exercised.
///
/// Both look identical from inside the other tests here, which skip an
/// unmapped directory and carry on.
#[test]
fn every_sample_directory_is_mapped_or_declared_unmapped() {
    let orphans: BTreeSet<String> = sample_files()
        .into_iter()
        .map(|(source, _)| source)
        .filter(|source| transform_for(source).is_none())
        .filter(|source| !UNMAPPED.contains(&source.as_str()))
        .collect();

    assert!(
        orphans.is_empty(),
        "these sample directories feed no transform and are not declared \
         unmapped, so nothing exercises them: {orphans:?}"
    );
}

/// The provenance record travels with the data: every source directory is
/// named in it, so no file arrives without a licence.
#[test]
fn every_sample_directory_is_named_in_the_provenance() {
    let provenance = std::fs::read_to_string(Path::new(SAMPLES).join("PROVENANCE.md"))
        .expect("PROVENANCE.md sits beside the samples");

    for (source, _) in sample_files() {
        assert!(
            provenance.contains(&format!("`{source}/")),
            "{source}/ has no licence row in PROVENANCE.md"
        );
    }
}
