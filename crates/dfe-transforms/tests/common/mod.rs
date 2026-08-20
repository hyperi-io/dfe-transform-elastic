// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Shared test helpers for integration tests.
//!
//! Simulates the Elastic ingest pipeline test framework:
//! - Raw events from .log files are wrapped into the `message` field as JSON strings
//! - Config YAML provides pre-set fields (@timestamp, tags, etc.)
//! - The transform then processes message → event.original → json.* → ECS fields

// Shared by the integration and e2e test binaries, which use different subsets.
#![allow(dead_code)]

pub mod test_infra;

use std::path::Path;

use dfe_runtime::event::Event;
use dfe_runtime::testutil::diff::{JsonDiff, MatchMode};
use dfe_runtime::testutil::harness::load_integration_expected;
use dfe_runtime::transform::Transform;
use serde_json::{Map, Value, json};

/// Load the test config YAML that provides pre-set fields.
///
/// Config files are named `test-common-config.yml` or `{name}-config.yml`.
/// They contain `fields:` with key-value pairs to merge into each event.
fn load_config_fields(dir: &Path, log_name: &str) -> Map<String, Value> {
    // Try fixture-specific config first (Elastic convention: {name}.log-config.yml)
    let specific_log = dir.join(format!("{log_name}.log-config.yml"));
    let specific_json = dir.join(format!("{log_name}.json-config.yml"));
    let specific_plain = dir.join(format!("{log_name}-config.yml"));
    let common = dir.join("test-common-config.yml");

    let config_path = if specific_log.exists() {
        specific_log
    } else if specific_json.exists() {
        specific_json
    } else if specific_plain.exists() {
        specific_plain
    } else if common.exists() {
        common
    } else {
        return Map::new();
    };

    let content = std::fs::read_to_string(&config_path).unwrap_or_default();
    let yaml: Value = serde_yaml_ng::from_str(&content).unwrap_or(Value::Null);

    yaml.get("fields")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default()
}

/// Load events from a fixture file, wrapping for pipeline processing.
///
/// For .log files: each line is a raw JSON event string. We wrap it into
/// `{"message": "<json-string>", ...config_fields}` to simulate what
/// Filebeat/Agent sends to the ingest pipeline.
///
/// For .json files with {"events": [...]}: events are already structured,
/// wrap each into message field.
fn load_and_wrap_events(path: &Path, config_fields: &Map<String, Value>) -> Vec<Event> {
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let trimmed = content.trim();

    // Collect raw event strings/values
    let raw_events = collect_raw_events(trimmed);

    // Wrap each into message field with config fields
    raw_events
        .into_iter()
        .map(|raw| wrap_event(raw, config_fields))
        .collect()
}

/// A raw event is either a JSON string (from .log) or a structured Value (from .json).
enum RawEvent {
    /// JSON string to put in message field
    JsonString(String),
    /// Already structured — serialise to string for message field
    Structured(Value),
}

fn collect_raw_events(content: &str) -> Vec<RawEvent> {
    // Try {"events": [...]} wrapper
    if let Ok(wrapper) = serde_json::from_str::<Value>(content) {
        if let Some(events) = wrapper.get("events").and_then(|v| v.as_array()) {
            return events
                .iter()
                .map(|v| RawEvent::Structured(v.clone()))
                .collect();
        }
        // Bare array
        if let Some(arr) = wrapper.as_array() {
            return arr
                .iter()
                .map(|v| RawEvent::Structured(v.clone()))
                .collect();
        }
        // Single object
        if wrapper.is_object() {
            return vec![RawEvent::Structured(wrapper)];
        }
    }

    // Try concatenated multi-line JSON objects
    let mut events = Vec::new();
    let mut de = serde_json::Deserializer::from_str(content).into_iter::<Value>();
    while let Some(Ok(val)) = de.next() {
        if val.is_object() {
            events.push(RawEvent::Structured(val));
        }
    }
    if !events.is_empty() {
        return events;
    }

    // Line-delimited JSON
    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| RawEvent::JsonString(l.trim().to_string()))
        .collect()
}

