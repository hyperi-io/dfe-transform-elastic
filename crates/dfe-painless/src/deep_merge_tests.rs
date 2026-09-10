// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `keycloak`'s merge, clause by clause, and the merge that is not it.
//!
//! Both vendor scripts here are their CALL SITE's own literal, escapes and all
//! -- a generated site holds the pipeline's folded YAML written back out as an
//! escaped string, so its newlines are two characters. A test written with real
//! newlines passes over the step that resolves them and production still fails.
//!
//! The decline cases at the end are CONSTRUCTED, and say so: they cover the
//! structural rules -- a merge into itself, a script that does more than merge,
//! a call naming two ordinary paths -- that no vendor script spells today.

use serde_json::json;

use super::*;
use crate::common::normalise;

/// Verbatim from `crates/dfe-transforms/src/filebeat/keycloak_log/default.rs`.
const KEYCLOAK: &str = r#"\ndef mergeMaps(Map map1, Map map2) {\n  for (def key : map2.keySet()) {\n    if (!map1.containsKey(key)\n        || map1[key] == null\n        || map1[key] == \"\"\n        || (map1[key] instanceof Map && map1[key].isEmpty())) {\n      // If map1's value is absent or empty, use map2's.\n      map1[key] = map2[key];\n    } else if (map1[key] != map2[key]) {\n      if (map1[key] instanceof Map && map2[key] instanceof Map) {\n        // If values in map1 and map2 for key are both maps, merge them.\n        map1[key] = mergeMaps(map1[key], map2[key]);\n      } else if (map1[key] instanceof List) {\n        // If map1's value is a list, merge map2's value, remove any duplicates.\n        def combined = new LinkedHashSet(map1[key]);\n        if (map2[key] instanceof List) {\n          combined.addAll(map2[key]);\n        } else if (map2[key] != null) {\n          combined.add(map2[key]);\n        }\n        map1[key] = new ArrayList(combined);\n      }\n    }\n  }\n  return map1;\n}\nmergeMaps(ctx, ctx.json);"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/ti_opencti_indicator/default.rs`.
///
/// The same function name and the opposite guard: it takes the arriving value
/// whenever the held key is ABSENT OR EQUAL, coerces a held scalar into a list,
/// and reaches the document through a fold rather than a single call.
const TI_OPENCTI: &str = r#"def mergeMaps(Map map1, Map map2) {\n  for (def key : map2.keySet()) {\n    if (map1.containsKey(key) && map1[key] != map2[key]) {\n      if (map1[key] instanceof Map && map2[key] instanceof Map) {\n        map1[key] = mergeMaps(map1[key], map2[key]);\n      } else {\n        if (!(map1[key] instanceof List)) {\n          map1[key] = [map1[key]];\n        }\n        def combined = new HashSet(map1[key]);\n        if (map2[key] instanceof List) {\n          combined.addAll(map2[key]);\n        } else {\n          combined.add(map2[key]);\n        }\n        map1[key] = new ArrayList(combined);\n      }\n    } else {\n      map1[key] = map2[key];\n    }\n  }\n  return map1;\n}\ndef mergeListOfMaps(List list) {\n  def merged = new HashMap();\n  for (def map : list) {\n    merged = mergeMaps(merged, map);\n  }\n  return merged;\n}\nif (ctx.opencti?.containsKey('observable') == true) {\n  for (def key : ctx.opencti.observable.keySet()) {\n    if (ctx.opencti.observable[key] instanceof List) {\n      ctx.opencti.observable[key] = mergeListOfMaps(ctx.opencti.observable[key]);\n    }\n  }\n}\nif (ctx.opencti?.indicator?.containsKey('external_reference') == true && ctx.opencti.indicator.external_reference instanceof List) {\n  ctx.opencti.indicator.external_reference = mergeListOfMaps(ctx.opencti.indicator.external_reference);\n}\nif (ctx.threat.indicator.containsKey('file') && ctx.threat.indicator.file instanceof List) {\n  ctx.threat.indicator.file = mergeListOfMaps(ctx.threat.indicator.file);\n}\nif (ctx.threat.indicator.containsKey('as') && ctx.threat.indicator.as instanceof List) {\n  ctx.threat.indicator.as = mergeListOfMaps(ctx.threat.indicator.as);\n}\nif (ctx.threat.indicator.containsKey('url') && ctx.threat.indicator.url instanceof List) {\n  ctx.threat.indicator.url = mergeListOfMaps(ctx.threat.indicator.url);\n}\nif (ctx.threat.indicator.containsKey('registry') && ctx.threat.indicator.registry instanceof List) {\n  ctx.threat.indicator.registry = mergeListOfMaps(ctx.threat.indicator.registry);\n}\nif (ctx.threat.indicator.containsKey('x509') && ctx.threat.indicator.x509 instanceof List) {\n  ctx.threat.indicator.x509 = mergeListOfMaps(ctx.threat.indicator.x509);\n}\n"#;

