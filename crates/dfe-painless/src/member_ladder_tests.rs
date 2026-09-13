// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The per-element severity ladder, read and run, in both vendor spellings.

use serde_json::json;

use super::*;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/google_secops_alert_v2/default.rs`, escapes and all.
const EQUALS_IGNORE_CASE: &str = r#"if (ctx.google_secops?.alert_v2?.event?.security_result instanceof List) {\n  for (list in ctx.google_secops?.alert_v2?.event.security_result) {\n    if (list[\"severity\"] != null && list[\"severity\"] != '') {\n      if (list[\"severity\"].equalsIgnoreCase('critical')) {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].equalsIgnoreCase('error')) {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].equalsIgnoreCase('high')) {\n        ctx.event.severity = 73\n      } else if (list[\"severity\"].equalsIgnoreCase('informational')) {\n        ctx.event.severity = 21\n      }  else if (list[\"severity\"].equalsIgnoreCase('low')) {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].equalsIgnoreCase('medium')) {\n        ctx.event.severity = 47\n      } else if (list[\"severity\"].equalsIgnoreCase('none')) {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].equalsIgnoreCase('unknown_severity')) {\n        ctx.event.severity = 21\n      }\n    }\n  }\n}"#;

/// Verbatim from `filebeat/google_secops_alert/default.rs` -- the same ladder
/// spelled as an upper-cased comparison against an upper-case literal.
const TO_UPPER_CASE: &str = r#"if (ctx.google_secops?.alert?.event?.securityResult instanceof List) {\n  for (list in ctx.google_secops?.alert?.event.securityResult) {\n    if (list[\"severity\"] != null && list[\"severity\"] != '') {\n      if (list[\"severity\"].toUpperCase() == 'CRITICAL') {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].toUpperCase() == 'ERROR') {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].toUpperCase() == 'HIGH') {\n        ctx.event.severity = 73\n      } else if (list[\"severity\"].toUpperCase() == 'INFORMATIONAL') {\n        ctx.event.severity = 21\n      }  else if (list[\"severity\"].toUpperCase() == 'LOW') {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].toUpperCase() == 'MEDIUM') {\n        ctx.event.severity = 47\n      } else if (list[\"severity\"].toUpperCase() == 'NONE') {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].toUpperCase() == 'UNKNOWN_SEVERITY') {\n        ctx.event.severity = 21\n      }\n    }\n  }\n}"#;

fn pattern(script: &str) -> MemberLadder {
    parse_member_ladder(&crate::common::normalise(script)).expect("declined the severity ladder")
}

/// The `equalsIgnoreCase` spelling reached `CaseInsensitiveLadder`, whose arm
/// returns the plan it built -- so the arm here has to sit AHEAD of it or the
/// script stays claimed by a reader that declines at run time.
#[test]
fn both_spellings_bind_to_this_arm() {
    for script in [EQUALS_IGNORE_CASE, TO_UPPER_CASE] {
        let binding = PainlessPlan::new(script).binding();
        assert_eq!(binding.len(), 1, "bound {binding:?}");
        assert!(binding[0].starts_with("MemberLadder"), "bound {binding:?}");
    }
}

#[test]
fn the_list_the_member_and_every_band_are_read_off_the_script() {
    let pattern = pattern(EQUALS_IGNORE_CASE);
    assert_eq!(pattern.list, "google_secops.alert_v2.event.security_result");
    assert_eq!(pattern.key, "severity");
    assert_eq!(pattern.target, "event.severity");
    assert_eq!(
        pattern.bands,
        [
            ("critical".to_owned(), json!(99)),
            ("error".to_owned(), json!(99)),
            ("high".to_owned(), json!(73)),
            ("informational".to_owned(), json!(21)),
            ("low".to_owned(), json!(21)),
            ("medium".to_owned(), json!(47)),
            ("none".to_owned(), json!(21)),
            ("unknown_severity".to_owned(), json!(21)),
        ]
    );
}

/// The two spellings differ only in the call and the case of their literals, so
/// they have to resolve to the same bands.
#[test]
fn the_upper_case_spelling_reads_the_same_bands() {
    let folded = pattern(TO_UPPER_CASE);
    assert_eq!(folded.bands, pattern(EQUALS_IGNORE_CASE).bands);
    assert_eq!(folded.list, "google_secops.alert.event.securityResult");
}

fn run(script: &str, document: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(document);
    assert!(member_ladder(&mut event, &pattern(script)));
    event.into_value()
}

#[test]
fn a_banded_word_is_written_whatever_its_case() {
    for script in [EQUALS_IGNORE_CASE, TO_UPPER_CASE] {
        let path = if script == TO_UPPER_CASE {
            json!({ "google_secops": { "alert": { "event": {
                "securityResult": [{ "severity": "HIGH" }] } } } })
        } else {
            json!({ "google_secops": { "alert_v2": { "event": {
                "security_result": [{ "severity": "high" }] } } } })
        };
        assert_eq!(run(script, path)["event"]["severity"], json!(73));
    }
}

/// The write is an assignment inside the loop, so the LAST element that matches
/// an arm decides the value -- not the first, and not the highest band.
#[test]
fn the_last_matching_element_wins() {
    let out = run(
        EQUALS_IGNORE_CASE,
        json!({ "google_secops": { "alert_v2": { "event": { "security_result": [
            { "severity": "critical" },
            { "severity": "medium" },
        ] } } } }),
    );
    assert_eq!(out["event"]["severity"], json!(47));
}

/// An element the ladder does not name matches no arm, so it leaves whatever an
/// earlier element wrote.
#[test]
fn an_unnamed_word_leaves_the_earlier_band_standing() {
    let out = run(
        EQUALS_IGNORE_CASE,
        json!({ "google_secops": { "alert_v2": { "event": { "security_result": [
            { "severity": "medium" },
            { "severity": "cataclysmic" },
        ] } } } }),
    );
    assert_eq!(out["event"]["severity"], json!(47));
}

/// The script's own guard skips an empty member, and an absent list runs
/// nothing at all -- neither writes the field.
#[test]
fn an_empty_member_and_an_absent_list_both_write_nothing() {
    for document in [
        json!({ "google_secops": { "alert_v2": { "event": { "security_result": [
            { "severity": "" },
        ] } } } }),
        json!({ "google_secops": { "alert_v2": { "event": {} } } }),
    ] {
        let out = run(EQUALS_IGNORE_CASE, document);
        assert!(out.get("event").is_none(), "wrote {out}");
    }
}

/// The scalar ladder's subject is a bare local read straight off the document.
/// Claiming one here would band a field by a member key it does not have.
#[test]
fn a_scalar_subject_declines() {
    assert!(
        parse_member_ladder(
            "for (s in ctx.a) { if (s.equalsIgnoreCase('high')) { ctx.b = 1 } else if (s.equalsIgnoreCase('low')) { ctx.b = 2 } }"
        )
        .is_none()
    );
}

/// Two arms writing different fields is a ladder this reader has not read, and
/// running it would drop one of them.
#[test]
fn arms_writing_different_fields_decline() {
    assert!(
        parse_member_ladder(
            "for (e in ctx.a) { if (e[\"k\"].equalsIgnoreCase('x')) { ctx.b = 1 } else if (e[\"k\"].equalsIgnoreCase('y')) { ctx.c = 2 } }"
        )
        .is_none()
    );
}

/// A closing `else` bands every word the ladder does not name, which this
/// reader resolves from a named word alone.
#[test]
fn a_closing_else_declines() {
    assert!(
        parse_member_ladder(
            "for (e in ctx.a) { if (e[\"k\"].equalsIgnoreCase('x')) { ctx.b = 1 } else if (e[\"k\"].equalsIgnoreCase('y')) { ctx.b = 2 } else { ctx.b = 3 } }"
        )
        .is_none()
    );
}

/// A loop body carrying a statement beside the ladder is a script this reader
/// has not read.
#[test]
fn work_beside_the_ladder_declines() {
    assert!(
        parse_member_ladder(
            "for (e in ctx.a) { if (e[\"k\"].equalsIgnoreCase('x')) { ctx.b = 1 } else if (e[\"k\"].equalsIgnoreCase('y')) { ctx.b = 2 } ctx.seen = true; }"
        )
        .is_none()
    );
}

/// A guard that SELECTS which elements are banded is not the presence test the
/// reader steps over, so stepping over it would band every element.
#[test]
fn a_selecting_guard_declines() {
    assert!(
        parse_member_ladder(
            "for (e in ctx.a) { if (e[\"kind\"] == 'primary') { if (e[\"k\"].equalsIgnoreCase('x')) { ctx.b = 1 } else if (e[\"k\"].equalsIgnoreCase('y')) { ctx.b = 2 } } }"
        )
        .is_none()
    );
}

/// A single band is an `if`, and every reader of a lone guarded write has a
/// better claim on it than a ladder with one arm.
#[test]
fn a_single_band_declines() {
    assert!(
        parse_member_ladder(
            "for (e in ctx.a) { if (e[\"k\"].equalsIgnoreCase('x')) { ctx.b = 1 } }"
        )
        .is_none()
    );
}
