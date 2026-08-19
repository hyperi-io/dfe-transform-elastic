// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How much of the Painless in these transforms the runtime actually runs.
//!
//! `painless_exec` skips any script nothing recognises. That is the right
//! behaviour, but it means a transform can look like it ran while most of its
//! logic did nothing. This measures the gap and ratchets it.
//!
//! Raising the floor here is the point: every script the runtime learns to
//! execute moves it up, and the assertion stops it sliding back.
//!
//! Run `cargo test -p dfe-transforms --test painless_coverage -- --nocapture`
//! to see which scripts are still unhandled, most frequent first.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use dfe_runtime::testutil::harness::load_test_events;
use dfe_runtime::{Transform, painless_stats};
use dfe_transforms::filebeat;
use serde_json::{Map, Value, json};

/// The measured floor: 619 of 1,119 scripts run across 80 fixture files, or
/// 55.3%. It started at 5.9%. Raise it as the runtime learns more Painless;
/// never lower it without saying why.
///
/// The denominator fell when `!= null` stopped opening on an explicit null:
/// scripts that had been running against a field the vendor pipeline would
/// have skipped no longer run at all.
const COVERAGE_FLOOR: f64 = 0.55;

/// Every fixture directory with a transform to drive it.
///
/// `cisco/amp`, `cisco/asa` and `cisco/ftd` have fixtures but no registered
/// transform, so they are absent rather than silently skipped.
fn corpus() -> Vec<(&'static str, &'static dyn Transform)> {
    vec![
        ("okta/system", &filebeat::okta::default::Default),
        (
            "crowdstrike/falcon",
            &filebeat::crowdstrike::default::Default,
        ),
        (
            "azure/activitylogs",
            &filebeat::azure_activitylogs::default::Default,
        ),
        (
            "azure/auditlogs",
            &filebeat::azure_auditlogs::default::Default,
        ),
        (
            "azure/platformlogs",
            &filebeat::azure_platformlogs::default::Default,
        ),
        (
            "azure/signinlogs",
            &filebeat::azure_signinlogs::default::Default,
        ),
        ("fortinet/fortigate", &filebeat::fortinet::default::Default),
        ("o365/audit", &filebeat::o365::default::Default),
        ("panw/panos", &filebeat::panw::traffic::Traffic),
        ("cisco/ios", &filebeat::cisco_ios::default::Default),
        ("cisco/nexus", &filebeat::cisco_nexus::default::Default),
    ]
}

/// Every `.log` file directly under `dir`.
fn logs_in(dir: &Path) -> Vec<std::path::PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut logs: Vec<std::path::PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "log"))
        .collect();
    logs.sort();
    logs
}

/// Wrap each raw line the way Beats delivers it: the vendor payload as a
/// string in `message`.
fn beats_events(log: &Path) -> Vec<dfe_runtime::Event> {
    load_test_events(log)
        .unwrap_or_default()
        .into_iter()
        .map(|event| {
            let raw = serde_json::to_string(event.as_value()).unwrap_or_default();
            let mut obj = Map::new();
            obj.insert("message".to_string(), json!(raw));
            dfe_runtime::Event::new(Value::Object(obj))
        })
        .collect()
}

/// The fixtures live at the repo root, two levels above this crate.
fn fixtures_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/fixtures")
}

/// Drive every fixture, returning how many files were read.
fn drive_corpus() -> usize {
    let mut files = 0;
    for (dir, transform) in corpus() {
        for log in logs_in(&fixtures_root().join(dir)) {
            files += 1;
            for mut event in beats_events(&log) {
                let _ = transform.transform(&mut event);
            }
        }
    }
    files
}

/// Drive the corpus and report what fraction of Painless actually executed.
#[test]
fn painless_coverage_does_not_regress() {
    painless_stats::reset();
    painless_stats::enable_catalogue(true);
    let files = drive_corpus();
    painless_stats::enable_catalogue(false);

    assert!(
        files > 0,
        "no fixtures found under {} -- this test would pass vacuously",
        fixtures_root().display()
    );

    let handled = painless_stats::handled();
    let unhandled = painless_stats::unhandled();
    let coverage = painless_stats::coverage();

    println!(
        "painless across {files} fixture file(s): {handled} handled, \
         {unhandled} skipped, coverage {:.1}%",
        coverage * 100.0
    );
    for (script, count) in painless_stats::catalogue().iter().take(10) {
        let head: String = script.chars().take(90).collect();
        println!("  {count:>5}x {head}");
    }

    assert!(
        painless_stats::total() > 0,
        "no Painless ran at all -- either the fixtures stopped loading or \
         `painless_exec` stopped being called, and both make this test blind"
    );
    assert!(
        coverage >= COVERAGE_FLOOR,
        "painless coverage fell to {coverage:.3}, below the floor of {COVERAGE_FLOOR:.3}"
    );

    // The catalogue is the input to the string-processing work: it names the
    // scripts worth teaching the runtime, in frequency order.
    if unhandled > 0 {
        assert!(
            !painless_stats::catalogue().is_empty(),
            "scripts were skipped but the catalogue recorded none of them"
        );
    }
}
