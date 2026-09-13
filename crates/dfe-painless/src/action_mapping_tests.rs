// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The four-tier action mapping, read and run.

use serde_json::json;

use super::*;
use crate::common::normalise;
use crate::plan::PainlessPlan;

/// Verbatim from `filebeat/atlassian_cloud_audit/default.rs`, escapes and all.
const ATLASSIAN: &str = r#"def action = ctx.event.action;\nctx.event.kind = 'event';\ndef mapping = null;\nif (params.exact.containsKey(action)) {\n  mapping = params.exact[action];\n}\nif (mapping == null && params.prefixes instanceof List) {\n  for (def entry : params.prefixes) {\n    if (action.startsWith(entry.prefix)) {\n      mapping = entry;\n      break;\n    }\n  }\n}\nif (mapping == null && action.endsWith(params.policy_suffix)) {\n  mapping = params.policy;\n}\nif (mapping == null) {\n  mapping = params.default;\n}\nctx.event.category = mapping.category;\nctx.event.type = mapping.type;\nif (mapping.containsKey('outcome')) {\n  ctx.event.outcome = mapping.outcome;\n}"#;

/// The script as the matchers see it, escapes resolved.
fn atlassian() -> String {
    normalise(ATLASSIAN).into_owned()
}

fn pattern() -> ActionMapping {
    parse_action_mapping(&atlassian()).expect("declined atlassian_cloud's action mapping")
}

/// The rows the vendor ships, cut to the ones the tiers are tested through --
/// the prefix ORDER is the vendor's own, longest first.
fn params() -> Map<String, Value> {
    let table = json!({
        "exact": {
            "user_login": { "category": ["authentication"], "type": ["info"], "outcome": "success" },
            "org_settings_update": {
                "category": ["iam", "configuration"], "type": ["change"], "outcome": "success"
            },
        },
        "prefixes": [
            {
                "prefix": "user_login_failed",
                "category": ["authentication"], "type": ["info"], "outcome": "failure"
            },
            {
                "prefix": "user_login",
                "category": ["authentication"], "type": ["info"], "outcome": "success"
            },
            { "prefix": "organization_users_", "category": ["iam"], "type": ["info"] },
        ],
        "default": { "category": ["configuration"], "type": ["info"], "outcome": "success" },
        "policy_suffix": "_policy_update",
        "policy": {
            "category": ["iam", "configuration"], "type": ["change"], "outcome": "success"
        },
    });
    table.as_object().expect("the table is an object").clone()
}

fn classify(action: &str) -> Event {
    let mut event = Event::new(json!({ "event": { "action": action } }));
    assert!(run_action_mapping(&mut event, &pattern(), &params()));
    event
}

/// The whole point of the arm: the params ladder has to reach it.
#[test]
fn the_ladder_binds_the_script_to_this_arm() {
    let binding = PainlessPlan::new(ATLASSIAN).binding();
    assert_eq!(binding.len(), 1, "bound {binding:?}");
    assert!(binding[0].starts_with("ActionMapping"), "bound {binding:?}");
}

#[test]
fn every_tier_is_read_off_the_script() {
    let pattern = pattern();
    assert_eq!(pattern.source, "event.action");
    assert_eq!(pattern.stamp, ("event.kind".to_owned(), "event".to_owned()));
    assert_eq!(pattern.exact, "exact");
    assert_eq!(
        pattern.prefixes,
        ("prefixes".to_owned(), "prefix".to_owned())
    );
    assert_eq!(
        pattern.suffix,
        ("policy_suffix".to_owned(), "policy".to_owned())
    );
    assert_eq!(pattern.fallback, "default");
    assert_eq!(
        pattern
            .writes
            .iter()
            .map(|write| (write.target.as_str(), write.guarded))
            .collect::<Vec<_>>(),
        [
            ("event.category", false),
            ("event.type", false),
            ("event.outcome", true),
        ]
    );
}

#[test]
fn the_exact_table_answers_first() {
    let event = classify("user_login");
    assert_eq!(event.get("event.kind"), Some(&json!("event")));
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["authentication"]))
    );
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));
    assert_eq!(event.get("event.outcome"), Some(&json!("success")));
}

