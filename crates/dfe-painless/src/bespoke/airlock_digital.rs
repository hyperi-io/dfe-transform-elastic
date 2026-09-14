// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `airlock_digital`'s two `params` lookup scripts, transcribed.
//!
//! Both read a numeric code off the vendor payload and answer with the label
//! the processor's own `params` block carries for it -- an agent status and an
//! execution type. The code arrives as a JSON number, so the key is its Java
//! rendering rather than the value itself.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// `script_map_status_to_corresponding_agent_status_6ac9c74d` in the agent data
/// stream: `airlock_digital.agent.status_value` from the status code.
///
/// The script's own early return is null-or-empty-string, so a zero code still
/// reaches the lookup.
fn map_status(event: &mut Event, params: &Value) {
    let Some(label) = label_for(event, params, "json.status") else {
        return;
    };
    let _ = event.set("airlock_digital.agent.status_value", label);
}

/// `script_map_type_to_corresponding_execution_type_dfd09bea` in the
/// execution-histories data stream:
/// `airlock_digital.execution_histories.type_value` from the type code.
fn map_execution_type(event: &mut Event, params: &Value) {
    let Some(label) = label_for(event, params, "json.type") else {
        return;
    };
    let _ = event.set("airlock_digital.execution_histories.type_value", label);
}

/// The `params` entry for the code at `path`, rendered as Painless renders it.
///
/// A code the table does not carry answers null in Painless, and the pipeline's
/// own empty-value prune takes that straight back out -- so writing nothing is
/// the same document.
fn label_for(event: &Event, params: &Value, path: &str) -> Option<Value> {
    let code = event.get(path)?;
    if code.is_null() || code.as_str() == Some("") {
        return None;
    }
    params.get(painless_to_string(code)).cloned()
}

/// Every `airlock_digital` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "919ce8f7a5d3a2b4aefe2ac11fccc1f44476986fe388f1442d8322115b35d01d",
        source: "airlock_digital",
        name: "map_status",
        run: map_status,
    },
    Entry {
        hash: "22628de3aacf9f2ea07375e6e8f9e3456fd0bdc55b9e7be5d504b7ae85d7d773",
        source: "airlock_digital",
        name: "map_execution_type",
        run: map_execution_type,
    },
];
