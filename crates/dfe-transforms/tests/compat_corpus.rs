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

/// The transform a capture belongs to.
///
/// The corpus is keyed by Elastic's PACKAGE and data stream, which is not our
/// source name: the azure package holds four of our modules, and fortinet's
/// package is `fortinet_fortigate`. A capture with no transform here is a gap
/// worth failing on rather than skipping.
fn transform_for(package: &str, data_stream: &str) -> Option<&'static dyn Transform> {
    Some(match (package, data_stream) {
        ("azure", "activitylogs") => &filebeat::azure_activitylogs::default::Default,
        ("azure", "auditlogs") => &filebeat::azure_auditlogs::default::Default,
        ("azure", "platformlogs") => &filebeat::azure_platformlogs::default::Default,
        ("azure", "signinlogs") => &filebeat::azure_signinlogs::default::Default,
        ("cisco_ios", _) => &filebeat::cisco_ios::default::Default,
        ("cisco_meraki", _) => &filebeat::cisco_meraki::default::Default,
        ("cisco_nexus", _) => &filebeat::cisco_nexus::default::Default,
        ("crowdstrike", _) => &filebeat::crowdstrike::default::Default,
        ("fortinet_fortigate", _) => &filebeat::fortinet::default::Default,
        ("o365", _) => &filebeat::o365::default::Default,
        ("okta", _) => &filebeat::okta::default::Default,
        ("panw", _) => &filebeat::panw::default::Default,
        _ => return None,
    })
}

/// One fixture's confirmed output, and where it came from.
struct Captured {
    source: String,
    data_stream: String,
    fixture: String,
    engine: String,
    /// The pipeline `compat.py` installed. Anything not prefixed `compat-`
    /// was written by an earlier tool and the capture is stale.
    entry_pipeline: String,
    input: Vec<Value>,
    expected: Vec<Value>,
}

impl Captured {
    /// Whether Elastic's own run failed on every event.
    ///
    /// A uniform `pipeline_error` almost always means the input shape rather
    /// than the pipeline -- a fixture already in the Beats envelope wrapped a
    /// second time. Comparing against it reads as a broken transform.
    fn capture_failed(&self) -> bool {
        !self.expected.is_empty()
            && self
                .expected
                .iter()
                .all(|e| e.pointer("/event/kind").and_then(Value::as_str) == Some("pipeline_error"))
    }

    fn is_stale(&self) -> bool {
        !self.entry_pipeline.starts_with("compat-")
    }
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
            let stream_name = stream.file_name().to_string_lossy().into_owned();
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
                    data_stream: stream_name.clone(),
                    fixture: fixture.file_name().to_string_lossy().into_owned(),
                    engine: meta
                        .get("elasticsearch_version")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    entry_pipeline: meta
                        .get("entry_pipeline")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
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
        let Some(transform) = transform_for(&capture.source, &capture.data_stream) else {
            unmapped.push(format!("{}/{}", capture.source, capture.data_stream));
            continue;
        };

        // Say why a capture is not worth comparing BEFORE printing a score
        // against it, or a zero reads as a broken transform.
        if capture.is_stale() {
            println!(
                "[{}/{}] SKIPPED: written by an earlier tool ({}), regenerate it",
                capture.source, capture.fixture, capture.entry_pipeline,
            );
            continue;
        }
        if capture.capture_failed() {
            println!(
                "[{}/{}] SKIPPED: Elastic errored on all {} events, so the capture \
                 carries no expectation -- check the input shape",
                capture.source,
                capture.fixture,
                capture.expected.len(),
            );
            continue;
        }

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
                        // One stream, so a difference always follows the
                        // header of the fixture it came from.
                        println!("  {}[{i}]: {diff}", capture.fixture);
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

/// Every package and data stream `compat.py` captures must reach a transform.
#[test]
fn every_captured_source_is_a_registered_transform() {
    for capture in captured() {
        assert!(
            transform_for(&capture.source, &capture.data_stream).is_some(),
            "{}/{} is in the corpus with no transform",
            capture.source,
            capture.data_stream
        );
    }
}

/// The same check with no corpus on disk. The corpus is gitignored, so the
/// test above is vacuous on a fresh clone and a newly declared source would be
/// silently skipped rather than scored.
#[test]
fn every_declared_source_reaches_a_transform() {
    #[derive(serde::Deserialize)]
    struct Declaration {
        sources: std::collections::BTreeMap<String, Declared>,
    }

    #[derive(serde::Deserialize)]
    struct Declared {
        package: String,
        data_stream: String,
    }

    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sources.yaml");
    let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
    let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

    for (name, declared) in &declaration.sources {
        assert!(
            transform_for(&declared.package, &declared.data_stream).is_some(),
            "{name} is declared as {}/{} and `transform_for` does not map it",
            declared.package,
            declared.data_stream
        );
    }
}
