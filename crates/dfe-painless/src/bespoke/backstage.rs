// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `backstage`'s scaffolder metadata, kept as text because its members are
//! whatever the operator's own software template declared.

use serde_json::Value;

use super::Entry;
use crate::helpers::java_to_string;
use dfe_core::event::Event;

/// `backstage/logs`, processor `serialize_task_parameters`: the task's input
/// parameters under `backstage.task.parameters`.
///
/// Java's own `toString`, so the members walk the hash table rather than the
/// document and the rendering carries no quotes.
fn serialize_task_parameters(event: &mut Event, _params: &Value) {
    let rendered = match event.get("json.meta.taskParameters") {
        Some(parameters) => java_to_string(parameters),
        None => return,
    };
    let _ = event.set("backstage.task.parameters", rendered);
}

/// `backstage/logs`, processor `serialize_meta_residual`: whatever `json.meta`
/// still holds once the mapped keys have been taken out of it.
///
/// Nothing is left there for any event the vendor ships today, so this writes
/// only where Backstage has added a plugin key we do not map yet.
fn serialize_meta_residual(event: &mut Event, _params: &Value) {
    let rendered = match event.get("json.meta") {
        Some(meta) => java_to_string(meta),
        None => return,
    };
    let _ = event.set("backstage.meta", rendered);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "7fc57527579530bfa0ff5ef83022caf4c73f2333131f31372b5924a86ffe31ef",
        source: "backstage",
        name: "serialize_task_parameters",
        run: serialize_task_parameters,
    },
    Entry {
        hash: "0a823edf4885a0b80c7b971e30508d354afebf1679caebd3517f68d4003fc808",
        source: "backstage",
        name: "serialize_meta_residual",
        run: serialize_meta_residual,
    },
];
