// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_mandiant_advantage`'s companion hashes for an md5 indicator, and the
//! flat list `related.hash` is later renamed from.

use serde_json::Value;

use super::Entry;
use crate::helpers::painless_truthy;
use dfe_core::event::Event;

/// `ti_mandiant_advantage/threat_intelligence`, the untagged hash processor:
/// `threat.indicator.file.hash.sha1`, `threat.indicator.file.hash.sha256` and
/// `json.hashes`.
///
/// `json.hashes` arrives as the vendor's list of typed objects and leaves as a
/// list of hash VALUES, which is what makes the rename into `related.hash`
/// read.
fn companion_hashes(event: &mut Event, _params: &Value) {
    // Painless raises where the hash map itself is absent, and reads an absent
    // md5 as the null it then appends.
    if !event.has("threat.indicator.file.hash") {
        return;
    }
    let md5 = event
        .get("threat.indicator.file.hash.md5")
        .cloned()
        .unwrap_or(Value::Null);
    let Some(associated) = event.get_array("json.associated_hashes").cloned() else {
        return;
    };
    let mut hashes = vec![md5];
    for entry in associated {
        // A null member ends the script, keeping whatever it has already
        // written to the two hash fields and leaving `json.hashes` alone.
        if entry.is_null() {
            return;
        }
        let kind = entry.get("type").and_then(Value::as_str);
        let Some(value) = entry.get("value").cloned() else {
            continue;
        };
        let field = match kind {
            Some("sha1") => "threat.indicator.file.hash.sha1",
            Some("sha256") => "threat.indicator.file.hash.sha256",
            _ => continue,
        };
        let _ = event.set(field, value.clone());
        hashes.push(value);
    }
    let _ = event.set("json.hashes", Value::Array(hashes));
}

/// `ti_mandiant_advantage/threat_intelligence`, the untagged MISP processor:
/// `json.misp_warning_list_hits` and `json.misp_warning_list_misses`.
///
/// The vendor sends one boolean per warning list, and the two lists this
/// writes are the names split by that flag, in name order.
fn split_misp_warning_lists(event: &mut Event, _params: &Value) {
    let (hits, misses) = {
        let Some(lists) = event.get_object("json.misp") else {
            return;
        };
        let mut names: Vec<&String> = lists.keys().collect();
        names.sort();
        let mut hits = Vec::new();
        let mut misses = Vec::new();
        for name in names {
            let flagged = lists.get(name).is_some_and(painless_truthy);
            let target = if flagged { &mut hits } else { &mut misses };
            target.push(Value::from(name.clone()));
        }
        (hits, misses)
    };
    let _ = event.set("json.misp_warning_list_hits", Value::Array(hits));
    let _ = event.set("json.misp_warning_list_misses", Value::Array(misses));
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "c230cb2712b8172e9e52789370fcb3758fc3aebc13f44c8f50133c07685f961a",
        source: "ti_mandiant_advantage",
        name: "companion_hashes",
        run: companion_hashes,
    },
    Entry {
        hash: "02479fb7b0ade727ac8d24dd33176c9c2b9d6a1d9d5a3ff61183f531f53e628a",
        source: "ti_mandiant_advantage",
        name: "split_misp_warning_lists",
        run: split_misp_warning_lists,
    },
];
