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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::expect_used)]
#[path = "byte_size_tests.rs"]
mod tests;
