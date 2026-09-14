// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_recordedfuture`'s entity collection and its targets rename.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The alert entities the first script walks.
const ENTITIES: &str = "recordedfuture.triggered_alert.entities";

/// The panel targets the second script reads.
const TARGETS: &str = "recordedfuture.playbook_alert.panel_status.targets";

/// Where a string-valued targets list is moved to.
const TARGETS_STR: &str = "recordedfuture.playbook_alert.panel_status.targets_str";

/// `script_to_set_related_ip_with_entity_name` in
/// `ti_recordedfuture/triggered_alert`: `related.ip` from the alert's entities.
///
/// A name holding a slash is a CIDR range rather than an address, so it is left
/// out -- `related.ip` is an address list.
fn related_ip_from_entities(event: &mut Event, _params: &Value) {
    let addresses = {
        let Some(entities) = event.get_array(ENTITIES) else {
            return;
        };
        let mut addresses = Vec::new();
        for entity in entities {
            let kind = entity.get("type").and_then(Value::as_str);
            let name = entity.get("name").and_then(Value::as_str);
            let (Some(kind), Some(name)) = (kind, name) else {
                continue;
            };
            if kind.to_lowercase() == "ipaddress" && !name.contains('/') {
                addresses.push(Value::String(name.to_string()));
            }
        }
        addresses
    };
    let _ = event.set("related.ip", Value::Array(addresses));
}

/// `set_panel_status_target_str` in `ti_recordedfuture/playbook_alert`:
/// `panel_status.targets` moved to `targets_str` where it holds strings.
///
/// The vendor sends either objects or strings under the one name, and the
/// mapping declares an object -- so the string form takes a name of its own.
fn panel_status_targets_to_str(event: &mut Event, _params: &Value) {
    let strings = event
        .get_array(TARGETS)
        .is_some_and(|targets| targets.first().is_some_and(Value::is_string));
    if !strings {
        return;
    }
    let Some(targets) = event.take_array(TARGETS) else {
        return;
    };
    let _ = event.set(TARGETS_STR, Value::Array(targets));
    event.remove(TARGETS);
}

/// Every `ti_recordedfuture` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "0f5e891702f537498da88922a6a78010e1d1bbd6279150b14d5d533f8cc3a716",
        source: "ti_recordedfuture",
        name: "related_ip_from_entities",
        run: related_ip_from_entities,
    },
    Entry {
        hash: "e684b2aeb8d47dffcf615053988cd6a03236f7da40a47e479c7b83d6bb37c8df",
        source: "ti_recordedfuture",
        name: "panel_status_targets_to_str",
        run: panel_status_targets_to_str,
    },
];
