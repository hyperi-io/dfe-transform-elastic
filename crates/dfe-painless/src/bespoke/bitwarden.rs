// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `bitwarden`'s event-type lookup, which names the numeric type and
//! categorises it in one step.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `bitwarden/event`, the untagged categorisation processor:
/// `bitwarden.event.type.name`, `event.category`, `event.type` and
/// `event.outcome`.
///
/// A row that names only an outcome writes an explicit null over the other
/// three, which the pipeline's closing prune then takes out again.
fn set_event_type(event: &mut Event, params: &Value) {
    let Some(value) = event.get_string("bitwarden.event.type.value") else {
        return;
    };
    let Some(row) = params.get(&value) else {
        return;
    };
    let member = |name: &str| row.get(name).cloned().unwrap_or(Value::Null);
    let _ = event.set("bitwarden.event.type.name", member("name"));

    // Painless raises on the assignment where `ctx.event` is absent.
    if !event.has("event") {
        return;
    }
    let _ = event.set("event.category", member("category"));
    let _ = event.set("event.type", member("type"));
    let _ = event.set("event.outcome", member("outcome"));
}

/// `bitwarden/member`, processor `script_to_set_organization_user_status`:
/// `bitwarden.member.status.name`.
///
/// The vendor numbers its statuses from -1, so the name is one PAST the value
/// in the table -- reading the table at the value itself names the status
/// before the one the member is in.
fn set_member_status(event: &mut Event, params: &Value) {
    let Some(names) = params
        .get("OrganizationUserStatusType")
        .and_then(Value::as_array)
    else {
        return;
    };
    let Some(value) = event
        .get_str("bitwarden.member.status.value")
        .and_then(|raw| raw.parse::<i32>().ok())
    else {
        return;
    };
    let Some(name) = usize::try_from(value.wrapping_add(1))
        .ok()
        .and_then(|index| names.get(index))
        .cloned()
    else {
        return;
    };
    let _ = event.set("bitwarden.member.status.name", name);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "eeb935132bb74890e5d43fb85f0bc8d5246b95eab7fc78ddd61d95e2c46f557b",
        source: "bitwarden",
        name: "set_event_type",
        run: set_event_type,
    },
    Entry {
        hash: "529441cfbb7d5620c8a7550d158e35a84dde5b0c331192ce2595cbfd00a6c00d",
        source: "bitwarden",
        name: "set_member_status",
        run: set_member_status,
    },
];
