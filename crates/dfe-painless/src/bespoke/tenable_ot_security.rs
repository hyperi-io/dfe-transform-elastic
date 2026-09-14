// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `tenable_ot_security`'s GraphQL unwrapping, plus the MAC spelling an event
//! is normalised to.
//!
//! The vendor answers a GraphQL query, so every collection arrives wrapped in a
//! `nodes` member. Both data streams walk their own subtree and lift each
//! wrapper's contents into the key that held it.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The events subtree both event scripts work over.
const EVENTS: &str = "tenable_ot_security.events";

/// The assets subtree.
const ASSETS: &str = "tenable_ot_security.assets";

/// The addresses the last event script rewrites, in the order it takes them.
const MAC_FIELDS: [&str; 3] = ["dst_mac", "src_mac", "src_interface.mac"];

/// The untagged normaliser in `tenable_ot_security/events`: every `nodes`
/// wrapper under the event replaced by what it wrapped.
fn normalise_event_nodes(event: &mut Event, _params: &Value) {
    unwrap_nodes(event, EVENTS);
}

/// The untagged normaliser in `tenable_ot_security/assets`: the same, over the
/// asset record.
fn normalise_asset_nodes(event: &mut Event, _params: &Value) {
    unwrap_nodes(event, ASSETS);
}

/// The untagged MAC normaliser in `tenable_ot_security/events`: the vendor's
/// colon-separated, mixed-case addresses in the dash-separated upper case ECS
/// asks for.
fn normalise_event_macs(event: &mut Event, _params: &Value) {
    if !event.has_value(EVENTS) {
        return;
    }
    for tail in MAC_FIELDS {
        let path = format!("{EVENTS}.{tail}");
        let Some(held) = event.get(&path) else {
            continue;
        };
        if held.is_null() {
            continue;
        }
        // The script calls `replace` straight on the value, so anything but a
        // string raises there and nothing after it is rewritten.
        let Some(address) = held.as_str() else {
            return;
        };
        let normalised = address.replace(':', "-").to_uppercase();
        let _ = event.update(&path, normalised);
    }
}

/// Rewrite one subtree, where the document carries it.
fn unwrap_nodes(event: &mut Event, path: &str) {
    let Some(subtree) = event.get(path) else {
        return;
    };
    let normalised = normalise(subtree);
    let _ = event.update(path, normalised);
}

/// One value with every member that is a `nodes` wrapper replaced by its
/// contents.
///
/// The lift happens as the PARENT is walked, so a wrapper the lift itself
/// exposes is left alone -- the recursion goes into the contents, not back over
/// the key that held them.
fn normalise(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut rebuilt = Map::with_capacity(map.len());
            for (key, held) in map {
                let lifted = match held {
                    Value::Object(wrapper) => wrapper.get("nodes").unwrap_or(held),
                    other => other,
                };
                rebuilt.insert(key.clone(), normalise(lifted));
            }
            Value::Object(rebuilt)
        }
        Value::Array(items) => Value::Array(items.iter().map(normalise).collect()),
        other => other.clone(),
    }
}

/// Every `tenable_ot_security` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "befb894b8b5e7247fa7deaa56f88b2e6c1fe329936b3eea63505cfae492fbcc6",
        source: "tenable_ot_security",
        name: "normalise_event_nodes",
        run: normalise_event_nodes,
    },
    Entry {
        hash: "3b645054334881015da8b7f6d2b3960ac454cdc0069924d4503f8b3f50d72c1a",
        source: "tenable_ot_security",
        name: "normalise_asset_nodes",
        run: normalise_asset_nodes,
    },
    Entry {
        hash: "76dd0c7cb40589cd342d700663d0db0fd7d41374029f1254c65322a0765d65c0",
        source: "tenable_ot_security",
        name: "normalise_event_macs",
        run: normalise_event_macs,
    },
];
