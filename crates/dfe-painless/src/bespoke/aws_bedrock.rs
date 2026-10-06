// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `aws_bedrock`'s invocation scripts, transcribed.
//!
//! A model invocation log carries the request and the response as parsed JSON,
//! and the pipeline puts them back into text so the two `gen_ai` fields can
//! hold them whole. Around that sit the normalisers -- a message whose content
//! is a bare string, a streamed response whose last element is a done marker
//! rather than a block -- and the guardrail reader, which flattens an
//! assessment tree into the ECS policy and compliance arrays.

use std::collections::BTreeSet;

use serde_json::{Map, Value};
use sha1::{Digest, Sha1};

use super::Entry;
use crate::helpers::{java_bucket, java_set_order, java_table_size, painless_to_string};
use dfe_core::event::Event;

/// The request body, under the name the vendor sends.
const INPUT_BODY_SENT: &str = "aws_bedrock.invocation.input.inputBodyJson";

/// The response body, under the name the vendor sends.
const OUTPUT_BODY_SENT: &str = "aws_bedrock.invocation.output.outputBodyJson";

/// The request body, after the pipeline's own rename.
const INPUT_BODY: &str = "aws_bedrock.invocation.input.input_body_json";

/// The response body, after the pipeline's own rename.
const OUTPUT_BODY: &str = "aws_bedrock.invocation.output.output_body_json";

/// The messages the caller sent, which sit beside the request body.
const MESSAGES: &str = "aws_bedrock.invocation.messages";

/// The system prompt, which the vendor sends as a string or an object.
const SYSTEM: &str = "aws_bedrock.invocation.system";

/// What Elasticsearch will hold in one keyword, and the length both size
/// scripts measure against.
const MAX_KEYWORD: usize = 32766;

// -- the two bodies, back to text -----------------------------------------

/// `aws_bedrock/invocation`, `remarshal_bodies`: `gen_ai.prompt` and
/// `gen_ai.completion`, each the parsed body written back out as text.
///
/// It reads the bodies under the names the VENDOR sent, because it runs ahead
/// of the rename that gives them their snake-case names.
fn remarshal_bodies(event: &mut Event, _params: &Value) {
    let prompt = marshal(event, INPUT_BODY_SENT);
    let _ = event.set("gen_ai.prompt", prompt);
    let completion = marshal(event, OUTPUT_BODY_SENT);
    let _ = event.set("gen_ai.completion", completion);
}

/// One body as the script's own writer renders it, null where there is none.
fn marshal(event: &Event, path: &str) -> Value {
    match event.get(path) {
        None | Some(Value::Null) => Value::Null,
        Some(value) => Value::from(painless_json(value)),
    }
}

/// The script's own JSON writer, which is NOT a JSON writer.
///
/// It sorts a map's keys, and it wraps a string in quotes without escaping
/// anything inside it -- the vendor's own comment says so. A model reply
/// holding a quote or a newline therefore lands in the field as text that no
/// parser will read back, and reproducing that is the parity.
fn painless_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::String(text) => format!("\"{text}\""),
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut out = String::from("{");
            for (at, key) in keys.into_iter().enumerate() {
                if at != 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(key);
                out.push_str("\":");
                match map.get(key) {
                    Some(Value::String(text)) => {
                        out.push('"');
                        out.push_str(text);
                        out.push('"');
                    }
                    Some(member) => out.push_str(&painless_json(member)),
                    None => out.push_str("null"),
                }
            }
            out.push('}');
            out
        }
        Value::Array(items) => {
            let mut out = String::from("[");
            for (at, item) in items.iter().enumerate() {
                if at != 0 {
                    out.push(',');
                }
                out.push_str(&painless_json(item));
            }
            out.push(']');
            out
        }
        other => painless_to_string(other),
    }
}

