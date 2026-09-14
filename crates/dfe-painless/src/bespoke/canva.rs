// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `canva`'s audit-log normalising, transcribed.
//!
//! Two jobs, both reading the action the audit event names. One turns the
//! vendor's upper-case action name into an ECS `event.type`; the other walks
//! the list of changes and reads the permission flags the API sends as the
//! strings `"true"` and `"false"` back into real booleans.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The list of changes the audit event carries.
const CHANGES: &str = "json.action.changes";

/// The members whose value is a flag rather than a name.
const FLAGS: [&str; 3] = ["read", "write", "owning_team_only"];

/// The members that hold a map of flags.
const FLAG_MAPS: [&str; 5] = [
    "access",
    "new_access",
    "new_link_role",
    "old_access",
    "old_link_role",
];

/// `canva/audit`, the untagged event-type script in `default`: `event.type`,
/// appended to whatever the pipeline has already put there.
fn event_type_from_action(event: &mut Event, _params: &Value) {
    let Some(action) = event.get_string("json.action.type") else {
        return;
    };
    let action = action.to_lowercase();
    let kind = if action.contains("remove") || action.contains("delete") {
        "deletion"
    } else if action.contains("update") || action.contains("change") {
        "change"
    } else if action.contains("create") || action.contains("add") {
        "creation"
    } else if action.contains("user") {
        "user"
    } else if action.contains("group") {
        "group"
    } else {
        "info"
    };

    let mut kinds = match event.get("event.type") {
        Some(Value::Array(held)) => held.clone(),
        Some(Value::Null) | None => Vec::new(),
        // A non-list `event.type` is what the script appends to, and Painless
        // raises on it -- so nothing is written.
        Some(_) => return,
    };
    kinds.push(Value::from(kind));
    let _ = event.set("event.type", Value::Array(kinds));
}

/// `canva/audit`, the untagged flag script in `default`: every permission flag
/// in the change list read from its string spelling into a boolean.
fn change_flags_to_booleans(event: &mut Event, _params: &Value) {
    let Some(Value::Array(mut changes)) = event.get(CHANGES).cloned() else {
        return;
    };
    for change in &mut changes {
        let Some(change) = change.as_object_mut() else {
            continue;
        };
        for member in FLAG_MAPS {
            convert_member(change, member);
        }
        // The two link roles nest their flags one deeper, and the pass above
        // has already rebuilt the outer map.
        for role in ["new_link_role", "old_link_role"] {
            if let Some(Value::Object(role)) = change.get_mut(role) {
                convert_member(role, "access");
            }
        }
        let rebuilt = convert_flags(change);
        *change = rebuilt;
    }
    event.update(CHANGES, Value::Array(changes));
}

/// One member of a map rebuilt with its flags read as booleans.
fn convert_member(map: &mut Map<String, Value>, member: &str) {
    // A member that is not a map raises on the cast in Painless, so it is left
    // exactly as it is.
    let converted = match map.get(member) {
        Some(Value::Object(held)) => convert_flags(held),
        _ => return,
    };
    map.insert(member.to_owned(), Value::Object(converted));
}

/// A map copied with every flag member read as a boolean.
///
/// `Boolean.parseBoolean` answers true for `"true"` whatever its case and
/// false for everything else, including a value the script would raise on --
/// we cannot raise, so such a value is copied through instead.
fn convert_flags(map: &Map<String, Value>) -> Map<String, Value> {
    let mut converted = Map::with_capacity(map.len());
    for (key, value) in map {
        let held = match value {
            Value::String(text) if FLAGS.contains(&key.as_str()) => {
                Value::Bool(text.eq_ignore_ascii_case("true"))
            }
            other => other.clone(),
        };
        converted.insert(key.clone(), held);
    }
    converted
}

/// Every `canva` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "9954eadc3ec2efedefd9a363d83da39acbc246f6ddeb64bc4e943fa5a86242bd",
        source: "canva",
        name: "event_type_from_action",
        run: event_type_from_action,
    },
    Entry {
        hash: "e54c05321e6a38174746a366ca41c90eca63005e09d6c6d6dfb636eb97854da1",
        source: "canva",
        name: "change_flags_to_booleans",
        run: change_flags_to_booleans,
    },
];
