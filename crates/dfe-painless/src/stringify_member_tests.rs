// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The list-member stringifier, read and run.

use serde_json::json;

use super::*;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/darktrace_model_breach_alert/default.rs`.
const DARKTRACE: &str = r#"for (component in ctx.json.triggeredComponents) { component.logic.data = component?.logic?.data.toString(); }"#;

/// The whole point of the arm: the ladder has to reach it. Nothing above claims
/// this script -- it spells no `.add(` and no `.replace(`, so neither hard stop
/// is in its path.
#[test]
fn the_ladder_binds_the_script_to_this_arm() {
    let binding = PainlessPlan::new(DARKTRACE).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(
        binding[0].starts_with("StringifyMember"),
        "bound {binding:?}"
    );
}

#[test]
fn the_list_and_the_member_are_read_off_the_script() {
    let pattern = parse_stringify_member(DARKTRACE).expect("declined darktrace's stringifier");
    assert_eq!(pattern.list, "json.triggeredComponents");
    assert_eq!(pattern.member, "logic.data");
}

/// A loop that renders one member ONTO ANOTHER means something else, and running
/// it as this would put the text where the vendor does not.
#[test]
fn a_loop_writing_a_different_member_declines() {
    assert!(
        parse_stringify_member("for (c in ctx.a.b) { c.logic.text = c?.logic?.data.toString(); }")
            .is_none()
    );
}

/// The body has to be the one statement this reproduces.
#[test]
fn a_loop_doing_more_than_the_one_write_declines() {
    assert!(
        parse_stringify_member(
            "for (c in ctx.a.b) { c.logic.data = c?.logic?.data.toString(); c.seen = true; }"
        )
        .is_none()
    );
}

/// The rendering Elasticsearch actually published, verbatim from the corpus
/// capture of `darktrace/model_breach_alert` event 10.
///
/// The payload spells the keys `left, operator, right` and Elasticsearch writes
/// `left, right, operator` at EVERY level -- buckets 5, 11 and 14 of a 16-entry
/// table. This is the whole reason the renderer models `HashMap` order.
#[test]
fn the_tree_renders_the_way_elasticsearch_published_it() {
    let data = json!({
        "left": {
            "left": "A",
            "operator": "AND",
            "right": { "left": "B", "operator": "AND", "right": "C" }
        },
        "operator": "OR",
        "right": {
            "left": "A",
            "operator": "AND",
            "right": { "left": "B", "operator": "AND", "right": "D" }
        }
    });
    assert_eq!(
        java_text(&data),
        "{left={left=A, right={left=B, right=C, operator=AND}, operator=AND}, \
         right={left=A, right={left=B, right=D, operator=AND}, operator=AND}, operator=OR}"
    );
}

#[test]
fn a_list_renders_with_java_brackets_and_no_quotes() {
    assert_eq!(java_text(&json!(["a", "b"])), "[a, b]");
    assert_eq!(java_text(&json!([1, [2, 3]])), "[1, [2, 3]]");
    assert_eq!(java_text(&json!("plain text")), "plain text");
    assert_eq!(java_text(&json!(true)), "true");
    assert_eq!(java_text(&json!(Option::<i32>::None)), "null");
}

/// A whole double keeps the `.0` Java prints and Rust drops.
#[test]
fn numbers_render_the_way_java_prints_them() {
    assert_eq!(java_text(&json!(1594)), "1594");
    assert_eq!(java_text(&json!(-7)), "-7");
    assert_eq!(java_text(&json!(1.0)), "1.0");
    assert_eq!(java_text(&json!(0.674)), "0.674");
}

/// A table that has grown moves keys between buckets, so the order moves with
/// it. Thirteen entries is the first size past three quarters of sixteen.
#[test]
fn the_table_grows_at_three_quarters_full() {
    assert_eq!(table_size(0), 16);
    assert_eq!(table_size(12), 16);
    assert_eq!(table_size(13), 32);
    assert_eq!(table_size(24), 32);
    assert_eq!(table_size(25), 64);
}

/// Java hashes UTF-16 code units, which is what decides every bucket.
#[test]
fn the_hash_is_javas() {
    assert_eq!(java_hash("left"), 3_317_767);
    assert_eq!(java_hash("right"), 108_511_772);
    assert_eq!(java_hash(""), 0);
    assert_eq!(bucket("left", 16), 5);
    assert_eq!(bucket("right", 16), 11);
    assert_eq!(bucket("operator", 16), 14);
}

fn run(document: serde_json::Value) -> serde_json::Value {
    let pattern = parse_stringify_member(DARKTRACE).expect("declined darktrace's stringifier");
    let mut event = Event::new(document);
    assert!(stringify_member(&mut event, &pattern));
    event.into_value()
}

#[test]
fn every_record_gets_its_member_rendered() {
    let out = run(json!({
        "json": { "triggeredComponents": [
            { "logic": { "data": { "left": "A", "operator": "AND", "right": "B" } } },
            { "logic": { "data": ["x", "y"] } }
        ] }
    }));
    let records = out["json"]["triggeredComponents"].as_array().unwrap();
    assert_eq!(
        records[0]["logic"]["data"],
        json!("{left=A, right=B, operator=AND}")
    );
    assert_eq!(records[1]["logic"]["data"], json!("[x, y]"));
}

/// `null.toString()` throws, and an ingest processor that throws keeps the
/// writes it already made and abandons the rest. `darktrace`'s test-model event
/// is exactly this, and Elasticsearch leaves its `logic` map empty.
#[test]
fn a_null_member_stops_the_walk_and_keeps_what_ran() {
    let out = run(json!({
        "json": { "triggeredComponents": [
            { "logic": { "data": { "left": "A" } } },
            { "logic": { "data": null } },
            { "logic": { "data": { "left": "B" } } }
        ] }
    }));
    let records = out["json"]["triggeredComponents"].as_array().unwrap();
    assert_eq!(records[0]["logic"]["data"], json!("{left=A}"));
    assert_eq!(records[1]["logic"]["data"], json!(null));
    assert_eq!(records[2]["logic"]["data"], json!({ "left": "B" }));
}

#[test]
fn a_missing_member_stops_the_walk() {
    let out = run(json!({
        "json": { "triggeredComponents": [{ "metric": { "label": "System" } }] }
    }));
    assert_eq!(
        out["json"]["triggeredComponents"],
        json!([{ "metric": { "label": "System" } }])
    );
}

#[test]
fn an_absent_list_writes_nothing() {
    let out = run(json!({ "json": {} }));
    assert_eq!(out, json!({ "json": {} }));
}