/// `aws_bedrock/invocation`, the untagged request-size script: how long
/// `gen_ai.prompt` is, and the body dropped for a hash where it will not fit a
/// keyword.
fn request_size_from_prompt(event: &mut Event, _params: &Value) {
    let Some(prompt) = event.get_string("gen_ai.prompt") else {
        return;
    };
    let length = utf16_len(&prompt);
    let _ = event.set("gen_ai.performance.request_size", count(length));
    if length > MAX_KEYWORD {
        let _ = event.set(
            "aws_bedrock.invocation.input.input_body_json_massive_hash",
            sha1_hex(&prompt),
        );
        event.remove(INPUT_BODY);
        event.remove("gen_ai.prompt");
    }
}

/// `aws_bedrock/invocation`, the untagged response-size script: the same for
/// `gen_ai.completion`.
fn response_size_from_completion(event: &mut Event, _params: &Value) {
    let Some(completion) = event.get_string("gen_ai.completion") else {
        return;
    };
    let length = utf16_len(&completion);
    let _ = event.set("gen_ai.performance.response_size", count(length));
    if length > MAX_KEYWORD {
        let _ = event.set(
            "aws_bedrock.invocation.output.output_body_json_massive_hash",
            sha1_hex(&completion),
        );
        event.remove(OUTPUT_BODY);
        event.remove("gen_ai.completion");
    }
}

/// A Java `String.length()`, which counts UTF-16 units and not characters.
fn utf16_len(text: &str) -> usize {
    text.encode_utf16().count()
}

/// A length as the JSON number the script stores.
fn count(length: usize) -> i64 {
    i64::try_from(length).unwrap_or(i64::MAX)
}

/// Painless's `String.sha1()`: the digest of the UTF-8 bytes, lower-case hex.
fn sha1_hex(text: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(text.as_bytes());
    super::lower_hex(&hasher.finalize())
}

// -- the normalisers ------------------------------------------------------

/// `aws_bedrock/invocation`, `script_promote_system_to_object`: a system prompt
/// sent as a bare string, given the `type` and `text` the object form has.
fn promote_system_to_object(event: &mut Event, _params: &Value) {
    let Some(text) = event.get_string(SYSTEM) else {
        return;
    };
    let mut system = Map::with_capacity(2);
    system.insert("type".to_owned(), Value::from("text"));
    system.insert("text".to_owned(), Value::from(text));
    event.update(SYSTEM, Value::Object(system));
}

/// `aws_bedrock/invocation`, the first untagged message script: every message's
/// content given a `type` and a `text`, whatever form the caller sent it in.
///
/// A content item that already carries text is left exactly as it is, which is
/// why an ordinary text message passes through untouched.
fn normalise_message_content(event: &mut Event, _params: &Value) {
    walk_messages(event, |content| match content {
        Value::String(text) => {
            let mut promoted = Map::with_capacity(2);
            promoted.insert("type".to_owned(), Value::from("text"));
            promoted.insert("text".to_owned(), Value::String(std::mem::take(text)));
            *content = Value::Object(promoted);
        }
        Value::Array(items) => {
            for item in items {
                type_and_text(item);
            }
        }
        Value::Object(_) => type_and_text(content),
        _ => {}
    });
}

/// `aws_bedrock/invocation`, the second untagged message script: the same
/// again for a content item the first left without text.
///
/// It differs from the first in two ways -- it adds no `type`, and it prunes
/// every other field whether or not it wrote the text.
fn ensure_message_content_text(event: &mut Event, _params: &Value) {
    walk_messages(event, |content| match content {
        Value::Array(items) => {
            for item in items {
                text_only(item);
            }
        }
        Value::Object(_) => text_only(content),
        _ => {}
    });
}

/// Each message's `content`, whether the vendor sent one message or a list.
fn walk_messages(event: &mut Event, mut visit: impl FnMut(&mut Value)) {
    let Some(mut messages) = event.get(MESSAGES).cloned() else {
        return;
    };
    match &mut messages {
        Value::Array(items) => {
            for item in items {
                visit_content(item, &mut visit);
            }
        }
        single => visit_content(single, &mut visit),
    }
    event.update(MESSAGES, messages);
}

/// One message's content, where it is a map carrying one.
fn visit_content(message: &mut Value, visit: &mut impl FnMut(&mut Value)) {
    let Some(message) = message.as_object_mut() else {
        return;
    };
    if let Some(content) = message.get_mut("content") {
        visit(content);
    }
}

