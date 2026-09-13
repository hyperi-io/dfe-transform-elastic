// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The label classifier, read and run.

use serde_json::json;

use super::*;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/darktrace_model_breach_alert/default.rs`.
const DARKTRACE: &str = r#"for (component in ctx.json.triggeredComponents) { if (component?.metric?.label?.toLowerCase().contains('connection')) { ctx.event?.type?.add('connection'); if (ctx.event.category == null) { ctx.event.category = new ArrayList(); } ctx.event.category.add('network'); } }"#;

fn pattern() -> MemberTags {
    parse_member_tags(DARKTRACE).expect("declined darktrace's label classifier")
}

/// The arm has to be reachable. The script spells `.add(`, so the one hard stop
/// that could swallow it is `AppendEach` -- and that arm asks for a
/// `.splitOnToken(` or an `instanceof Map` BEFORE the `.add(`, neither of which
/// is here.
#[test]
fn the_ladder_binds_the_script_to_this_arm() {
    let binding = PainlessPlan::new(DARKTRACE).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(binding[0].starts_with("MemberTags"), "bound {binding:?}");
}

#[test]
fn the_loop_the_test_and_both_appends_are_read_off_the_script() {
    let pattern = pattern();
    assert_eq!(pattern.list, "json.triggeredComponents");
    assert_eq!(pattern.member, "metric.label");
    assert_eq!(pattern.word, "connection");
    assert!(pattern.lower);
    assert_eq!(
        pattern
            .appends
            .iter()
            .map(|append| (
                append.target.as_str(),
                append.value.as_str(),
                append.create,
                append.null_safe
            ))
            .collect::<Vec<_>>(),
        [
            ("event.type", "connection", false, true),
            ("event.category", "network", true, false),
        ]
    );
}

/// A body carrying a statement this cannot run declines, rather than being
/// claimed and half-applied.
#[test]
fn a_loop_doing_more_than_appending_declines() {
    assert!(
        parse_member_tags(
            "for (c in ctx.a) { if (c?.x.contains('y')) { ctx.b.add('z'); ctx.seen = true; } }"
        )
        .is_none()
    );
}

fn run(document: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(document);
    assert!(member_tags(&mut event, &pattern()));
    event.into_value()
}

/// The corpus case: `event.type` already exists and is appended to,
/// `event.category` does not and is created first.
#[test]
fn a_matching_label_appends_to_one_list_and_creates_the_other() {
    let out = run(json!({
        "event": { "type": ["info"] },
        "json": { "triggeredComponents": [{ "metric": { "label": "External Connections" } }] }
    }));
    assert_eq!(out["event"]["type"], json!(["info", "connection"]));
    assert_eq!(out["event"]["category"], json!(["network"]));
}

/// The other corpus case: an alert already carries `threat`, and `network`
/// joins it rather than replacing it.
#[test]
fn an_existing_category_is_appended_to() {
    let out = run(json!({
        "event": { "type": ["info"], "category": ["threat"] },
        "json": { "triggeredComponents": [{ "metric": { "label": "Connections" } }] }
    }));
    assert_eq!(out["event"]["type"], json!(["info", "connection"]));
    assert_eq!(out["event"]["category"], json!(["threat", "network"]));
}

#[test]
fn a_label_without_the_word_writes_nothing() {
    let out = run(json!({
        "event": { "type": ["info"] },
        "json": { "triggeredComponents": [{ "metric": { "label": "System" } }] }
    }));
    assert_eq!(out["event"], json!({ "type": ["info"] }));
}

/// `ctx.event?.type?.add(...)` is null-safe, so an absent `event.type` stays
/// absent. Reading the two appends alike would create it.
#[test]
fn a_null_safe_append_leaves_an_absent_list_absent() {
    let out = run(json!({
        "event": {},
        "json": { "triggeredComponents": [{ "metric": { "label": "Connections" } }] }
    }));
    assert!(out["event"].get("type").is_none());
    assert_eq!(out["event"]["category"], json!(["network"]));
}

/// `component?.metric?.label?.toLowerCase()` is null when any link breaks, and
/// `.contains` on null throws -- so the loop stops where the vendor's does.
#[test]
fn a_record_with_no_label_stops_the_walk() {
    let out = run(json!({
        "event": { "type": ["info"] },
        "json": { "triggeredComponents": [
            { "metric": { "label": "Connections" } },
            { "metric": {} },
            { "metric": { "label": "Connections" } }
        ] }
    }));
    assert_eq!(out["event"]["type"], json!(["info", "connection"]));
    assert_eq!(out["event"]["category"], json!(["network"]));
}

#[test]
fn an_absent_list_writes_nothing() {
    let out = run(json!({ "event": { "type": ["info"] } }));
    assert_eq!(out, json!({ "event": { "type": ["info"] } }));
}
