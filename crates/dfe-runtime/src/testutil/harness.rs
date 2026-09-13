// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Test harness for running transforms against fixture data.
//!
//! Supports two fixture formats:
//! - **Log format:** `.log` files (one JSON per line) + `-expected.json` (flat dot-notation)
//! - **Integration format:** `{"events": [...]}` + `{"expected": [...]}` (nested JSON)
//!
//! The integration format matches Elastic's `_dev/test/pipeline/` convention.

use std::path::Path;

use serde_json::{Map, Value};

use crate::event::Event;
use crate::transform::{Transform, TransformResult};

use super::diff::{JsonDiff, MatchMode};

/// Outcome of a single event comparison within a test run.
#[derive(Debug)]
pub struct EventTestResult {
    pub index: usize,
    pub diff: JsonDiff,
    pub dropped: bool,
}

/// Outcome of running a full test file through a transform.
#[derive(Debug)]
pub struct TestRunResult {
    pub event_results: Vec<EventTestResult>,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

impl TestRunResult {
    pub fn all_passed(&self) -> bool {
        self.failed == 0
    }
}

impl std::fmt::Display for TestRunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}/{} events matched", self.passed, self.total)?;
        for result in &self.event_results {
            if result.dropped {
                writeln!(f, "  event[{}]: DROPPED", result.index)?;
            } else if !result.diff.is_match() {
                writeln!(f, "  event[{}]: FAILED", result.index)?;
                write!(f, "    {}", result.diff)?;
            }
        }
        Ok(())
    }
}

/// Load test events from a `.log` file (one JSON object per line).
///
/// Blank lines are skipped. Each non-blank line is parsed as a complete JSON event.
pub fn load_test_events(path: &Path) -> crate::error::Result<Vec<Event>> {
    let content = std::fs::read_to_string(path).map_err(crate::error::TransformError::Io)?;

    let mut events = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        events.push(Event::from_json(trimmed)?);
    }

    Ok(events)
}

/// Load expected outputs from a `-expected.json` file.
///
/// The file contains a JSON array of objects with flat dot-notation keys.
/// Returns each object as a `Map<String, Value>` (already flat).
pub fn load_expected_outputs(path: &Path) -> crate::error::Result<Vec<Map<String, Value>>> {
    let content = std::fs::read_to_string(path).map_err(crate::error::TransformError::Io)?;

    let array: Vec<Map<String, Value>> = serde_json::from_str(&content)?;
    Ok(array)
}

/// Run a transform against input events and compare with expected outputs.
///
/// - `input_path`: path to `.log` file with one JSON event per line
/// - `expected_path`: path to `-expected.json` with flat dot-notation expected array
/// - `transform`: the transform to test
/// - `mode`: comparison strictness
pub fn run_transform_test(
    input_path: &Path,
    expected_path: &Path,
    transform: &dyn Transform,
    mode: MatchMode,
) -> crate::error::Result<TestRunResult> {
    let mut events = load_test_events(input_path)?;
    let expected = load_expected_outputs(expected_path)?;

    let total = events.len();
    let mut event_results = Vec::with_capacity(total);
    let mut passed = 0;
    let mut failed = 0;

    for (i, event) in events.iter_mut().enumerate() {
        let transform_result = transform.transform(event)?;

        let dropped = transform_result == TransformResult::Drop;

        let expected_flat = expected.get(i);

        let diff = if dropped {
            // Dropped events have no output to compare
            JsonDiff {
                diffs: Vec::new(),
                mode,
            }
        } else if let Some(expected_map) = expected_flat {
            JsonDiff::compare_flat_expected(expected_map, event.as_value(), mode)
        } else {
            // More events than expected outputs — mark as a diff
            JsonDiff {
                diffs: vec![super::diff::FieldDiff {
                    path: "<event count>".to_string(),
                    kind: super::diff::DiffKind::Extra {
                        actual: Value::String(format!("event {i} has no expected output")),
                    },
                }],
                mode,
            }
        };

        if diff.is_match() {
            passed += 1;
        } else {
            failed += 1;
        }

        event_results.push(EventTestResult {
            index: i,
            diff,
            dropped,
        });
    }

    Ok(TestRunResult {
        event_results,
        total,
        passed,
        failed,
    })
}

