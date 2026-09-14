// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `abnormal_security`'s three copy-out scripts: the two that lift a chosen
//! set of keys off a vendor case, and the one that reads an attachment's
//! extension off its name.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// `abnormal_security/vendor_case`, processor `parse_and_append_insights`:
/// `abnormal_security.vendor_case.insights`.
fn parse_and_append_insights(event: &mut Event, params: &Value) {
    copy_keys(
        event,
        params,
        "json.insights",
        "abnormal_security.vendor_case.insights",
        false,
    );
}

/// `abnormal_security/vendor_case`, processor `parse_and_append_timeline`:
/// `abnormal_security.vendor_case.timeline`.
///
/// The same copy as the insights above, with the vendor's camelCase key
/// spelled `snake_case` on the way out.
fn parse_and_append_timeline(event: &mut Event, params: &Value) {
    copy_keys(
        event,
        params,
        "json.timeline",
        "abnormal_security.vendor_case.timeline",
        true,
    );
}

/// Copy `params.keysToCopy` out of every member of the list at `from` into a
/// list of maps at `to`, keeping only the keys a member actually carries.
fn copy_keys(event: &mut Event, params: &Value, from: &str, to: &str, snake: bool) {
    let Some(wanted) = params.get("keysToCopy").and_then(Value::as_array) else {
        return;
    };
    let Some(members) = event.get_array(from) else {
        return;
    };
    let copied: Vec<Value> = members
        .iter()
        .map(|member| {
            let mut out = Map::new();
            for key in wanted.iter().filter_map(Value::as_str) {
                if let Some(value) = member.get(key) {
                    let name = if snake { to_snake(key) } else { key.to_owned() };
                    out.insert(name, value.clone());
                }
            }
            Value::Object(out)
        })
        .collect();

    // Painless raises on the assignment where the parent map is absent.
    let parent = to.rsplit_once('.').map_or(to, |(head, _)| head);
    if !event.has(parent) {
        return;
    }
    let _ = event.set(to, Value::Array(copied));
}

/// The script's own inline conversion: an underscore before every uppercase
/// letter bar a leading one, and the letter lowered.
fn to_snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    for (position, ch) in key.chars().enumerate() {
        if ch.is_uppercase() {
            if position > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// `abnormal_security/threat`, processor `script_to_set_email_attachments_field`:
/// `email.attachments`, named and typed off the attachment file names.
///
/// The script replaces `email` wholesale with a fresh map, which is harmless
/// because every other `email.*` field is written after it.
fn set_email_attachments(event: &mut Event, _params: &Value) {
    let Some(names) = event.get_array("abnormal_security.threat.attachment_names") else {
        return;
    };
    let attachments: Vec<Value> = names
        .iter()
        .map(|name| {
            let mut file = Map::new();
            file.insert("name".to_owned(), name.clone());
            // A name with no dot in it keeps no extension, and the last token
            // wins where there are several.
            if let Some(text) = name.as_str() {
                let tokens: Vec<&str> = text.split('.').collect();
                if tokens.len() > 1
                    && let Some(last) = tokens.last()
                {
                    file.insert("extension".to_owned(), Value::from(*last));
                }
            }
            let mut attachment = Map::new();
            attachment.insert("file".to_owned(), Value::Object(file));
            Value::Object(attachment)
        })
        .collect();
    let _ = event.set("email", Value::Object(Map::new()));
    let _ = event.set("email.attachments", Value::Array(attachments));
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "886d9ee266a20993ef5c72dcfbf358357f54693dd1d886bd73218cf9c2f08e94",
        source: "abnormal_security",
        name: "parse_and_append_insights",
        run: parse_and_append_insights,
    },
    Entry {
        hash: "5c49c1868227177f7a018d80a89e3baedf36945ba0bf8923d6344de62d760070",
        source: "abnormal_security",
        name: "parse_and_append_timeline",
        run: parse_and_append_timeline,
    },
    Entry {
        hash: "15309e58a334520c906fe941efc0577ef2c543e6e50e905996a3287764855298",
        source: "abnormal_security",
        name: "set_email_attachments",
        run: set_email_attachments,
    },
];
