// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `claroty_ctd`'s two event scripts, transcribed.
//!
//! The CEF extensions carry one worker's health as a Python-style dict printed
//! into a single extension value, so the whole `claroty_ctd.event.worker`
//! subtree is text until the pipeline parses it back. The other script reads
//! the event class id to decide what kind of event this is.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// The CEF field the event class is reported in.
const CLASS_ID: &str = "cef.device.event_class_id";

/// The subtree the worker renames land in.
const WORKER: &str = "claroty_ctd.event.worker";

/// `claroty_ctd/event`, processor `script_to_set_event_kind_type_catgeory`:
/// `event.kind`, and `event.category` and `event.type` where the class id says
/// what the event is about.
///
/// The fourth arm the vendor spells is unreachable -- the first already claims
/// every class id holding `Event` -- and is transcribed as written.
fn event_kind_type_category(event: &mut Event, _params: &Value) {
    let Some(class_id) = event.get_string(CLASS_ID) else {
        return;
    };
    // `ctx.event.category = ...` reaches through `ctx.event`, which the
    // pipeline's own rename of `message` has already created.
    if !event.has("event") {
        return;
    }

    if class_id.contains("Insight") || class_id.contains("Event") {
        set_kind(event, &["network"], &["info"], "event");
    } else if class_id.contains("ActivityLog") || class_id.contains("HealthCheck") {
        let _ = event.set("event.kind", "event");
    } else if class_id.contains("Alert") {
        set_kind(event, &["threat"], &["indicator"], "alert");
    } else if class_id.contains("Event") {
        set_kind(event, &["network"], &["info"], "event");
    }
}

/// The three ECS fields one arm writes, in the order the script writes them.
fn set_kind(event: &mut Event, category: &[&str], kind_of: &[&str], kind: &str) {
    let _ = event.set("event.category", list(category));
    let _ = event.set("event.type", list(kind_of));
    let _ = event.set("event.kind", kind);
}

/// One Painless list literal of strings.
fn list(members: &[&str]) -> Value {
    Value::Array(members.iter().map(|member| Value::from(*member)).collect())
}

/// `claroty_ctd/event`, the untagged `objectify` script: every worker value
/// the appliance printed as a dict, parsed back into an object.
///
/// The appliance renders the dict with Python's single quotes, so the text is
/// requoted before it is parsed. Anything the parse declines keeps the
/// requoted text, which is what the script's own `catch` does.
///
/// The walk is two levels and no deeper: the map itself, then each member of
/// it that is a map -- which is where the grouped renames such as
/// `worker.web.auth` put their values.
fn objectify_workers(event: &mut Event, _params: &Value) {
    let Some(Value::Object(mut workers)) = event.get(WORKER).cloned() else {
        return;
    };
    objectify(&mut workers);

    let nested: Vec<String> = workers
        .iter()
        .filter(|(_, value)| value.is_object())
        .map(|(key, _)| key.clone())
        .collect();
    for key in nested {
        if let Some(Value::Object(inner)) = workers.get_mut(&key) {
            objectify(inner);
        }
    }

    event.update(WORKER, Value::Object(workers));
}

/// Every value of one map that reads as a printed dict, parsed in place.
fn objectify(map: &mut Map<String, Value>) {
    let keys: Vec<String> = map
        .iter()
        .filter(|(_, value)| match value {
            Value::String(text) => text.starts_with('{'),
            _ => false,
        })
        .map(|(key, _)| key.clone())
        .collect();
    for key in keys {
        let Some(Value::String(text)) = map.get(&key) else {
            continue;
        };
        let requoted = text.replace('\'', "\"");
        let parsed = match serde_json::from_str::<Value>(&requoted) {
            Ok(value) => value,
            Err(_) => Value::String(requoted),
        };
        map.insert(key, parsed);
    }
}

/// Every `claroty_ctd` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "2a0c566aae094f0f6c308015ae8774469888773b7cffcf5889c7498e83fbe3ee",
        source: "claroty_ctd",
        name: "event_kind_type_category",
        run: event_kind_type_category,
    },
    Entry {
        hash: "1b8ac3a056b17cfa34e2ede88ab9a504793246e1be964985b8e252a7b3871964",
        source: "claroty_ctd",
        name: "objectify_workers",
        run: objectify_workers,
    },
];
