// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The member rename with a scalar wrap, read and run.

use serde_json::json;

use super::*;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/trend_micro_vision_one_alert/default.rs`.
const ENTITIES: &str = r#"for (def entity : ctx.trend_micro_vision_one.alert.impact_scope.entities) {\n  if (entity.containsKey('entity_value')) {\n    def ev = entity.remove('entity_value');\n    if (ev instanceof Map) {\n      entity.put('value', ev);\n    } else {\n      entity.put('value', ['account_value': ev]);\n    }\n  }\n}\n"#;

fn pattern() -> MemberValueWrap {
    parse_member_value_wrap(&crate::common::normalise(ENTITIES)).expect("declined the entity wrap")
}

#[test]
fn the_script_binds_to_this_arm() {
    let binding = PainlessPlan::new(ENTITIES).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(
        binding[0].starts_with("MemberValueWrap"),
        "bound {binding:?}"
    );
}

#[test]
fn the_walk_the_rename_and_the_wrapping_key_are_read_off_the_script() {
    let pattern = pattern();
    assert_eq!(
        pattern.list,
        "trend_micro_vision_one.alert.impact_scope.entities"
    );
    assert_eq!(pattern.from, "entity_value");
    assert_eq!(pattern.to, "value");
    assert_eq!(pattern.wrap, "account_value");
}

fn run(document: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(document);
    assert!(member_value_wrap(&mut event, &pattern()));
    event.into_value()
}

fn entities(records: &serde_json::Value) -> serde_json::Value {
    run(
        json!({ "trend_micro_vision_one": { "alert": { "impact_scope": { "entities": records } } } }),
    )["trend_micro_vision_one"]["alert"]["impact_scope"]["entities"]
        .clone()
}

/// The two corpus cases, in one walk: a scalar takes the wrapping key and a map
/// is moved across whole.
#[test]
fn a_scalar_is_wrapped_and_a_map_is_moved_whole() {
    let out = entities(&json!([
        { "entity_type": "account", "entity_value": "desktop-example\\dummy" },
        { "entity_type": "host", "entity_value": { "guid": "ABC", "ips": ["81.2.69.192"] } },
    ]));
    assert_eq!(
        out[0]["value"],
        json!({ "account_value": "desktop-example\\dummy" })
    );
    assert_eq!(
        out[1]["value"],
        json!({ "guid": "ABC", "ips": ["81.2.69.192"] })
    );
    assert!(out[0].get("entity_value").is_none());
    assert!(out[1].get("entity_value").is_none());
}

/// Remove-then-put leaves the renamed key at the END, and every other member
/// keeps the place the vendor gave it.
#[test]
fn the_renamed_member_lands_last_and_the_rest_keep_their_places() {
    let out = entities(&json!([
        { "entity_type": "host", "entity_value": "x", "related_entities": ["y"] },
    ]));
    let keys: Vec<&String> = out[0].as_object().unwrap().keys().collect();
    assert_eq!(keys, ["entity_type", "related_entities", "value"]);
}

/// `containsKey` is the whole guard, so a record without the member is left
/// exactly as it stands rather than gaining an empty one.
#[test]
fn a_record_without_the_member_is_left_alone() {
    let out = entities(&json!([{ "entity_type": "host" }]));
    assert_eq!(out[0], json!({ "entity_type": "host" }));
}

/// A list a record carries is not a map, so it takes the wrapping key the same
/// way a string does.
#[test]
fn a_list_is_wrapped_like_any_other_scalar() {
    let out = entities(&json!([{ "entity_value": ["a", "b"] }]));
    assert_eq!(out[0]["value"], json!({ "account_value": ["a", "b"] }));
}

#[test]
fn an_absent_list_writes_nothing() {
    let out = run(json!({ "trend_micro_vision_one": { "alert": {} } }));
    assert!(
        out["trend_micro_vision_one"]["alert"]
            .get("impact_scope")
            .is_none()
    );
}

/// The two arms writing different members is a script choosing between two
/// names, and this reader has read only one of them.
#[test]
fn arms_writing_different_members_decline() {
    let script = crate::common::normalise(ENTITIES).replace(
        "entity.put('value', ['account_value'",
        "entity.put('other', ['account_value'",
    );
    assert!(parse_member_value_wrap(&script).is_none());
}

/// A loop doing anything beside the rename is a script this reader has not
/// read.
#[test]
fn work_beside_the_rename_declines() {
    let script =
        crate::common::normalise(ENTITIES).replace("  }\n}", "  }\n  entity.seen = true;\n}");
    assert!(parse_member_value_wrap(&script).is_none());
}

/// The type-gated rename is the conditional half of the same idea and leaves
/// the other spelling alone, so it must keep declining this script.
#[test]
fn the_type_gated_rename_still_declines_this_script() {
    assert!(
        crate::typed_member_rename::parse_typed_member_rename(&crate::common::normalise(ENTITIES))
            .is_none()
    );
}
