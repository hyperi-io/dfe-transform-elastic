// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cef`'s Check Point extension mapper, transcribed.
//!
//! Check Point spends the CEF custom slots -- `deviceCustomString1` through
//! `6`, the two custom dates, the flex numbers -- on a different field per
//! event type, and says which one in the matching `...Label` extension. The
//! script walks the package's table and builds the list of copies the
//! `foreach` behind it applies, so nothing here writes a shipped field
//! directly.
//!
//! It also carries the whole Check Point arm: the list lands in `_tmp_copy`
//! and the `remove` that closes the `foreach` has no `ignore_missing`, so a
//! skipped script raises `field not found` and takes every later processor
//! with it.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// Where the extensions the table names live.
const EXTENSIONS: &str = "cef.extensions";

/// The list the `foreach` behind this script walks.
const TARGET: &str = "_tmp_copy";

/// `cef/log`, the Check Point pipeline's `_tmp_copy` script: one
/// `{value, to}` entry per extension the package's table maps, either
/// directly or through the label the vendor set beside it.
fn collect_copies(event: &mut Event, params: &Value) {
    let Some(extensions) = params.get("extensions").and_then(Value::as_array) else {
        return;
    };
    // `ctx.cef?.extensions` -- the script returns before writing the list, and
    // the `remove` behind it then raises, which is what Elasticsearch does too.
    let Some(source) = event.get_object(EXTENSIONS).cloned() else {
        return;
    };

    let mut actions: Vec<Value> = Vec::with_capacity(extensions.len());
    for entry in extensions {
        let Some(name) = entry.get("name").and_then(Value::as_str) else {
            continue;
        };
        let Some(value) = source.get(name).filter(|value| !value.is_null()) else {
            continue;
        };
        // A converted entry replaces the value with the table's, and drops the
        // copy where the table has no row for it.
        let value = match entry.get("convert").and_then(Value::as_object) {
            Some(table) => {
                let Some(converted) = table.get(&painless_to_string(value).to_lowercase()) else {
                    continue;
                };
                converted
            }
            None => value,
        };

        if let Some(to) = entry.get("to").and_then(Value::as_str) {
            actions.push(copy(value, to));
            continue;
        }
        let Some(label) = source.get(&format!("{name}Label")).and_then(Value::as_str) else {
            continue;
        };
        let Some(destination) = entry
            .get("labels")
            .and_then(Value::as_object)
            .and_then(|labels| labels.get(&label.to_lowercase()))
            .and_then(Value::as_str)
        else {
            continue;
        };
        actions.push(copy(value, destination));
    }

    let _ = event.set(TARGET, Value::Array(actions));
}

/// One entry of the list, in the script's own key order.
fn copy(value: &Value, to: &str) -> Value {
    let mut entry = Map::with_capacity(2);
    entry.insert("value".to_owned(), value.clone());
    entry.insert("to".to_owned(), Value::from(to));
    Value::Object(entry)
}

/// Every `cef` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "c35f3f0d96015c4589fa3ff784fb1f1c568ead9288aead5c199b8fda53518aeb",
    source: "cef",
    name: "collect_copies",
    run: collect_copies,
}];
