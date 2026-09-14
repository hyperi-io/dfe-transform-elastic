// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `google_scc`'s MITRE id lookup, transcribed.
//!
//! A finding names its tactics and techniques in Google's own upper-case
//! spelling -- `PERSISTENCE`, `ADDITIONAL_CLOUD_CREDENTIALS` -- and the
//! processor's params block carries the two tables that turn them into ATT&CK
//! ids. A name the table does not hold contributes a null, which the module's
//! closing prune takes out.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `google_scc/finding`, the untagged lookup script in `default`:
/// `threat.tactic.id` and `threat.technique.id`, one id per name.
fn threat_ids_from_names(event: &mut Event, params: &Value) {
    look_up(event, params, "threat.tactic", "tactic");
    look_up(event, params, "threat.technique", "technique");
}

/// One `name` list read through one params table into the `id` beside it.
fn look_up(event: &mut Event, params: &Value, namespace: &str, table: &str) {
    let name_path = format!("{namespace}.name");
    let Some(Value::Array(names)) = event.get(&name_path) else {
        return;
    };
    let table = params.get(table);
    let ids: Vec<Value> = names
        .iter()
        .map(|name| {
            name.as_str()
                .and_then(|name| table.and_then(|table| table.get(name)))
                .cloned()
                .unwrap_or(Value::Null)
        })
        .collect();
    let _ = event.set(&format!("{namespace}.id"), Value::Array(ids));
}

/// Every `google_scc` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "803aa7d676a5d13dccf1fe41519bfa2cc9d1b585c70a86e49bb8d5331054b17f",
    source: "google_scc",
    name: "threat_ids_from_names",
    run: threat_ids_from_names,
}];
