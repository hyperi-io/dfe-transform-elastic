// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `tychon`'s agent-output scripts, transcribed.
//!
//! The agent reports Windows values the way .NET prints them, so a file's
//! attribute list carries the enum's own members beside the strings, a link
//! speed arrives as "1 Gbps", and a certificate's parties hold single strings
//! where the mapping expects arrays.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The attribute list both streams prune.
const ATTRIBUTES: &str = "tychon.file.attributes";

/// Where the adapter's reported speed sits.
const LINK_SPEED: &str = "tychon.host.adapter.link_speed";

/// `tychon/systemcerts`, `script_remove_non_strings_from_file_attributes`:
/// the file attribute list with everything that is not a string dropped.
fn remove_non_strings(event: &mut Event, _params: &Value) {
    let Some(attributes) = event.get_array(ATTRIBUTES) else {
        return;
    };
    let kept: Vec<Value> = attributes
        .iter()
        .filter(|value| value.is_string())
        .cloned()
        .collect();
    let _ = event.set(ATTRIBUTES, Value::Array(kept));
}

/// `tychon/ciphers`, the same prune spelled as a `removeIf` over the list in
/// place.
fn remove_non_strings_in_place(event: &mut Event, params: &Value) {
    remove_non_strings(event, params);
}

/// `tychon/networkadapter`, the link-speed script: the adapter's reported
/// speed as bits per second.
///
/// The else branch writes a ZERO rather than leaving the field alone, so an
/// adapter that reports no speed still carries one.
fn link_speed_to_bits(event: &mut Event, _params: &Value) {
    let reported = event.get_string(LINK_SPEED);
    let speed = match reported.as_deref() {
        Some(text) if text.contains(' ') => {
            let mut parts = text.split(' ');
            let amount = parts.next().unwrap_or_default();
            let unit = parts.next().unwrap_or_default();
            // `Double.parseDouble` raises on anything else, which takes the
            // whole processor with it and leaves the field as it was.
            let Ok(amount) = amount.parse::<f64>() else {
                return;
            };
            match unit {
                "Kbps" => amount * 1e3,
                "Mbps" => amount * 1e6,
                "Gbps" => amount * 1e9,
                _ => amount,
            }
        }
        // Painless raises where `ctx.tychon` itself is absent, because the
        // else branch builds the maps under it rather than the whole path.
        _ if !event.has("tychon") => return,
        _ => 0.0,
    };
    #[allow(clippy::cast_possible_truncation)] // The script's own `(long)` cast.
    let _ = event.set(LINK_SPEED, speed as i64);
}

/// `tychon/systemcerts`, the x509 party script: each named certificate field
/// wrapped in a list, and dropped where the agent sent an empty string.
fn wrap_x509_party_fields(event: &mut Event, params: &Value) {
    let Some(parties) = params.get("party_names").and_then(Value::as_array) else {
        return;
    };
    let Some(fields) = params.get("field_names").and_then(Value::as_array) else {
        return;
    };
    if !event.has("tychon.x509") {
        return;
    }

    for party in parties.iter().filter_map(Value::as_str) {
        if !event.has_value(&format!("tychon.x509.{party}")) {
            continue;
        }
        for field in fields.iter().filter_map(Value::as_str) {
            let path = format!("tychon.x509.{party}.{field}");
            let Some(value) = event.get_str(&path).map(str::to_owned) else {
                continue;
            };
            if value.is_empty() {
                event.remove(&path);
            } else {
                let _ = event.set(&path, Value::Array(vec![Value::from(value)]));
            }
        }
    }
}

/// Every `tychon` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "876124fc95551302e2783d9931bc83a8d7890673c0e98face3b30ba1ef10f6d3",
        source: "tychon",
        name: "remove_non_strings",
        run: remove_non_strings,
    },
    Entry {
        hash: "336b1751d4a25649629a0ad13811e8d00bf0a9ac62ec2f6e5e1f91c4a91eaad5",
        source: "tychon",
        name: "remove_non_strings_in_place",
        run: remove_non_strings_in_place,
    },
    Entry {
        hash: "c8c1df0bb978c1f8e365506c429281ea1105a466772fc2ee3cbce0a76fe21e10",
        source: "tychon",
        name: "link_speed_to_bits",
        run: link_speed_to_bits,
    },
    Entry {
        hash: "1d88a383a4f792e064058a0dc70648068ec579e9f0779f803ae450a430d8df39",
        source: "tychon",
        name: "wrap_x509_party_fields",
        run: wrap_x509_party_fields,
    },
];
