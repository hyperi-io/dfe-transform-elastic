// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Named fields parsed from a hex string into a number, in place.
//!
//! `iptables` logs its low-level packet fields as hex text -- `ether_type` is
//! `0800`, `tos` and `precedence_bits` and `tcp_reserved_bits` are two-digit
//! strings -- and the pipeline converts each to the number Elasticsearch
//! indexes. The captured output settles the type: `"ether_type": 2048`, not
//! `"0800"`.
//!
//! The field NAMES come from the processor's `params`, not from the script, so
//! a fifth field added upstream needs no change here.
//!
//! Unmatched it is 124 of iptables' 203 wrong fields, and `tcp.reserved_bits`
//! alone unlocks 17 of its 39 events.

use serde_json::{Value, json};

use dfe_core::Event;

/// The map whose named members are hex strings, and where the names come from.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HexFields {
    /// The map holding the fields, `iptables` here.
    container: String,
    /// The `params` member listing which of its keys to convert.
    table: String,
}

impl HexFields {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(container: impl Into<String>, table: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            table: table.into(),
        }
    }
}

/// Convert each named field from hex text to a number.
///
/// A field the document does not carry is skipped, as the script's own `if
/// (field == null) continue;` does. An EMPTY string is left alone: the script
/// writes back inside its per-character loop, so a string with no characters
/// never reaches a write at all.
///
/// Non-hex characters are IGNORED rather than failing the field, because the
/// script accumulates only when its per-character value is non-negative.
pub fn hex_fields(
    event: &mut Event,
    pattern: &HexFields,
    params: &serde_json::Map<String, Value>,
) -> bool {
    let Some(names) = params.get(&pattern.table).and_then(Value::as_array) else {
        return true;
    };
    let mut converted = Vec::new();
    for name in names {
        let Some(name) = name.as_str() else {
            continue;
        };
        let path = format!("{}.{name}", pattern.container);
        let Some(text) = event.get(&path).and_then(Value::as_str) else {
            continue;
        };
        if text.is_empty() {
            continue;
        }
        let mut value: i64 = 0;
        for c in text.chars() {
            let Some(digit) = c.to_digit(16) else {
                continue;
            };
            value = value.saturating_mul(16).saturating_add(i64::from(digit));
        }
        converted.push((path, value));
    }
    for (path, value) in converted {
        let _ = event.set(&path, json!(value));
    }
    true
}

/// `for (key in params.<table>) { .. def field = <container>[key]; .. }`
#[must_use]
pub fn parse_hex_fields(script: &str) -> Option<HexFields> {
    // The per-character accumulation is what makes this a hex parse rather
    // than any other loop over a params list.
    if !script.contains("* 16 +") && !script.contains("*16+") {
        return None;
    }
    let (_, tail) = script.split_once("for (key in params.")?;
    let table = tail.split(')').next()?.trim().to_owned();

    // `def iptables = ctx['iptables'];` then `iptables[key]`.
    let (head, _) = script.split_once("[key]")?;
    let local = head.trim_end().rsplit([' ', '=', '\n']).next()?.trim();
    let container = script
        .split_once(&format!("{local} = ctx["))?
        .1
        .split(']')
        .next()?
        .trim()
        .trim_matches(['\'', '"'])
        .to_owned();

    (!table.is_empty() && !container.is_empty() && !table.contains(' '))
        .then(|| HexFields::new(container, table))
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "hex_tests.rs"]
mod tests;
