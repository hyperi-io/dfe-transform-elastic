// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The acknowledgement ladder, read and run.

use serde_json::json;

use super::*;
use crate::common::normalise;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/darktrace_model_breach_alert/default.rs`, escapes and
/// all: a stored script arrives with its newlines escaped, so this is the text
/// the ladder is actually handed.
const DARKTRACE: &str = r#"ctx.darktrace = ctx.darktrace ?: [:];\nctx.darktrace.model_breach_alert = ctx.darktrace?.model_breach_alert ?: [:];\nif (ctx.darktrace?.model_breach_alert?.acknowledged == null) {\n  ctx.darktrace.model_breach_alert.is_acknowledged = false;\n  return;\n}\nif (!(ctx.darktrace.model_breach_alert.acknowledged instanceof Map)) {\n  // It appears that some versions of the data from Darktrace\n  // have the value at acknowledged as a boolean. Don't handle\n  // that here. Rename it to is_acknowledged below.\n  return;\n}\nif (ctx.darktrace.model_breach_alert.acknowledged.time == null) {\n  // No time, so be non-commital.\n  return;\n}\nif (ctx.darktrace.model_breach_alert.time == null) {\n  // Assume any time noted in json.acknowledged.time\n  // is in the past.\n  ctx.darktrace.model_breach_alert.is_acknowledged = true;\n  return;\n}\ndef time = ctx.darktrace.model_breach_alert.time;\ndef acknowledged = ctx.darktrace.model_breach_alert.acknowledged.time;\nctx.darktrace.model_breach_alert.is_acknowledged = ZonedDateTime.parse(acknowledged).isBefore(ZonedDateTime.parse(time));\n"#;

fn pattern() -> TimeOrderFlag {
    parse_time_order_flag(&normalise(DARKTRACE)).expect("declined darktrace's acknowledgement")
}

/// The arm has to be reachable. The script spells neither `.add(` nor
/// `.replace(`, so neither hard stop is in its path.
#[test]
fn the_ladder_binds_the_script_to_this_arm() {
    let binding = PainlessPlan::new(DARKTRACE).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(binding[0].starts_with("TimeOrderFlag"), "bound {binding:?}");
}

#[test]
fn every_path_comes_off_the_script() {
    let pattern = pattern();
    assert_eq!(
        pattern.ensure,
        ["darktrace", "darktrace.model_breach_alert"]
    );
    assert_eq!(
        pattern.container,
        "darktrace.model_breach_alert.acknowledged"
    );
    assert_eq!(
        pattern.earlier,
        "darktrace.model_breach_alert.acknowledged.time"
    );
    assert_eq!(pattern.later, "darktrace.model_breach_alert.time");
    assert_eq!(pattern.flag, "darktrace.model_breach_alert.is_acknowledged");
}

/// The final comparison has to be over the two instants the guards named. A
/// script comparing something else means something else.
#[test]
fn a_comparison_over_other_instants_declines() {
    let swapped = normalise(DARKTRACE).replace(
        "ZonedDateTime.parse(acknowledged).isBefore(ZonedDateTime.parse(time))",
        "ZonedDateTime.parse(time).isBefore(ZonedDateTime.parse(acknowledged))",
    );
    assert!(parse_time_order_flag(&swapped).is_none());
}

fn run(document: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(document);
    assert!(time_order_flag(&mut event, &pattern()));
    event.into_value()
}

fn flag(document: serde_json::Value) -> Option<bool> {
    run(document)["darktrace"]["model_breach_alert"]
        .get("is_acknowledged")
        .and_then(Value::as_bool)
}

/// Arm one, on six of the stream's eleven events: nothing acknowledged it.
#[test]
fn an_absent_record_answers_false() {
    assert_eq!(
        flag(
            json!({ "darktrace": { "model_breach_alert": { "time": "2022-07-13T02:12:46.000Z" } } })
        ),
        Some(false)
    );
    assert_eq!(
        flag(json!({
            "darktrace": { "model_breach_alert": { "acknowledged": null, "time": "2022-07-13T02:12:46.000Z" } }
        })),
        Some(false)
    );
}

/// Arm two: some versions of the vendor send a bare boolean here, and the
/// pipeline renames that one instead. Writing anything would be a guess.
#[test]
fn a_boolean_record_answers_nothing() {
    assert_eq!(
        flag(json!({
            "darktrace": { "model_breach_alert": { "acknowledged": true, "time": "2022-07-13T02:12:46.000Z" } }
        })),
        None
    );
}

/// Arm three, and the event that makes it visible: acknowledged by someone, with
/// no instant recorded. Elasticsearch leaves the field absent.
#[test]
fn a_record_with_no_instant_answers_nothing() {
    assert_eq!(
        flag(json!({
            "darktrace": { "model_breach_alert": {
                "acknowledged": { "username": "user@company" },
                "time": "2022-07-13T02:12:46.000Z"
            } }
        })),
        None
    );
}

/// Arm four: any instant recorded against an undated breach is in the past.
#[test]
fn a_breach_with_no_instant_answers_true() {
    assert_eq!(
        flag(json!({
            "darktrace": { "model_breach_alert": {
                "acknowledged": { "time": "2022-07-13T02:12:45.000Z" }
            } }
        })),
        Some(true)
    );
}

/// Arm five, over the three corpus events that reach it: before is true, and
/// both same-instant and after are false.
#[test]
fn the_comparison_is_strictly_before() {
    let at = |acknowledged: &str, breach: &str| {
        flag(json!({
            "darktrace": { "model_breach_alert": {
                "acknowledged": { "time": acknowledged },
                "time": breach
            } }
        }))
    };
    assert_eq!(
        at("2022-07-13T02:12:45.000Z", "2022-07-13T02:12:46.000Z"),
        Some(true)
    );
    assert_eq!(
        at("2022-07-13T02:12:46.000Z", "2022-07-13T02:12:46.000Z"),
        Some(false)
    );
    assert_eq!(
        at("2022-07-13T02:12:47.000Z", "2022-07-13T02:12:46.000Z"),
        Some(false)
    );
    // Offsets compare as instants, the way `ZonedDateTime` does.
    assert_eq!(
        at("2022-07-13T04:12:45.000+02:00", "2022-07-13T02:12:46.000Z"),
        Some(true)
    );
}

/// `ZonedDateTime.parse` throws on text it cannot read, and a thrown processor
/// has written nothing.
#[test]
fn an_unreadable_instant_writes_nothing() {
    assert_eq!(
        flag(json!({
            "darktrace": { "model_breach_alert": {
                "acknowledged": { "time": "yesterday" },
                "time": "2022-07-13T02:12:46.000Z"
            } }
        })),
        None
    );
}

/// The two `?: [:]` statements are the parents Painless will not make for
/// itself, so the runner makes them too.
#[test]
fn the_parents_the_script_ensures_are_created() {
    let out = run(json!({}));
    assert_eq!(
        out["darktrace"]["model_breach_alert"]["is_acknowledged"],
        json!(false)
    );
}
