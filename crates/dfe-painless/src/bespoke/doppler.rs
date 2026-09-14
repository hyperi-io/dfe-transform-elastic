// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `doppler`'s metadata scripts, transcribed.
//!
//! Doppler puts the interesting part of an event in a free-form `metadata`
//! object it then removes, so both scripts lift what they need out of it
//! before it goes. The secret-read stream also abbreviates every key to one
//! letter, which is why the reshape exists at all.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// `doppler/secret_read`, `reshape_secrets`: the read secrets as full-named
/// records, plus the four hunting arrays and the count.
///
/// A single object where the vendor normally sends a list is read as a
/// one-element list, so a lone read is not dropped in silence.
fn reshape_secrets(event: &mut Event, _params: &Value) {
    let Some(raw) = event.get("doppler.metadata.secrets").cloned() else {
        return;
    };
    let secrets = match raw {
        Value::Array(items) => items,
        Value::Object(_) => vec![raw],
        _ => Vec::new(),
    };

    let mut out: Vec<Value> = Vec::with_capacity(secrets.len());
    let mut names: Vec<Value> = Vec::new();
    let mut projects: Vec<Value> = Vec::new();
    let mut environments: Vec<Value> = Vec::new();
    let mut configs: Vec<Value> = Vec::new();

    for secret in &secrets {
        let Some(secret) = secret.as_object() else {
            continue;
        };
        let mut record = Map::new();
        expand(secret, "p", "project", &mut record, Some(&mut projects));
        expand(
            secret,
            "e",
            "environment",
            &mut record,
            Some(&mut environments),
        );
        expand(secret, "c", "config", &mut record, Some(&mut configs));
        expand(secret, "s", "name", &mut record, Some(&mut names));
        expand(secret, "v", "version", &mut record, None);

        // A secret read through another config carries where it came from.
        if let Some(inherited) = secret.get("r").and_then(Value::as_object) {
            let mut from = Map::new();
            expand(inherited, "p", "project", &mut from, None);
            expand(inherited, "e", "environment", &mut from, None);
            expand(inherited, "c", "config", &mut from, None);
            record.insert("inherited_from".to_owned(), Value::Object(from));
        }
        out.push(Value::Object(record));
    }

    // The count is of the records actually built, so a malformed entry is not
    // counted against the list beside it.
    let count = out.len();
    let _ = event.set("doppler.secret_read.secrets", Value::Array(out));
    let _ = event.set("doppler.secret_read.secret_names", Value::Array(names));
    let _ = event.set("doppler.secret_read.projects", Value::Array(projects));
    let _ = event.set(
        "doppler.secret_read.environments",
        Value::Array(environments),
    );
    let _ = event.set("doppler.secret_read.configs", Value::Array(configs));
    let _ = event.set("doppler.secret_read.secret_count", count);
}

/// One abbreviated key onto its full name, and onto the hunting array that
/// collects it.
fn expand(
    secret: &Map<String, Value>,
    short: &str,
    full: &str,
    record: &mut Map<String, Value>,
    collect: Option<&mut Vec<Value>>,
) {
    let Some(value) = secret.get(short).filter(|value| !value.is_null()) else {
        return;
    };
    record.insert(full.to_owned(), value.clone());
    if let Some(collect) = collect
        && !collect.contains(value)
    {
        collect.push(value.clone());
    }
}

/// `doppler/activity`, `promote_added_members`: the people an invite added,
/// as `user.target` and as the members list.
///
/// `user.target` takes the FIRST member only, and only where the enrichment
/// ahead of it left the field empty.
fn promote_added_members(event: &mut Event, _params: &Value) {
    let Some(raw) = event.get("doppler.metadata.addedMembers").cloned() else {
        return;
    };
    let members = match raw {
        Value::Array(items) => items,
        Value::Object(_) => vec![raw],
        _ => Vec::new(),
    };

    let mut emails: Vec<Value> = Vec::new();
    for member in &members {
        let Some(email) = member
            .as_object()
            .and_then(|member| member.get("email"))
            .filter(|email| !email.is_null())
        else {
            continue;
        };
        if !emails.contains(email) {
            emails.push(email.clone());
        }
    }

    if let Some(first) = members.first().filter(|first| first.is_object()) {
        let email = first.get("email").filter(|email| !email.is_null()).cloned();
        let name = first.get("name").filter(|name| !name.is_null()).cloned();
        if !event.get("user").is_some_and(Value::is_object) {
            let _ = event.set("user", Value::Object(Map::new()));
        }
        if !event.get("user.target").is_some_and(Value::is_object) {
            let _ = event.set("user.target", Value::Object(Map::new()));
        }
        if let Some(email) = email
            && !event.has_value("user.target.email")
        {
            let _ = event.set("user.target.email", email);
        }
        if let Some(name) = name
            && !event.has_value("user.target.name")
        {
            let _ = event.set("user.target.name", name);
        }
    }

    if !emails.is_empty() {
        let _ = event.set("doppler.activity.members_added", Value::Array(emails));
    }
}

/// Every `doppler` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "3393144025c63c528ca2173e798933d25b1843b90b4c14b7d76fd778d8cc8a98",
        source: "doppler",
        name: "reshape_secrets",
        run: reshape_secrets,
    },
    Entry {
        hash: "b22150d21cc216ed618f1769110ec48c3fd267b7eddade7f406b9e79f022fae7",
        source: "doppler",
        name: "promote_added_members",
        run: promote_added_members,
    },
];
