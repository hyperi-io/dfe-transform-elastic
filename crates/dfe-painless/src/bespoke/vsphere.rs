// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The two host utilisation ratios `vsphere` computes, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_f64;

/// The untagged script in `vsphere/host`: `cpu.used.pct` from the used and
/// total clock rates.
fn cpu_used_pct(event: &mut Event, _params: &Value) {
    ratio(
        event,
        "vsphere.host.cpu.used.mhz",
        "vsphere.host.cpu.total.mhz",
        "vsphere.host.cpu.used.pct",
    );
}

/// The untagged script in `vsphere/host`: `memory.used.pct` from the used and
/// total byte counts.
fn memory_used_pct(event: &mut Event, _params: &Value) {
    ratio(
        event,
        "vsphere.host.memory.used.bytes",
        "vsphere.host.memory.total.bytes",
        "vsphere.host.memory.used.pct",
    );
}

/// The division both scripts spell, in Java's 32-bit `float`.
///
/// The quotient is published as the shortest decimal that reads back as the
/// same `float`, because widening the single to a double instead prints its
/// binary tail and 0.014603313 arrives as 0.014603313025832176.
#[allow(clippy::cast_possible_truncation)] // The script declares both operands and the quotient as Java `float`.
fn ratio(event: &mut Event, used: &str, total: &str, target: &str) {
    let Some(used) = event.get(used).map(painless_to_f64) else {
        return;
    };
    let Some(total) = event.get(total).map(painless_to_f64) else {
        return;
    };
    let quotient = used as f32 / total as f32;
    let Ok(rounded) = format!("{quotient}").parse::<f64>() else {
        return;
    };
    let _ = event.set(target, rounded);
}

/// Every `vsphere` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "d67a7eb6c4e28b369f83d828f296e77959500b0a0a4e201d7c6dd058bd12dce5",
        source: "vsphere",
        name: "cpu_used_pct",
        run: cpu_used_pct,
    },
    Entry {
        hash: "4104c9bd9e63fc91ff3fb49b403eebdc4e9ea521e31114dcd7342d5fb09ad3cf",
        source: "vsphere",
        name: "memory_used_pct",
        run: memory_used_pct,
    },
];
