// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Structured JSON comparison with readable diff output.
//!
//! Three match modes for different testing scenarios:
//! - **Exact:** every key must match exactly (bidirectional)
//! - **Subset:** expected fields must be present in actual (extra fields allowed)
//! - **Semantic:** like exact, but ignores non-deterministic fields

use std::fmt;

use serde_json::{Map, Value};

use super::flatten::flatten_value;

/// How strictly to compare expected vs actual JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchMode {
    /// Every field in expected must match actual, and vice versa.
    Exact,
    /// Every field in expected must be present in actual. Extra fields in actual are ignored.
    Subset,
    /// Like Exact, but skip fields listed as non-deterministic (timestamps, UUIDs, etc.).
    Semantic,
}

/// A single field-level difference between expected and actual.
#[derive(Debug, Clone)]
pub struct FieldDiff {
    pub path: String,
    pub kind: DiffKind,
}

/// What kind of difference was found.
#[derive(Debug, Clone)]
pub enum DiffKind {
    /// Field exists in expected but not in actual.
    Missing { expected: Value },
    /// Field exists in actual but not in expected (only reported in Exact mode).
    Extra { actual: Value },
    /// Both have the field but values differ.
    Mismatch { expected: Value, actual: Value },
}

/// Result of comparing two JSON values.
#[derive(Debug, Clone)]
pub struct JsonDiff {
    pub diffs: Vec<FieldDiff>,
    pub mode: MatchMode,
}

impl JsonDiff {
    /// Compare expected and actual JSON values using the given match mode.
    ///
    /// Knows no source, so every policy rule applies whatever it is scoped to.
    /// [`Self::compare_for`] is the scoped form.
    pub fn compare(expected: &Value, actual: &Value, mode: MatchMode) -> Self {
        Self::compare_for(None, expected, actual, mode)
    }

    /// Compare, naming the source so the policy's scoped rules can apply.
    pub fn compare_for(
        source: Option<&str>,
        expected: &Value,
        actual: &Value,
        mode: MatchMode,
    ) -> Self {
        let expected_flat = flatten_value(expected);
        let actual_flat = flatten_value(actual);
        let diffs = compare_flat(source, &expected_flat, &actual_flat, mode);
        Self { diffs, mode }
    }

    /// Compare when expected is already in flat dot-notation format
    /// (as in Elastic test fixture `-expected.json` files).
    pub fn compare_flat_expected(
        expected_flat: &Map<String, Value>,
        actual: &Value,
        mode: MatchMode,
    ) -> Self {
        let actual_flat = flatten_value(actual);
        let diffs = compare_flat(None, expected_flat, &actual_flat, mode);
        Self { diffs, mode }
    }

    /// Whether the comparison found no differences.
    pub fn is_match(&self) -> bool {
        self.diffs.is_empty()
    }
}

impl fmt::Display for JsonDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.diffs.is_empty() {
            return write!(f, "no differences");
        }

        writeln!(
            f,
            "{} difference(s) found ({:?} mode):",
            self.diffs.len(),
            self.mode
        )?;
        for diff in &self.diffs {
            match &diff.kind {
                DiffKind::Missing { expected } => {
                    writeln!(
                        f,
                        "  MISSING  {}: expected {}",
                        diff.path,
                        format_value(expected)
                    )?;
                }
                DiffKind::Extra { actual } => {
                    writeln!(f, "  EXTRA    {}: got {}", diff.path, format_value(actual))?;
                }
                DiffKind::Mismatch { expected, actual } => {
                    writeln!(
                        f,
                        "  MISMATCH {}: expected {} got {}",
                        diff.path,
                        format_value(expected),
                        format_value(actual)
                    )?;
                }
            }
        }
        Ok(())
    }
}