/// One content item given a `type` and a `text`, and pruned to those two.
fn type_and_text(item: &mut Value) {
    let Some(map) = item.as_object_mut() else {
        return;
    };
    if !map.contains_key("type") {
        map.insert("type".to_owned(), Value::from("text"));
    }
    if matches!(map.get("text"), Some(Value::String(text)) if !text.is_empty()) {
        return;
    }
    let text = text_for(map, false);
    map.insert("text".to_owned(), Value::from(text));
    prune_to_type_and_text(map);
}

/// One content item given a `text` where it has none, and pruned regardless.
fn text_only(item: &mut Value) {
    let Some(map) = item.as_object_mut() else {
        return;
    };
    let missing = !matches!(map.get("text"), Some(Value::String(text)) if !text.is_empty());
    if missing {
        let text = text_for(map, true);
        map.insert("text".to_owned(), Value::from(text));
    }
    prune_to_type_and_text(map);
}

/// The text one content item stands for: its own nested content where it has
/// one, and otherwise everything but its type written out as JSON.
///
/// `empty_for_nothing` is the second script's extra arm -- an item with nothing
/// left but a type gets an empty text rather than `{}`.
fn text_for(map: &Map<String, Value>, empty_for_nothing: bool) -> String {
    if let Some(nested) = map.get("content") {
        return match nested {
            Value::String(text) => text.clone(),
            other => json_dump(other),
        };
    }
    let copy: Map<String, Value> = map
        .iter()
        .filter(|(key, _)| key.as_str() != "type")
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    if empty_for_nothing && copy.is_empty() {
        return String::new();
    }
    json_dump(&Value::Object(copy))
}

/// Everything but `type` and `text` removed from a content item.
fn prune_to_type_and_text(map: &mut Map<String, Value>) {
    map.retain(|key, _| key == "type" || key == "text");
}

/// Elasticsearch's `Json.dump`: real JSON, with each map's members in the order
/// a Java `HashMap` iterates them.
///
/// The order is visible in the output, because the dump lands in a field --
/// a tool call's copied fields come out `input`, `name`, `id` and not in the
/// order the caller sent them.
fn json_dump(value: &Value) -> String {
    serde_json::to_string(&java_ordered(value)).unwrap_or_default()
}

/// One value with every map inside it in Java's iteration order.
fn java_ordered(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut ordered = Map::with_capacity(map.len());
            for (key, member) in java_entries(map) {
                ordered.insert(key, java_ordered(&member));
            }
            Value::Object(ordered)
        }
        Value::Array(items) => Value::Array(items.iter().map(java_ordered).collect()),
        other => other.clone(),
    }
}

/// One map's entries in Java's iteration order: bucket index ascending, and
/// insertion order within a bucket.
fn java_entries(map: &Map<String, Value>) -> Vec<(String, Value)> {
    let table = java_table_size(map.len());
    let mut entries: Vec<(usize, usize, String, Value)> = map
        .iter()
        .enumerate()
        .map(|(at, (key, value))| (java_bucket(key, table), at, key.clone(), value.clone()))
        .collect();
    entries.sort_by_key(|(bucket, at, _, _)| (*bucket, *at));
    entries
        .into_iter()
        .map(|(_, _, key, value)| (key, value))
        .collect()
}

// -- the response body ----------------------------------------------------

/// `aws_bedrock/invocation`, the untagged wrapping script: a streamed response
/// element that is not an object, put under a `value` key.
///
/// A stream ends on the literal `"[DONE]"`, and everything after this treats
/// the list as maps.
fn wrap_scalar_output_bodies(event: &mut Event, _params: &Value) {
    let Some(mut bodies) = event.take_array(OUTPUT_BODY) else {
        return;
    };
    for body in &mut bodies {
        if body.is_object() {
            continue;
        }
        let mut wrapper = Map::with_capacity(1);
        wrapper.insert("value".to_owned(), std::mem::take(body));
        *body = Value::Object(wrapper);
    }
    event.update(OUTPUT_BODY, Value::Array(bodies));
}