/// Load input events from an Elastic integration test file.
///
/// The file contains `{"events": [{ ... }, { ... }]}`.
/// Each element in the array is a complete event object.
pub fn load_integration_events(path: &Path) -> crate::error::Result<Vec<Event>> {
    let content = std::fs::read_to_string(path).map_err(crate::error::TransformError::Io)?;
    let wrapper: Value = serde_json::from_str(&content)?;

    let events_array = wrapper
        .get("events")
        .and_then(|v| v.as_array())
        .ok_or_else(|| crate::error::TransformError::ParseError {
            path: path.display().to_string(),
            message: "expected {\"events\": [...]}".to_string(),
        })?;

    events_array
        .iter()
        .map(|v| Ok(Event::new(v.clone())))
        .collect()
}

/// Load expected outputs from an Elastic integration expected file.
///
/// Either `{"expected": [{ ... }]}` or the bare `[{ ... }]` the older
/// fixtures use. Each element is a nested JSON object, not flat dot-notation.
pub fn load_integration_expected(path: &Path) -> crate::error::Result<Vec<Value>> {
    let content = std::fs::read_to_string(path).map_err(crate::error::TransformError::Io)?;
    let wrapper: Value = serde_json::from_str(&content)?;

    let expected_array = wrapper
        .get("expected")
        .and_then(Value::as_array)
        .or_else(|| wrapper.as_array())
        .ok_or_else(|| crate::error::TransformError::ParseError {
            path: path.display().to_string(),
            message: "expected {\"expected\": [...]} or a bare [...]".to_string(),
        })?;

    Ok(expected_array.clone())
}

