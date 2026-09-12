// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A vendor's human-readable size string parsed onto a byte count.
//!
//! `digital_guardian` ships `"10.4 KB"` and wants `10400` beside it. The factors
//! are DECIMAL -- `KB` is 1000, not 1024 -- and they are read off the script's
//! own table rather than assumed, because another vendor writing the same
//! helper with binary factors would otherwise be silently scaled wrong.

use serde_json::Value;

use crate::params::clean_path;
use dfe_core::event::Event;

/// The helper's unit table and every field it is applied to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteSizeFields {
    /// `(unit, multiplier)`, in the order the script lists them.
    factors: Vec<(String, i64)>,
    /// `(the size string, where the byte count lands)`.
    fields: Vec<(String, String)>,
}

/// Read the factor table, or `None` where the script lists none.
///
/// The literal is `["KB": 1000L, "MB": 1000000L, ...]`, so each entry is a
/// quoted unit, a colon, and a long with its `L` suffix.
fn factor_table(script: &str) -> Option<Vec<(String, i64)>> {
    let (_, rest) = script.split_once("factors = [")?;
    let (body, _) = rest.split_once(']')?;
    let mut factors = Vec::new();
    for entry in body.split(',') {
        let (unit, scale) = entry.split_once(':')?;
        let unit = unit.trim().trim_matches(['"', '\'']);
        let scale = scale.trim().trim_end_matches(['L', 'l']);
        let scale: i64 = scale.parse().ok()?;
        if unit.is_empty() {
            return None;
        }
        factors.push((unit.to_string(), scale));
    }
    (!factors.is_empty()).then_some(factors)
}

/// Read `if (ctx.<source> instanceof String) { ctx.<target> = <helper>(ctx.<source>); }`
/// for every field the script applies the helper to.
///
/// The guard is not re-read at run time: `event.get_str` answers the same
/// question, and a non-string field is skipped either way.
fn applied_fields(script: &str, helper: &str) -> Vec<(String, String)> {
    let call = format!("= {helper}(ctx.");
    let mut fields = Vec::new();
    for (at, marker) in script.match_indices(&call) {
        let Some(source) = script[at + marker.len()..].split(')').next() else {
            continue;
        };
        let head = &script[..at];
        let Some(target) = head.rfind("ctx.").map(|i| &head[i + 4..]) else {
            continue;
        };
        let target = clean_path(target.trim());
        let source = clean_path(source.trim());
        if !target.is_empty() && !source.is_empty() {
            fields.push((source, target));
        }
    }
    fields
}

/// The helper's own name, which the vendor is free to choose.
fn helper_name(script: &str) -> Option<&str> {
    let (head, _) = script.split_once("(String str)")?;
    head.rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
}

/// Read the whole pattern, or `None` where any part of it is absent.
#[must_use]
pub fn parse_byte_size_fields(script: &str) -> Option<ByteSizeFields> {
    let factors = factor_table(script)?;
    let fields = applied_fields(script, helper_name(script)?);
    (!fields.is_empty()).then_some(ByteSizeFields { factors, fields })
}

/// One size string as a byte count, under the script's own factor table.
///
/// A unit the table does not carry -- and an absent unit -- falls back to the
/// bare number, which is what the script's `else` branch does. Java's `(long)`
/// cast truncates toward zero, so the arithmetic stays in `f64` and truncates
/// at the end rather than rounding.
// Java widens the table's `long` to a `double` for the multiply, so the same
// precision is lost there and matching it is the point.
#[allow(clippy::cast_precision_loss)]
fn bytes_from(text: &str, factors: &[(String, i64)]) -> Option<i64> {
    let mut parts = text.split(' ');
    let number: f64 = parts.next()?.parse().ok()?;
    let scale = parts
        .next()
        .and_then(|unit| factors.iter().find(|(name, _)| name == unit))
        .map_or(1.0, |(_, scale)| *scale as f64);
    let scaled = number * scale;
    // Painless throws on a value no `long` can hold; writing nothing is the
    // conservative reading of a throw the vendor never hits.
    (scaled.is_finite() && scaled.abs() < 9.2e18).then_some(scaled as i64)
}

