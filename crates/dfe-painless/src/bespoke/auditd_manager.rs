// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `auditd_manager`'s key rewrite, transcribed.
//!
//! auditd names several of its own fields with a hyphen -- `old-ses`,
//! `old-auid`, `selected-context` -- and Elasticsearch cannot index a document
//! beside a mapping that spells them with an underscore. The script walks the
//! whole document and rewrites every hyphenated key, which is also what makes
//! the rename of `user.old_auid` a few processors later find its source.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// `auditd_manager/auditd`, the untagged `handleMap` script in `default`:
/// every hyphenated key in the document, hyphens turned to underscores.
fn rewrite_hyphenated_keys(event: &mut Event, _params: &Value) {
    if let Value::Object(root) = event.as_value_mut() {
        handle_map(root);
    }
}

/// One map rewritten, and every map under it first.
///
/// The keys are snapshotted before the walk, the way the script's own
/// `keySet().toArray(...)` does, so a key it writes is not visited again.
fn handle_map(map: &mut Map<String, Value>) {
    let keys: Vec<String> = map.keys().cloned().collect();
    for key in keys {
        // A LIST of maps is not descended into -- the script tests the value
        // itself, and `auditd.data` is flat.
        if let Some(Value::Object(child)) = map.get_mut(&key) {
            handle_map(child);
        }
        if !key.contains('-') {
            continue;
        }
        // `shift_remove`, never `remove`: under `preserve_order` the plain one
        // drops the last key into the freed slot.
        if let Some(value) = map.shift_remove(&key) {
            map.insert(key.replace('-', "_"), value);
        }
    }
}

/// Every `auditd_manager` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "a43f0caf53a71d9ddf4055de84d7a687c0fc6f67f28c81cc01452a6207e1d249",
    source: "auditd_manager",
    name: "rewrite_hyphenated_keys",
    run: rewrite_hyphenated_keys,
}];