/// `aws_bedrock/invocation`, `get_response_id_and_role_from_list`:
/// `gen_ai.response.id` and the replying role, off the first streamed element
/// that names a message.
fn response_id_and_role_from_list(event: &mut Event, _params: &Value) {
    let Some(Value::Array(bodies)) = event.get(OUTPUT_BODY).cloned() else {
        return;
    };
    // `ctx.gen_ai.request.model.role` reaches through a map the model id made.
    if !event.has("gen_ai.request.model") {
        return;
    }
    for body in &bodies {
        let message = body.get("message");
        let Some(id) = message
            .and_then(|message| message.get("id"))
            .filter(|id| !id.is_null())
        else {
            continue;
        };
        let role = message
            .and_then(|message| message.get("role"))
            .cloned()
            .unwrap_or(Value::Null);
        let _ = event.set("gen_ai.request.model.role", role);
        let _ = event.set("gen_ai.response.id", id.clone());
        return;
    }
}

/// `aws_bedrock/invocation`, `get_response_time`: the latencies the model
/// reported, off the LAST streamed element that carries them.
fn response_times_from_metrics(event: &mut Event, _params: &Value) {
    let Some(Value::Array(bodies)) = event.get(OUTPUT_BODY).cloned() else {
        return;
    };
    for body in bodies.iter().rev() {
        let metrics = body.get("amazon_bedrock_invocation_metrics");
        let Some(latency) = metrics
            .and_then(|metrics| metrics.get("invocation_latency"))
            .filter(|latency| !latency.is_null())
        else {
            continue;
        };
        let first_byte = metrics
            .and_then(|metrics| metrics.get("first_byte_latency"))
            .cloned()
            .unwrap_or(Value::Null);
        let _ = event.set("gen_ai.performance.start_response_time", first_byte);
        let _ = event.set("gen_ai.performance.response_time", latency.clone());
        return;
    }
}

/// `aws_bedrock/invocation`, the untagged completion script: the model's reply
/// as one run of text, gathered from whichever response form the model uses.
fn completion_text(event: &mut Event, _params: &Value) {
    let mut text = String::new();
    match event.get(OUTPUT_BODY) {
        Some(Value::Array(blocks)) => {
            for block in blocks {
                append_streamed_block(&mut text, block);
            }
        }
        Some(Value::Object(block)) => append_whole_block(&mut text, block),
        Some(Value::String(raw)) => text.push_str(raw),
        _ => {}
    }
    if utf16_len(&text) > MAX_KEYWORD {
        text = truncate_utf16(&text, MAX_KEYWORD);
    }
    let _ = event.set("aws_bedrock.invocation.output.completion_text", text);
}

/// One element of a streamed response, appended in whichever form it takes.
fn append_streamed_block(text: &mut String, block: &Value) {
    let Some(block) = block.as_object() else {
        return;
    };
    if let Some(Value::Object(delta)) = block.get("delta")
        && let Some(value) = delta.get("text")
    {
        text.push_str(&painless_to_string(value));
    } else if let Some(Value::Array(outputs)) = block.get("outputs") {
        for output in outputs {
            if let Some(output) = output.as_object()
                && let Some(value) = output.get("text")
            {
                text.push_str(&painless_to_string(value));
            }
        }
    } else if let Some(value) = block.get("generation") {
        text.push_str(&painless_to_string(value));
    } else if let Some(value) = block.get("outputText") {
        text.push_str(&painless_to_string(value));
    }
}

/// A response returned whole rather than streamed.
fn append_whole_block(text: &mut String, block: &Map<String, Value>) {
    if let Some(Value::Object(output)) = block.get("output") {
        if let Some(Value::Object(message)) = output.get("message")
            && let Some(Value::Array(content)) = message.get("content")
        {
            append_content_text(text, content);
        }
    } else if let Some(Value::Array(content)) = block.get("content") {
        append_content_text(text, content);
    }
}

