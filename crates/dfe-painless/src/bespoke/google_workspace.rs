// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `google_workspace`'s report scripts, transcribed.
//!
//! Two payload shapes run through here. The reports API sends an event's
//! detail as a `parameters` list of name/value objects, where the value
//! arrives under one of several keys according to its type; the Gmail log
//! instead comes straight out of `BigQuery`, as a row of positional values with
//! a separate schema naming and typing each one. Both need the schema and the
//! payload read together, which is why a fold that reads only `value`, or a
//! date processor with no conversion in front of it, leaves the whole stream
//! empty.

use std::collections::HashSet;

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::java_string_set_order;

/// The reports API's own event detail.
const PARAMETERS: &str = "json.events.parameters";

/// `google_workspace/gmail`, `script_to_transform_bigquery_api_result`: the
/// `BigQuery` row read through its schema into a named document.
///
/// A row is positional -- the Nth cell belongs to the Nth schema field -- and
/// a `RECORD` field carries a nested row of its own under `v.f`. A `REPEATED`
/// field is a list of such cells whatever its type.
fn gmail_bigquery_transform(event: &mut Event, _params: &Value) {
    let (Some(row), Some(schema)) = (
        event.get_array("json.row.f").cloned(),
        event.get_array("json.schema.fields").cloned(),
    ) else {
        return;
    };
    if !event.has("google_workspace") {
        let _ = event.set("google_workspace", Value::Object(Map::new()));
    }
    let record = transform_record(&row, &schema);
    let _ = event.set("google_workspace.gmail", Value::Object(record));
}

/// One row through its schema.
fn transform_record(row: &[Value], schema: &[Value]) -> Map<String, Value> {
    let mut record = Map::with_capacity(schema.len());
    for (index, field) in schema.iter().enumerate() {
        let name = field
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let repeated = field.get("mode").and_then(Value::as_str) == Some("REPEATED");
        let cell = row
            .get(index)
            .filter(|cell| !cell.is_null())
            .and_then(|cell| cell.get("v"))
            .unwrap_or(&Value::Null);

        if field.get("type").and_then(Value::as_str) == Some("RECORD") {
            let nested = field.get("fields").and_then(Value::as_array);
            if repeated {
                let mut members = Vec::new();
                if let Value::Array(items) = cell {
                    for item in items {
                        let inner = item.get("v").and_then(|value| value.get("f"));
                        match (inner.and_then(Value::as_array), nested) {
                            (Some(inner), Some(nested)) => {
                                members.push(Value::Object(transform_record(inner, nested)));
                            }
                            // A cell that is not a nested row goes through as
                            // the scalar it is.
                            _ => members.push(item.clone()),
                        }
                    }
                }
                record.insert(name.to_owned(), Value::Array(members));
            } else if let Value::Object(_) = cell {
                let inner = cell.get("f").and_then(Value::as_array);
                let walked = match (inner, nested) {
                    (Some(inner), Some(nested)) => transform_record(inner, nested),
                    _ => Map::new(),
                };
                record.insert(name.to_owned(), Value::Object(walked));
            }
        } else if repeated {
            let members: Vec<Value> = match cell {
                Value::Array(items) => items
                    .iter()
                    .map(|item| item.get("v").cloned().unwrap_or(Value::Null))
                    .collect(),
                _ => Vec::new(),
            };
            record.insert(name.to_owned(), Value::Array(members));
        } else {
            record.insert(name.to_owned(), cell.clone());
        }
    }
    record
}

/// `google_workspace/gmail`: every attachment SHA-256 an interaction carries,
/// gathered into `related.hash`.
fn gmail_attachment_hashes(event: &mut Event, _params: &Value) {
    let Some(attachments) = event
        .get_array("google_workspace.gmail.message_info.post_delivery_info.interaction.attachment")
    else {
        return;
    };
    let mut shas: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for attachment in attachments {
        let Some(sha) = attachment.get("sha256").and_then(Value::as_str) else {
            continue;
        };
        if seen.insert(sha.to_owned()) {
            shas.push(sha.to_owned());
        }
    }
    if shas.is_empty() {
        return;
    }
    if !event.has("related") {
        let _ = event.set("related", Value::Object(Map::new()));
    }
    let _ = event.set("related.hash", Value::Array(java_string_set_order(shas)));
}

/// `google_workspace/gmail`: the event's microsecond stamp as milliseconds,
/// for the date processor that reads it next.
///
/// Both operands are Java longs, so the division TRUNCATES -- carrying the
/// remainder into a fraction leaves the date processor a string it cannot read
/// as epoch milliseconds, and the event takes a pipeline error instead of a
/// timestamp.
fn gmail_temp_timestamp(event: &mut Event, _params: &Value) {
    let Some(microseconds) = event.get_as_i64("google_workspace.gmail.event_info.timestamp_usec")
    else {
        return;
    };
    // The script REPLACES `_temp_` rather than adding to it.
    let _ = event.set("_temp_", Value::Object(Map::new()));
    let _ = event.set("_temp_.timestamp_usec", microseconds / 1000);
}

