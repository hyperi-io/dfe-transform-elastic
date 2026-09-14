// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `kolide`'s ECS categorisation, which both of its lookup tables fall back
//! out of rather than leaving a document uncategorised, and the timestamp an
//! issue is dated by.

use chrono::{DateTime, FixedOffset, SecondsFormat};
use serde_json::Value;

use super::Entry;
use dfe_core::date_formats::iso8601_is_before;
use dfe_core::event::Event;

/// Action suffixes the audit fallback reads as a creation.
const CREATION_SUFFIXES: [&str; 5] = [
    "_created",
    "_added",
    "_invited",
    "_generated",
    "_created_and_run",
];

/// Action suffixes the audit fallback reads as a deletion.
const DELETION_SUFFIXES: [&str; 2] = ["_deleted", "_removed"];

/// `kolide/issues`, the untagged `@timestamp` script: the latest instant the
/// issue itself carries, pulled back to ingest where that is in the future.
///
/// An issue record has no timestamp of its own, so the pipeline dates it by
/// the last thing that happened to it. This is what the categorisation below
/// measures the block deadline against, so an unwritten `@timestamp` costs
/// that script its `blocked` arm rather than costing this field.
fn issue_timestamp(event: &mut Event, _params: &Value) {
    // Elasticsearch holds the ingest stamp as a `ZonedDateTime` the script
    // casts back; here it is the string the runtime minted.
    let Some(now) = instant_at(event, "_tmp.ingest_timestamp") else {
        return;
    };
    let Some(mut latest) = instant_at(event, "json.detected_at") else {
        return;
    };
    for path in ["json.resolved_at", "json.blocks_device_at"] {
        if !event.has_value(path) {
            continue;
        }
        let Some(candidate) = instant_at(event, path) else {
            return;
        };
        if candidate > latest {
            latest = candidate;
        }
    }
    if latest > now {
        latest = now;
    }
    // Java's `ISO_INSTANT` writes no fraction at all on a whole second, and
    // otherwise writes it in groups of three.
    let _ = event.set(
        "@timestamp",
        latest.to_rfc3339_opts(SecondsFormat::AutoSi, true),
    );
}

/// One ISO 8601 instant off the document, or `None` where `ZonedDateTime.parse`
/// would have raised.
fn instant_at(event: &Event, path: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(event.get_str(path)?).ok()
}

/// `kolide/issues`, processor `set_ecs_categorization`: the ECS categorisation
/// of an issue, and the action that says where in its life the issue sits.
///
/// `blocks_device_at` is a DEADLINE rather than a state. The device is blocked
/// only once that instant has passed relative to this event, which is the whole
/// difference between `blocked` and `will_be_blocked`.
fn issue_categorization(event: &mut Event, params: &Value) {
    let action = event.get_string("event.action");
    let row = action
        .as_deref()
        .and_then(|name| params.get("exact").and_then(|table| table.get(name)));
    let (kind, category) = match row {
        Some(row) => (
            row.get("kind").cloned().unwrap_or(Value::Null),
            row.get("category")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
        ),
        None => (Value::from("event"), vec![Value::from("configuration")]),
    };
    let _ = event.set("event.kind", kind);
    let _ = event.set("event.category", Value::Array(category));

    // A security-domain category keyed by the check, kept BESIDE the base one
    // rather than replacing it.
    if let Some(id) = event.get_as_string("rule.id")
        && let Some(domain) = params
            .get("check_category")
            .and_then(|table| table.get(&id))
            .cloned()
    {
        let _ = event.append_unique("event.category", domain);
    }

    let pending_block = event.has_value("kolide.issues.blocks_device_at");
    let mut blocked = false;
    if pending_block && event.has_value("@timestamp") {
        let (Some(block_at), Some(at)) = (
            event.get_string("kolide.issues.blocks_device_at"),
            event.get_string("@timestamp"),
        ) else {
            // `ZonedDateTime.parse` over something that is not a string raises,
            // and the rest of the script never runs.
            return;
        };
        let Some(before_deadline) = iso8601_is_before(&at, &block_at) else {
            return;
        };
        blocked = !before_deadline;
    }
    let resolved = event.has_value("kolide.issues.resolved_at")
        || action.as_deref() == Some("issues.resolved");

    let lifecycle = if pending_block || resolved {
        "change"
    } else {
        "creation"
    };
    let _ = event.set("event.type", Value::Array(vec![Value::from(lifecycle)]));

    // The fingerprint processor in the default pipeline reads this, so pending,
    // blocked and resolved each fingerprint to an id of their own.
    let _ = event.set("_tmp.blocked", blocked);

    // A resolved issue takes priority over one still blocking, and a pending
    // deadline over a plain open issue.
    if action.as_deref() == Some("issue") {
        let settled = if resolved {
            "resolved"
        } else if blocked {
            "blocked"
        } else if pending_block {
            "will_be_blocked"
        } else {
            return;
        };
        let _ = event.set("event.action", settled);
    }
}

/// `kolide/audit`, processor `set_ecs_categorization`: the audit action's ECS
/// categorisation, with a verb-derived fallback for an action the table has
/// not got.
///
/// The fallback is what carries a dynamic action such as an Okta event name,
/// which is written from the grok long after the lookup table was drawn up.
fn audit_categorization(event: &mut Event, params: &Value) {
    let _ = event.set("event.kind", "event");
    let Some(action) = event.get_string("event.action") else {
        return;
    };
    if let Some(row) = params.get("exact").and_then(|table| table.get(&action)) {
        let category = row
            .get("category")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let lifecycle = row
            .get("type")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let outcome = row.get("outcome").cloned();
        let _ = event.set("event.category", Value::Array(category));
        let _ = event.set("event.type", Value::Array(lifecycle));
        if let Some(outcome) = outcome {
            let _ = event.set("event.outcome", outcome);
        }
        return;
    }

    let lifecycle = if CREATION_SUFFIXES
        .iter()
        .any(|suffix| action.ends_with(suffix))
    {
        "creation"
    } else if DELETION_SUFFIXES
        .iter()
        .any(|suffix| action.ends_with(suffix))
    {
        "deletion"
    } else {
        "change"
    };
    let _ = event.set(
        "event.category",
        Value::Array(vec![Value::from("configuration")]),
    );
    let _ = event.set("event.type", Value::Array(vec![Value::from(lifecycle)]));
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "46874178d40278e7d579b84db17760e7fb181710715f86f6a6815e21c09c738c",
        source: "kolide",
        name: "issue_timestamp",
        run: issue_timestamp,
    },
    Entry {
        hash: "57cf020730ca8c35cc0267b0cc9164e4d681e63eeed61d731917e48ff9470dc3",
        source: "kolide",
        name: "issue_categorization",
        run: issue_categorization,
    },
    Entry {
        hash: "6a785318b983aabb2fd69cbe7c9de76549fea87d6a3f8f6c128ba0634aef685e",
        source: "kolide",
        name: "audit_categorization",
        run: audit_categorization,
    },
];