/// Every `text` member of a content list, in order.
fn append_content_text(text: &mut String, content: &[Value]) {
    for item in content {
        if let Some(item) = item.as_object()
            && let Some(value) = item.get("text")
        {
            text.push_str(&painless_to_string(value));
        }
    }
}

/// A string cut to `units` UTF-16 units, on a character boundary.
fn truncate_utf16(text: &str, units: usize) -> String {
    let mut taken = 0usize;
    let mut out = String::with_capacity(units);
    for character in text.chars() {
        let width = character.len_utf16();
        if taken + width > units {
            break;
        }
        out.push(character);
        taken += width;
    }
    out
}

// -- the request body -----------------------------------------------------

/// `aws_bedrock/invocation`, `get_response_id_type_and_version`: the model
/// family the id names, and the API version the request body declares for it.
fn model_type_and_version(event: &mut Event, _params: &Value) {
    let Some(id) = event.get_string("gen_ai.request.model.id") else {
        return;
    };
    let Some(dot) = id.find('.') else {
        return;
    };
    let family = id[..dot].to_owned();
    let version = event
        .get(INPUT_BODY)
        .and_then(|body| body.get(format!("{family}_version")))
        .cloned()
        .unwrap_or(Value::Null);
    let _ = event.set("gen_ai.request.model.type", family);
    let _ = event.set("gen_ai.request.model.version", version);
}

/// `aws_bedrock/invocation`, `append_input_message_content_kinds`: the distinct
/// kinds of content the request's messages carry, such as `text` or
/// `document/pdf`.
///
/// The script collects into a `HashSet` and hands the set straight to a list,
/// so the stored order is Java's own and neither sorted nor the order the
/// kinds were seen in.
fn append_input_message_content_kinds(event: &mut Event, _params: &Value) {
    let kinds_path = "aws_bedrock.invocation.input.messages_content_kinds";
    let mut kinds: Vec<Value> = Vec::new();
    if let Some(existing) = event.get_array(kinds_path) {
        for kind in existing {
            if !kinds.contains(kind) {
                kinds.push(kind.clone());
            }
        }
    }
    let Some(Value::Array(messages)) = event.get(&format!("{INPUT_BODY}.messages")).cloned() else {
        return;
    };
    for message in &messages {
        let Some(Value::Array(items)) = message.get("content") else {
            continue;
        };
        for item in items {
            let Some(item) = item.as_object() else {
                continue;
            };
            let Some(kind) = content_kind(item) else {
                continue;
            };
            let kind = Value::from(kind);
            if !kinds.contains(&kind) {
                kinds.push(kind);
            }
        }
    }
    let _ = event.set(kinds_path, Value::Array(java_set_order(kinds)));
}

/// The kind one content item is, absent where it is neither text nor a
/// document.
fn content_kind(item: &Map<String, Value>) -> Option<String> {
    if item.contains_key("text") {
        return Some("text".to_owned());
    }
    let document = item.get("document")?;
    let mut kind = String::from("document");
    if let Value::Object(document) = document
        && let Some(format) = document.get("format")
    {
        kind.push('/');
        kind.push_str(&painless_to_string(format));
    }
    Some(kind)
}

// -- the guardrail assessments --------------------------------------------

