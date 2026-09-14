// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `swimlane`'s source-address unwrapping, transcribed.
//!
//! The audit API sends the client address as a one-element list, and the grok
//! that turns it into `source.ip` reads a string. Each data stream ships the
//! same script under its own spelling of the field -- `sourceIp` in
//! `audit_logs`, `SourceIp` in `turbine_api`.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `swimlane/audit_logs`, the untagged unwrap script in `default`.
fn unwrap_source_ip_lower(event: &mut Event, _params: &Value) {
    unwrap_first(event, "swimlane.audit_log.sourceIp");
}

/// `swimlane/turbine_api`, the untagged unwrap script in `default`.
fn unwrap_source_ip_upper(event: &mut Event, _params: &Value) {
    unwrap_first(event, "swimlane.audit_log.SourceIp");
}

/// The field left holding its first element where it is a non-empty list, and
/// left alone otherwise.
fn unwrap_first(event: &mut Event, path: &str) {
    let Some(first) = (match event.get(path) {
        Some(Value::Array(items)) => items.first().cloned(),
        _ => None,
    }) else {
        return;
    };
    event.update(path, first);
}

/// Every `swimlane` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "ae92e77b6a0d8b047a0ae9a2276f2aa0fc36622db9bac933332e4a4d48a5c28d",
        source: "swimlane",
        name: "unwrap_source_ip_lower",
        run: unwrap_source_ip_lower,
    },
    Entry {
        hash: "390696d151fb983aac51c570b6a74a077782fadf7cf7f1519a540da1f2085dab",
        source: "swimlane",
        name: "unwrap_source_ip_upper",
        run: unwrap_source_ip_upper,
    },
];
