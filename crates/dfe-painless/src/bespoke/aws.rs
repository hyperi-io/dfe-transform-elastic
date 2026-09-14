// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the aws package ships, transcribed by hand.

use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::{java_to_string, java_to_string_sized};
use dfe_core::event::Event;

/// The cloudtrail members kept twice, rendered under their own name and whole
/// under `flattened` where the deployment asked for the duplicate.
const RENDERED_MEMBERS: &[(&str, &str)] = &[
    (
        "json.requestParameters",
        "aws.cloudtrail.request_parameters",
    ),
    ("json.responseElements", "aws.cloudtrail.response_elements"),
    (
        "json.additionalEventData",
        "aws.cloudtrail.additional_eventdata",
    ),
    (
        "json.serviceEventDetails",
        "aws.cloudtrail.service_event_details",
    ),
];

/// Elasticsearch's keyword ceiling. Over it the rendered copy is kept and the
/// flattened one is not.
const KEYWORD_CEILING: usize = 32_766;

/// The prefix `script_retain_minimal_flattened` strips off each required path.
const FLATTENED_PREFIX: &str = "aws.cloudtrail.flattened.";

/// `aws/cloudtrail`, processor `script_duplicate_rules_fields`: render each
/// request/response member as Java's `toString` and, where the deployment keeps
/// duplicates, hold the member itself under `flattened`.
///
/// Every write goes through `ctx.aws.cloudtrail` with no null guard on the way
/// down, so a document that never built `aws.cloudtrail` raises on the first
/// one. The processor ignores its own failures, so nothing at all is written
/// there -- which is why a minimal cloudtrail record carries neither the
/// rendered copy nor the flattened one.
fn duplicate_rules_fields(event: &mut Event, _params: &Value) {
    if !matches!(event.get("aws.cloudtrail"), Some(Value::Object(_))) {
        return;
    }
    let keep = event.get_bool("_conf.keep_flattened_duplicates") == Some(true);
    if keep && !event.has_value("aws.cloudtrail.flattened") {
        let _ = event.set("aws.cloudtrail.flattened", Value::Object(Map::new()));
    }
    for (source, target) in RENDERED_MEMBERS {
        let Some(value) = event.get(source).filter(|held| !held.is_null()).cloned() else {
            continue;
        };
        // Rendered at the path it was pruned at: an empty-value prune that
        // crossed a bucket-table boundary left the map iterating through the
        // table it was BUILT with, which reorders the members.
        let mut at = (*source).to_string();
        let rendered = java_to_string_sized(&value, &mut at, &|path| event.map_capacity(path));
        let short_enough = rendered.len() < KEYWORD_CEILING;
        let _ = event.set(target, rendered);
        if keep
            && short_enough
            && let Some((prefix, name)) = target.rsplit_once('.')
        {
            let _ = event.set(&format!("{prefix}.flattened.{name}"), value);
        }
    }
}

/// `aws/cloudtrail`, processor `script_retain_minimal_flattened`: rebuild
/// `flattened` holding only the members the detection rules actually read.
///
/// The whole subtree is replaced, so anything the duplicating script put there
/// and this list does not name is gone.
fn retain_minimal_flattened(event: &mut Event, params: &Value) {
    let mut flattened = Map::new();
    if let Some(required) = params
        .get("required_flattened_fields")
        .and_then(Value::as_array)
    {
        for field in required {
            let Some(path) = field.as_str() else {
                continue;
            };
            let Some(tail) = path.strip_prefix(FLATTENED_PREFIX) else {
                continue;
            };
            let Some(value) = event.get(path).filter(|held| !held.is_null()).cloned() else {
                continue;
            };
            insert_at(&mut flattened, tail, value);
        }
    }
    let _ = event.set("aws.cloudtrail.flattened", Value::Object(flattened));
}

/// `aws/lambda_logs`, the `aws-lambda-json` pipeline: join a JSON stack trace
/// into the one string a later rename moves to `aws.lambda.error.stack_trace`.
///
/// The separator is a literal backslash followed by an `n`, not a newline --
/// `"\\n"` in the Painless source is an escaped backslash. Both spellings the
/// runtime emits are handled, the bare list and the one nested under `message`.
fn flatten_stack_trace(event: &mut Event, _params: &Value) {
    for (source, target) in [
        ("parsed.stackTrace", "parsed.stack_trace_flattened"),
        (
            "parsed.message.stackTrace",
            "parsed.message.stack_trace_flattened",
        ),
    ] {
        let Some(frames) = event.get_array(source) else {
            continue;
        };
        let joined = frames
            .iter()
            .map(java_to_string)
            .collect::<Vec<String>>()
            .join("\\n");
        let _ = event.set(target, joined);
    }
}

/// `computeIfAbsent` down a dotted path, then the leaf.
///
/// A segment already holding something that is not a map ends the walk, the
/// way the script's own subscript would raise on it.
fn insert_at(root: &mut Map<String, Value>, path: &str, value: Value) {
    let mut current = root;
    let mut rest = path;
    while let Some((head, tail)) = rest.split_once('.') {
        let slot = current
            .entry(head.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        let Value::Object(next) = slot else {
            return;
        };
        current = next;
        rest = tail;
    }
    current.insert(rest.to_string(), value);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "eb7dcf86eb859267bdd353d7abd4c675febedf759de966c25e687e01ebe20953",
        source: "aws",
        name: "duplicate_rules_fields",
        run: duplicate_rules_fields,
    },
    Entry {
        hash: "9e00a0490f712c778819d8572239a79e8db4e395d8638129578818a150e20993",
        source: "aws",
        name: "retain_minimal_flattened",
        run: retain_minimal_flattened,
    },
    Entry {
        hash: "602e5be43994631e60ac065e2d66986be02ab667280fe8c8d618788c5a2b3f7e",
        source: "aws",
        name: "flatten_stack_trace",
        run: flatten_stack_trace,
    },
];