/// Write each field's byte count beside it.
pub fn run_byte_size_fields(event: &mut Event, pattern: &ByteSizeFields) -> bool {
    for (source, target) in &pattern.fields {
        // Only a STRING is converted, exactly as the script's `instanceof`
        // guard says -- a field the vendor already sent as a number is left
        // alone rather than re-scaled.
        let Some(text) = event.get_str(source).map(str::to_string) else {
            continue;
        };
        if let Some(bytes) = bytes_from(&text, &pattern.factors) {
            let _ = event.set(target, Value::from(bytes));
        }
    }
    true
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::common::normalise;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/digital_guardian_arc/default.rs`,
    /// in the escaped one-line form a stored script arrives in.
    const SCRIPT: &str = r#"long bytesFromStr(String str) {\n  def factors = [\n    \"KB\": 1000L,\n    \"MB\": 1000000L,\n    \"GB\": 1000000000L,\n    \"TB\": 1000000000000L\n  ];\n  def parts = str.splitOnToken(' ');\n  double num = Double.parseDouble(parts[0]);\n  String unit = parts.length > 1 ? parts[1] : null;\n  if (factors.containsKey(unit)) {\n    return (long) (num * factors[unit]);\n  } else {\n    return (long) num;\n  }\n}\nif (ctx.digital_guardian?.arc?.dg_attachments?.dg_file_size instanceof String) {\n  ctx.digital_guardian.arc.dg_attachments.dg_file_size_bytes = bytesFromStr(ctx.digital_guardian.arc.dg_attachments.dg_file_size);\n}\nif (ctx.digital_guardian?.arc?.dg_file_size instanceof String) {\n  ctx.digital_guardian.arc.dg_file_size_bytes = bytesFromStr(ctx.digital_guardian.arc.dg_file_size);\n}\nif (ctx.digital_guardian?.arc?.pi_fs instanceof String) {\n  ctx.digital_guardian.arc.pi_fs_bytes = bytesFromStr(ctx.digital_guardian.arc.pi_fs);\n}\nif (ctx.digital_guardian?.arc?.uad_br instanceof String) {\n  ctx.digital_guardian.arc.uad_br_bytes = bytesFromStr(ctx.digital_guardian.arc.uad_br);\n}\nif (ctx.digital_guardian?.arc?.uad_bw instanceof String) {\n  ctx.digital_guardian.arc.uad_bw_bytes = bytesFromStr(ctx.digital_guardian.arc.uad_bw);\n}"#;

    #[test]
    fn the_factor_table_and_every_applied_field_are_read() {
        let pattern = parse_byte_size_fields(&normalise(SCRIPT)).expect("the helper is applied");
        assert_eq!(
            pattern.factors,
            vec![
                ("KB".to_string(), 1_000),
                ("MB".to_string(), 1_000_000),
                ("GB".to_string(), 1_000_000_000),
                ("TB".to_string(), 1_000_000_000_000),
            ]
        );
        assert_eq!(pattern.fields.len(), 5);
        assert_eq!(
            pattern.fields[0],
            (
                "digital_guardian.arc.dg_attachments.dg_file_size".to_string(),
                "digital_guardian.arc.dg_attachments.dg_file_size_bytes".to_string()
            )
        );
    }

    /// The captured values, and the decimal factors they prove.
    #[test]
    fn a_fractional_size_scales_by_a_thousand_not_by_a_kibibyte() {
        let pattern = parse_byte_size_fields(&normalise(SCRIPT)).expect("the helper is applied");
        let mut event = Event::new(serde_json::json!({ "digital_guardian": { "arc": {
            "dg_file_size": "10.4 KB",
            "dg_attachments": { "dg_file_size": "1.8 MB" }
        } } }));
        assert!(run_byte_size_fields(&mut event, &pattern));
        assert_eq!(
            event.get("digital_guardian.arc.dg_file_size_bytes"),
            Some(&serde_json::json!(10_400))
        );
        assert_eq!(
            event.get("digital_guardian.arc.dg_attachments.dg_file_size_bytes"),
            Some(&serde_json::json!(1_800_000))
        );
    }

    #[test]
    fn a_bare_number_and_an_unknown_unit_both_take_the_number_alone() {
        let pattern = parse_byte_size_fields(&normalise(SCRIPT)).expect("the helper is applied");
        let mut event = Event::new(serde_json::json!({ "digital_guardian": { "arc": {
            "dg_file_size": "512",
            "pi_fs": "7.9 PB"
        } } }));
        assert!(run_byte_size_fields(&mut event, &pattern));
        assert_eq!(
            event.get("digital_guardian.arc.dg_file_size_bytes"),
            Some(&serde_json::json!(512))
        );
        // Truncated toward zero, as Java's `(long)` cast does.
        assert_eq!(
            event.get("digital_guardian.arc.pi_fs_bytes"),
            Some(&serde_json::json!(7))
        );
    }

    #[test]
    fn a_field_that_is_absent_or_unparseable_is_left_alone() {
        let pattern = parse_byte_size_fields(&normalise(SCRIPT)).expect("the helper is applied");
        let mut event = Event::new(serde_json::json!({ "digital_guardian": { "arc": {
            "dg_file_size": "unknown",
            "uad_br": 4096
        } } }));
        assert!(run_byte_size_fields(&mut event, &pattern));
        assert!(!event.has("digital_guardian.arc.dg_file_size_bytes"));
        // A number is not a String, so the script's guard skips it.
        assert!(!event.has("digital_guardian.arc.uad_br_bytes"));
    }

    #[test]
    fn a_script_with_no_factor_table_is_declined() {
        assert!(parse_byte_size_fields("ctx.a = ctx.b;").is_none());
    }
}
