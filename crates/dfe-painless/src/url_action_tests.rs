// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One test per arm, each against a URL the ece corpus actually carries.

use super::*;
use crate::common::normalise;
use serde_json::json;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/ece_adminconsole/default.rs:459`.
const ECE_ACTION: &str = r#"def temp = params.get(ctx.http.request.method.toLowerCase());\n// elastic.co/api/v1/deployments\n// and we only want the part after the /api/v1/\nString url_parts = ctx.url.original.splitOnToken(\"/api/v1/\")[1];\nif (url_parts.contains('elasticsearch/elasticsearch/proxy/')){\n  // we handle the special cases here\n  url_parts = url_parts.splitOnToken('elasticsearch/elasticsearch/proxy/')[1];\n  // that should give us `_search`, `_cat/indices`\n  url_parts = url_parts.splitOnToken('?')[0];\n  // we do not care about params ? this will just overload the event.action\n  url_parts = url_parts.splitOnToken('/')[0];\n  // we do not care about '/' like in `_cat/indices`, just getting `_cat` back is good enough\n  ctx.putIfAbsent(\"event\", [:]);\n  ctx.event.action = \"elasticsearch_api_through_ece\" + \"-\" + url_parts.toLowerCase();\n}\nelse if (temp != null){\n    if (temp.get(url_parts) != null){\n        ctx.putIfAbsent(\"event\", [:]);\n        ctx.event.action = temp.get(url_parts);\n    }\n}\nif (ctx.event?.action == null){\n    ctx.putIfAbsent(\"event\", [:]);\n    ctx.event.action = ctx.http.request.method.toLowerCase() + \"_\" + url_parts.splitOnToken(\"/\")[0];\n}\n"#;

const TABLE: &str =
    r#"{"get":{"deployments":"list_deployments"},"post":{"deployments":"create_deployment"}}"#;

fn run(url: &str, method: &str) -> Event {
    let pattern = parse_url_tail_action(&normalise(ECE_ACTION)).expect("declined the script");
    let params: Map<String, Value> = serde_json::from_str(TABLE).unwrap();
    let mut event = Event::new(json!({
        "url": { "original": url },
        "http": { "request": { "method": method } },
    }));
    assert!(url_tail_action(&mut event, &pattern, &params));
    event
}

#[test]
fn the_parts_are_read_off_the_script() {
    assert_eq!(
        parse_url_tail_action(&normalise(ECE_ACTION)),
        Some(UrlTailAction {
            url: "url.original".into(),
            method: "http.request.method".into(),
            target: "event.action".into(),
            after: "/api/v1/".into(),
            proxy: Some(ProxyArm {
                marker: "elasticsearch/elasticsearch/proxy/".into(),
                prefix: "elasticsearch_api_through_ece".into(),
            }),
        })
    );
}

/// The table answers where the tail is one of its keys.
#[test]
fn the_table_names_a_known_call() {
    let event = run("89.160.20.112:12443/api/v1/deployments", "POST");
    assert_eq!(event.get_str("event.action"), Some("create_deployment"));
}

/// A tail the table does not carry falls back to method and first segment.
#[test]
fn an_unknown_tail_is_named_from_the_method_and_the_first_segment() {
    let event = run("89.160.20.112:12443/api/v1/users/auth/_refresh", "POST");
    assert_eq!(event.get_str("event.action"), Some("post_users"));

    let deployment = run(
        "89.160.20.112:12443/api/v1/deployments/53e007f109f14b328a4c190ccad5696a",
        "PUT",
    );
    assert_eq!(deployment.get_str("event.action"), Some("put_deployments"));
}

/// The proxy arm cuts the tail again and names it after the marker.
#[test]
fn a_call_proxied_to_elasticsearch_is_named_for_its_own_verb() {
    let event = run(
        "89.160.20.112:12443/api/v1/deployments/1fe7/elasticsearch/elasticsearch/proxy/_cat/indices",
        "POST",
    );
    assert_eq!(
        event.get_str("event.action"),
        Some("elasticsearch_api_through_ece-_cat")
    );
}

/// A different proxy spelling is NOT that arm, so it takes the fallback --
/// which is what Elastic's capture holds for the two `main-elasticsearch`
/// calls the corpus carries.
#[test]
fn another_proxy_spelling_is_left_to_the_fallback() {
    let event = run(
        "89.160.20.112:12443/api/v1/deployments/53e0/elasticsearch/main-elasticsearch/proxy/_search",
        "POST",
    );
    assert_eq!(event.get_str("event.action"), Some("post_deployments"));
}

/// An action an earlier processor wrote survives the fallback.
#[test]
fn an_action_already_written_is_left_alone() {
    let pattern = parse_url_tail_action(&normalise(ECE_ACTION)).unwrap();
    let params: Map<String, Value> = serde_json::from_str(TABLE).unwrap();
    let mut event = Event::new(json!({
        "url": { "original": "89.160.20.112:12443/api/v1/regions/ece-region" },
        "http": { "request": { "method": "GET" } },
        "event": { "action": "non-api" },
    }));

    assert!(url_tail_action(&mut event, &pattern, &params));

    assert_eq!(event.get_str("event.action"), Some("non-api"));
}

/// A URL with no separator writes nothing rather than guessing.
#[test]
fn a_url_without_the_separator_writes_nothing() {
    let event = run("89.160.20.112:12443/status", "GET");
    assert_eq!(event.get("event.action"), None);
}
