// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `azure_openai`'s key rewriting and its two content-filter summaries,
//! transcribed.
//!
//! Azure reports a filter verdict per category as its own object, and the
//! pipeline flattens the filtered ones into a list the mapping can hold.

use serde_json::{Map, Value, json};

use dfe_core::Event;

use super::Entry;

/// The categories Azure reports a verdict for, in the script's order.
const CATEGORIES: [&str; 4] = ["self_harm", "sexual", "hate", "violence"];

/// The rejection, where the prompt's own verdicts sit.
const INNER_ERROR: &str = "azure.open_ai.properties.backend_response_body.error.innererror";

/// `azure-json-keys-to-snake-case` in `azure_openai/logs`: every key under
/// `azure.open_ai` taken from Azure's camelCase to snake case.
fn keys_to_snake_case(event: &mut Event, _params: &Value) {
    let converted = {
        // The vendor's `keysToSnakeCase(Map m)` raises on anything else.
        let Some(held) = event.get_object("azure.open_ai") else {
            return;
        };
        snake_case_keys(held)
    };
    let _ = event.update("azure.open_ai", Value::Object(converted));
}

/// One map's keys rewritten, and a nested map's with it.
///
/// A map inside a LIST is rewritten too, but only one level down: the vendor
/// script does not recurse into a list of lists.
fn snake_case_keys(map: &Map<String, Value>) -> Map<String, Value> {
    let mut out = Map::with_capacity(map.len());
    for (key, value) in map {
        let value = match value {
            Value::Object(nested) => Value::Object(snake_case_keys(nested)),
            Value::Array(items) => Value::Array(
                items
                    .iter()
                    .map(|item| match item {
                        Value::Object(nested) => Value::Object(snake_case_keys(nested)),
                        other => other.clone(),
                    })
                    .collect(),
            ),
            other => other.clone(),
        };
        out.insert(snake_case_key(key), value);
    }
    out
}

/// The vendor's `_?([a-z])([A-Z]+)` to `$1_$2`, then the whole key lowered.
///
/// The optional leading underscore is dropped by the replacement, so a key
/// already carrying a separator does not collect a second one.
fn snake_case_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    let mut out = String::with_capacity(key.len() + 4);
    let mut at = 0;
    while at < chars.len() {
        let mut cursor = at;
        if chars[cursor] == '_' {
            cursor += 1;
        }
        let run = uppercase_run(&chars, cursor);
        if run > cursor + 1 {
            out.push(chars[cursor]);
            out.push('_');
            out.extend(&chars[cursor + 1..run]);
            at = run;
            continue;
        }
        out.push(chars[at]);
        at += 1;
    }
    out.to_lowercase()
}

/// The end of a lowercase-then-uppercase run starting at `from`, or `from`
/// itself where no lowercase letter sits there.
fn uppercase_run(chars: &[char], from: usize) -> usize {
    if !chars.get(from).is_some_and(char::is_ascii_lowercase) {
        return from;
    }
    let mut end = from + 1;
    while chars.get(end).is_some_and(char::is_ascii_uppercase) {
        end += 1;
    }
    end
}

/// The untagged reply summariser in `azure_openai/logs`: the filtered
/// categories across every choice the model returned.
///
/// Only a category Azure actually filtered reaches the list, so a reply that
/// tripped nothing leaves an empty one for the module's closing prune.
fn response_content_filters(event: &mut Event, _params: &Value) {
    let filtered = {
        let Some(choices) =
            event.get_array("azure.open_ai.properties.backend_response_body.choices")
        else {
            return;
        };
        let mut filtered = Vec::new();
        for choice in choices {
            let Some(results) = choice.get("content_filter_results") else {
                continue;
            };
            for name in CATEGORIES {
                if let Some(verdict) = filtered_category(results, name) {
                    filtered.push(verdict);
                }
            }
        }
        filtered
    };
    let _ = event.set(
        "azure.open_ai.properties.backend_response_body.content_filtered_categories",
        Value::Array(filtered),
    );
}

/// The untagged rejection summariser in `azure_openai/logs`: the prompt's own
/// filter verdicts.
///
/// Every category is appended whether or not it tripped, so three of the four
/// maps are EMPTY on a normal rejection and the module's closing prune is what
/// takes them out again.
fn prompt_content_filters(event: &mut Event, _params: &Value) {
    // Painless raises where the rejection is absent, and the processor
    // swallows it, so an accepted prompt keeps nothing.
    if !event.has(INNER_ERROR) {
        return;
    }
    let verdicts = {
        let results = event.get(
            "azure.open_ai.properties.backend_response_body.error.innererror.content_filter_result",
        );
        CATEGORIES
            .iter()
            .map(|name| {
                results
                    .and_then(|results| filtered_category(results, name))
                    .unwrap_or_else(|| Value::Object(Map::new()))
            })
            .collect()
    };
    let _ = event.set(
        "azure.open_ai.properties.backend_response_body.error.innererror.content_filtered_categories",
        Value::Array(verdicts),
    );
}

/// One category's verdict, where Azure says it filtered on that category.
fn filtered_category(results: &Value, name: &str) -> Option<Value> {
    let category = results.get(name)?;
    if category.get("filtered").and_then(Value::as_bool) != Some(true) {
        return None;
    }
    let mut verdict = Map::with_capacity(2);
    verdict.insert("category_name".to_string(), json!(name));
    verdict.insert(
        "severity".to_string(),
        category.get("severity").cloned().unwrap_or(Value::Null),
    );
    Some(Value::Object(verdict))
}

/// Every `azure_openai` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "891ec7306240546833e7cc71b1eed08383bc2aef2fd446d263d8d2485c4632b0",
        source: "azure_openai",
        name: "keys_to_snake_case",
        run: keys_to_snake_case,
    },
    Entry {
        hash: "9789e2dfe5ab29131825524554984a8c704735ce5edf7f855122cf2affb2645c",
        source: "azure_openai",
        name: "response_content_filters",
        run: response_content_filters,
    },
    Entry {
        hash: "b061a2c053dc2f2c0920eb44f3c2f7eded31cbb183da087ffea6edb3655ba2c1",
        source: "azure_openai",
        name: "prompt_content_filters",
        run: prompt_content_filters,
    },
];
