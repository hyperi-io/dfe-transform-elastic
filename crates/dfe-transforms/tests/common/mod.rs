// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Shared test helpers for integration tests.

use std::path::Path;

use dfe_runtime::event::Event;
use dfe_runtime::testutil::diff::{JsonDiff, MatchMode};
use dfe_runtime::testutil::harness::{load_integration_expected, load_test_events};
use dfe_runtime::transform::Transform;
use serde_json::Value;

/// Load events from a fixture file, auto-detecting the format.
///
/// Supports:
/// - Line-delimited JSON (.log with one JSON object per line)
/// - JSON wrapper (`{"events": [...]}` or bare JSON array `[...]`)
/// - Single JSON object (wrapped as single-element vec)
fn load_events_auto(path: &Path) -> Vec<Event> {
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let trimmed = content.trim();

    // Try {"events": [...]} wrapper first
    if let Ok(wrapper) = serde_json::from_str::<Value>(trimmed) {
        if let Some(events) = wrapper.get("events").and_then(|v| v.as_array()) {
            return events.iter().map(|v| Event::new(v.clone())).collect();
        }
        // Bare array
        if let Some(arr) = wrapper.as_array() {
            return arr.iter().map(|v| Event::new(v.clone())).collect();
        }
        // Single object
        if wrapper.is_object() {
            return vec![Event::new(wrapper)];
        }
    }

    // Try concatenated multi-line JSON objects (common in Elastic test fixtures)
    let mut events = Vec::new();
    let mut de = serde_json::Deserializer::from_str(trimmed).into_iter::<Value>();
    while let Some(Ok(val)) = de.next() {
        if val.is_object() {
            events.push(Event::new(val));
        }
    }
    if !events.is_empty() {
        return events;
    }

    // Last resort: line-delimited JSON
    load_test_events(path).unwrap_or_else(|e| panic!("load events from {}: {e}", path.display()))
}

/// Run a fixture test: input file + {"expected": [...]} output.
///
/// Uses Semantic mode (skips @timestamp, event.created, @metadata).
/// Reports results — does not assert failure (transforms are still being refined).
pub fn run_fixture(transform: &dyn Transform, fixture_dir: &str, log_name: &str) {
    let dir = Path::new(fixture_dir);

    // Try common file extensions
    let (log_path, expected_path) = find_fixture_pair(dir, log_name);

    if !log_path.exists() {
        eprintln!("SKIP: fixture not found: {}", log_path.display());
        return;
    }
    if !expected_path.exists() {
        eprintln!("SKIP: expected not found: {}", expected_path.display());
        return;
    }

    let mut events = load_events_auto(&log_path);
    let expected =
        load_integration_expected(&expected_path).unwrap_or_else(|e| panic!("load expected: {e}"));

    let total = events.len();
    let mut passed = 0;
    let mut errors = 0;

    for (i, event) in events.iter_mut().enumerate() {
        match transform.transform(event) {
            Err(e) => {
                eprintln!("  event[{i}]: transform error: {e}");
                errors += 1;
            }
            Ok(_) => {
                if let Some(expected_val) = expected.get(i) {
                    let diff =
                        JsonDiff::compare(expected_val, event.as_value(), MatchMode::Semantic);
                    if diff.is_match() {
                        passed += 1;
                    } else {
                        eprintln!("  event[{i}]: {diff}");
                    }
                }
            }
        }
    }

    println!(
        "[{}] {passed}/{total} matched, {errors} errors (fixture: {log_name})",
        transform.name(),
    );
}

/// Find the input + expected file pair for a fixture name.
///
/// Tries: `{name}.log` / `{name}.log-expected.json`,
///        `{name}.json` / `{name}.json-expected.json`
fn find_fixture_pair(dir: &Path, name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let log = dir.join(format!("{name}.log"));
    let log_expected = dir.join(format!("{name}.log-expected.json"));
    if log.exists() {
        return (log, log_expected);
    }

    let json = dir.join(format!("{name}.json"));
    let json_expected = dir.join(format!("{name}.json-expected.json"));
    if json.exists() {
        return (json, json_expected);
    }

    // Default to .log
    (log, log_expected)
}
