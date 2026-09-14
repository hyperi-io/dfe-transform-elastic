// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `iis`'s repeated-capture collapse, transcribed.
//!
//! An access line can name the same address twice -- once in the `c-ip` column
//! and once in an `X-Forwarded-For` header -- and the grok captures both into a
//! list. The runtime keeps duplicates, as Elasticsearch does, so the pipeline
//! ships a script that dedupes each named field and unwraps a list left holding
//! one value. Without it `source.address` stays a two-element list, the
//! `convert` to `source.ip` behind it fails, and `related.ip` and the network
//! `event.category` behind THAT never run.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `iis/access`, the untagged dedupe scripts in `default`: one per side of the
/// connection, each reading its field names out of `params.fields`.
fn distinct_source(event: &mut Event, params: &Value) {
    distinct_under(event, params, "source");
}

fn distinct_destination(event: &mut Event, params: &Value) {
    distinct_under(event, params, "destination");
}

/// `ctx.<root>[field]` deduped in place, for each name the params list gives.
///
/// A field that is not a list is left alone, which is the script's own
/// `instanceof List` guard -- and a list that dedupes to one value is replaced
/// by that value rather than by a one-element list.
fn distinct_under(event: &mut Event, params: &Value, root: &str) {
    let Some(fields) = params.get("fields").and_then(Value::as_array) else {
        return;
    };
    for field in fields.iter().filter_map(Value::as_str) {
        let path = format!("{root}.{field}");
        let Some(Value::Array(values)) = event.get(&path) else {
            continue;
        };
        let mut seen: Vec<Value> = Vec::with_capacity(values.len());
        for value in values {
            if !seen.contains(value) {
                seen.push(value.clone());
            }
        }
        let collapsed = match seen.len() {
            1 => seen.swap_remove(0),
            _ => Value::Array(seen),
        };
        // `update` rather than `set`: the path already exists, and a vendor key
        // holding a dot of its own must not gain a nested twin beside it.
        event.update(&path, collapsed);
    }
}

/// Every `iis` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "da8d489184e8a68b1e3e7ea771bf3f02540b827c043603e92333cc0bbaa5f43c",
        source: "iis",
        name: "distinct_destination",
        run: distinct_destination,
    },
    Entry {
        hash: "001b0ea750503d8ff12395d4a14c3c3ca8d40aa580b88dac124c7e493519c5aa",
        source: "iis",
        name: "distinct_source",
        run: distinct_source,
    },
];
