// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One test per spelling `symantec_endpoint_security` ships, all verbatim from
//! the generated call sites -- escapes, stray semicolons and all.

use super::*;
use crate::common::normalise;
use serde_json::json;

/// `symantec_endpoint_security_event/default.rs`, tagged
/// `script_to_add_email_direction`. A member of every element of a list.
const EMAILS: &str = r#"def var = new HashSet();\n  if (ctx.email != null && ctx.email.direction != null) {\n      var = ctx.email.direction;\n  } else {\n    if (ctx.email == null)\n    {\n      ctx.email = new HashMap();\n    }\n  }\n\nfor (def email : ctx.ses.cybox.emails) {\n  def direction = email.direction_id;\n  if (params.containsKey(direction.toString())) {\n    def type = params.get(direction.toString());\n    var.add(type);\n  }\n}\nctx.email.put('direction', var)"#;

/// The same job for ONE email rather than a list.
const ONE_EMAIL: &str = r#"def var = new HashSet(); if (ctx.email != null && ctx.email.direction != null) {\n    var = ctx.email.direction;\n} else {\n  if (ctx.email == null)\n  {\n    ctx.email = new HashMap();\n  }\n} def direction = ctx.ses.email.direction_id; if (params.containsKey(direction.toString())) { def type = params.get(direction.toString()); var.add(type); }\nctx.email.put('direction', var)"#;

/// Every element of the list IS an id.
const ATTRIBUTE_IDS: &str = r#"def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def obj : ctx.ses.file.attribute_ids) {\n  if (params.containsKey(obj.toString())) {\n    def type = params.get(obj.toString());\n    var.add(type);\n  }\n}\nctx.file.put('attributes', var)"#;

/// A member that is itself a LIST, reached through a second loop and the
/// bracket spelling of the lookup.
const NESTED_IDS: &str = r#"def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def file : ctx.ses.cybox.files) {\n  if (file.attribute_ids == null) {\n    continue;\n  }\n  for (def id : file.attribute_ids) {\n    def type = params[id.toString()];\n    if (type != null) {\n      var.add(type);\n    }\n  }\n} ctx.file.put('attributes', var)"#;

/// A scalar member of every element, with the bracket lookup and a `continue`.
const FILE_TYPES: &str = r#"def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n}\nfor (def file : ctx.ses.cybox.files) {\n  if (file.type_id == null) {\n    continue;\n  }\n  def type = params[file.type_id.toString()];\n  if (type != null) {\n      var.add(type);\n  }  \n}\nctx.file.put('type', var); "#;

/// `new ArrayList()`, no seed, a parenthesised inline key, and two levels of
/// parent creation before the write.
const OS_TYPE: &str = r#"def var = new ArrayList(); if (params.containsKey((ctx.ses.device_os_type_id).toString())) {\n    def type = params.get((ctx.ses.device_os_type_id).toString());\n    var.add(type);\n} if (ctx.host == null) {\n    ctx.host = new HashMap();\n} if (ctx.host.os == null) {\n    ctx.host.os = new HashMap();\n} ctx.host.os.put('type', var)"#;

const DIRECTIONS: &str = r#"{"0":"unknown","1":"inbound","2":"outbound"}"#;
const ATTRIBUTES: &str = r#"{"1":"archive","2":"compressed","5":"hidden"}"#;
const FILE_TYPE_ROWS: &str = r#"{"1":"file","2":"dir","6":"symlink"}"#;
const OS_ROWS: &str = r#"{"100":"windows","200":"linux","400":"macos"}"#;

fn run(script: &str, params: &str, event: &mut Event) -> bool {
    let script = normalise(script);
    let pattern = parse_collect_params_rows(&script).expect("declined the script");
    let params: Map<String, Value> = serde_json::from_str(params).unwrap();
    collect_params_rows(event, &pattern, &params)
}

#[test]
fn a_member_of_every_element_becomes_a_row() {
    let mut event = Event::new(json!({
        "ses": { "cybox": { "emails": [
            { "direction_id": "0" },
            { "direction_id": "1" },
        ] } },
    }));

    assert!(run(EMAILS, DIRECTIONS, &mut event));

    assert_eq!(
        event.get("email.direction"),
        Some(&json!(["unknown", "inbound"]))
    );
}

#[test]
fn a_repeated_id_is_collected_once() {
    let mut event = Event::new(json!({
        "ses": { "cybox": { "emails": [
            { "direction_id": "1" },
            { "direction_id": "1" },
            { "direction_id": "2" },
        ] } },
    }));

    assert!(run(EMAILS, DIRECTIONS, &mut event));

    assert_eq!(
        event.get("email.direction"),
        Some(&json!(["inbound", "outbound"]))
    );
}