/// Run a transform against Elastic integration test fixtures.
///
/// - `input_path`: path to JSON file with `{"events": [...]}`
/// - `expected_path`: path to JSON file with `{"expected": [...]}`
/// - `transform`: the transform to test
/// - `mode`: comparison strictness (Semantic recommended — skips timestamps)
pub fn run_integration_test(
    input_path: &Path,
    expected_path: &Path,
    transform: &dyn Transform,
    mode: MatchMode,
) -> crate::error::Result<TestRunResult> {
    let mut events = load_integration_events(input_path)?;
    let expected = load_integration_expected(expected_path)?;

    let total = events.len();
    let mut event_results = Vec::with_capacity(total);
    let mut passed = 0;
    let mut failed = 0;

    for (i, event) in events.iter_mut().enumerate() {
        let transform_result = transform.transform(event)?;
        let dropped = transform_result == TransformResult::Drop;

        let diff = if dropped {
            JsonDiff {
                diffs: Vec::new(),
                mode,
            }
        } else if let Some(expected_val) = expected.get(i) {
            // Integration fixtures use nested JSON, not flat dot-notation
            JsonDiff::compare(expected_val, event.as_value(), mode)
        } else {
            JsonDiff {
                diffs: vec![super::diff::FieldDiff {
                    path: "<event count>".to_string(),
                    kind: super::diff::DiffKind::Extra {
                        actual: Value::String(format!("event {i} has no expected output")),
                    },
                }],
                mode,
            }
        };

        if diff.is_match() {
            passed += 1;
        } else {
            failed += 1;
        }

        event_results.push(EventTestResult {
            index: i,
            diff,
            dropped,
        });
    }

    Ok(TestRunResult {
        event_results,
        total,
        passed,
        failed,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::event::Event;
    use crate::transform::{Transform, TransformResult};
    use serde_json::json;
    use std::io::Write;

    struct NoopTransform;

    impl Transform for NoopTransform {
        fn name(&self) -> &'static str {
            "noop"
        }

        fn transform(&self, _event: &mut Event) -> crate::error::Result<TransformResult> {
            Ok(TransformResult::Continue)
        }
    }

    struct SetKindTransform;

    impl Transform for SetKindTransform {
        fn name(&self) -> &'static str {
            "set_kind"
        }

        fn transform(&self, event: &mut Event) -> crate::error::Result<TransformResult> {
            event.set("event.kind", "event")?;
            Ok(TransformResult::Continue)
        }
    }

    #[test]
    fn load_events_from_log_file() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let log_path = dir.path().join("test.log");
        {
            let mut f = std::fs::File::create(&log_path).expect("create file");
            writeln!(f, r#"{{"message": "line1"}}"#).expect("write");
            writeln!(f, r#"{{"message": "line2"}}"#).expect("write");
            writeln!(f).expect("write blank");
            writeln!(f, r#"{{"message": "line3"}}"#).expect("write");
        }

        let events = load_test_events(&log_path).expect("load events");
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].get_str("message"), Some("line1"));
        assert_eq!(events[2].get_str("message"), Some("line3"));
    }

    #[test]
    fn load_expected_flat_format() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let expected_path = dir.path().join("test-expected.json");
        std::fs::write(
            &expected_path,
            r#"[{"event.kind": "event", "tags": ["forwarded"]}, {"event.kind": "alert"}]"#,
        )
        .expect("write");

        let expected = load_expected_outputs(&expected_path).expect("load expected");
        assert_eq!(expected.len(), 2);
        assert_eq!(expected[0].get("event.kind"), Some(&json!("event")));
        assert_eq!(expected[1].get("event.kind"), Some(&json!("alert")));
    }

    #[test]
    fn run_test_noop_subset_match() {
        let dir = tempfile::tempdir().expect("create temp dir");

        let log_path = dir.path().join("test.log");
        std::fs::write(
            &log_path,
            r#"{"event": {"kind": "event"}, "message": "hello", "extra": true}"#,
        )
        .expect("write");

        let expected_path = dir.path().join("test-expected.json");
        std::fs::write(
            &expected_path,
            r#"[{"event.kind": "event", "message": "hello"}]"#,
        )
        .expect("write");

        let result =
            run_transform_test(&log_path, &expected_path, &NoopTransform, MatchMode::Subset)
                .expect("run test");
        assert!(result.all_passed(), "{result}");
    }

    #[test]
    fn run_test_transform_adds_field() {
        let dir = tempfile::tempdir().expect("create temp dir");

        let log_path = dir.path().join("test.log");
        std::fs::write(&log_path, r#"{"message": "hello"}"#).expect("write");

        let expected_path = dir.path().join("test-expected.json");
        std::fs::write(
            &expected_path,
            r#"[{"message": "hello", "event.kind": "event"}]"#,
        )
        .expect("write");

        let result = run_transform_test(
            &log_path,
            &expected_path,
            &SetKindTransform,
            MatchMode::Exact,
        )
        .expect("run test");
        assert!(result.all_passed(), "{result}");
    }

    #[test]
    fn run_test_reports_failures() {
        let dir = tempfile::tempdir().expect("create temp dir");

        let log_path = dir.path().join("test.log");
        std::fs::write(&log_path, r#"{"message": "hello"}"#).expect("write");

        let expected_path = dir.path().join("test-expected.json");
        std::fs::write(
            &expected_path,
            r#"[{"message": "hello", "event.kind": "alert"}]"#,
        )
        .expect("write");

        let result = run_transform_test(
            &log_path,
            &expected_path,
            &SetKindTransform,
            MatchMode::Exact,
        )
        .expect("run test");
        assert!(!result.all_passed());
        assert_eq!(result.failed, 1);
        let output = result.to_string();
        assert!(output.contains("MISMATCH"));
    }

    #[test]
    fn load_integration_events_format() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("test.json");
        std::fs::write(
            &path,
            r#"{"events": [{"message": "one"}, {"message": "two"}]}"#,
        )
        .expect("write");

        let events = load_integration_events(&path).expect("load");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].get_str("message"), Some("one"));
        assert_eq!(events[1].get_str("message"), Some("two"));
    }

    #[test]
    fn load_integration_expected_format() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("test-expected.json");
        std::fs::write(
            &path,
            r#"{"expected": [{"event": {"kind": "event"}, "tags": ["forwarded"]}]}"#,
        )
        .expect("write");

        let expected = load_integration_expected(&path).expect("load");
        assert_eq!(expected.len(), 1);
        assert_eq!(expected[0]["event"]["kind"], "event");
    }

    #[test]
    fn run_integration_test_noop() {
        let dir = tempfile::tempdir().expect("create temp dir");

        let input = dir.path().join("input.json");
        std::fs::write(
            &input,
            r#"{"events": [{"event": {"kind": "event"}, "message": "hello"}]}"#,
        )
        .expect("write");

        let expected = dir.path().join("expected.json");
        std::fs::write(
            &expected,
            r#"{"expected": [{"event": {"kind": "event"}, "message": "hello"}]}"#,
        )
        .expect("write");

        let result =
            run_integration_test(&input, &expected, &NoopTransform, MatchMode::Exact).expect("run");
        assert!(result.all_passed(), "{result}");
    }

    #[test]
    fn run_integration_test_semantic_skips_timestamp() {
        let dir = tempfile::tempdir().expect("create temp dir");

        let input = dir.path().join("input.json");
        std::fs::write(
            &input,
            r#"{"events": [{"@timestamp": "2024-01-01", "message": "hi"}]}"#,
        )
        .expect("write");

        let expected = dir.path().join("expected.json");
        std::fs::write(
            &expected,
            r#"{"expected": [{"@timestamp": "2020-06-15", "message": "hi"}]}"#,
        )
        .expect("write");

        let result = run_integration_test(&input, &expected, &NoopTransform, MatchMode::Semantic)
            .expect("run");
        assert!(result.all_passed(), "{result}");
    }
}