/// The loop breaks on the first match, so the list's own order decides:
/// `user_login_failed` is listed ahead of `user_login` and must win.
#[test]
fn the_first_matching_prefix_wins() {
    let event = classify("user_login_failed_too_many_attempts");
    assert_eq!(event.get("event.outcome"), Some(&json!("failure")));
}

#[test]
fn a_later_prefix_answers_what_the_first_does_not() {
    let event = classify("user_login_from_new_device");
    assert_eq!(event.get("event.outcome"), Some(&json!("success")));
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["authentication"]))
    );
}

#[test]
fn the_suffix_rule_answers_where_no_prefix_does() {
    let event = classify("mfa_policy_update");
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["iam", "configuration"]))
    );
    assert_eq!(event.get("event.type"), Some(&json!(["change"])));
}

#[test]
fn an_action_no_tier_names_takes_the_fallback() {
    let event = classify("something_the_vendor_added_last_week");
    assert_eq!(event.get("event.category"), Some(&json!(["configuration"])));
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));
    assert_eq!(event.get("event.outcome"), Some(&json!("success")));
}

/// `containsKey('outcome')` guards that one write, so a row without the member
/// leaves the field absent rather than null.
#[test]
fn a_guarded_write_is_skipped_where_the_row_has_no_member() {
    let event = classify("organization_users_viewed");
    assert_eq!(event.get("event.category"), Some(&json!(["iam"])));
    assert_eq!(event.get("event.outcome"), None);
}

/// An UNGUARDED write assigns Painless's null, which Elasticsearch keeps.
#[test]
fn an_unguarded_write_assigns_null_where_the_row_has_no_member() {
    let mut params = params();
    params.insert("default".to_owned(), json!({ "type": ["info"] }));
    let mut event = Event::new(json!({ "event": { "action": "unlisted" } }));
    assert!(run_action_mapping(&mut event, &pattern(), &params));
    assert_eq!(event.get("event.category"), Some(&json!(null)));
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));
}

/// `mapping == null` is re-tested at every tier, so a table row that IS null
/// falls through rather than stopping the search.
#[test]
fn a_null_row_falls_through_to_the_next_tier() {
    let mut params = params();
    params.insert("exact".to_owned(), json!({ "user_login": null }));
    let mut event = Event::new(json!({ "event": { "action": "user_login" } }));
    assert!(run_action_mapping(&mut event, &pattern(), &params));
    // The prefix list answers instead, and `user_login_failed` does not match.
    assert_eq!(event.get("event.outcome"), Some(&json!("success")));
}

/// The stamp is written before any lookup, so it survives a params block with
/// no tier in it at all.
#[test]
fn the_kind_is_stamped_even_where_no_tier_answers() {
    let mut event = Event::new(json!({ "event": { "action": "anything" } }));
    assert!(run_action_mapping(&mut event, &pattern(), &Map::new()));
    assert_eq!(event.get("event.kind"), Some(&json!("event")));
    assert_eq!(event.get("event.category"), None);
}

/// An action that is not a string throws in Painless rather than classifying.
#[test]
fn a_non_string_action_classifies_nothing() {
    let mut event = Event::new(json!({ "event": { "action": 42 } }));
    assert!(run_action_mapping(&mut event, &pattern(), &params()));
    assert_eq!(event.get("event.kind"), Some(&json!("event")));
    assert_eq!(event.get("event.category"), None);
}

/// A script running three tiers where the vendor runs four picks the WRONG row
/// rather than none, so every tier is demanded.
#[test]
fn a_script_missing_the_suffix_tier_is_declined() {
    let script = atlassian().replace(
        "if (mapping == null && action.endsWith(params.policy_suffix)) {\n  mapping = params.policy;\n}\n",
        "",
    );
    assert!(parse_action_mapping(&script).is_none());
}

#[test]
fn a_script_missing_the_prefix_walk_is_declined() {
    let script = atlassian().replace(
        "for (def entry : params.prefixes)",
        "for (def entry : params.other)",
    );
    assert!(parse_action_mapping(&script).is_none());
}

#[test]
fn a_script_that_writes_nothing_is_declined() {
    let script = atlassian().replace(" = mapping.", " = other.");
    assert!(parse_action_mapping(&script).is_none());
}