fn compare_flat(
    source: Option<&str>,
    expected: &Map<String, Value>,
    actual: &Map<String, Value>,
    mode: MatchMode,
) -> Vec<FieldDiff> {
    let mut diffs = Vec::new();

    let policy = super::policy::policy();
    let skipped = |key: &str| mode == MatchMode::Semantic && policy.skips(source, key);

    for (key, expected_val) in expected {
        if skipped(key) {
            continue;
        }

        match actual.get(key) {
            None => {
                diffs.push(FieldDiff {
                    path: key.clone(),
                    kind: DiffKind::Missing {
                        expected: expected_val.clone(),
                    },
                });
            }
            Some(actual_val) => {
                if !values_equal(key, expected_val, actual_val, mode) {
                    diffs.push(FieldDiff {
                        path: key.clone(),
                        kind: DiffKind::Mismatch {
                            expected: expected_val.clone(),
                            actual: actual_val.clone(),
                        },
                    });
                }
            }
        }
    }

    if mode == MatchMode::Exact || mode == MatchMode::Semantic {
        for (key, actual_val) in actual {
            if skipped(key) {
                continue;
            }
            if !expected.contains_key(key) {
                diffs.push(FieldDiff {
                    path: key.clone(),
                    kind: DiffKind::Extra {
                        actual: actual_val.clone(),
                    },
                });
            }
        }
    }

    diffs.sort_by(|a, b| a.path.cmp(&b.path));
    diffs
}

/// Compare two values at `path`.
///
/// The path decides whether an array is a set: `tests/compare-policy.yaml`
/// names the ECS fields whose members carry no ordering, and an array it does
/// not name is compared in order, deliberately.
fn values_equal(path: &str, a: &Value, b: &Value, mode: MatchMode) -> bool {
    match (a, b) {
        (Value::Array(xs), Value::Array(ys))
            if xs.len() == ys.len()
                && mode == MatchMode::Semantic
                && super::policy::policy().is_unordered(path) =>
        {
            let mut remaining: Vec<&Value> = ys.iter().collect();
            xs.iter().all(|x| {
                remaining
                    .iter()
                    .position(|y| values_equal(path, x, y, mode))
                    .map(|at| remaining.swap_remove(at))
                    .is_some()
            })
        }
        (Value::Number(na), Value::Number(nb)) => {
            // Handle integer vs float comparison (1 == 1.0)
            if let (Some(fa), Some(fb)) = (na.as_f64(), nb.as_f64()) {
                // Scaled by magnitude, because `f64::EPSILON` is the step at
                // 1.0 and nothing else. One step at 58 is 7.1e-15, thirty-two
                // times that, so a capture whose DECIMAL TEXT round-trips one
                // step low read as a defect: rapid7's `58.282000000000004`
                // parses below the value `582.82 / 10.0` computes, and ours is
                // the correctly-rounded one. The tolerance stays inside a
                // single rounding step, which no arithmetic defect fits within.
                let scale = fa.abs().max(fb.abs()).max(1.0);
                (fa - fb).abs() <= f64::EPSILON * scale
            } else {
                na == nb
            }
        }
        (Value::String(sa), Value::String(sb)) => {
            if sa == sb {
                return true;
            }
            // Try timestamp equivalence (2018-09-13T13:45:39.000Z == 2018-09-13T13:45:39+00:00)
            if let (Ok(ta), Ok(tb)) = (
                chrono::DateTime::parse_from_rfc3339(sa),
                chrono::DateTime::parse_from_rfc3339(sb),
            ) {
                return ta == tb;
            }
            false
        }
        _ => a == b,
    }
}