/// `google_workspace/gmail`, `set_email_content_type_based_on_smime_type_of_message`:
/// the S/MIME type number as its media type.
///
/// A number the table does not name writes a NULL, which the pipeline's
/// closing prune drops -- writing the number back instead ships a content type
/// of `0`.
fn gmail_email_content_type(event: &mut Event, params: &Value) {
    let Some(kind) = event.get_str("google_workspace.gmail.message_info.smime_content_type") else {
        return;
    };
    let content_type = params.get(kind).cloned().unwrap_or(Value::Null);
    if !event.has("email") {
        let _ = event.set("email", Value::Object(Map::new()));
    }
    let _ = event.set("email.content_type", content_type);
}

/// `google_workspace/admin`: the reports API's `parameters` list folded onto
/// `google_workspace.admin`, keyed by each parameter's name.
///
/// The value arrives under `value`, `intValue`, `multiValue` or
/// `messageValue`, and the script takes the LAST of those the parameter
/// carries rather than the first.
fn admin_parameters(event: &mut Event, _params: &Value) {
    fold_parameters(
        event,
        "google_workspace.admin",
        &["value", "intValue", "multiValue", "messageValue"],
    );
}

/// `google_workspace/user_accounts`: the same fold, without `messageValue`.
fn user_accounts_parameters(event: &mut Event, _params: &Value) {
    fold_parameters(
        event,
        "google_workspace.user_accounts",
        &["value", "intValue", "multiValue"],
    );
}

/// The fold itself.
///
/// A parameter name can carry a dot -- the vendor spells several that way --
/// so the members go into the map directly rather than through a dotted set,
/// which would nest them.
fn fold_parameters(event: &mut Event, target: &str, members: &[&str]) {
    let Some(parameters) = event.get_array(PARAMETERS).cloned() else {
        return;
    };
    let mut folded = event
        .get_object(target)
        .cloned()
        .unwrap_or_else(|| Map::with_capacity(parameters.len()));
    for parameter in &parameters {
        let Some(name) = parameter.get("name").and_then(Value::as_str) else {
            continue;
        };
        for member in members {
            if let Some(value) = parameter.get(*member).filter(|value| !value.is_null()) {
                folded.insert(name.to_owned(), value.clone());
            }
        }
    }
    let _ = event.set(target, Value::Object(folded));
}

/// `google_workspace/admin`: the `SETTING_METADATA` sub-parameters lifted out
/// of their list and onto the metadata map itself.
fn admin_setting_metadata(event: &mut Event, _params: &Value) {
    const METADATA: &str = "google_workspace.admin.SETTING_METADATA";

    let Some(parameters) = event.get_array(&format!("{METADATA}.parameter")).cloned() else {
        return;
    };
    let Some(mut metadata) = event.get_object(METADATA).cloned() else {
        return;
    };
    for parameter in &parameters {
        let (Some(name), Some(value)) = (
            parameter.get("name").and_then(Value::as_str),
            parameter.get("value").filter(|value| !value.is_null()),
        ) else {
            continue;
        };
        metadata.insert(name.to_owned(), value.clone());
    }
    let _ = event.set(METADATA, Value::Object(metadata));
}

/// `google_workspace/chat`, `script_to_extract_file_extension`: whatever
/// follows the last dot of the shared file's name.
fn chat_file_extension(event: &mut Event, _params: &Value) {
    let Some(name) = event.get_string("google_workspace.chat.filename") else {
        return;
    };
    // The script takes everything past the last dot, and an absent dot makes
    // that the whole name -- which its processor's guard rules out.
    let extension = match name.rfind('.') {
        Some(at) => name[at + 1..].to_owned(),
        None => name,
    };
    // Painless raises on the assignment where the parent map is absent.
    if !event.has("file") {
        return;
    }
    let _ = event.set("file.extension", Value::from(extension));
}

/// `google_workspace/drive`: the same extension, off `file.name`, and nothing
/// written where the name carries no dot.
fn drive_file_extension(event: &mut Event, _params: &Value) {
    let Some(name) = event.get_string("file.name") else {
        return;
    };
    let Some(at) = name.rfind('.') else {
        return;
    };
    let _ = event.set("file.extension", Value::from(name[at + 1..].to_owned()));
}

