// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `azure_network_watcher_vnet`'s flow tuples, transcribed.
//!
//! The virtual-network flow log is the security-group log's younger sibling:
//! the same thirteen comma-separated fields under a different nesting, with the
//! protocol left as its IANA number and an encryption column where the security
//! group reports its allow-or-deny decision. The three readers it shares with
//! that log live beside it.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use super::azure_network_watcher_nsg::{code, take, write_set};

/// Where the parsed log holds its flows.
const FLOWS: &str = "json.flowRecords.flows";

/// How many fields one flow line carries. A line of any other width is skipped.
const TUPLE_WIDTH: usize = 13;

/// What one batch of flow lines contributed to the ECS fields.
#[derive(Default)]
struct Gathered {
    source_ip: Vec<Value>,
    destination_ip: Vec<Value>,
    source_port: Vec<Value>,
    destination_port: Vec<Value>,
    source_packets: Vec<Value>,
    source_bytes: Vec<Value>,
    destination_packets: Vec<Value>,
    destination_bytes: Vec<Value>,
    direction: Vec<Value>,
    iana: Vec<Value>,
}

/// `azure_network_watcher_vnet/log`, the untagged flow script in `default`: the
/// parsed `tuples` and the ECS fields the lines fill.
fn flow_tuples(event: &mut Event, _params: &Value) {
    let Some(Value::Array(mut flows)) = event.get(FLOWS).cloned() else {
        return;
    };

    let mut gathered = Gathered::default();
    for flow in &mut flows {
        let Some(flow) = flow.as_object_mut() else {
            continue;
        };
        let Some(Value::Array(groups)) = flow.get_mut("flowGroups") else {
            continue;
        };
        for group in groups {
            let Some(group) = group.as_object_mut() else {
                continue;
            };
            rewrite_tuples(group, &mut gathered);
        }
    }
    event.update(FLOWS, Value::Array(flows));

    for root in ["destination", "source", "network"] {
        if !event.has_value(root) {
            let _ = event.set(root, Value::Object(Map::new()));
        }
    }
    write_set(event, "destination.packets", gathered.destination_packets);
    write_set(event, "destination.bytes", gathered.destination_bytes);
    write_set(event, "source.packets", gathered.source_packets);
    write_set(event, "source.bytes", gathered.source_bytes);
    write_set(event, "source.port", gathered.source_port);
    write_set(event, "destination.port", gathered.destination_port);
    write_set(event, "source.ip", gathered.source_ip);
    write_set(event, "destination.ip", gathered.destination_ip);
    write_set(event, "network.direction", gathered.direction);
    write_set(event, "network.iana_number", gathered.iana);
}

/// One flow group's raw lines parsed into `tuples`, the raw list taken out.
fn rewrite_tuples(group: &mut Map<String, Value>, gathered: &mut Gathered) {
    let lines = match group.get("flowTuples") {
        Some(Value::Array(lines)) => lines.clone(),
        _ => Vec::new(),
    };

    let mut tuples = Vec::with_capacity(lines.len());
    for line in &lines {
        let Some(line) = line.as_str() else {
            continue;
        };
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != TUPLE_WIDTH {
            continue;
        }
        tuples.push(one_tuple(&parts, gathered));
    }

    group.insert("tuples".to_owned(), Value::Array(tuples));
    // `shift_remove`, never `remove`: the plain one reorders under
    // `preserve_order`.
    group.shift_remove("flowTuples");
}

/// One flow line as the map the script builds, gathering as it goes.
fn one_tuple(parts: &[&str], gathered: &mut Gathered) -> Value {
    let mut flow = Map::new();
    let mut source = Map::new();
    let mut destination = Map::new();
    let mut bytes = Map::new();
    let mut packets = Map::new();
    let mut tuple = Map::new();

    if !parts[0].is_empty() {
        tuple.insert("timestamp".to_owned(), Value::from(parts[0]));
    }
    take(parts[1], &mut gathered.source_ip, &mut source, "ip");
    take(
        parts[2],
        &mut gathered.destination_ip,
        &mut destination,
        "ip",
    );
    take(parts[3], &mut gathered.source_port, &mut source, "port");
    take(
        parts[4],
        &mut gathered.destination_port,
        &mut destination,
        "port",
    );
    take(parts[5], &mut gathered.iana, &mut tuple, "protocol");

    let direction = code(parts[6], &[('I', "Inbound"), ('O', "Outbound")]);
    if !parts[6].is_empty() {
        gathered.direction.push(Value::from(direction.clone()));
        flow.insert("direction".to_owned(), Value::from(direction));
    }

    let state = code(
        parts[7],
        &[
            ('B', "Begin"),
            ('C', "Continuing"),
            ('E', "End"),
            ('D', "Deny"),
        ],
    );
    if !parts[7].is_empty() {
        flow.insert("state".to_owned(), Value::from(state));
    }

    if !parts[8].is_empty() {
        flow.insert("encryption".to_owned(), Value::from(parts[8]));
    }

    take(parts[9], &mut gathered.source_packets, &mut packets, "sent");
    take(parts[10], &mut gathered.source_bytes, &mut bytes, "sent");
    take(
        parts[11],
        &mut gathered.destination_packets,
        &mut packets,
        "received",
    );
    take(
        parts[12],
        &mut gathered.destination_bytes,
        &mut bytes,
        "received",
    );

    let mut whole = Map::with_capacity(tuple.len() + 5);
    whole.insert("flow".to_owned(), Value::Object(flow));
    whole.insert("source".to_owned(), Value::Object(source));
    whole.insert("destination".to_owned(), Value::Object(destination));
    whole.insert("bytes".to_owned(), Value::Object(bytes));
    whole.insert("packets".to_owned(), Value::Object(packets));
    for (key, value) in tuple {
        whole.insert(key, value);
    }
    Value::Object(whole)
}

/// Every `azure_network_watcher_vnet` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "56ee775ebf8a06b2180f32fea3e22e1716f363a41cd89941d5d5f2069dd60c28",
    source: "azure_network_watcher_vnet",
    name: "flow_tuples",
    run: flow_tuples,
}];
