// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `lastpass`'s limit-shared-folder split, transcribed.
//!
//! The event report gives this one action a `Data` field of the form
//! `<folder name> <user email>`, and a folder name may hold spaces of its own.
//! The script puts a comma before the LAST space so the grok that follows has
//! a separator it can anchor on.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The raw field the split reads.
const ORIGINAL: &str = "lastpass.event_report.data.original";

/// The action this runs for.
const ACTION: &str = "limit shared folder";

/// `lastpass/event_report`, the untagged split script in `default`: `_temp`,
/// the raw field with its last space turned into a comma.
fn comma_before_last_space(event: &mut Event, _params: &Value) {
    if !acts(event) {
        return;
    }
    let Some(original) = event.get_string(ORIGINAL) else {
        return;
    };
    // No space indexes at -1 and raises in Painless, so nothing is written.
    let Some(at) = original.rfind(' ') else {
        return;
    };
    let _ = event.set(
        "_temp",
        format!("{},{}", &original[..at], &original[at + 1..]),
    );
}

/// Whether `event.action` names the action, whether it is one string or a list.
fn acts(event: &Event) -> bool {
    match event.get("event.action") {
        Some(Value::Array(actions)) => actions.iter().any(|a| a.as_str() == Some(ACTION)),
        Some(Value::String(action)) => action.contains(ACTION),
        _ => false,
    }
}

/// Every `lastpass` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "bb0787215942f145792511049b48046572846c5ee41caddb419999c1b5189aba",
    source: "lastpass",
    name: "comma_before_last_space",
    run: comma_before_last_space,
}];
