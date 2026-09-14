// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `azure_network_watcher_nsg`'s flow tuples, transcribed.
//!
//! A security-group flow log carries each connection as one comma-separated
//! line of thirteen fields, nested two deep under the rule that matched it. The
//! script reads every line into a map, replaces the raw `flowTuples` list with
//! the parsed `tuples`, and gathers the addresses, ports and counters it saw
//! into the ECS fields beside them.
//!
//! The single-letter codes are matched with `contains` rather than equality,
//! which is the vendor's own reading and is kept.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::java_set_order;

/// Where the parsed log holds its flows.
const FLOWS: &str = "json.properties.flows";

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
    transport: Vec<Value>,
}

/// `azure_network_watcher_nsg/log`, the untagged flow script in `default`: the
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
        let Some(Value::Array(inner_flows)) = flow.get_mut("flows") else {
            continue;
        };
        for inner in inner_flows {
            let Some(inner) = inner.as_object_mut() else {
                continue;
            };
            rewrite_tuples(inner, &mut gathered);
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
    write_set(event, "network.transport", gathered.transport);
}

/// One inner flow's raw lines parsed into `tuples`, the raw list taken out.
fn rewrite_tuples(inner: &mut Map<String, Value>, gathered: &mut Gathered) {
    // `shift_remove`, never `remove`: the plain one reorders under
    // `preserve_order`. Removing BEFORE the walk hands the lines over instead
    // of copying them, and leaves the same key order -- `tuples` is appended
    // at the end whichever way round the two run.
    let lines = match inner.shift_remove("flowTuples") {
        Some(Value::Array(lines)) => lines,
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

    inner.insert("tuples".to_owned(), Value::Array(tuples));
}

/// One flow line as the map the script builds, gathering as it goes.
fn one_tuple(parts: &[&str], gathered: &mut Gathered) -> Value {
    let mut traffic = Map::new();
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

    let protocol = code(parts[5], &[('T', "TCP"), ('U', "UDP")]);
    if !parts[5].is_empty() {
        gathered.transport.push(Value::from(protocol.clone()));
        tuple.insert("protocol".to_owned(), Value::from(protocol));
    }

    let flow = code(parts[6], &[('I', "Inbound"), ('O', "Outbound")]);
    if !parts[6].is_empty() {
        gathered.direction.push(Value::from(flow.clone()));
        traffic.insert("flow".to_owned(), Value::from(flow));
    }

    let decision = code(parts[7], &[('A', "Allowed"), ('D', "Denied")]);
    if !parts[7].is_empty() {
        traffic.insert("decision".to_owned(), Value::from(decision));
    }

    let state = code(
        parts[8],
        &[('B', "Begin"), ('C', "Continuing"), ('E', "End")],
    );
    if !parts[8].is_empty() {
        tuple.insert("flow_state".to_owned(), Value::from(state));
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
    whole.insert("traffic".to_owned(), Value::Object(traffic));
    whole.insert("source".to_owned(), Value::Object(source));
    whole.insert("destination".to_owned(), Value::Object(destination));
    whole.insert("bytes".to_owned(), Value::Object(bytes));
    whole.insert("packets".to_owned(), Value::Object(packets));
    for (key, value) in tuple {
        whole.insert(key, value);
    }
    Value::Object(whole)
}

/// One non-empty field gathered and stored under `name`.
///
/// Shared with the vnet log, which ships the same reader over its own tuple.
pub(super) fn take(
    part: &str,
    gathered: &mut Vec<Value>,
    into: &mut Map<String, Value>,
    name: &str,
) {
    if part.is_empty() {
        return;
    }
    gathered.push(Value::from(part));
    into.insert(name.to_owned(), Value::from(part));
}

/// One coded field spelled out, or left as it was where no code matches.
///
/// Shared with the vnet log, whose codes differ but whose reading does not.
pub(super) fn code(part: &str, codes: &[(char, &str)]) -> String {
    for (letter, spelled) in codes {
        if part.contains(*letter) {
            return (*spelled).to_owned();
        }
    }
    part.to_owned()
}

/// One gathered list written where the pipeline has not already filled it.
///
/// Shared with the vnet log, which writes the same ten fields bar one.
pub(super) fn write_set(event: &mut Event, path: &str, gathered: Vec<Value>) {
    if event.has_value(path) {
        return;
    }
    let mut distinct: Vec<Value> = Vec::with_capacity(gathered.len());
    for member in gathered {
        if !distinct.contains(&member) {
            distinct.push(member);
        }
    }
    let _ = event.set(path, Value::Array(java_set_order(distinct)));
}

/// Every `azure_network_watcher_nsg` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "b6ae82ad3b6a4922b361a502ec931d5db8e9ca6fd67567c4514303ccb23dff3c",
    source: "azure_network_watcher_nsg",
    name: "flow_tuples",
    run: flow_tuples,
}];
