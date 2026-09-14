// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `snyk`'s blank extension bucket and its scan-item tags, transcribed.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// The per-extension counts of files the scanner could not read.
const NOT_SUPPORTED: &str = "snyk.audit_logs.content.notSupported";

/// The tags the issue's scan item carries.
const SCAN_ITEM_TAGS: &str = "snyk.issues.relationships.scan_item.data.attributes.tags";

/// `rename_blank_notSupported` in `snyk/audit_logs`: the count keyed by the
/// EMPTY string given a name.
///
/// A file with no extension buckets under `""`, which no mapping can hold and
/// no query can name.
fn name_the_blank_extension(event: &mut Event, _params: &Value) {
    let rebuilt = {
        let Some(held) = event.get_object(NOT_SUPPORTED) else {
            return;
        };
        let Some(blank) = held.get("").filter(|count| !count.is_null()) else {
            return;
        };
        let blank = blank.clone();
        let mut rebuilt = held.clone();
        rebuilt.insert("no_extension".to_string(), blank);
        rebuilt.shift_remove("");
        rebuilt
    };
    let _ = event.update(NOT_SUPPORTED, Value::Object(rebuilt));
}

/// The untagged tag restructurer in `snyk/issues`: each scan-item tag as one
/// `key:value` string.
///
/// A tag the vendor already sent as a string passes through, and anything else
/// is dropped.
fn join_scan_item_tags(event: &mut Event, _params: &Value) {
    let Some(tags) = event.take_array(SCAN_ITEM_TAGS) else {
        return;
    };
    let joined: Vec<Value> = tags
        .iter()
        .filter_map(|tag| match tag {
            Value::Object(fields) => Some(Value::String(format!(
                "{}:{}",
                member(fields, "key"),
                member(fields, "value")
            ))),
            Value::String(_) => Some(tag.clone()),
            _ => None,
        })
        .collect();
    let _ = event.update(SCAN_ITEM_TAGS, Value::Array(joined));
}

/// One tag member, rendered the way Java's `List.join` renders it -- an absent
/// member reads as the word `null`.
fn member(fields: &Map<String, Value>, key: &str) -> String {
    painless_to_string(fields.get(key).unwrap_or(&Value::Null))
}

/// Every snyk script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "8e89773e2e77483c595410e1b25bafdbb26643d506007132e08ee7ea6a57de00",
        source: "snyk",
        name: "name_the_blank_extension",
        run: name_the_blank_extension,
    },
    Entry {
        hash: "4660968d1b1e7d815c9a0e3a22cb056bb4f05b36f160930f7347bdfaea4d4980",
        source: "snyk",
        name: "join_scan_item_tags",
        run: join_scan_item_tags,
    },
];
