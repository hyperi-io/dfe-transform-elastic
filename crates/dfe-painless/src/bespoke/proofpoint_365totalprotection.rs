// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `proofpoint_365totalprotection`'s attachment reshaping: the vendor's list of
//! attachment records turned into ECS attachments, and the same list replaced
//! by the `yes`/`no` flag the vendor's own field carries afterwards.
//!
//! The names and the extensions are collected in the one walk, so the three
//! vendor fields and the ECS list all come out of a single pass.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The vendor's attachment list, which the script reads and then overwrites.
const ATTACHMENTS: &str = "proofpoint.email.attachments";

/// `process_attachments_to_ecs` in `proofpoint_365totalprotection/email`: the
/// names, the distinct extensions, the `yes`/`no` flag and `email.attachments`.
fn process_attachments_to_ecs(event: &mut Event, _params: &Value) {
    let Some(attachments) = event.get_array(ATTACHMENTS).cloned() else {
        return;
    };

    let mut names: Vec<Value> = Vec::with_capacity(attachments.len());
    let mut extensions: Vec<Value> = Vec::new();
    let mut ecs: Vec<Value> = Vec::with_capacity(attachments.len());

    for attachment in &attachments {
        let Some(record) = attachment.as_object() else {
            continue;
        };
        let Some(name) = record.get("name") else {
            continue;
        };
        names.push(name.clone());

        // The script reads the name as a String, so a vendor sending anything
        // else raises there and the processor writes nothing at all.
        let Some(filename) = name.as_str() else {
            return;
        };
        let extension = extension_of(filename);
        if let Some(extension) = &extension {
            let candidate = Value::from(extension.as_str());
            if !extensions.contains(&candidate) {
                extensions.push(candidate);
            }
        }

        let mut file = Map::new();
        file.insert("name".to_owned(), name.clone());
        if let Some(extension) = extension {
            file.insert("extension".to_owned(), Value::from(extension));
        }
        if let Some(size) = record.get("size") {
            file.insert("size".to_owned(), size.clone());
        }
        let mut wrapper = Map::new();
        wrapper.insert("file".to_owned(), Value::Object(file));
        ecs.push(Value::Object(wrapper));
    }

    let any = !names.is_empty();
    let _ = event.set("proofpoint.email.attachmentsList", Value::Array(names));
    let _ = event.set(
        "proofpoint.email.attachmentsExtensions",
        Value::Array(extensions),
    );
    let _ = event.update(ATTACHMENTS, Value::from(if any { "yes" } else { "no" }));

    if !event.has("email") {
        let _ = event.set("email", Value::Object(Map::new()));
    }
    let _ = event.set("email.attachments", Value::Array(ecs));
}

/// The lowercased extension of a filename, where it has one.
///
/// A leading dot is not a separator and a trailing one leaves nothing to take,
/// which is what the script's two bounds say.
fn extension_of(filename: &str) -> Option<String> {
    let chars: Vec<char> = filename.chars().collect();
    let at = chars.iter().rposition(|ch| *ch == '.')?;
    if at == 0 || at >= chars.len() - 1 {
        return None;
    }
    Some(chars[at + 1..].iter().collect::<String>().to_lowercase())
}

/// Every `proofpoint_365totalprotection` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "942ece0329387367f5edf2a99fd314510afb6effa5464434f27d10cee780aa4c",
    source: "proofpoint_365totalprotection",
    name: "process_attachments_to_ecs",
    run: process_attachments_to_ecs,
}];
