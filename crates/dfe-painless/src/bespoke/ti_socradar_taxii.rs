// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_socradar_taxii`'s closing unescape, which spares `event.original`.
//!
//! The TAXII feed doubles its backslashes the same way `ti_custom`'s does, and
//! the walk that halves them is the same walk. The difference is one the vendor
//! writes out longhand: the raw indicator is lifted out of the document before
//! the walk and put back after it, so the copy kept for replay still reads as
//! the feed sent it.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use super::ti_custom::unescape_value;

/// Where the raw indicator is kept.
const ORIGINAL: &str = "event.original";

/// `ti_socradar_taxii/indicator`, `script_unescape_values`: every string in
/// the document with its doubled backslashes halved, `event.original` alone
/// left as it arrived.
///
/// Only a string is saved, because the script declares the saved copy a
/// `String` and nothing but `preserve_original_event` writes the field.
fn unescape_values(event: &mut Event, _params: &Value) {
    // `ctx.event?.original` reads null when `ctx.event` itself does, and the
    // removal and the write back are guarded on that same subtree.
    let has_event = event.has_value("event");
    let saved = has_event.then(|| event.get_string(ORIGINAL)).flatten();
    if has_event {
        event.remove(ORIGINAL);
    }

    unescape_value(event.as_value_mut());

    if let Some(saved) = saved {
        let _ = event.set(ORIGINAL, saved);
    }
}

/// Every `ti_socradar_taxii` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "d93cb354343f2410769a7949780d2870b7f027168ee1285665772f78378424e4",
    source: "ti_socradar_taxii",
    name: "unescape_values",
    run: unescape_values,
}];

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn the_raw_indicator_keeps_its_escapes_and_everything_else_loses_them() {
        let mut event = Event::new(json!({
            "event": { "kind": "enrichment", "original": r"{'pattern':'HKLM\\System'}" },
            "ti_socradar_taxii": { "stix": { "pattern": r"[key = 'HKLM\\System']" } },
            "threat": { "indicator": { "registry": { "path": [r"HKLM\\System"] } } },
        }));
        unescape_values(&mut event, &Value::Null);

        assert_eq!(event.get_str(ORIGINAL), Some(r"{'pattern':'HKLM\\System'}"));
        assert_eq!(
            event.get_str("ti_socradar_taxii.stix.pattern"),
            Some(r"[key = 'HKLM\System']")
        );
        assert_eq!(
            event.get("threat.indicator.registry.path"),
            Some(&json!([r"HKLM\System"]))
        );
    }

    /// The field is removed and written back, so it moves to the end of the
    /// `event` subtree -- which is what the script's own remove-then-put does.
    #[test]
    fn the_saved_copy_is_appended_after_the_members_that_followed_it() {
        let mut event = Event::new(json!({
            "event": { "original": "raw", "kind": "enrichment", "dataset": "d" },
        }));
        unescape_values(&mut event, &Value::Null);

        let keys: Vec<&String> = event
            .get_object("event")
            .map(|subtree| subtree.keys().collect())
            .unwrap_or_default();
        assert_eq!(keys, ["kind", "dataset", "original"]);
    }

    /// With no `event` subtree there is nothing to save and nothing to put
    /// back, and the walk still runs over the rest of the document.
    #[test]
    fn a_document_without_the_subtree_is_still_unescaped() {
        let mut event = Event::new(json!({ "message": r"a\\b" }));
        unescape_values(&mut event, &Value::Null);

        assert_eq!(event.get_str("message"), Some(r"a\b"));
        assert!(!event.has("event"));
    }
}
