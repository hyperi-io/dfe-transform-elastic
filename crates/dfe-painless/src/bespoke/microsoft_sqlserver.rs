// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `microsoft_sqlserver`'s audit scripts, transcribed.
//!
//! A SQL Server audit record names its action and its object class by
//! two-letter code -- `SL` for a select, `U` for a table -- so both are looked
//! up against the package's own tables and the ECS categorisation comes out of
//! the same row.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// The audit record's own subtree.
const AUDIT: &str = "sqlserver.audit";

/// `microsoft_sqlserver/audit`, the action-code script: the action and object
/// class spelled out, and the ECS action, type and category that follow.
///
/// An action code the table does not carry is named rather than dropped, and
/// it stops there -- no type, no category, no spelled-out action id.
fn resolve_action_codes(event: &mut Event, params: &Value) {
    let action_id = event.get_string(&format!("{AUDIT}.action_id"));

    if let Some(class_type) = event.get_string(&format!("{AUDIT}.class_type"))
        && let Some(spelled) = params
            .get("classtypes")
            .and_then(|table| table.get(&class_type))
    {
        let spelled = spelled.clone();
        event.update(&format!("{AUDIT}.class_type"), spelled);
    }

    let Some(action_id) = action_id else {
        return;
    };
    let Some(row) = params
        .get("actions")
        .and_then(|table| table.get(&action_id))
    else {
        let _ = event.set(
            "event.action",
            format!("unknown-{}", action_id.to_lowercase()),
        );
        let _ = event.set("event.type", Value::Array(vec![Value::from("info")]));
        return;
    };
    let row = row.clone();

    if let Some(value) = row.get("value") {
        event.update(&format!("{AUDIT}.action_id"), value.clone());
    }
    if let Some(types) = row.get("type") {
        let _ = event.set("event.type", types.clone());
    }
    // APPENDED, so the `database` the pipeline set ahead of this stays in
    // front of whatever the row adds.
    if let Some(categories) = row.get("category").and_then(Value::as_array) {
        for category in categories {
            let _ = event.append("event.category", category.clone());
        }
    }
    if let Some(action) = row.get("action") {
        let _ = event.set("event.action", action.clone());
    }
}

/// `microsoft_sqlserver/audit`, the duration script: the statement's own
/// elapsed time in nanoseconds, which is what ECS asks for.
fn duration_to_nanos(event: &mut Event, _params: &Value) {
    let Some(value) = event.get(&format!("{AUDIT}.duration_milliseconds")) else {
        return;
    };
    let Ok(milliseconds) = painless_to_string(value).parse::<i64>() else {
        return;
    };
    let _ = event.set("event.duration", milliseconds * 1_000_000);
}

/// Every `microsoft_sqlserver` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "686fe34d7a1700217abca6ee554bcf88d83fd48f3939561464e2db7afe76c5c1",
        source: "microsoft_sqlserver",
        name: "resolve_action_codes",
        run: resolve_action_codes,
    },
    Entry {
        hash: "f73e6e44dd51b513a7dc3c4d3622c94c7e6e1477af3f21682654198ea40c9a1d",
        source: "microsoft_sqlserver",
        name: "duration_to_nanos",
        run: duration_to_nanos,
    },
];
