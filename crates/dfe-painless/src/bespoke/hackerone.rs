// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `hackerone`'s report scripts, transcribed.
//!
//! A report's workflow state is the whole of its ECS categorisation, and the
//! people on it are spread across four different relationship shapes -- the
//! assignee, the collaborators, the summary authors and whoever wrote the
//! remediation guidance -- each nested differently in the JSON:API payload.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// `hackerone/report`, `categorize_report_state`: the ECS event fields the
/// report's own workflow state decides.
///
/// The script REPLACES `ctx.event`, so a state the table does not carry
/// leaves the document exactly as it found it.
fn categorize_report_state(event: &mut Event, params: &Value) {
    let Some(state) = event.get("hackerone.report.attributes.state") else {
        return;
    };
    let state = painless_to_string(state);
    let Some(row) = params.get(&state).and_then(Value::as_object) else {
        return;
    };
    let outcome = row.get("outcome").cloned().unwrap_or(Value::Null);
    let action = row.get("action").cloned().unwrap_or(Value::Null);

    // `ctx.containsKey('event') && ctx.event instanceof Map`: anything else
    // there is thrown away for a fresh map.
    if !event.get("event").is_some_and(Value::is_object) {
        let _ = event.set("event", Value::Object(Map::new()));
    }
    let _ = event.set("event.kind", "event");
    let _ = event.set(
        "event.category",
        Value::Array(vec![Value::from("vulnerability")]),
    );
    let _ = event.set("event.type", Value::Array(vec![Value::from("info")]));
    let _ = event.set("event.outcome", outcome);
    let _ = event.set("event.action", action);
}

/// `hackerone/report`, `aggregate_related_user`: every username the report
/// names, in the order the script walks them.
fn aggregate_related_user(event: &mut Event, _params: &Value) {
    let mut users: Vec<String> = Vec::new();
    if let Some(name) = event.get("user.name").filter(|value| !value.is_null()) {
        push_unique(&mut users, painless_to_string(name));
    }

    if let Some(relationships) = event.get_object("hackerone.report.relationships").cloned() {
        collect_assignee(&relationships, &mut users);
        collect_collaborators(&relationships, &mut users);
        collect_summaries(&relationships, &mut users);
        collect_remediation_author(&relationships, &mut users);
    }

    if users.is_empty() {
        return;
    }
    // Rebuilt from the existing LIST, so anything else already sitting at the
    // path is replaced rather than folded into an array beside the names.
    let mut related: Vec<Value> = event.get_array("related.user").cloned().unwrap_or_default();
    for user in users {
        let user = Value::from(user);
        if !related.contains(&user) {
            related.push(user);
        }
    }
    let _ = event.set("related.user", Value::Array(related));
}

/// The assignee, which is only counted where the relationship names a user
/// rather than a group.
fn collect_assignee(relationships: &Map<String, Value>, users: &mut Vec<String>) {
    let Some(data) = relationships
        .get("assignee")
        .and_then(|assignee| assignee.get("data"))
        .filter(|data| data.is_object())
    else {
        return;
    };
    if data.get("type").and_then(Value::as_str) != Some("user") {
        return;
    }
    push_username(data.get("attributes"), users);
}

/// Everyone added to the report after it was filed.
fn collect_collaborators(relationships: &Map<String, Value>, users: &mut Vec<String>) {
    let Some(rows) = relationships
        .get("collaborators")
        .and_then(|collaborators| collaborators.get("data"))
        .and_then(Value::as_array)
    else {
        return;
    };
    for row in rows {
        let Some(user) = row.get("user").filter(|user| user.is_object()) else {
            continue;
        };
        push_username(user.get("attributes"), users);
    }
}

/// Whoever wrote each summary on the report.
fn collect_summaries(relationships: &Map<String, Value>, users: &mut Vec<String>) {
    let Some(rows) = relationships
        .get("summaries")
        .and_then(|summaries| summaries.get("data"))
        .and_then(Value::as_array)
    else {
        return;
    };
    for row in rows {
        let Some(data) = row
            .get("relationships")
            .and_then(|nested| nested.get("user"))
            .and_then(|user| user.get("data"))
            .filter(|data| data.is_object())
        else {
            continue;
        };
        push_username(data.get("attributes"), users);
    }
}

/// Whoever wrote the programme's own remediation guidance.
fn collect_remediation_author(relationships: &Map<String, Value>, users: &mut Vec<String>) {
    let Some(data) = relationships
        .get("custom_remediation_guidance")
        .and_then(|guidance| guidance.get("data"))
        .and_then(|data| data.get("relationships"))
        .and_then(|nested| nested.get("author"))
        .and_then(|author| author.get("data"))
        .filter(|data| data.is_object())
    else {
        return;
    };
    push_username(data.get("attributes"), users);
}

/// The `username` an attributes map carries, if it carries one.
fn push_username(attributes: Option<&Value>, users: &mut Vec<String>) {
    let Some(username) = attributes
        .filter(|attributes| attributes.is_object())
        .and_then(|attributes| attributes.get("username"))
        .filter(|username| !username.is_null())
    else {
        return;
    };
    push_unique(users, painless_to_string(username));
}

/// The `LinkedHashSet` the script collects into: first-seen order, no repeats.
fn push_unique(users: &mut Vec<String>, user: String) {
    if !users.contains(&user) {
        users.push(user);
    }
}

/// Every `hackerone` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "cc7ca1400b23119604e65b98156527fd05eb1632b5448176673b352f0e26dea9",
        source: "hackerone",
        name: "categorize_report_state",
        run: categorize_report_state,
    },
    Entry {
        hash: "3577a82b26c6d9c47825d4fc9bb81bae1944e7993c2f4c32acf6ca740fbf9725",
        source: "hackerone",
        name: "aggregate_related_user",
        run: aggregate_related_user,
    },
];
