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

use crate::Event;

/// The map whose named members are hex strings, and where the names come from.
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from `pipelines/iptables/log/default.yml`.
    fn script() -> &'static str {
        "def iptables = ctx['iptables']; if (iptables != null) {\n  \
         for (key in params.hex_fields_to_convert) {\n    long value = 0;\n    \
         def field = iptables[key];\n    if (field == null) continue;\n    \
         char[] hex = field.toLowerCase().toCharArray();\n    for (chr in hex) {\n      \
         long v = -1;\n      if (chr >= (char) 'a' && chr <= (char) 'f') \
         v = (long) chr - (char) 'a' + 10;\n      else if (chr >= (char) '0' && \
         chr <= (char) '9') v = (long) chr - (char) '0';\n      if (v >= 0) {\n        \
         value = value * 16 + v;\n      }\n      iptables[key] = value;\n    }\n  }\n}"
    }

    fn table() -> serde_json::Map<String, Value> {
        json!({ "hex_fields_to_convert": ["ether_type", "tos", "precedence_bits"] })
            .as_object()
            .expect("a params table")
            .clone()
    }

    /// The captured output carries numbers: `"ether_type": 2048`, from `0800`.
    #[test]
    fn each_named_field_becomes_its_number() {
        let pattern = parse_hex_fields(script()).expect("the conversion is recognised");
        let mut event = Event::new(json!({
            "iptables": { "ether_type": "0800", "tos": "00", "other": "ff" }
        }));
        assert!(hex_fields(&mut event, &pattern, &table()));

        assert_eq!(event.get("iptables.ether_type"), Some(&json!(2048)));
        assert_eq!(event.get("iptables.tos"), Some(&json!(0)));
        // A field the table does not name is left as it was.
        assert_eq!(event.get_str("iptables.other"), Some("ff"));
    }

    /// Upper case reads the same, since the script lower-cases first.
    #[test]
    fn case_does_not_matter() {
        let pattern = parse_hex_fields(script()).expect("recognised");
        let mut event = Event::new(json!({ "iptables": { "ether_type": "08FF" } }));
        assert!(hex_fields(&mut event, &pattern, &table()));
        assert_eq!(event.get("iptables.ether_type"), Some(&json!(2303)));
    }

    /// An absent field is skipped and an EMPTY one is left alone -- the script
    /// writes back inside its per-character loop, which an empty string never
    /// enters.
    #[test]
    fn absent_and_empty_are_both_left_alone() {
        let pattern = parse_hex_fields(script()).expect("recognised");
        let mut event = Event::new(json!({ "iptables": { "ether_type": "" } }));
        assert!(hex_fields(&mut event, &pattern, &table()));
        assert_eq!(event.get_str("iptables.ether_type"), Some(""));
        assert!(!event.has("iptables.tos"));
    }

    /// The container and the table name are read OFF the script.
    #[test]
    fn the_names_come_from_the_script() {
        let pattern = parse_hex_fields(script()).expect("recognised");
        assert_eq!(pattern, HexFields::new("iptables", "hex_fields_to_convert"));
    }

    /// A loop over a params list that is not accumulating hex is declined.
    #[test]
    fn a_loop_that_is_not_a_hex_parse_is_declined() {
        let script = "def m = ctx['m']; for (key in params.fields) { m[key] = m[key].trim(); }";
        assert!(parse_hex_fields(script).is_none());
    }
}
