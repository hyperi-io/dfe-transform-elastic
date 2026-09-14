// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `checkpoint`'s dropped-packet tuples and its protocol ladder, transcribed.

use serde_json::{Map, Value, json};

use dfe_core::Event;

use super::Entry;

/// `script_parse_checkpoint_packets_dropped` in `checkpoint/firewall`: the
/// `<src,sport,dst,dport,proto;iface>` tuples the device lists for a drop.
///
/// A gateway under load reports a SAMPLE rather than every packet and says so
/// in a prefix, which becomes a flag of its own.
fn packets_dropped_from_packets(event: &mut Event, _params: &Value) {
    let Some(raw) = event.get_string("checkpoint.packets") else {
        return;
    };
    let mut text = raw.trim();
    if text.starts_with("(sample")
        && let Some(close) = text.find(')')
        && close > 0
    {
        let _ = event.set("checkpoint.packets_data_is_sampled", true);
        text = text[close + 1..].trim();
    }
    if let Some(head) = text.strip_suffix("\";") {
        text = head;
    }

    let mut parsed = Vec::new();
    for entry in text.split('>') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let entry = entry.strip_prefix('<').unwrap_or(entry);
        let mut parts = entry.split(';');
        let tuple = parts.next().unwrap_or_default();
        let interface = parts.next();
        let fields: Vec<&str> = tuple.split(',').collect();
        if fields.len() < 5 {
            continue;
        }
        // `Long.parseLong` raises on anything else, which takes the whole
        // script down and leaves the tuples where they were.
        let (Ok(source_port), Ok(destination_port)) =
            (fields[1].parse::<i64>(), fields[3].parse::<i64>())
        else {
            return;
        };
        let mut packet = Map::with_capacity(4);
        if let Some(name) = interface {
            packet.insert("interface".to_string(), json!({ "name": name }));
        }
        packet.insert(
            "source".to_string(),
            json!({ "ip": fields[0], "port": source_port }),
        );
        packet.insert(
            "destination".to_string(),
            json!({ "ip": fields[2], "port": destination_port }),
        );
        packet.insert("network".to_string(), json!({ "iana_number": fields[4] }));
        parsed.push(Value::Object(packet));
    }

    if parsed.is_empty() {
        return;
    }
    let _ = event.set("checkpoint.packets_dropped", Value::Array(parsed));
    event.remove("checkpoint.packets");
}

/// `script_d5029733` in `checkpoint/firewall`: `network.transport` from the
/// IANA protocol number.
///
/// Protocol 0 is hop-by-hop options over IPv6 and reserved over IPv4, so the
/// first arm decides on the source address rather than the number alone.
fn transport_from_iana_number(event: &mut Event, _params: &Value) {
    let held = match event.get("network.iana_number") {
        Some(value) => value.clone(),
        None => return,
    };
    let Some(number) = held.as_str().map(str::to_owned) else {
        // A number matches none of the string literals, so it reaches the
        // closing arm and is copied across unchanged.
        let _ = event.set("network.transport", held);
        return;
    };
    let ipv6_source = event
        .get_str("source.ip")
        .is_some_and(|ip| ip.contains(':'));
    let transport = match number.as_str() {
        "0" if ipv6_source => "hopopt",
        "1" => "icmp",
        "2" => "igmp",
        "6" => "tcp",
        "8" => "egp",
        "17" => "udp",
        "47" => "gre",
        "50" => "esp",
        "58" => "ipv6-icmp",
        "112" => "vrrp",
        "114" => "0-hop",
        "132" => "sctp",
        // The device's own "no protocol" sentinel, which the script drops
        // rather than copying across.
        "4294967295" => return,
        other => other,
    };
    let transport = transport.to_string();
    let _ = event.set("network.transport", transport);
}

/// Every checkpoint script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "33d6ddb34a78692a6890108f2e0e945eb7ae4c18872230fc5794606007eb1c8f",
        source: "checkpoint",
        name: "packets_dropped_from_packets",
        run: packets_dropped_from_packets,
    },
    Entry {
        hash: "0cbe4e1a13349b4d9640df832fd1206d7ccbb27c76619e05db8b3a7080dfd55a",
        source: "checkpoint",
        name: "transport_from_iana_number",
        run: transport_from_iana_number,
    },
];
