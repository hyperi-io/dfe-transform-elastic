// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_custom`'s closing unescape, which halves the backslashes a STIX pattern
//! arrives doubled with.
//!
//! A STIX indicator carries its pattern as a quoted string inside the JSON
//! document, so a Windows registry path or an x509 distinguished name reaches
//! the pipeline with every backslash doubled. The grok that lifts the pattern
//! into `threat.indicator.*` runs before this, so each extracted value carries
//! the doubling too, as does `event.original`. One walk over the whole document
//! takes it back out of all of them at once.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The doubled backslash a value arrives escaped with.
const ESCAPED: &str = r"\\";

/// The single backslash one stands for.
const LITERAL: &str = r"\";

/// `ti_custom/indicator`, `script_unscape_values`: every string in the
/// document with its doubled backslashes halved, `event.original` included.
///
/// `ti_anyrun/ioc` ships the same script character for character, so this one
/// entry serves both call sites.
fn unescape_values(event: &mut Event, _params: &Value) {
    unescape_value(event.as_value_mut());
}

/// Halve the doubled backslashes in every string at or under `value`.
///
/// The script recurses into a member before testing whether it is a string,
/// and recursing into a string does nothing, so which of the two is written
/// first makes no difference to the result.
pub(super) fn unescape_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for member in map.values_mut() {
                unescape_value(member);
            }
        }
        Value::Array(items) => {
            for item in items {
                unescape_value(item);
            }
        }
        // The guard is what keeps the walk cheap: most strings in the document
        // carry no doubled backslash, and those keep the allocation they hold.
        Value::String(text) if text.contains(ESCAPED) => {
            *text = text.replace(ESCAPED, LITERAL);
        }
        _ => {}
    }
}

/// Every `ti_custom` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "34c13ee13bbaacc1d35bd32447461390592a901f717e548c5ae8d0d447009caf",
    source: "ti_custom",
    name: "unescape_values",
    run: unescape_values,
}];

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// The replacement is Java's literal, left-to-right and non-overlapping, so
    /// an odd run of backslashes keeps its last one whole.
    #[test]
    fn a_doubled_backslash_is_halved_wherever_it_sits() {
        let mut event = Event::new(json!({
            "event": { "original": r"a\\b" },
            "stix": { "pattern": r"HKLM\\System\\Services" },
            "threat": { "indicator": { "registry": { "path": [r"HKLM\\System"] } } },
            "stix_count": 3,
            "odd": r"a\\\b",
        }));
        unescape_values(&mut event, &Value::Null);

        assert_eq!(event.get_str("event.original"), Some(r"a\b"));
        assert_eq!(event.get_str("stix.pattern"), Some(r"HKLM\System\Services"));
        assert_eq!(
            event.get("threat.indicator.registry.path"),
            Some(&json!([r"HKLM\System"]))
        );
        assert_eq!(event.get_i64("stix_count"), Some(3));
        assert_eq!(event.get_str("odd"), Some(r"a\\b"));
    }

    /// A key is not a value, and the script rewrites only what `getValue`
    /// hands it.
    #[test]
    fn a_key_spelled_with_backslashes_is_left_alone() {
        let mut root = serde_json::Map::new();
        root.insert(r"a\\b".to_string(), Value::from(r"c\\d"));
        let mut event = Event::new(Value::Object(root));
        unescape_values(&mut event, &Value::Null);

        let keys: Vec<&String> = event
            .as_value()
            .as_object()
            .map(|root| root.keys().collect())
            .unwrap_or_default();
        assert_eq!(keys, [r"a\\b"]);
        assert_eq!(event.get(r"a\\b"), Some(&json!(r"c\d")));
    }
}
