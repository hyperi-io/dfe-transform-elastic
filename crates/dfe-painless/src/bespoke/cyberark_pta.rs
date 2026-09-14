// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cyberark_pta`'s single-element wrap of the syslog observer address.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The untagged script in `cyberark_pta/events`: `observer.ip` wrapped in a
/// one-element list.
///
/// The syslog header grok writes the address as a string and the mapping wants
/// the list form every other arm of that pipeline produces.
fn observer_ip_as_list(event: &mut Event, _params: &Value) {
    let Some(held) = event.get("observer.ip").cloned() else {
        return;
    };
    let _ = event.update("observer.ip", Value::Array(vec![held]));
}

/// Every `cyberark_pta` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "df53f3fdec838ed3b00105132a0d6f48bc6b03fe19e47460cf57b4824df0e6fd",
    source: "cyberark_pta",
    name: "observer_ip_as_list",
    run: observer_ip_as_list,
}];
