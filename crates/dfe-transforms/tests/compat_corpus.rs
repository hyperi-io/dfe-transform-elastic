// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Compare every transform against what Elastic's own engine produced.
//!
//! `scripts/compat.py generate` runs raw source data through the real ingest
//! pipelines in a throwaway Elasticsearch container and writes the confirmed
//! documents to `testdata/compat/<source>/<data stream>/<fixture>/`. This test
//! reads those files. It starts no container and needs no network.
//!
//! The corpus derives from Elastic-Licensed pipelines, so it is not committed.
//! A missing corpus is reported and skipped -- unlike the committed fixtures,
//! where absence is a failure.

use std::path::{Path, PathBuf};

use dfe_runtime::event::Event;
use dfe_runtime::testutil::diff::{JsonDiff, MatchMode};
use dfe_runtime::transform::Transform;
use dfe_transforms::filebeat;
use serde_json::Value;

/// Where `compat.py` writes by default. `DFE_COMPAT_CORPUS` overrides it, the
/// same variable the tool reads.
fn corpus_root() -> PathBuf {
    std::env::var_os("DFE_COMPAT_CORPUS").map_or_else(
        || {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../testdata/compat")
                .to_path_buf()
        },
        PathBuf::from,
    )
}

/// The transform a corpus source directory names.
///
/// The corpus is keyed by our source name, so this is the same set the service
/// registry exposes; a source in the corpus with no transform here is a gap
/// worth failing on rather than skipping.
fn transform_for(source: &str) -> Option<&'static dyn Transform> {
    Some(match source {
        "azure_activitylogs" => &filebeat::azure_activitylogs::default::Default,
        "azure_auditlogs" => &filebeat::azure_auditlogs::default::Default,
        "azure_platformlogs" => &filebeat::azure_platformlogs::default::Default,
        "azure_signinlogs" => &filebeat::azure_signinlogs::default::Default,
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

/// One fixture's confirmed output, and where it came from.
struct Captured {
    source: String,
    fixture: String,
    engine: String,
    input: Vec<Value>,
    expected: Vec<Value>,
}

fn read_ndjson(path: &Path) -> Vec<Value> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Every captured fixture under the corpus root.
fn captured() -> Vec<Captured> {
    let root = corpus_root();
    let mut out = Vec::new();

    let Ok(sources) = std::fs::read_dir(&root) else {
        return out;
    };
    for source in sources.flatten().filter(|e| e.path().is_dir()) {
        let source_name = source.file_name().to_string_lossy().into_owned();
        let Ok(streams) = std::fs::read_dir(source.path()) else {
            continue;
        };
        for stream in streams.flatten().filter(|e| e.path().is_dir()) {
            let Ok(fixtures) = std::fs::read_dir(stream.path()) else {
                continue;
            };
            for fixture in fixtures.flatten().filter(|e| e.path().is_dir()) {
                let dir = fixture.path();
                let input = read_ndjson(&dir.join("input.ndjson"));
                let expected = read_ndjson(&dir.join("expected.ndjson"));
                if input.is_empty() || expected.is_empty() {
                    continue;
                }
                let meta: Value = std::fs::read_to_string(dir.join("meta.json"))
                    .ok()
                    .and_then(|t| serde_json::from_str(&t).ok())
                    .unwrap_or(Value::Null);
                out.push(Captured {
                    source: source_name.clone(),
                    fixture: fixture.file_name().to_string_lossy().into_owned(),
                    engine: meta
                        .get("elasticsearch_version")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    input,
                    expected,
                });
            }
        }
    }
    out.sort_by(|a, b| (&a.source, &a.fixture).cmp(&(&b.source, &b.fixture)));
    out
}

/// Report how each transform compares against Elastic's confirmed output.
///
/// Reporting, not ratcheting: the corpus is regenerated against whichever
/// engine and integrations commit the operator chose, so a number here is only
/// comparable to another run over the same corpus. `scripts/compat.py audit`
/// is where the tracked figures live.
#[test]
fn transforms_match_elastics_confirmed_output() {
    let fixtures = captured();
    if fixtures.is_empty() {
        println!(
            "no compat corpus at {} -- run `python3 scripts/compat.py generate --all`",
            corpus_root().display()
        );
        return;
    }

    let mut unmapped = Vec::new();
    for capture in &fixtures {
        let Some(transform) = transform_for(&capture.source) else {
            unmapped.push(capture.source.clone());
            continue;
        };

        let mut matched = 0;
        let mut errors = 0;
        for (i, raw) in capture.input.iter().enumerate() {
            let mut event = Event::new(raw.clone());
            match transform.transform(&mut event) {
                Err(_) => errors += 1,
                Ok(_) => {
                    let Some(expected) = capture.expected.get(i) else {
                        continue;
                    };
                    let diff = JsonDiff::compare(expected, event.as_value(), MatchMode::Semantic);
                    if diff.is_match() {
                        matched += 1;
                    } else {
                        eprintln!("  {}[{i}]: {diff}", capture.fixture);
                    }
                }
            }
        }

        println!(
            "[{}/{}] {matched}/{} matched, {errors} errors (es {})",
            capture.source,
            capture.fixture,
            capture.input.len(),
            capture.engine,
        );
    }

    assert!(
        unmapped.is_empty(),
        "the corpus holds sources with no transform mapped in this test: {unmapped:?}"
    );
}

/// The corpus is keyed by our source names, so a name that reaches it must be
/// one the service can actually run.
#[test]
fn every_captured_source_is_a_registered_transform() {
    for capture in captured() {
        assert!(
            transform_for(&capture.source).is_some(),
            "{} is in the corpus with no transform",
            capture.source
        );
    }
}