/// `google_workspace/rules`: the recipients' domains added to
/// `related.hosts`, which is rebuilt as a flat, deduplicated list.
fn rules_related_hosts(event: &mut Event, _params: &Value) {
    let mut domains: Vec<Value> = match event.get("related.hosts") {
        Some(Value::Array(hosts)) => hosts.clone(),
        Some(host) if !host.is_null() => vec![host.clone()],
        _ => Vec::new(),
    };
    if !event.has("related") {
        let _ = event.set("related", Value::Object(Map::new()));
    }

    match event
        .get("google_workspace.rules.resource.recipients")
        .cloned()
    {
        Some(Value::Array(recipients)) => {
            for recipient in &recipients {
                if let Some(domain) = after_at(recipient) {
                    domains.push(Value::from(domain));
                }
            }
        }
        Some(recipient) => {
            if let Some(domain) = after_at(&recipient) {
                domains.push(Value::from(domain));
            }
        }
        None => {}
    }

    let mut seen: HashSet<String> = HashSet::new();
    domains.retain(|domain| seen.insert(domain.to_string()));
    let _ = event.set("related.hosts", Value::Array(domains));
}

/// The part of an address after its first `@`, where there is one.
fn after_at(value: &Value) -> Option<String> {
    let text = value.as_str()?;
    if !text.contains('@') {
        return None;
    }
    text.split('@').nth(1).map(str::to_owned)
}

/// `google_workspace/calendar`: the event's start, which the vendor sends as
/// seconds from the Gregorian epoch, as epoch milliseconds.
fn calendar_start_time(event: &mut Event, _params: &Value) {
    gregorian_millis(event, "google_workspace.calendar.start_time");
}

/// `google_workspace/calendar`: the same conversion for the event's end.
fn calendar_end_time(event: &mut Event, _params: &Value) {
    gregorian_millis(event, "google_workspace.calendar.end_time");
}

/// Seconds counted from `0001-01-01` as epoch milliseconds.
///
/// 62,135,683,200 is the vendor's own offset between the two epochs, and it is
/// reproduced rather than recomputed.
fn gregorian_millis(event: &mut Event, path: &str) {
    const GREGORIAN_OFFSET: i64 = 62_135_683_200;

    let Some(seconds) = event
        .get_str(path)
        .and_then(|text| text.trim().parse::<i64>().ok())
    else {
        return;
    };
    let _ = event.update(path, (seconds - GREGORIAN_OFFSET) * 1000);
}

/// Every `google_workspace` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "a1dd684dd83a395d087233df33b8a70697611c14e426537c7468da801b63b720",
        source: "google_workspace",
        name: "gmail_bigquery_transform",
        run: gmail_bigquery_transform,
    },
    Entry {
        hash: "72505d6ec53ac15a93eafb48d050473db2a798ebacc90e79b38331b0a6685e5c",
        source: "google_workspace",
        name: "gmail_attachment_hashes",
        run: gmail_attachment_hashes,
    },
    Entry {
        hash: "33d24e468a9c16f08adf8d605ed7f6d4e5ed3c3e4612992976bff070c0dbf242",
        source: "google_workspace",
        name: "gmail_temp_timestamp",
        run: gmail_temp_timestamp,
    },
    Entry {
        hash: "a6fdd5a231c47c47ed9eb6aad809ebd425f9091513baace287162c156b08a047",
        source: "google_workspace",
        name: "gmail_email_content_type",
        run: gmail_email_content_type,
    },
    Entry {
        hash: "04ee596c5089dc4a50c80c100471cd68b7357c04e06ad28be76fa57f9451354c",
        source: "google_workspace",
        name: "admin_parameters",
        run: admin_parameters,
    },
    Entry {
        hash: "574803c8f22729a11a6fc75372717c9ff8896bbbc8c92c90553e116dbe966e33",
        source: "google_workspace",
        name: "admin_setting_metadata",
        run: admin_setting_metadata,
    },
    Entry {
        hash: "31ce68ebc4656ddd15198f197964c2f5f480183486f28b2bfcde834513d07cc5",
        source: "google_workspace",
        name: "user_accounts_parameters",
        run: user_accounts_parameters,
    },
    Entry {
        hash: "2734fb3ff4b0dff85e2a0b98ac463d10a86d6128fe5d56f47826d4c5189e8200",
        source: "google_workspace",
        name: "chat_file_extension",
        run: chat_file_extension,
    },
    Entry {
        hash: "1f792fd4813c0751f6075143136cb24431fa4d7c86b519aeab5ec76c7378765c",
        source: "google_workspace",
        name: "drive_file_extension",
        run: drive_file_extension,
    },
    Entry {
        hash: "8ee0c3a0c570cda4a278c34c0d103cfc793b7ad5c22f8d6d424803943017d65f",
        source: "google_workspace",
        name: "rules_related_hosts",
        run: rules_related_hosts,
    },
    Entry {
        hash: "1c200ece385f0b6c7442ce243d7801ac508c235d57fb60a71f3719959d7f1b24",
        source: "google_workspace",
        name: "calendar_start_time",
        run: calendar_start_time,
    },
    Entry {
        hash: "67de320c144de9cc47bda11bf3fdec18e0b587a0138a434fbf0f0021720ef74e",
        source: "google_workspace",
        name: "calendar_end_time",
        run: calendar_end_time,
    },
];
