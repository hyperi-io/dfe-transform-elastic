// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `splunk`'s two passes: a protocol given either as a name or as a number
//! answered on both ECS fields, and a saved search's own ECS fields lifted out
//! of the result and onto the document.

use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::painless_to_string;
use dfe_core::event::Event;

/// `splunk/alert`, processor `normalize_network_transport_and_iana_number`:
/// `network.iana_number` and `network.transport`.
///
/// The vendor sends `proto` as a number on some alerts and a name on others,
/// and the pipeline's params carry the IANA registry both ways round, so
/// either spelling fills both fields.
fn normalize_network_transport_and_iana_number(event: &mut Event, params: &Value) {
    if !event.has_value("network") {
        let _ = event.set("network", Value::Object(Map::new()));
    }

    let proto = event
        .get("splunk.alert.proto")
        .filter(|value| !value.is_null())
        .map(painless_to_string);
    if let Some(number) = proto {
        let name = lookup(params, "number_to_name", &number);
        let _ = event.set("network.iana_number", number);
        if let Some(name) = name {
            let _ = event.set("network.transport", name);
        }
    }

    let Some(transport) = event.get_string("network.transport") else {
        return;
    };
    let transport = transport.to_lowercase();
    event.update("network.transport", transport.clone());
    if !event.has_value("network.iana_number")
        && let Some(number) = lookup(params, "name_to_number", &transport)
    {
        let _ = event.set("network.iana_number", number);
    }
}

/// `splunk/search`, processor `distribute_expanded_fields`: every field the
/// search's own result carried under `splunk.search.ecs_result`, merged onto
/// the document and the staging field dropped.
///
/// A search can return ECS fields of its own, which the pipeline parses aside
/// and dot-expands before this hands them to the document. The merge is deep,
/// so a map meets a map and a list gains only the members it is missing;
/// anything else the result carries wins outright.
fn distribute_expanded_fields(event: &mut Event, _params: &Value) {
    let Some(Value::Object(expanded)) = event.get("splunk.search.ecs_result").cloned() else {
        return;
    };
    if let Some(document) = event.as_value_mut().as_object_mut() {
        merge_into(document, &expanded);
    }
    event.remove("splunk.search.ecs_result");
}

/// Merge `source` into `target`, recursing where both sides hold a map and
/// appending the unseen members where both hold a list.
fn merge_into(target: &mut Map<String, Value>, source: &Map<String, Value>) {
    for (key, value) in source {
        let deep = matches!(
            (value, target.get(key)),
            (Value::Object(_), Some(Value::Object(_))) | (Value::Array(_), Some(Value::Array(_)))
        );
        if !deep {
            target.insert(key.clone(), value.clone());
            continue;
        }
        match (value, target.get_mut(key)) {
            (Value::Object(nested), Some(Value::Object(held))) => merge_into(held, nested),
            (Value::Array(items), Some(Value::Array(held))) => {
                for item in items {
                    if !held.contains(item) {
                        held.push(item.clone());
                    }
                }
            }
            _ => {}
        }
    }
}

/// One entry of a params lookup table, absent where the table has no such key.
fn lookup(params: &Value, table: &str, key: &str) -> Option<Value> {
    params
        .get(table)?
        .get(key)
        .filter(|value| !value.is_null())
        .cloned()
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "034f4fd024a3e6d279ad95a792c312142110ebd560ee11a0887fe11b7d205134",
        source: "splunk",
        name: "normalize_network_transport_and_iana_number",
        run: normalize_network_transport_and_iana_number,
    },
    Entry {
        hash: "2b44cdbaaa4224bf4ed974717f9173dae994cd390e7bda9bb2a7637dca7eb4d9",
        source: "splunk",
        name: "distribute_expanded_fields",
        run: distribute_expanded_fields,
    },
];
