// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `github`'s split of the audit log's `events` field, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The audit log's `events` field, whichever form it took.
const EVENTS: &str = "github.events";

/// `decide_events_and_events_object` in `github/audit`: an `events` list of
/// objects moved to `github.events_object`.
///
/// The vendor sends the field as a count, a list of strings or a list of
/// objects, and only the last of those needs its own mapping -- moving it out
/// of the way is what stops the `convert` behind this script rendering a map as
/// Java's `{test=yes}`.
fn events_object_from_events(event: &mut Event, _params: &Value) {
    let holds_objects = event
        .get_array(EVENTS)
        .and_then(|events| events.first())
        .is_some_and(Value::is_object);
    if !holds_objects {
        return;
    }
    let Some(events) = event.take_array(EVENTS) else {
        return;
    };
    event.remove(EVENTS);
    let _ = event.set("github.events_object", Value::Array(events));
}

/// Every `github` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "e92a500bd4ca92b74ba288556a79ad0b425a9eb691a9facccf3cc58dd8651a3f",
    source: "github",
    name: "events_object_from_events",
    run: events_object_from_events,
}];
