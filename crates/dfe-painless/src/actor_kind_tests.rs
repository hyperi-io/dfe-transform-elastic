// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The actor classifier, read and run.

use serde_json::json;

use super::*;
use crate::common::normalise;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/atlassian_cloud_audit/default.rs`, escapes and all:
/// a stored script arrives with its newlines escaped, so this is the text the
/// ladder is actually handed.
const ATLASSIAN: &str = r#"def actor = ctx.json.attributes.actor;\nboolean isUser = false;\nif (actor.email instanceof String && actor.email.indexOf('@') >= 0) {\n  isUser = true;\n} else if (actor.id instanceof String) {\n  String id = actor.id;\n  if (id.contains(':') || id ==~ /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/) {\n    isUser = true;\n  }\n}\nif (isUser) {\n  ctx.user = ctx.user != null ? ctx.user : new HashMap();\n  if (actor.id != null) {\n    ctx.user.id = actor.id;\n  }\n  if (actor.name != null) {\n    ctx.user.name = actor.name;\n  }\n  if (actor.email instanceof String && actor.email.indexOf('@') >= 0) {\n    ctx.user.email = actor.email;\n  }\n} else {\n  ctx.atlassian_cloud = ctx.atlassian_cloud != null ? ctx.atlassian_cloud : new HashMap();\n  ctx.atlassian_cloud.audit = ctx.atlassian_cloud.audit != null ? ctx.atlassian_cloud.audit : new HashMap();\n  ctx.atlassian_cloud.audit.actor = ctx.atlassian_cloud.audit.actor != null ? ctx.atlassian_cloud.audit.actor : new HashMap();\n  def app = ctx.atlassian_cloud.audit.actor.app != null ? ctx.atlassian_cloud.audit.actor.app : new HashMap();\n  if (actor.id != null) {\n    app.id = actor.id;\n  }\n  if (actor.name != null) {\n    app.name = actor.name;\n  }\n  ctx.atlassian_cloud.audit.actor.app = app;\n}"#;

/// The script as the matchers see it, escapes resolved.
fn atlassian() -> String {
    normalise(ATLASSIAN).into_owned()
}

fn pattern() -> ActorKind {
    parse_actor_kind(&atlassian()).expect("declined atlassian_cloud's actor classifier")
}

/// The whole point of the arm: the ladder has to reach it. Nothing above it
/// claims this script -- neither hard stop applies, because it spells no
/// `.add(` and no `.replace(`.
#[test]
fn the_ladder_binds_the_script_to_this_arm() {
    let binding = PainlessPlan::new(ATLASSIAN).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(binding[0].starts_with("ActorKind"), "bound {binding:?}");
}

#[test]
fn the_record_and_both_destinations_are_read_off_the_script() {
    let pattern = pattern();
    assert_eq!(pattern.actor, "json.attributes.actor");
    assert_eq!(pattern.address, "email");
    assert_eq!(pattern.identifier, "id");
    assert_eq!(pattern.person.parent, "user");
    assert_eq!(
        pattern.application.parent,
        "atlassian_cloud.audit.actor.app"
    );
    assert_eq!(
        pattern
            .person
            .copies
            .iter()
            .map(|copy| copy.target.as_str())
            .collect::<Vec<_>>(),
        ["user.id", "user.name", "user.email"]
    );
    assert_eq!(
        pattern
            .application
            .copies
            .iter()
            .map(|copy| copy.target.as_str())
            .collect::<Vec<_>>(),
        [
            "atlassian_cloud.audit.actor.app.id",
            "atlassian_cloud.audit.actor.app.name",
        ]
    );
}

#[test]
fn an_address_makes_the_actor_a_person() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": {
            "id": "5f8a1e2b3c4d5e6f7a8b9c0d",
            "name": "Ada Lovelace",
            "email": "ada@example.com",
        } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("user.id"),
        Some(&json!("5f8a1e2b3c4d5e6f7a8b9c0d"))
    );
    assert_eq!(event.get("user.name"), Some(&json!("Ada Lovelace")));
    assert_eq!(event.get("user.email"), Some(&json!("ada@example.com")));
    assert_eq!(event.get("atlassian_cloud"), None);
}

#[test]
fn a_uuid_identifier_makes_the_actor_a_person_without_an_address() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": {
            "id": "8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7f",
            "name": "Grace Hopper",
        } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("user.id"),
        Some(&json!("8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7f"))
    );
    assert_eq!(event.get("user.name"), Some(&json!("Grace Hopper")));
    // The address copy is guarded on the address test, not on the flag: an
    // actor that qualified by its identifier alone carries no address.
    assert_eq!(event.get("user.email"), None);
}

#[test]
fn a_colon_in_the_identifier_makes_the_actor_a_person() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": { "id": "ari:cloud:identity::user/42" } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("user.id"),
        Some(&json!("ari:cloud:identity::user/42"))
    );
}

#[test]
fn anything_else_is_an_application() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": {
            "id": "557058f7f0c2a3b4",
            "name": "Jira Automation",
        } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("atlassian_cloud.audit.actor.app.id"),
        Some(&json!("557058f7f0c2a3b4"))
    );
    assert_eq!(
        event.get("atlassian_cloud.audit.actor.app.name"),
        Some(&json!("Jira Automation"))
    );
    assert_eq!(event.get("user"), None);
}

/// The script builds its destination map before writing into it, so a record
/// carrying neither member still leaves the map behind -- and Elasticsearch
/// emits it.
#[test]
fn an_empty_record_still_leaves_the_map_the_script_builds() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": { "id": null } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("atlassian_cloud.audit.actor.app"),
        Some(&json!({}))
    );
}

/// A map the document already carries is kept, the way the ternary keeps it.
#[test]
fn an_existing_destination_is_added_to_rather_than_replaced() {
    let mut event = Event::new(json!({
        "user": { "domain": "example.com" },
        "json": { "attributes": { "actor": { "email": "ada@example.com" } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(event.get("user.domain"), Some(&json!("example.com")));
    assert_eq!(event.get("user.email"), Some(&json!("ada@example.com")));
}

/// An address that is not one: `indexOf('@') >= 0` is the whole test, and a
/// name with no `@` in it leaves the actor to the identifier arm.
#[test]
fn an_address_without_an_at_sign_does_not_make_a_person() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": { "email": "none", "id": "557058f7" } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("atlassian_cloud.audit.actor.app.id"),
        Some(&json!("557058f7"))
    );
    assert_eq!(event.get("user"), None);
}

/// `instanceof String` is part of both tests, so a numeric identifier is not
/// scanned for a colon.
#[test]
fn a_numeric_identifier_is_not_a_person() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": { "id": 5_570_587 } } },
    }));
    assert!(run_actor_kind(&mut event, &pattern()));
    assert_eq!(
        event.get("atlassian_cloud.audit.actor.app.id"),
        Some(&json!(5_570_587))
    );
}

/// A record that is not a map is a Painless throw, not a write.
#[test]
fn a_record_that_is_not_a_map_is_left_alone() {
    let mut event = Event::new(json!({
        "json": { "attributes": { "actor": "ada@example.com" } },
    }));
    assert!(!run_actor_kind(&mut event, &pattern()));
    assert_eq!(event.get("user"), None);
}

#[test]
fn a_missing_record_is_left_alone() {
    let mut event = Event::new(json!({ "json": { "attributes": {} } }));
    assert!(!run_actor_kind(&mut event, &pattern()));
    assert_eq!(event.get("user"), None);
}

/// The UUID test is the script's own anchored pattern, byte for byte.
#[test]
fn the_uuid_test_is_anchored_and_fixed_width() {
    assert!(is_uuid("8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7f"));
    assert!(is_uuid("8E5A2F10-3B4C-4D5E-9F01-2A3B4C5D6E7F"));
    // One character short, one long, and a hyphen out of place.
    assert!(!is_uuid("8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7"));
    assert!(!is_uuid("8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7ff"));
    assert!(!is_uuid("8e5a2f103-b4c-4d5e-9f01-2a3b4c5d6e7f"));
    // Anchored: the pattern is a FULL match, so a UUID inside other text is not
    // one.
    assert!(!is_uuid("id=8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7f"));
    // Hex only.
    assert!(!is_uuid("8e5a2f10-3b4c-4d5e-9f01-2a3b4c5d6e7g"));
    assert!(!is_uuid(""));
}

/// A script testing a DIFFERENT pattern means something else by "person", so
/// the arm declines rather than reading it as this one.
#[test]
fn a_different_identifier_test_is_declined() {
    let script = atlassian().replace(
        "==~ /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/",
        "==~ /^[0-9]+$/",
    );
    assert!(parse_actor_kind(&script).is_none());
}

/// An UNGUARDED copy would write a null where the vendor writes nothing.
#[test]
fn an_unguarded_copy_is_declined() {
    let script = atlassian().replace(
        "if (actor.name != null) {\n    ctx.user.name = actor.name;\n  }",
        "ctx.user.name = actor.name;",
    );
    assert!(parse_actor_kind(&script).is_none());
}

/// A branch that does not build the map it writes into is a different script:
/// Painless will not create a parent, so that one throws.
#[test]
fn a_branch_that_builds_no_map_is_declined() {
    let script = atlassian().replace(
        "ctx.user = ctx.user != null ? ctx.user : new HashMap();\n  ",
        "",
    );
    assert!(parse_actor_kind(&script).is_none());
}

/// A write to somewhere other than the map the branch built is redirected by
/// nothing -- the arm declines it.
#[test]
fn a_write_outside_the_built_map_is_declined() {
    let script = atlassian().replace(
        "ctx.user.name = actor.name;",
        "ctx.source.user.name = actor.name;",
    );
    assert!(parse_actor_kind(&script).is_none());
}