/// `aws_bedrock/invocation`, `get_guardrail_details`: every guardrail policy
/// the response reports, flattened onto the ECS `gen_ai.policy.*` and
/// `gen_ai.compliance.*` arrays.
///
/// A guardrail assessment nests four deep -- the response body, the guardrail
/// id, the policy name, then the policy's own list of findings -- and each of
/// the six fields is one projection of that flattening.
fn guardrail_details(event: &mut Event, _params: &Value) {
    let Some(body) = event.get(OUTPUT_BODY).cloned() else {
        return;
    };
    let bodies: Vec<Value> = match body {
        Value::Array(items) => items,
        object @ Value::Object(_) => vec![object],
        _ => return,
    };

    let findings = policy_findings(&bodies);

    let mut ids: Vec<Value> = Vec::new();
    for body in &bodies {
        for source in [
            assessment(body, "trace", "inputAssessment"),
            assessment(body, "amazon_bedrock_trace", "input"),
        ] {
            let Some(Value::Object(map)) = source else {
                continue;
            };
            for key in map.keys() {
                push_distinct(&mut ids, Value::from(key.clone()));
            }
        }
    }
    let _ = event.set("gen_ai.guardrail_id", sorted(&ids));

    let names = findings.iter().map(|(name, _)| Value::from(name.clone()));
    let _ = event.set("gen_ai.policy.name", sorted(&distinct(names)));

    let actions = findings
        .iter()
        .filter_map(|(_, details)| details.get("action").cloned());
    let _ = event.set("gen_ai.policy.action", sorted(&distinct(actions)));

    let matched = findings
        .iter()
        .filter(|(name, details)| {
            details.get("match").is_some_and(|value| !value.is_null())
                || name == "contextual_grounding_policy"
        })
        .map(|(_, details)| details.clone());
    let _ = event.set(
        "gen_ai.policy.match_detail",
        Value::Array(distinct(matched)),
    );

    let confidences = findings
        .iter()
        .filter_map(|(_, details)| details.get("confidence").cloned())
        .filter(|value| !value.is_null());
    let _ = event.set("gen_ai.policy.confidence", sorted(&distinct(confidences)));

    let codes = findings
        .iter()
        .filter_map(|(_, details)| details.get("type").cloned())
        .filter(|value| !value.is_null());
    let codes = sorted(&distinct(codes));
    let intervened = codes.as_array().is_some_and(|codes| !codes.is_empty())
        && bodies.iter().any(|body| {
            body.get("amazon_bedrock_guardrail_action")
                .and_then(Value::as_str)
                == Some("INTERVENED")
                || body.get("stop_reason").and_then(Value::as_str) == Some("guardrail_intervened")
        });
    let _ = event.set("gen_ai.compliance.violation_code", codes);

    if intervened {
        let _ = event.set("gen_ai.compliance.violation_detected", true);
        let _ = event.set("event.outcome", "failure");
    }
}

