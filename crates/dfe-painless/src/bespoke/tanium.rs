// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `tanium`'s threat-response detail: the vendor's `Match Details` object
//! merged onto the namespace, and the document cut where it runs deeper than
//! the mapping can hold.
//!
//! The alert carries a process ancestry that nests without bound, so the
//! vendor's own script drops any map past twenty levels and records the path
//! it dropped. The closing prune then takes the null it left behind.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// How deep the document may run before a map is dropped.
const MAX_DEPTH: usize = 20;

/// `script_handle_match_details_content_the_same_as_when_it_appears_in_an_encoded_payload`
/// in `compat-tanium-threat_response-default`.
fn merge_match_details(event: &mut Event, _params: &Value) {
    const TARGET: &str = "tanium.threat_response.match_details";

    let Some(source) = event.get_object("json.Match Details").cloned() else {
        return;
    };
    let mut merged = event.get_object(TARGET).cloned().unwrap_or_default();
    for (key, value) in source {
        merged.insert(key, value);
    }

    let _ = event.set(TARGET, Value::Object(merged));
}

/// `script_truncate_at_a_maximum_depth` in
/// `compat-tanium-threat_response-default`.
fn truncate_at_max_depth(event: &mut Event, _params: &Value) {
    let mut truncations: Vec<Value> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    if let Value::Object(root) = event.as_value_mut() {
        truncate_map(root, 1, &mut path, &mut truncations);
    }

    let _ = event.set("tanium.truncations", Value::Array(truncations));
}

/// Walk one map, cutting whatever runs deeper than [`MAX_DEPTH`].
///
/// False where this node is itself past the depth, which is the caller's
/// signal to null the member holding it.
fn truncate_map(
    node: &mut Map<String, Value>,
    depth: usize,
    path: &mut Vec<String>,
    truncations: &mut Vec<Value>,
) -> bool {
    if depth > MAX_DEPTH {
        truncations.push(Value::from(path.join(".")));
        return false;
    }

    for (key, value) in node.iter_mut() {
        path.push(key.clone());
        let cut = match value {
            Value::Object(child) => !truncate_map(child, depth + 1, path, truncations),
            // A list in a map does not add depth -- several values are
            // treated the same as one.
            Value::Array(items) => !truncate_list(items, depth, path, truncations),
            _ => false,
        };
        if cut {
            *value = Value::Null;
        }
        path.pop();
    }

    true
}

/// The same walk over a list, which is emptied before the depth is even
/// tested -- so a list past it is left empty AND nulled by its holder.
fn truncate_list(
    node: &mut Vec<Value>,
    depth: usize,
    path: &mut Vec<String>,
    truncations: &mut Vec<Value>,
) -> bool {
    let input = std::mem::take(node);
    if depth > MAX_DEPTH {
        truncations.push(Value::from(path.join(".")));
        return false;
    }

    for mut item in input {
        let cut = match &mut item {
            Value::Object(child) => !truncate_map(child, depth + 1, path, truncations),
            Value::Array(items) => !truncate_list(items, depth + 1, path, truncations),
            _ => false,
        };
        node.push(if cut { Value::Null } else { item });
    }

    true
}

/// Every `tanium` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "4c840404186829ef951c682ffe28648e62cd743dca298dbbc77b7213a521d429",
        source: "tanium",
        name: "merge_match_details",
        run: merge_match_details,
    },
    Entry {
        hash: "00c5e190a09cf3e65145d131872b50bd778c7fd8818c14d347cb7316c6c36405",
        source: "tanium",
        name: "truncate_at_max_depth",
        run: truncate_at_max_depth,
    },
];
