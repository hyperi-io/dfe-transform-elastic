// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The DNS, registry and file scripts of `sentinel_one_cloud_funnel/event`,
//! transcribed.

use serde_json::{Map, Value, json};

use dfe_core::Event;

use super::Entry;

/// The vendor's DNS question, as one line of `type: <number> <name>`.
const DNS_REQUEST: &str = "sentinel_one_cloud_funnel.event.dns.request";

/// The vendor's DNS answers, semicolon-separated.
const DNS_RESPONSE: &str = "sentinel_one_cloud_funnel.event.dns.response";

/// `script_dnsFieldParsing_to_ecs` in `sentinel_one_cloud_funnel/event`: the
/// two packed DNS strings taken apart into `dns.*`.
///
/// An answer line naming a record type carries its data as the third field and
/// anything else is an address, which is why the addresses need no parsing of
/// their own. `related.hosts` is ASSIGNED here rather than appended, so the
/// names this finds replace the endpoint names appended upstream.
fn dns_fields_to_ecs(event: &mut Event, params: &Value) {
    let mut ips: Vec<Value> = Vec::new();
    let mut related_hosts: Vec<Value> = Vec::new();
    let mut dns = Map::new();

    if let Some(request) = non_empty(event, DNS_REQUEST) {
        let mut question = Map::new();
        let parts = java_split_whitespace(&request);
        if parts.len() == 3 {
            question.insert("type".to_owned(), named_type(params, parts[1]));
            question.insert("name".to_owned(), Value::from(parts[2]));
            related_hosts.push(Value::from(parts[2]));
        } else {
            question.insert("name".to_owned(), Value::from(request.as_str()));
        }
        dns.insert("question".to_owned(), Value::Object(question));
    }

    if let Some(response) = non_empty(event, DNS_RESPONSE) {
        let mut answers: Vec<Value> = Vec::new();
        for answer in response.split(';') {
            if answer.is_empty() {
                continue;
            }
            if !answer.starts_with("type:") {
                ips.push(Value::from(answer));
                continue;
            }
            let parts = java_split_whitespace(answer);
            if parts.len() < 2 {
                // The script throws here, which leaves the document untouched
                // because nothing has been written to it yet.
                return;
            }
            if parts.len() == 3 {
                answers.push(json!({
                    "type": named_type(params, parts[1]),
                    "data": parts[2],
                }));
                related_hosts.push(Value::from(parts[2]));
            } else {
                answers.push(json!({ "type": named_type(params, parts[1]) }));
            }
        }
        if !answers.is_empty() {
            dns.insert("answers".to_owned(), Value::Array(answers));
        }
        if !ips.is_empty() {
            dns.insert("resolved_ip".to_owned(), Value::Array(ips));
        }
        if !related_hosts.is_empty() {
            if !event.has_value("related") {
                let _ = event.set("related", json!({}));
            }
            let _ = event.set("related.hosts", Value::Array(related_hosts));
        }
    }

    let _ = event.set("dns", Value::Object(dns));
}

/// The record type `params` names for a number, null where it names none.
fn named_type(params: &Value, number: &str) -> Value {
    params.get(number).cloned().unwrap_or(Value::Null)
}

/// A field's text, where it is a non-empty string.
fn non_empty(event: &Event, path: &str) -> Option<String> {
    event
        .get_str(path)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

/// `String.split` on a whitespace run, with no limit.
///
/// Java discards trailing empty fields and keeps the leading one, so this is
/// the tokens with an empty first field where the text opens on whitespace. No
/// engine is needed for a pattern this is the whole of.
fn java_split_whitespace(text: &str) -> Vec<&str> {
    let mut parts: Vec<&str> = Vec::new();
    if text.starts_with(char::is_whitespace) {
        parts.push("");
    }
    parts.extend(text.split_whitespace());
    parts
}

/// The untagged script in `sentinel_one_cloud_funnel/event`'s registry
/// pipeline: `registry.path` split at its last separator.
fn registry_key_and_value(event: &mut Event, _params: &Value) {
    let Some(path) = event.get_string("registry.path") else {
        return;
    };
    let Some(at) = path.rfind('\\') else {
        return;
    };
    let key = path[..at].to_owned();
    let value = path[at + 1..].to_owned();
    let _ = event.set("registry.key", key);
    let _ = event.set("registry.value", value);
}

/// The untagged script in `sentinel_one_cloud_funnel/event`'s file pipeline:
/// `file.name`, `file.directory`, `file.extension` and `file.drive_letter` off
/// the target path.
///
/// The extension is only taken where the record says the target is a file, and
/// the drive letter is written outside the separator branch -- so a path with a
/// drive but no separator raises rather than writing one.
fn file_info_from_tgt_path(event: &mut Event, _params: &Value) {
    let Some(path) = event.get_string("sentinel_one_cloud_funnel.event.tgt.file.path") else {
        return;
    };
    if let Some(at) = path.rfind('\\').or_else(|| path.rfind('/')) {
        if !event.has_value("file") {
            let _ = event.set("file", json!({}));
        }
        let name = path[at + 1..].to_owned();
        let directory = path[..at].to_owned();
        let extension = name
            .rfind('.')
            .filter(|_| event.get_str("file.type") == Some("file"))
            .map(|dot| name[dot + 1..].to_owned());
        let _ = event.set("file.name", name);
        let _ = event.set("file.directory", directory);
        if let Some(extension) = extension {
            let _ = event.set("file.extension", extension);
        }
    }
    if path.find(':') == Some(1) && event.has("file") {
        let _ = event.set("file.drive_letter", path[..1].to_uppercase());
    }
}

/// Every `sentinel_one_cloud_funnel` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "dfdc942f9327060a0fd95800eef850a11b2a19b4847833e404ecdd6acc626d59",
        source: "sentinel_one_cloud_funnel",
        name: "dns_fields_to_ecs",
        run: dns_fields_to_ecs,
    },
    Entry {
        hash: "cfee1fe416838fa61d7bcd448b889baf5066b2d7288c6e501a37de6ff1514718",
        source: "sentinel_one_cloud_funnel",
        name: "registry_key_and_value",
        run: registry_key_and_value,
    },
    Entry {
        hash: "7f01f43aff0343500d11533a54dd252911026dff327bafc2ea7b1507835fdeae",
        source: "sentinel_one_cloud_funnel",
        name: "file_info_from_tgt_path",
        run: file_info_from_tgt_path,
    },
];