/// Every `(policy name, finding)` pair the response carries, in the order the
/// script's own stream walks them.
fn policy_findings(bodies: &[Value]) -> Vec<(String, Value)> {
    let mut findings = Vec::new();
    for body in bodies {
        for assessments in assessment_sources(body) {
            for entry in assessments {
                let Some(guardrails) = entry.as_object() else {
                    continue;
                };
                for (_, policies) in java_entries(guardrails) {
                    let Some(policies) = policies.as_object() else {
                        continue;
                    };
                    for (name, policy) in java_entries(policies) {
                        if !name.ends_with("_policy") {
                            continue;
                        }
                        let Some(policy) = policy.as_object() else {
                            continue;
                        };
                        for (_, group) in java_entries(policy) {
                            let Some(group) = group.as_array() else {
                                continue;
                            };
                            for detail in group {
                                findings.push((name.clone(), detail.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    findings
}

/// The four places one response body reports an assessment, in the order the
/// script lists them.
fn assessment_sources(body: &Value) -> Vec<Vec<Value>> {
    vec![
        single(assessment(body, "trace", "inputAssessment")),
        many(assessment(body, "trace", "outputAssessments")),
        single(assessment(body, "amazon_bedrock_trace", "input")),
        many(assessment(body, "amazon_bedrock_trace", "outputs")),
    ]
}

/// One assessment slot under a body's guardrail trace.
fn assessment<'a>(body: &'a Value, trace: &str, slot: &str) -> Option<&'a Value> {
    body.get(trace)?.get("guardrail")?.get(slot)
}

/// A slot the script wraps in a list of its own, which drops a null.
fn single(value: Option<&Value>) -> Vec<Value> {
    match value {
        Some(value) if !value.is_null() => vec![value.clone()],
        _ => Vec::new(),
    }
}

/// A slot the script streams directly, which is a list or nothing.
fn many(value: Option<&Value>) -> Vec<Value> {
    match value {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// The values of a stream with the repeats dropped, first occurrence kept.
fn distinct(values: impl Iterator<Item = Value>) -> Vec<Value> {
    let mut kept: Vec<Value> = Vec::new();
    for value in values {
        push_distinct(&mut kept, value);
    }
    kept
}

/// One value appended where the list does not already hold it.
fn push_distinct(kept: &mut Vec<Value>, value: Value) {
    if !kept.contains(&value) {
        kept.push(value);
    }
}

/// A list in Java's natural string order.
///
/// A null never reaches here: Java's own comparator throws on one, and every
/// stream this is used on filters nulls out first or reads a member the
/// vendor always sends.
fn sorted(values: &[Value]) -> Value {
    let ordered: BTreeSet<(String, usize)> = values
        .iter()
        .enumerate()
        .map(|(at, value)| (painless_to_string(value), at))
        .collect();
    Value::Array(
        ordered
            .into_iter()
            .map(|(_, at)| values[at].clone())
            .collect(),
    )
}

/// Every `aws_bedrock` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "414ac3ec4eb6dea11366a1b00e5849cf740b60a0728cb184c4264e3d5a34542b",
        source: "aws_bedrock",
        name: "remarshal_bodies",
        run: remarshal_bodies,
    },
    Entry {
        hash: "97bec2516ca64fa920dbf36d0f45418a17aaff41a23b890c76baac4895f19184",
        source: "aws_bedrock",
        name: "promote_system_to_object",
        run: promote_system_to_object,
    },
    Entry {
        hash: "3b733bb75041e36818f98adb0fa60315cab918b98d7b79d97a7f6bb0afec4d9e",
        source: "aws_bedrock",
        name: "normalise_message_content",
        run: normalise_message_content,
    },
    Entry {
        hash: "a9592ae374567c283b817576464ea6e93426a477d70f61844684c4eab03d516f",
        source: "aws_bedrock",
        name: "ensure_message_content_text",
        run: ensure_message_content_text,
    },
    Entry {
        hash: "1516b826d27996a34f9baeaa20650d1c5d06ae1e8a0d206c68f7017ec3aab05b",
        source: "aws_bedrock",
        name: "model_type_and_version",
        run: model_type_and_version,
    },
    Entry {
        hash: "f67e38da0b735f09c120cb228e2efd9a05059717d72b78966ad18bc1ac680a53",
        source: "aws_bedrock",
        name: "append_input_message_content_kinds",
        run: append_input_message_content_kinds,
    },
    Entry {
        hash: "4aee7cadeb7406b510bca77f329675668740eb4cc1c51c530a5dcce3c50be02c",
        source: "aws_bedrock",
        name: "wrap_scalar_output_bodies",
        run: wrap_scalar_output_bodies,
    },
    Entry {
        hash: "f1cbffacd2c6667a60e764fd4eff2d61e530c066850c6860e862eafcf42a77c4",
        source: "aws_bedrock",
        name: "response_id_and_role_from_list",
        run: response_id_and_role_from_list,
    },
    Entry {
        hash: "c70fc35a7a19eda11d89a23b85431ef43ec2909384a96d144a039647c0dde4ce",
        source: "aws_bedrock",
        name: "response_times_from_metrics",
        run: response_times_from_metrics,
    },
    Entry {
        hash: "b93dbb6b33d08693f643a44e200cdf73c72c1c0d76a8a9bf78a6a8933673ca23",
        source: "aws_bedrock",
        name: "guardrail_details",
        run: guardrail_details,
    },
    Entry {
        hash: "7a6359986918764f5954609ab2b9508bc3f557f09a2ed9eb602ce422b82a6009",
        source: "aws_bedrock",
        name: "request_size_from_prompt",
        run: request_size_from_prompt,
    },
    Entry {
        hash: "8a8f72d0e37bbd66f53b3e6616c748b1df6b5422d4f51529b444a3154f703583",
        source: "aws_bedrock",
        name: "completion_text",
        run: completion_text,
    },
    Entry {
        hash: "cd5488bb3ae04bd38827b9edb013c6fbc2676d1564246fd311dc64e4d9daea4e",
        source: "aws_bedrock",
        name: "response_size_from_completion",
        run: response_size_from_completion,
    },
];
