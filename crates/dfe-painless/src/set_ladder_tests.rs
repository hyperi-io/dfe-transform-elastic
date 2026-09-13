// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The category and type collectors, read and run, in both vendor spellings.

use serde_json::json;

use super::*;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/trend_micro_vision_one_alert/default.rs` -- one
/// subject, one chain, a closing `else`.
const ALERT: &str = r#"def eventCategory = new HashSet();\ndef eventType = new HashSet();\ndef description = ctx.trend_micro_vision_one.alert.description.toLowerCase();\nif (description.contains('logon')) {\n  eventCategory.add('authentication');\n  eventCategory.add('host');\n  eventType.add('info');\n} else if (description.contains('email')) {\n  eventCategory.add('email');\n  eventType.add('info');\n} else if (description.contains('network')) {\n  eventCategory.add('network');\n  eventType.add('info');\n} else {\n  eventCategory.add('malware');\n  eventType.add('info');\n}\nif (!eventCategory.isEmpty()) {\n  ctx.event.category = eventCategory;\n}\nif (!eventType.isEmpty()) {\n  ctx.event.type = eventType;\n}\n"#;

/// Verbatim from `filebeat/trend_micro_vision_one_audit/default.rs` -- two
/// subjects, independent `if`s, list membership, and one nested chain.
const AUDIT: &str = r#"def eventCategory = new HashSet();\ndef eventType = new HashSet();\ndef category = ctx.trend_micro_vision_one.audit.category.toLowerCase();\ndef activity = ctx.trend_micro_vision_one.audit.activity.toLowerCase();\nif (['logon and logoff', 'saml single sign-on'].contains(category)) {\n  eventCategory.add('authentication');\n  if (['log on', 'enable single sign-on'].contains(activity)) {\n    eventType.add('start');\n  }\n  if (['log off', 'disable single sign-on'].contains(activity)) {\n    eventType.add('end');\n  } else {\n    eventType.add('info');\n  }\n}\nif (['account management', 'product connector', 'Notifications', 'detection model management'].contains(category)) {\n  eventCategory.add('authentication');\n  eventType.add('info');\n}\nif (category == 'network inventory') {\n  eventCategory.add('network');\n  eventType.add('info');\n}\nif (activity == 'email') {\n  eventCategory.add('email');\n}\nif (!eventCategory.isEmpty()) {\n  ctx.event.category = eventCategory;\n}\nif (!eventType.isEmpty()) {\n  ctx.event.type = eventType;\n}\n"#;

fn pattern(script: &str) -> SetLadder {
    parse_set_ladder(&crate::common::normalise(script)).expect("declined the collector")
}

#[test]
fn both_streams_bind_to_this_arm() {
    for script in [ALERT, AUDIT] {
        let binding = PainlessPlan::new(script).binding();
        assert_eq!(binding.len(), 1, "bound {binding:?}");
        assert!(binding[0].starts_with("SetLadder"), "bound {binding:?}");
    }
}

#[test]
fn the_subjects_and_both_targets_are_read_off_the_script() {
    let alert = pattern(ALERT);
    assert_eq!(
        alert.subjects,
        [Subject {
            path: "trend_micro_vision_one.alert.description".to_owned(),
            folded: true
        }]
    );
    assert_eq!(alert.targets, ["event.category", "event.type"]);
    assert_eq!(alert.chains.len(), 1);
    assert_eq!(alert.chains[0].arms.len(), 4);

    let audit = pattern(AUDIT);
    assert_eq!(
        audit
            .subjects
            .iter()
            .map(|s| s.path.as_str())
            .collect::<Vec<_>>(),
        [
            "trend_micro_vision_one.audit.category",
            "trend_micro_vision_one.audit.activity"
        ]
    );
    assert_eq!(audit.chains.len(), 4);
}

fn run(script: &str, document: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(document);
    assert!(set_ladder(&mut event, &pattern(script)));
    event.into_value()
}

fn alert(description: &str) -> serde_json::Value {
    run(
        ALERT,
        json!({ "trend_micro_vision_one": { "alert": { "description": description } } }),
    )
}

/// The corpus case: the set is handed to the serialiser, which walks the hash
/// TABLE -- so `add('authentication')` then `add('host')` comes out host first.
#[test]
fn a_two_word_category_comes_out_in_the_tables_order() {
    let out = alert(
        "A user obtained account logon information that can be used to access remote systems",
    );
    assert_eq!(out["event"]["category"], json!(["host", "authentication"]));
    assert_eq!(out["event"]["type"], json!(["info"]));
}

#[test]
fn the_first_arm_that_holds_is_the_only_one_that_runs() {
    let out =
        alert("A backdoor was implanted after a user received a spear phishing email message");
    assert_eq!(out["event"]["category"], json!(["email"]));
    assert_eq!(out["event"]["type"], json!(["info"]));
}

/// The chain's closing `else` is what a description naming none of the words
/// takes, so the pair is never left unwritten.
#[test]
fn a_description_naming_no_word_takes_the_closing_else() {
    let out = alert("Something else entirely");
    assert_eq!(out["event"]["category"], json!(["malware"]));
    assert_eq!(out["event"]["type"], json!(["info"]));
}

/// The subject is bound through `toLowerCase()`, so the test is against the
/// folded text and the vendor's own casing does not decide the arm.
#[test]
fn the_subject_is_folded_before_the_test() {
    assert_eq!(
        alert("SUSPICIOUS EMAIL")["event"]["category"],
        json!(["email"])
    );
}

/// The corpus cases: the audit stream's two rows, one through the nested
/// `else` and one through the second list.
#[test]
fn the_audit_streams_two_rows_come_out_as_elasticsearch_wrote_them() {
    for (category, activity) in [
        ("Logon and Logoff", "string"),
        ("Product Connector", "Unregister product"),
    ] {
        let out = run(
            AUDIT,
            json!({ "trend_micro_vision_one": { "audit": {
                "category": category, "activity": activity } } }),
        );
        assert_eq!(out["event"]["category"], json!(["authentication"]));
        assert_eq!(out["event"]["type"], json!(["info"]));
    }
}

/// The nested `else` hangs off the `log off` test, not the outer arm, so a
/// logon writes BOTH words.
#[test]
fn a_logon_activity_collects_start_beside_the_nested_else() {
    let out = run(
        AUDIT,
        json!({ "trend_micro_vision_one": { "audit": {
            "category": "Logon and Logoff", "activity": "Log on" } } }),
    );
    assert_eq!(out["event"]["category"], json!(["authentication"]));
    assert_eq!(out["event"]["type"], json!(["start", "info"]));
}

/// A vendor list member cased against a folded subject can never match, and
/// folding it here would write a category Elasticsearch does not.
#[test]
fn a_list_member_the_vendor_left_cased_never_matches() {
    let out = run(
        AUDIT,
        json!({ "trend_micro_vision_one": { "audit": {
            "category": "Notifications", "activity": "string" } } }),
    );
    assert!(out.get("event").is_none(), "wrote {out}");
}

/// Every arm of the audit script is a separate `if`, so a row naming two of
/// them collects from both -- and the pair comes out in the table's order,
/// which reverses the order the script added them in.
#[test]
fn independent_arms_all_run() {
    let out = run(
        AUDIT,
        json!({ "trend_micro_vision_one": { "audit": {
            "category": "Network Inventory", "activity": "Email" } } }),
    );
    assert_eq!(out["event"]["category"], json!(["email", "network"]));
    assert_eq!(out["event"]["type"], json!(["info"]));
}

/// The processor is guarded on its subjects, so reaching the runner without one
/// means the guard held -- writing a default pair would be a category the
/// vendor never computed.
#[test]
fn an_absent_subject_writes_nothing() {
    let out = run(ALERT, json!({ "trend_micro_vision_one": { "alert": {} } }));
    assert!(out.get("event").is_none(), "wrote {out}");
}

/// A statement this reader cannot run declines the WHOLE parse, rather than
/// being claimed and half-applied.
#[test]
fn work_beside_the_collectors_declines() {
    assert!(
        parse_set_ladder(
            "def c = new HashSet();\ndef s = ctx.a.toLowerCase();\nif (s.contains('x')) { c.add('y'); }\nctx.seen = true;\nif (!c.isEmpty()) { ctx.b = c; }"
        )
        .is_none()
    );
}

/// An accumulator the script fills and never writes is work this reader would
/// drop, so the parse declines rather than the one set.
#[test]
fn an_accumulator_with_no_write_back_declines() {
    assert!(
        parse_set_ladder(
            "def c = new HashSet();\ndef d = new HashSet();\ndef s = ctx.a.toLowerCase();\nif (s.contains('x')) { c.add('y'); d.add('z'); }\nif (!c.isEmpty()) { ctx.b = c; }"
        )
        .is_none()
    );
}

/// An arm adding a value the script computes is not a word this reader holds,
/// and taking the first quoted run out of it would add a word it never adds.
#[test]
fn an_arm_adding_a_computed_value_declines() {
    assert!(
        parse_set_ladder(
            "def c = new HashSet();\ndef s = ctx.a.toLowerCase();\nif (s.contains('x')) { c.add(s + 'y'); }\nif (!c.isEmpty()) { ctx.b = c; }"
        )
        .is_none()
    );
}

/// A `new HashSet(<seed>)` starts from what the document already holds, which
/// this reader does not read.
#[test]
fn a_seeded_set_declines() {
    assert!(
        parse_set_ladder(
            "def c = new HashSet(ctx.b);\ndef s = ctx.a.toLowerCase();\nif (s.contains('x')) { c.add('y'); }\nif (!c.isEmpty()) { ctx.b = c; }"
        )
        .is_none()
    );
}
