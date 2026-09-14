// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `trendmicro`'s malware categorisation, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The Deep Security signatures the script calls malware.
const MALWARE_SIGNATURES: [i64; 10] = [
    4_000_000, 4_000_001, 4_000_002, 4_000_003, 4_000_010, 4_000_011, 4_000_012, 4_000_013,
    4_000_020, 4_000_030,
];

/// `script_to_set_ecs_categorization_in_malware_pipeline` in
/// `trendmicro/deep_security`: the ECS categorisation for a malware signature.
///
/// The script's own list holds Java `Long`s, which its `contains` would not
/// match against an `Integer` -- the pipeline's `convert_event_code` makes the
/// field a long first, so the comparison is numeric either way.
fn malware_categorisation(event: &mut Event, _params: &Value) {
    let Some(signature) = event.get_i64("trendmicro.deep_security.signature_id") else {
        return;
    };
    if !MALWARE_SIGNATURES.contains(&signature) {
        return;
    }
    // Painless raises on the assignment where the parent map is absent.
    if !event.has("event") {
        return;
    }
    let _ = event.set("event.category", Value::Array(vec![Value::from("malware")]));
    let _ = event.set("event.kind", Value::from("alert"));
    let _ = event.set("event.type", Value::Array(vec![Value::from("info")]));
}

/// Every trendmicro script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "ef24ecc47d8fb17205d912534a1bb32e4164cddbb284e833a1e332a6d6da7a3b",
    source: "trendmicro",
    name: "malware_categorisation",
    run: malware_categorisation,
}];
