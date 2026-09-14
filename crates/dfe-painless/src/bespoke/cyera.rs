// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cyera`'s ECS categorisation of an audit action, and the verb-derived
//! fallback its lookup table falls out of.

use serde_json::{Value, json};

use super::Entry;
use dfe_core::event::Event;

/// `cyera/audit`, processor `script_set_ecs_categorization_46707bc5`:
/// `event.category`, `event.type` and `event.outcome`.
///
/// The fallback reads the action's WORDS rather than its spelling, so a vendor
/// phrasing the table has not got is still categorised -- "User signed in" and
/// "User logged in" both reach the authentication arm.
fn set_ecs_categorization(event: &mut Event, params: &Value) {
    let Some(action) = event.get_string("event.action") else {
        return;
    };
    let exact = params
        .get("exact")
        .and_then(|table| table.get(&action))
        .cloned();
    let Some(entry) = exact.or_else(|| fallback(&action.to_lowercase())) else {
        return;
    };
    for field in ["category", "type", "outcome"] {
        if let Some(value) = entry.get(field)
            && !value.is_null()
        {
            let _ = event.set(format!("event.{field}").as_str(), value.clone());
        }
    }
}

/// The categorisation an action's own words imply, where the table has no row
/// for it.
fn fallback(action: &str) -> Option<Value> {
    let session = ["authentication", "session"];
    if ["logged in", "login", "log in"]
        .iter()
        .any(|phrase| action.contains(phrase))
    {
        let outcome = if action.contains("fail") {
            "failure"
        } else {
            "success"
        };
        return Some(json!({"category": session, "type": ["start"], "outcome": outcome}));
    }
    if ["logged out", "logout", "log out"]
        .iter()
        .any(|phrase| action.contains(phrase))
    {
        return Some(json!({"category": session, "type": ["end"], "outcome": "success"}));
    }
    if ["removed user", "deleted user"]
        .iter()
        .any(|prefix| action.starts_with(prefix))
    {
        return Some(json!({"category": ["iam"], "type": ["user", "deletion"]}));
    }
    if ["added user", "invited user"]
        .iter()
        .any(|prefix| action.starts_with(prefix))
    {
        return Some(json!({"category": ["iam"], "type": ["user", "creation"]}));
    }
    None
}

/// The transcription this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "120e71425894e85aaef2bb7bec37950a10924faf506202507a0bb7bed8ee52c6",
    source: "cyera",
    name: "set_ecs_categorization",
    run: set_ecs_categorization,
}];
