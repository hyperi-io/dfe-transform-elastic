// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The per-member key/value fold, read and run.

use serde_json::json;

use super::*;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/google_secops_alert_v2/default.rs`.
const DETECTION: &str = r#"String[] kvFields = new String[] {\"detection_fields\", \"outcomes\", \"rule_labels\"};\nfor (def detection : ctx.google_secops.alert_v2.detection) {\n  for (def fieldName : kvFields) {\n    if (!(detection[fieldName] instanceof List)) {\n      continue;\n    }\n    def flat = new HashMap();\n    for (def entry : detection[fieldName]) {\n      if (entry?.key == null || entry.key == '') {\n        continue;\n      }\n      if (entry.value != null && entry.value != '') {\n        flat[entry.key] = entry.value;\n      } else if (entry.source != null && entry.source != '') {\n        flat[entry.key] = entry.source;\n      }\n    }\n    if (flat.isEmpty()) {\n      detection.remove(fieldName);\n    } else {\n      detection[fieldName] = flat;\n    }\n  }\n}\n"#;

fn pattern() -> MemberKvFold {
    parse_member_kv_fold(&crate::common::normalise(DETECTION)).expect("declined the detection fold")
}

#[test]
fn the_script_binds_to_this_arm() {
    let binding = PainlessPlan::new(DETECTION).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(binding[0].starts_with("MemberKvFold"), "bound {binding:?}");
}

#[test]
fn the_list_the_members_and_both_value_members_are_read_off_the_script() {
    let pattern = pattern();
    assert_eq!(pattern.list, "google_secops.alert_v2.detection");
    assert_eq!(
        pattern.members,
        ["detection_fields", "outcomes", "rule_labels"]
    );
    assert_eq!(pattern.key, "key");
    assert_eq!(pattern.values, ["value", "source"]);
}

fn run(document: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(document);
    assert!(member_kv_fold(&mut event, &pattern()));
    event.into_value()
}

fn detection(record: &serde_json::Value) -> serde_json::Value {
    run(json!({ "google_secops": { "alert_v2": { "detection": [record] } } }))["google_secops"]
        ["alert_v2"]["detection"][0]
        .clone()
}

#[test]
fn each_named_member_folds_into_one_map() {
    let out = detection(&json!({
        "alert_state": "ALERTING",
        "detection_fields": [{ "key": "correlation_ip", "value": "1.128.0.0" }],
        "outcomes": [
            { "key": "risk_score", "value": "65" },
            { "key": "vendor_name", "value": "Google Cloud Platform" },
        ],
    }));
    assert_eq!(
        out["detection_fields"],
        json!({ "correlation_ip": "1.128.0.0" })
    );
    assert_eq!(
        out["outcomes"],
        json!({ "risk_score": "65", "vendor_name": "Google Cloud Platform" })
    );
    // A member the script does not name is left exactly as it stands.
    assert_eq!(out["alert_state"], json!("ALERTING"));
}

/// `value` is tried first and `source` only where it holds nothing, which is
/// what the arm order says.
#[test]
fn source_stands_in_where_value_holds_nothing() {
    let out = detection(&json!({
        "rule_labels": [
            { "key": "author", "value": "", "source": "GreyNoise" },
            { "key": "description", "value": "Detects events", "source": "ignored" },
            { "key": "orphan" },
        ],
    }));
    assert_eq!(
        out["rule_labels"],
        json!({ "author": "GreyNoise", "description": "Detects events" })
    );
}

/// An empty fold REMOVES the member: Elasticsearch emits no `detection_fields`
/// at all, and writing `{}` there would be a different document.
#[test]
fn a_member_whose_entries_all_fail_is_removed() {
    let out = detection(&json!({
        "alert_state": "ALERTING",
        "detection_fields": [{ "key": "", "value": "x" }, { "value": "y" }],
    }));
    assert!(out.get("detection_fields").is_none(), "kept {out}");
    assert_eq!(out["alert_state"], json!("ALERTING"));
}

/// `!(x instanceof List)` continues, so a member the record does not carry and
/// one already folded are both left alone.
#[test]
fn an_absent_or_already_folded_member_is_left_alone() {
    let out = detection(&json!({ "outcomes": { "risk_score": "65" } }));
    assert_eq!(out["outcomes"], json!({ "risk_score": "65" }));
    assert!(out.get("rule_labels").is_none());
}

/// Painless compares a number against `''` by value, so a zero is a usable
/// value and only a null or an empty string is refused.
#[test]
fn a_zero_is_a_usable_value() {
    let out = detection(&json!({
        "outcomes": [{ "key": "count", "value": 0 }, { "key": "gone", "value": null }],
    }));
    assert_eq!(out["outcomes"], json!({ "count": 0 }));
}

/// An absent list runs nothing, and the field stays absent rather than being
/// created empty.
#[test]
fn an_absent_list_writes_nothing() {
    let out = run(json!({ "google_secops": { "alert_v2": {} } }));
    assert!(out["google_secops"]["alert_v2"].get("detection").is_none());
}

/// The middle loop has to walk the array the script declared; one walking
/// anything else is a different script.
#[test]
fn a_middle_loop_over_something_else_declines() {
    let script = crate::common::normalise(DETECTION).replace(": kvFields)", ": ctx.other)");
    assert!(parse_member_kv_fold(&script).is_none());
}

/// Without the remove branch the empty fold would write an empty map, so a
/// script missing it is not this pattern.
#[test]
fn a_fold_that_never_removes_declines() {
    let script = crate::common::normalise(DETECTION).replace(
        "detection.remove(fieldName);",
        "detection[fieldName] = flat;",
    );
    assert!(parse_member_kv_fold(&script).is_none());
}
