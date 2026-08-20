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

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use dfe_runtime::event::Event;
use dfe_runtime::testutil::diff::{DiffKind, JsonDiff, MatchMode};
use dfe_runtime::testutil::{flatten_value, policy};
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
    /// The integrations commit the pipelines were taken from. A score is only
    /// comparable to another measured against the same one.
    integrations_sha: String,
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
                    integrations_sha: meta
                        .get("integrations_sha")
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

/// A running count of events and of the fields inside them.
///
/// An event score alone cannot separate a transform that is one field short on
/// every event from one that is unrecognisably wrong, and the two need
/// completely different work. 20% of events at 96% of fields is nearly done;
/// 20% at 40% is not started.
#[derive(Default, Clone, Copy)]
struct Score {
    events: usize,
    events_matched: usize,
    events_errored: usize,
    fields: usize,
    fields_wrong: usize,
    fields_extra: usize,
}

impl Score {
    fn add(&mut self, other: Self) {
        self.events += other.events;
        self.events_matched += other.events_matched;
        self.events_errored += other.events_errored;
        self.fields += other.fields;
        self.fields_wrong += other.fields_wrong;
        self.fields_extra += other.fields_extra;
    }

    fn line(&self) -> String {
        let pct = |n: usize, d: usize| {
            if d == 0 {
                100.0
            } else {
                100.0 * n as f64 / d as f64
            }
        };
        format!(
            "events {}/{} ({:.0}%), fields {}/{} ({:.1}%), {} extra, {} errors",
            self.events_matched,
            self.events,
            pct(self.events_matched, self.events),
            self.fields - self.fields_wrong,
            self.fields,
            pct(self.fields - self.fields_wrong, self.fields),
            self.fields_extra,
            self.events_errored,
        )
    }
}

/// How many of an expected document's fields are actually compared.
///
/// The policy's skipped paths are not measured -- counting them would inflate
/// every field score by the same fixed amount and hide movement.
fn compared_field_count(expected: &Value) -> usize {
    flatten_value(expected)
        .keys()
        .filter(|path| !policy().skips(path))
        .count()
}

/// One entry in the events-unlocked ranking.
struct Blocker {
    path: String,
    /// Failing events this path is wrong in.
    appears: usize,
    /// Events that would pass once this path AND everything above it is fixed.
    unlocks: usize,
}

/// The fields whose repair would unlock the most events.
///
/// Greedy set cover over the failing events: take the path wrong in the most
/// of them, count the events it finishes off, strip it, and repeat. Ranking by
/// raw frequency instead conflates "appears often" with "worth fixing" -- a
/// path wrong in 400 events that are ALSO wrong in four other places unlocks
/// nothing on its own, and `unlocks` is what says so.
fn events_unlocked(failures: &[BTreeSet<String>], top: usize) -> Vec<Blocker> {
    let mut events: Vec<BTreeSet<String>> = failures.to_vec();
    let mut ranked = Vec::new();

    while ranked.len() < top {
        events.retain(|event| !event.is_empty());
        if events.is_empty() {
            break;
        }

        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for event in &events {
            for path in event {
                *counts.entry(path.as_str()).or_default() += 1;
            }
        }

        // Ties break on the lexicographically first path, so the ranking is the
        // same on every run and a diff of two reports means something.
        let Some((path, appears)) = counts
            .iter()
            .max_by_key(|(path, count)| (**count, std::cmp::Reverse(*path)))
            .map(|(path, count)| ((*path).to_owned(), *count))
        else {
            break;
        };

        let unlocks = events
            .iter()
            .filter(|event| event.len() == 1 && event.contains(path.as_str()))
            .count();
        for event in &mut events {
            event.remove(path.as_str());
        }
        ranked.push(Blocker {
            path,
            appears,
            unlocks,
        });
    }
    ranked
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

    // Whole-diff output for one source. Printing every difference for every
    // source buries the summary the ranking exists to give.
    let detail = std::env::var("DFE_COMPAT_DETAIL").ok();

    let mut unmapped = Vec::new();
    let mut by_source: BTreeMap<String, Score> = BTreeMap::new();
    let mut failures: BTreeMap<String, Vec<BTreeSet<String>>> = BTreeMap::new();
    let mut total = Score::default();

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

        let mut score = Score::default();
        for (i, raw) in capture.input.iter().enumerate() {
            let Some(expected) = capture.expected.get(i) else {
                continue;
            };
            score.events += 1;
            score.fields += compared_field_count(expected);

            let mut event = Event::new(raw.clone());
            if transform.transform(&mut event).is_err() {
                score.events_errored += 1;
                continue;
            }

            let diff = JsonDiff::compare(expected, event.as_value(), MatchMode::Semantic);
            if diff.is_match() {
                score.events_matched += 1;
                continue;
            }

            if detail.as_deref() == Some(capture.source.as_str()) {
                println!("  {}[{i}]: {diff}", capture.fixture);
            }

            let mut paths = BTreeSet::new();
            for field in &diff.diffs {
                match field.kind {
                    DiffKind::Extra { .. } => score.fields_extra += 1,
                    DiffKind::Missing { .. } | DiffKind::Mismatch { .. } => score.fields_wrong += 1,
                }
                paths.insert(field.path.clone());
            }
            failures
                .entry(capture.source.clone())
                .or_default()
                .push(paths);
        }

        println!(
            "[{}/{}] {} (es {})",
            capture.source,
            capture.fixture,
            score.line(),
            capture.engine,
        );
        by_source
            .entry(capture.source.clone())
            .or_default()
            .add(score);
        total.add(score);
    }

    println!("\n=== per source ===");
    for (source, score) in &by_source {
        println!("{source:<20} {}", score.line());
        for blocker in events_unlocked(failures.get(source).map_or(&[], Vec::as_slice), 6) {
            println!(
                "      wrong in {:>5}, unlocks {:>5}   {}",
                blocker.appears, blocker.unlocks, blocker.path
            );
        }
    }
    println!("\n{:<20} {}", "TOTAL", total.line());

    assert!(
        unmapped.is_empty(),
        "the corpus holds sources with no transform mapped in this test: {unmapped:?}"
    );

    check_baseline(&by_source, &provenance(&fixtures));
}

