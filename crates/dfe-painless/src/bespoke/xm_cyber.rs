// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `xm_cyber`'s device and entity-inventory passes: the CVEs collected off an
//! asset's applications, and the two address lists parted by element type.

use std::collections::BTreeSet;

use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::painless_to_string;
use dfe_core::event::Event;

/// `xm_cyber/device`, processor `script_vulnerability_id_from_active_cves`:
/// `vulnerability.id` and `vulnerability.enumeration`.
///
/// The script collects into a `TreeSet`, so the identifiers come out sorted
/// and deduplicated however the applications listed them.
fn vulnerability_id_from_active_cves(event: &mut Event, _params: &Value) {
    let Some(apps) = event.get_array("xm_cyber.device.apps") else {
        return;
    };
    let mut cves: BTreeSet<String> = BTreeSet::new();
    for app in apps {
        if !app.is_object() {
            continue;
        }
        let Some(listed) = app.get("active_cves").and_then(Value::as_array) else {
            continue;
        };
        for cve in listed {
            if cve.is_null() || cve.as_str() == Some("") {
                continue;
            }
            cves.insert(painless_to_string(cve));
        }
    }
    if cves.is_empty() {
        return;
    }
    let ids: Vec<Value> = cves.into_iter().map(Value::from).collect();
    if !event.has_value("vulnerability") {
        let _ = event.set("vulnerability", Value::Object(Map::new()));
    }
    let _ = event.set("vulnerability.id", Value::Array(ids));
    let _ = event.set("vulnerability.enumeration", "CVE");
}

/// `xm_cyber/entity_inventory`, processor
/// `script_split_xm_cyber_entity_inventory_ipv4_by_shape`:
/// `xm_cyber.entity_inventory.ipv4` and `...ipv4_buffer`.
fn split_ipv4_by_type(event: &mut Event, _params: &Value) {
    part_addresses(
        event,
        "xm_cyber.entity_inventory.ipv4",
        "xm_cyber.entity_inventory.ipv4_buffer",
    );
}

/// `xm_cyber/entity_inventory`, processor
/// `script_split_xm_cyber_entity_inventory_ipv6_by_shape`:
/// `xm_cyber.entity_inventory.ipv6` and `...ipv6_buffer`.
fn split_ipv6_by_type(event: &mut Event, _params: &Value) {
    part_addresses(
        event,
        "xm_cyber.entity_inventory.ipv6",
        "xm_cyber.entity_inventory.ipv6_buffer",
    );
}

/// Part an address list by element type, so each half lands where the mapping
/// can hold it.
///
/// The vendor mixes encoded strings with `{"data": [...], "type": "Buffer"}`
/// objects in one list. The strings stay on the keyword field and the objects
/// move to its `_buffer` sibling; a list of nothing but objects leaves the
/// keyword field removed rather than empty.
fn part_addresses(event: &mut Event, addresses: &str, buffers: &str) {
    let Some(listed) = event.get_array(addresses).cloned() else {
        return;
    };
    let mut strings = Vec::new();
    let mut objects = Vec::new();
    for value in listed {
        if value.is_null() {
            continue;
        }
        if value.is_string() {
            strings.push(value);
        } else if value.is_object() {
            objects.push(value);
        }
    }
    if strings.is_empty() {
        event.remove(addresses);
    } else {
        event.update(addresses, Value::Array(strings));
    }
    if !objects.is_empty() {
        let _ = event.set(buffers, Value::Array(objects));
    }
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "f43c17d4c7a0a150a15d51f28d536cf186907f89d6c97c397d86a7114630a4b9",
        source: "xm_cyber",
        name: "vulnerability_id_from_active_cves",
        run: vulnerability_id_from_active_cves,
    },
    Entry {
        hash: "97d9d6f8238823f06bcd00b365c82c19a5ae32e7872f043e5709dacd9e5fbe52",
        source: "xm_cyber",
        name: "split_ipv4_by_type",
        run: split_ipv4_by_type,
    },
    Entry {
        hash: "17132318f4c4b1feeab202985b0801048b89c8f2402dd0c1407d12d514828089",
        source: "xm_cyber",
        name: "split_ipv6_by_type",
        run: split_ipv6_by_type,
    },
];