fn format_value(v: &Value) -> String {
    match v {
        Value::String(s) => format!("\"{s}\""),
        Value::Null => "null".to_string(),
        other => {
            let s = other.to_string();
            // Truncate by CHARACTER, not byte: a byte index lands inside a
            // codepoint on any non-ASCII value and slicing there panics.
            match s.char_indices().nth(77) {
                Some((end, _)) if s.chars().count() > 80 => format!("{}...", &s[..end]),
                _ => s,
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The truncation used to slice at byte 77, which cuts inside a codepoint
    /// and panics -- taking the whole fixture harness with it whenever a
    /// fixture carried non-ASCII text.
    #[test]
    fn format_value_truncates_on_a_character_boundary() {
        for filler in ["日", "Ä", "\u{1F600}", "한", "\u{0301}"] {
            let long = filler.repeat(200);
            let value = json!({ "message": long });
            let rendered = format_value(&value);
            assert!(
                rendered.ends_with("..."),
                "{filler:?}: long values must be truncated"
            );
        }
    }

    /// Truncation must not split a grapheme's underlying codepoint, and must
    /// stay bounded regardless of how wide the characters are.
    #[test]
    fn format_value_truncation_is_bounded_in_characters() {
        let value = json!({ "message": "日".repeat(500) });
        let rendered = format_value(&value);
        assert!(rendered.chars().count() <= 81, "{rendered}");
    }

    /// A short non-ASCII value is returned whole.
    #[test]
    fn format_value_leaves_short_values_alone() {
        assert_eq!(format_value(&json!("日本語")), "\"日本語\"");
        assert_eq!(format_value(&json!(42)), "42");
        assert_eq!(format_value(&Value::Null), "null");
    }

    #[test]
    fn exact_match_identical() {
        let a = json!({"event": {"kind": "event"}, "tags": ["forwarded"]});
        let diff = JsonDiff::compare(&a, &a, MatchMode::Exact);
        assert!(diff.is_match(), "{diff}");
    }

    #[test]
    fn exact_match_missing_field() {
        let expected = json!({"a": 1, "b": 2});
        let actual = json!({"a": 1});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Exact);
        assert!(!diff.is_match());
        assert_eq!(diff.diffs.len(), 1);
        assert!(matches!(diff.diffs[0].kind, DiffKind::Missing { .. }));
        assert_eq!(diff.diffs[0].path, "b");
    }

    #[test]
    fn exact_match_extra_field() {
        let expected = json!({"a": 1});
        let actual = json!({"a": 1, "b": 2});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Exact);
        assert!(!diff.is_match());
        assert_eq!(diff.diffs.len(), 1);
        assert!(matches!(diff.diffs[0].kind, DiffKind::Extra { .. }));
    }

    #[test]
    fn exact_match_value_mismatch() {
        let expected = json!({"a": {"b": "old"}});
        let actual = json!({"a": {"b": "new"}});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Exact);
        assert!(!diff.is_match());
        assert_eq!(diff.diffs[0].path, "a.b");
        assert!(matches!(diff.diffs[0].kind, DiffKind::Mismatch { .. }));
    }

    #[test]
    fn subset_allows_extra_fields() {
        let expected = json!({"a": 1});
        let actual = json!({"a": 1, "b": 2, "c": {"d": 3}});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Subset);
        assert!(diff.is_match(), "{diff}");
    }

    #[test]
    fn subset_catches_missing_and_mismatch() {
        let expected = json!({"a": 1, "b": "wrong"});
        let actual = json!({"a": 1, "b": "right"});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Subset);
        assert!(!diff.is_match());
        assert_eq!(diff.diffs.len(), 1);
        assert!(matches!(diff.diffs[0].kind, DiffKind::Mismatch { .. }));
    }

    #[test]
    fn semantic_skips_timestamp() {
        let expected = json!({"a": 1, "@timestamp": "2020-01-01T00:00:00Z"});
        let actual = json!({"a": 1, "@timestamp": "2024-06-15T12:00:00Z"});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Semantic);
        assert!(diff.is_match(), "{diff}");
    }

    #[test]
    fn semantic_skips_metadata_prefix() {
        let expected = json!({"a": 1});
        let actual = json!({"a": 1, "@metadata": {"beat": "filebeat"}});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Semantic);
        assert!(diff.is_match(), "{diff}");
    }

    #[test]
    fn compare_flat_expected_format() {
        let mut expected_flat = Map::new();
        expected_flat.insert("event.kind".to_string(), json!("event"));
        expected_flat.insert("tags".to_string(), json!(["forwarded"]));

        let actual = json!({"event": {"kind": "event"}, "tags": ["forwarded"]});
        let diff = JsonDiff::compare_flat_expected(&expected_flat, &actual, MatchMode::Exact);
        assert!(diff.is_match(), "{diff}");
    }

    #[test]
    fn numeric_int_float_equality() {
        let expected = json!({"port": 443});
        let actual = json!({"port": 443.0});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Exact);
        assert!(diff.is_match(), "{diff}");
    }

    #[test]
    fn display_output_readable() {
        let expected = json!({"a": 1, "b": "hello"});
        let actual = json!({"a": 2, "c": true});
        let diff = JsonDiff::compare(&expected, &actual, MatchMode::Exact);
        let output = diff.to_string();
        assert!(output.contains("MISMATCH"));
        assert!(output.contains("MISSING"));
        assert!(output.contains("EXTRA"));
    }
}