/// The integrations commit and engine version the whole corpus was taken at.
///
/// `None` when the captures disagree, which means the corpus was regenerated
/// piecemeal and no single baseline describes it.
fn provenance(fixtures: &[Captured]) -> Option<(String, String)> {
    let mut seen: Option<(String, String)> = None;
    for capture in fixtures {
        let here = (capture.integrations_sha.clone(), capture.engine.clone());
        match &seen {
            None => seen = Some(here),
            Some(first) if *first == here => {}
            Some(_) => return None,
        }
    }
    seen
}

/// Per-source scores this corpus is expected to reach.
#[derive(serde::Deserialize)]
struct Baseline {
    integrations_sha: String,
    elasticsearch_version: String,
    sources: BTreeMap<String, Expected>,
}

#[derive(serde::Deserialize)]
struct Expected {
    events: usize,
    events_total: usize,
    fields_wrong: usize,
}

/// Fail if a source scores below what it scored when the baseline was written.
///
/// Only asserted when the corpus on disk was captured at the same integrations
/// commit and engine version the baseline names. Anything else is reported --
/// a score against different pipelines is a different measurement, and
/// ratcheting one against the other would fail for the wrong reason.
fn check_baseline(measured: &BTreeMap<String, Score>, provenance: &Option<(String, String)>) {
    const PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/compat-baseline.json"
    );
    let Ok(text) = std::fs::read_to_string(PATH) else {
        println!("\nno baseline at {PATH}");
        return;
    };
    let baseline: Baseline = serde_json::from_str(&text).expect("parse compat-baseline.json");

    let Some((sha, engine)) = provenance else {
        println!("\nbaseline NOT asserted: the corpus mixes provenances, so regenerate it whole");
        return;
    };
    if *sha != baseline.integrations_sha || *engine != baseline.elasticsearch_version {
        println!(
            "\nbaseline NOT asserted: corpus is {}/{engine}, baseline is {}/{}",
            &sha[..12.min(sha.len())],
            &baseline.integrations_sha[..12.min(baseline.integrations_sha.len())],
            baseline.elasticsearch_version,
        );
        return;
    }

    let mut failures = Vec::new();
    let mut improved = Vec::new();
    for (source, expected) in &baseline.sources {
        let Some(score) = measured.get(source) else {
            failures.push(format!(
                "{source}: in the baseline and absent from the corpus"
            ));
            continue;
        };
        if score.events != expected.events_total {
            println!(
                "  {source}: {} events in the corpus, baseline was written against {} -- not asserted",
                score.events, expected.events_total
            );
            continue;
        }
        if score.events_matched < expected.events {
            failures.push(format!(
                "{source}: {}/{} events, baseline {}",
                score.events_matched, score.events, expected.events
            ));
        }
        if score.fields_wrong > expected.fields_wrong {
            failures.push(format!(
                "{source}: {} fields wrong, baseline {}",
                score.fields_wrong, expected.fields_wrong
            ));
        }
        if score.events_matched > expected.events || score.fields_wrong < expected.fields_wrong {
            improved.push(format!(
                "  \"{source}\": {{ \"events\": {}, \"events_total\": {}, \"fields_wrong\": {} }},",
                score.events_matched, score.events, score.fields_wrong
            ));
        }
    }

    for source in measured.keys() {
        if !baseline.sources.contains_key(source) {
            println!("  {source}: scored and not in the baseline");
        }
    }

    if !improved.is_empty() {
        println!("\nbaseline can be raised -- paste into tests/compat-baseline.json:");
        for line in &improved {
            println!("{line}");
        }
    }

    assert!(
        failures.is_empty(),
        "the corpus regressed against tests/compat-baseline.json:\n  {}",
        failures.join("\n  ")
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