/// keycloak's merge, read once.
fn keycloak() -> DeepMerge {
    parse_deep_merge(&normalise(KEYCLOAK)).expect("declined keycloak's merge")
}

/// A root carrying `json`, merged by keycloak's pattern.
fn merged(root: serde_json::Value) -> Event {
    let mut event = Event::new(root);
    assert!(run_deep_merge(&mut event, &keycloak()));
    event
}

#[test]
fn the_call_names_the_root_and_the_payload() {
    let pattern = keycloak();
    assert_eq!(pattern.into, "");
    assert_eq!(pattern.from, "json");
}

#[test]
fn the_ecs_payload_is_lifted_onto_the_root() {
    // The document as the pipeline has it at the merge: `ecs.version` already
    // stamped, `event` emptied by the rename that moved `event.original` into
    // `json`, and the payload dot-expanded under `json`.
    let event = merged(json!({
        "tags": ["preserve_original_event"],
        "ecs": { "version": "8.11.0" },
        "event": {},
        "json": {
            "@timestamp": "2025-05-01T01:16:31.591777597Z",
            "event": { "sequence": 44, "original": "{...}" },
            "log": { "logger": "org.keycloak.url.HostnameV2ProviderFactory", "level": "INFO" },
            "message": "If hostname is specified, hostname-strict is effectively ignored",
            "process": { "thread": { "name": "main", "id": 1 }, "name": "/bin/java", "pid": 1 },
            "mdc": {},
            "ndc": "",
            "host": { "hostname": "068a18c2f627" },
            "ecs": { "version": "1.12.2" },
            "data_stream": { "type": "logs" },
            "service": { "name": "Keycloak", "version": "26.2.2", "environment": "prod" }
        }
    }));

    assert_eq!(
        event.get_str("message"),
        Some("If hostname is specified, hostname-strict is effectively ignored")
    );
    assert_eq!(event.get_str("log.level"), Some("INFO"));
    assert_eq!(
        event.get_str("log.logger"),
        Some("org.keycloak.url.HostnameV2ProviderFactory")
    );
    assert_eq!(event.get_str("host.hostname"), Some("068a18c2f627"));
    assert_eq!(event.get_str("process.name"), Some("/bin/java"));
    assert_eq!(event.get_i64("process.pid"), Some(1));
    assert_eq!(event.get_i64("process.thread.id"), Some(1));
    assert_eq!(event.get_str("process.thread.name"), Some("main"));
    assert_eq!(event.get_str("data_stream.type"), Some("logs"));
    assert_eq!(event.get_str("service.environment"), Some("prod"));
    assert_eq!(
        event.get_str("@timestamp"),
        Some("2025-05-01T01:16:31.591777597Z")
    );

    // The empty `event` map is replaced whole, so both members arrive.
    assert_eq!(event.get_i64("event.sequence"), Some(44));
    assert_eq!(event.get_str("event.original"), Some("{...}"));

    // The pipeline's own ECS version survives the payload's.
    assert_eq!(event.get_str("ecs.version"), Some("8.11.0"));

    // A key only the root carries is untouched.
    assert_eq!(event.get("tags"), Some(&json!(["preserve_original_event"])));
}

#[test]
fn an_absent_key_is_taken() {
    let event = merged(json!({ "json": { "a": 1 } }));
    assert_eq!(event.get_i64("a"), Some(1));
}

#[test]
fn a_null_is_taken() {
    let event = merged(json!({ "a": null, "json": { "a": 1 } }));
    assert_eq!(event.get_i64("a"), Some(1));
}

#[test]
fn an_empty_string_is_taken() {
    let event = merged(json!({ "a": "", "json": { "a": "held" } }));
    assert_eq!(event.get_str("a"), Some("held"));
}

#[test]
fn an_empty_map_is_taken() {
    let event = merged(json!({ "a": {}, "json": { "a": { "b": 1, "c": 2 } } }));
    assert_eq!(event.get_i64("a.b"), Some(1));
    assert_eq!(event.get_i64("a.c"), Some(2));
}

#[test]
fn a_held_scalar_is_kept() {
    let event = merged(json!({ "a": "root", "json": { "a": "payload" } }));
    assert_eq!(event.get_str("a"), Some("root"));
}

#[test]
fn two_maps_recurse() {
    let event = merged(json!({
        "a": { "held": "root", "empty": "" },
        "json": { "a": { "held": "payload", "empty": "filled", "fresh": 3 } }
    }));
    // The held member wins, the empty one is filled, the absent one arrives.
    assert_eq!(event.get_str("a.held"), Some("root"));
    assert_eq!(event.get_str("a.empty"), Some("filled"));
    assert_eq!(event.get_i64("a.fresh"), Some(3));
}

#[test]
fn a_held_map_is_kept_where_the_arriving_value_is_not_one() {
    let event = merged(json!({ "a": { "b": 1 }, "json": { "a": "scalar" } }));
    assert_eq!(event.get_i64("a.b"), Some(1));
}

#[test]
fn a_list_unions_with_a_list() {
    let event = merged(json!({ "a": ["x", "y"], "json": { "a": ["y", "z"] } }));
    assert_eq!(event.get("a"), Some(&json!(["x", "y", "z"])));
}