#[test]
fn an_id_the_table_does_not_carry_writes_nothing_for_itself() {
    let mut event = Event::new(json!({
        "ses": { "cybox": { "emails": [{ "direction_id": "9" }] } },
    }));

    assert!(run(EMAILS, DIRECTIONS, &mut event));

    // The put is unconditional, so an empty collection is still written.
    assert_eq!(event.get("email.direction"), Some(&json!([])));
}

#[test]
fn the_target_is_seeded_from_what_it_already_holds() {
    let mut event = Event::new(json!({
        "email": { "direction": ["outbound"] },
        "ses": { "cybox": { "emails": [{ "direction_id": "1" }] } },
    }));

    assert!(run(EMAILS, DIRECTIONS, &mut event));

    assert_eq!(
        event.get("email.direction"),
        Some(&json!(["outbound", "inbound"]))
    );
}

#[test]
fn a_scalar_source_reads_its_own_path() {
    let mut event = Event::new(json!({ "ses": { "email": { "direction_id": 2 } } }));

    assert!(run(ONE_EMAIL, DIRECTIONS, &mut event));

    assert_eq!(event.get("email.direction"), Some(&json!(["outbound"])));
}

#[test]
fn a_list_of_bare_ids_is_walked() {
    let mut event = Event::new(json!({
        "ses": { "file": { "attribute_ids": [1, 5, 1] } },
    }));

    assert!(run(ATTRIBUTE_IDS, ATTRIBUTES, &mut event));

    assert_eq!(
        event.get("file.attributes"),
        Some(&json!(["archive", "hidden"]))
    );
}

#[test]
fn a_member_that_is_itself_a_list_is_flattened() {
    let mut event = Event::new(json!({
        "ses": { "cybox": { "files": [
            { "attribute_ids": [1, 2] },
            { "path": "no attributes here" },
            { "attribute_ids": [2, 5] },
        ] } },
    }));

    assert!(run(NESTED_IDS, ATTRIBUTES, &mut event));

    assert_eq!(
        event.get("file.attributes"),
        Some(&json!(["archive", "compressed", "hidden"]))
    );
}

#[test]
fn the_bracket_lookup_reads_the_same_member() {
    let mut event = Event::new(json!({
        "ses": { "cybox": { "files": [{ "type_id": 1 }, { "type_id": 6 }] } },
    }));

    assert!(run(FILE_TYPES, FILE_TYPE_ROWS, &mut event));

    assert_eq!(event.get("file.type"), Some(&json!(["file", "symlink"])));
}

#[test]
fn an_arraylist_keeps_a_repeat_and_overwrites_the_target() {
    let mut event = Event::new(json!({
        "host": { "os": { "type": ["stale"] } },
        "ses": { "device_os_type_id": 200 },
    }));

    assert!(run(OS_TYPE, OS_ROWS, &mut event));

    assert_eq!(event.get("host.os.type"), Some(&json!(["linux"])));
}

#[test]
fn the_parts_each_script_names_are_read_off_it() {
    let seeded = parse_collect_params_rows(&normalise(EMAILS)).unwrap();
    assert_eq!(
        seeded,
        CollectParamsRows {
            source: RowSource::Member {
                list: "ses.cybox.emails".into(),
                member: "direction_id".into(),
            },
            target: "email.direction".into(),
            dedup: true,
            seeded: true,
        }
    );

    let fresh = parse_collect_params_rows(&normalise(OS_TYPE)).unwrap();
    assert_eq!(
        fresh,
        CollectParamsRows {
            source: RowSource::Path("ses.device_os_type_id".into()),
            target: "host.os.type".into(),
            dedup: false,
            seeded: false,
        }
    );
}

#[test]
fn a_script_that_puts_something_other_than_a_collection_is_declined() {
    // tenable_io's own `.put(` takes a local the script never declares as a
    // collection, so nothing here may claim it.
    let script = "ctx.ses.incident.put(\"event\",params[ctx.ses.incident.event_id])";
    assert!(parse_collect_params_rows(&normalise(script)).is_none());
}

#[test]
fn a_lookup_with_no_put_is_left_to_the_guarded_reader() {
    let script = r#"def var = new ArrayList(); if (params.containsKey((ctx.ses.device_os_type_id).toString())) {\n    def type = params.get((ctx.ses.device_os_type_id).toString());\n    ctx.ses.device_os_type_value = type;\n}"#;
    assert!(parse_collect_params_rows(&normalise(script)).is_none());
}
