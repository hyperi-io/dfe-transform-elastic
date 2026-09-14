// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `modsecurity`'s apache audit pass: the rule messages split off the front of
//! each detail line, and the details themselves boxed into objects.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// `modsecurity/auditlog`, processor `script_7497121c`:
/// `modsec.audit.messages` and a re-boxed `modsec.audit.details`.
///
/// The mapping declares `modsec.audit.details` flattened, which has to be an
/// object, so every detail line is wrapped as `{"value": <line>}`. The message
/// is the text ahead of the first ` [`, where the rule's key-value tail starts.
fn split_audit_details(event: &mut Event, _params: &Value) {
    let Some(details) = event.get_array("modsec.audit.details").cloned() else {
        return;
    };
    if details.is_empty() {
        return;
    }
    let mut messages = Vec::with_capacity(details.len());
    let mut boxed = Vec::with_capacity(details.len());
    for detail in details {
        // Painless raises on a non-string here, so nothing at all is written.
        let Some(line) = detail.as_str() else {
            return;
        };
        let message = line.find(" [").map_or(line, |at| &line[..at]);
        messages.push(Value::from(message));
        let mut record = Map::new();
        record.insert("value".to_owned(), Value::from(line));
        boxed.push(Value::Object(record));
    }
    let _ = event.set("modsec.audit.messages", Value::Array(messages));
    event.update("modsec.audit.details", Value::Array(boxed));
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "50178a81c7c727038d28b38963b2a540e2ce10566a9bf56475d4fb7978c01359",
    source: "modsecurity",
    name: "split_audit_details",
    run: split_audit_details,
}];