#[test]
fn a_list_unions_with_a_scalar() {
    let event = merged(json!({ "a": ["x"], "json": { "a": "z" } }));
    assert_eq!(event.get("a"), Some(&json!(["x", "z"])));
}

#[test]
fn a_duplicate_is_dropped_and_insertion_order_holds() {
    // `new LinkedHashSet(map1[key])` dedups what the HELD list already carried,
    // and the set keeps the order the values first arrived in.
    let event = merged(json!({
        "a": ["b", "a", "b", "c"],
        "json": { "a": ["c", "d", "a"] }
    }));
    assert_eq!(event.get("a"), Some(&json!(["b", "a", "c", "d"])));
}

#[test]
fn an_equal_list_is_left_exactly_as_it_was() {
    // The script never reaches the set where the two sides compare equal, so a
    // duplicate the held list carried SURVIVES.
    let event = merged(json!({ "a": ["b", "b"], "json": { "a": ["b", "b"] } }));
    assert_eq!(event.get("a"), Some(&json!(["b", "b"])));
}

#[test]
fn a_null_arriving_at_a_list_adds_nothing() {
    let event = merged(json!({ "a": ["x"], "json": { "a": null } }));
    assert_eq!(event.get("a"), Some(&json!(["x"])));
}

#[test]
fn an_empty_held_list_unions_rather_than_being_replaced() {
    // An empty LIST is not one of the four the first clause takes, so a scalar
    // arriving at one is APPENDED rather than stored bare.
    let event = merged(json!({ "a": [], "json": { "a": "z" } }));
    assert_eq!(event.get("a"), Some(&json!(["z"])));
}

#[test]
fn a_source_that_is_not_a_map_is_declined() {
    // The call site gates the script on the source being an object, so reading
    // something else means the PATH is wrong and claiming it would report a
    // success the pipeline does not have.
    let mut event = Event::new(json!({ "json": "not a map" }));
    assert!(!run_deep_merge(&mut event, &keycloak()));
}

#[test]
fn the_opencti_merge_is_declined() {
    assert!(parse_deep_merge(&normalise(TI_OPENCTI)).is_none());
}

#[test]
fn a_merge_naming_two_ordinary_paths_is_read_off_the_call() {
    // CONSTRUCTED: no vendor script merges between two named paths today, and
    // the pair is read off the call rather than assumed to be the root.
    let script = KEYCLOAK.replace("mergeMaps(ctx, ctx.json);", "mergeMaps(ctx.a, ctx.b);");
    let pattern = parse_deep_merge(&normalise(&script)).expect("declined a two-path merge");
    assert_eq!(pattern.into, "a");
    assert_eq!(pattern.from, "b");

    let mut event = Event::new(json!({ "a": { "held": 1 }, "b": { "held": 2, "fresh": 3 } }));
    assert!(run_deep_merge(&mut event, &pattern));
    assert_eq!(event.get_i64("a.held"), Some(1));
    assert_eq!(event.get_i64("a.fresh"), Some(3));
}

#[test]
fn a_merge_into_itself_is_declined() {
    // CONSTRUCTED. It writes nothing wherever it is pointed.
    let script = KEYCLOAK.replace("mergeMaps(ctx, ctx.json);", "mergeMaps(ctx.a, ctx.a);");
    assert!(parse_deep_merge(&normalise(&script)).is_none());
}

#[test]
fn a_merge_into_the_source_own_descendant_is_declined() {
    // CONSTRUCTED. Cloning the source first cannot reproduce a merge that
    // writes back into the map it is reading.
    let script = KEYCLOAK.replace("mergeMaps(ctx, ctx.json);", "mergeMaps(ctx.a.b, ctx.a);");
    assert!(parse_deep_merge(&normalise(&script)).is_none());
}

#[test]
fn a_script_that_does_more_than_merge_is_declined() {
    // CONSTRUCTED. A third mention of `ctx` is a statement this reproduces
    // none of, and running the merge alone is a half-run.
    let script = format!("{KEYCLOAK}\\nctx.extra = 1;");
    assert!(parse_deep_merge(&normalise(&script)).is_none());
}

#[test]
fn a_merge_whose_first_clause_is_missing_is_declined() {
    // CONSTRUCTED: the empty-map guard removed. Without it a held empty map is
    // kept, which is the opposite of what keycloak wants for `event`.
    let script = KEYCLOAK.replace(
        "\\n        || (map1[key] instanceof Map && map1[key].isEmpty())",
        "",
    );
    assert!(parse_deep_merge(&normalise(&script)).is_none());
}

#[test]
fn a_merge_with_no_list_union_is_declined() {
    // CONSTRUCTED: the list arm removed, leaving a merge that silently drops
    // every arriving list member.
    let script = KEYCLOAK.replace("new LinkedHashSet(map1[key])", "new ArrayList()");
    assert!(parse_deep_merge(&normalise(&script)).is_none());
}