/// Wrap a raw event into an Event with `message` field + config fields.
///
/// This simulates what Filebeat/Agent does: the raw event JSON arrives as a
/// string in the `message` field, and the ingest pipeline parses it.
///
/// An object that ALREADY carries `message` is the envelope, not the payload.
/// Wrapping it again puts JSON text where the transform expects a log line.
fn wrap_event(raw: RawEvent, config_fields: &Map<String, Value>) -> Event {
    let message_str = match raw {
        RawEvent::JsonString(s) => s,
        RawEvent::Structured(Value::Object(obj)) if obj.contains_key("message") => {
            let mut event_obj = obj;
            for (k, v) in config_fields {
                event_obj.entry(k.clone()).or_insert_with(|| v.clone());
            }
            return Event::new(Value::Object(event_obj));
        }
        RawEvent::Structured(v) => serde_json::to_string(&v).unwrap_or_default(),
    };

    let mut event_obj = Map::new();
    event_obj.insert("message".to_string(), json!(message_str));

    // Merge config fields (e.g., @timestamp, tags)
    for (k, v) in config_fields {
        event_obj.insert(k.clone(), v.clone());
    }

    Event::new(Value::Object(event_obj))
}

/// Load a fixture's events, wrapped exactly as [`run_fixture`] would.
///
/// For the checks that assert individual fields rather than a whole event.
pub fn load_fixture_events(fixture_dir: &str, log_name: &str) -> Vec<Event> {
    let dir = Path::new(fixture_dir);
    let (log_path, _) = find_fixture_pair(dir, log_name);
    let config_fields = load_config_fields(dir, log_name);
    load_and_wrap_events(&log_path, &config_fields)
}

/// Run a fixture test: input file + {"expected": [...]} output.
///
/// Uses Semantic mode (skips @timestamp, event.created, @metadata).
///
/// `baseline` is the number of events that must match. It is a ratchet: set it
/// to the current measured rate, and raise it as transforms improve. A missing
/// fixture is a failure, not a skip -- a silently absent fixture reads as a
/// passing test.
pub fn run_fixture(transform: &dyn Transform, fixture_dir: &str, log_name: &str, baseline: usize) {
    let dir = Path::new(fixture_dir);
    let (log_path, expected_path) = find_fixture_pair(dir, log_name);

    assert!(
        log_path.exists(),
        "fixture input not found: {}",
        log_path.display()
    );
    assert!(
        expected_path.exists(),
        "fixture expectation not found: {}",
        expected_path.display()
    );

    let config_fields = load_config_fields(dir, log_name);
    let mut events = load_and_wrap_events(&log_path, &config_fields);
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
                    let diff = compare_expected(expected_val, event.as_value());
                    if diff.is_match() {
                        passed += 1;
                    } else {
                        // Print full diff for failing events
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

    assert!(
        passed >= baseline,
        "{} regressed on {log_name}: {passed}/{total} matched, baseline is {baseline}",
        transform.name(),
    );
}

/// Fields a Filebeat module capture strips before writing the golden file.
///
/// The PIPELINE leaves all five -- `message` still holds whatever the CSV or
/// grok did not consume. Reporting them as EXTRA compares our output against
/// a filter that ran after Elastic's, not against Elastic.
const CAPTURE_STRIPPED: &[&str] = &[
    "@metadata",
    "agent",
    "ecs.version",
    "log.file.path",
    "message",
];

/// Compare one expectation, in whichever of the two shapes it was committed.
///
/// Some fixtures store the expectation FLAT -- `"event.action": "..."` rather
/// than a nested object. Compared as nested, every one of those keys reads as
/// MISSING and the whole fixture scores zero however good the transform is.
/// That shape is also the marker for a module capture, so the capture's own
/// field filter is applied alongside it.
fn compare_expected(expected: &Value, actual: &Value) -> JsonDiff {
    match expected.as_object() {
        Some(map) if map.keys().any(|k| k.contains('.')) => {
            let mut actual = actual.clone();
            for field in CAPTURE_STRIPPED {
                remove_path(&mut actual, field);
            }
            JsonDiff::compare_flat_expected(map, &actual, MatchMode::Semantic)
        }
        _ => JsonDiff::compare(expected, actual, MatchMode::Semantic),
    }
}

/// Remove a dotted path from a JSON document, and its parent if that empties it.
fn remove_path(value: &mut Value, path: &str) {
    let Some(map) = value.as_object_mut() else {
        return;
    };
    let Some((head, rest)) = path.split_once('.') else {
        map.remove(path);
        return;
    };
    if let Some(child) = map.get_mut(head) {
        remove_path(child, rest);
        if child.as_object().is_some_and(serde_json::Map::is_empty) {
            map.remove(head);
        }
    }
}

/// Find the input + expected file pair for a fixture name.
fn find_fixture_pair(dir: &Path, name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let log = dir.join(format!("{name}.log"));
    let log_expected = dir.join(format!("{name}.log-expected.json"));
    if log.exists() {
        return (log, log_expected);
    }

    let json_file = dir.join(format!("{name}.json"));
    let json_expected = dir.join(format!("{name}.json-expected.json"));
    if json_file.exists() {
        return (json_file, json_expected);
    }

    (log, log_expected)
}
