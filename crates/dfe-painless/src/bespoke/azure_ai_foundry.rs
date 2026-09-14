// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `azure_ai_foundry`'s gateway-log scripts, transcribed.
//!
//! The gateway records a model's content-filter verdict as a map of per-category
//! flags, and the pipeline flattens the categories that actually fired into one
//! list. There are two of them because a filtered PROMPT is reported under the
//! error's `innererror` and a filtered RESPONSE under each choice.

use serde_json::{Map, Value, json};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{SnakeRule, to_snake_case};

/// Where the prompt-side verdict is written.
const PROMPT_TARGET: &str = "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filtered_categories";

/// The map the prompt-side script assigns through. Painless raises where it is
/// absent, and the processor's `ignore_failure` swallows that, so an event
/// without one is left alone rather than having the path built for it.
const INNERERROR: &str = "azure.ai_foundry.properties.backend_response_body.error.innererror";

/// Each prompt-side category, with the flag and the severity it reads.
const PROMPT_CATEGORIES: [(&str, &str, &str); 4] = [
    (
        "self_harm",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.self_harm.filtered",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.self_harm.severity",
    ),
    (
        "sexual",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.sexual.filtered",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.sexual.severity",
    ),
    (
        "hate",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.hate.filtered",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.hate.severity",
    ),
    (
        "violence",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.violence.filtered",
        "azure.ai_foundry.properties.backend_response_body.error.innererror.content_filter_result.violence.severity",
    ),
];

/// The choices the response-side script walks.
const CHOICES: &str = "azure.ai_foundry.properties.backend_response_body.choices";

/// Where the response-side verdict is written.
const RESPONSE_TARGET: &str =
    "azure.ai_foundry.properties.backend_response_body.content_filtered_categories";

/// The response-side categories, in the order the script tests them.
const RESPONSE_CATEGORIES: [&str; 4] = ["self_harm", "sexual", "hate", "violence"];

/// `azure-json-keys-to-snake-case` in `azure_ai_foundry/logs`: every key under
/// `azure.ai_foundry` rewritten.
///
/// The script's rule is the regex `_?([a-z])([A-Z]+)` replaced by `$1_$2` and
/// then lowercased, which is [`SnakeRule::CamelBreak`]. It recurses into a map
/// and into the MAP ELEMENTS of a list, and leaves anything else as it stands.
fn keys_to_snake_case(event: &mut Event, _params: &Value) {
    let Some(held) = event.get("azure.ai_foundry") else {
        return;
    };
    if !held.is_object() {
        return;
    }
    let rewritten = rewrite_keys(held);
    let _ = event.set("azure.ai_foundry", rewritten);
}

/// One map's keys, and its children's, under the script's own rule.
fn rewrite_keys(value: &Value) -> Value {
    let Value::Object(map) = value else {
        return value.clone();
    };
    let mut out = Map::with_capacity(map.len());
    for (key, held) in map {
        let held = match held {
            Value::Object(_) => rewrite_keys(held),
            Value::Array(items) => Value::Array(
                items
                    .iter()
                    .map(|item| {
                        if item.is_object() {
                            rewrite_keys(item)
                        } else {
                            item.clone()
                        }
                    })
                    .collect(),
            ),
            other => other.clone(),
        };
        out.insert(to_snake_case(key, SnakeRule::CamelBreak), held);
    }
    Value::Object(out)
}

/// The untagged prompt-side script in `azure_ai_foundry/logs`:
/// `...error.innererror.content_filtered_categories`.
///
/// A category that did not fire still contributes an EMPTY map, which is what
/// the script does -- the pipeline's closing empty-value prune is what takes
/// those back out, and a list left with none of them goes with it.
fn prompt_filtered_categories(event: &mut Event, _params: &Value) {
    if !event.get(INNERERROR).is_some_and(Value::is_object) {
        return;
    }
    let categories: Vec<Value> = PROMPT_CATEGORIES
        .iter()
        .map(|(name, filtered, severity)| {
            if event.get_bool(filtered) == Some(true) {
                category(name, event.get(severity).cloned())
            } else {
                Value::Object(Map::new())
            }
        })
        .collect();
    let _ = event.set(PROMPT_TARGET, Value::Array(categories));
}

/// The untagged response-side script in `azure_ai_foundry/logs`:
/// `...backend_response_body.content_filtered_categories`.
///
/// Here a category that did not fire adds nothing at all, so the list carries
/// only what actually fired, across every choice.
fn response_filtered_categories(event: &mut Event, _params: &Value) {
    let categories = {
        let Some(choices) = event.get_array(CHOICES) else {
            return;
        };
        let mut categories = Vec::new();
        for choice in choices {
            let results = choice.get("content_filter_results");
            for name in RESPONSE_CATEGORIES {
                let Some(verdict) = results.and_then(|held| held.get(name)) else {
                    continue;
                };
                if verdict.get("filtered") == Some(&json!(true)) {
                    categories.push(category(name, verdict.get("severity").cloned()));
                }
            }
        }
        categories
    };
    let _ = event.set(RESPONSE_TARGET, Value::Array(categories));
}

/// One `{category_name, severity}` entry. An absent severity is the null
/// Painless would put there, which the prune then drops.
fn category(name: &str, severity: Option<Value>) -> Value {
    let mut entry = Map::with_capacity(2);
    entry.insert("category_name".to_string(), json!(name));
    entry.insert("severity".to_string(), severity.unwrap_or(Value::Null));
    Value::Object(entry)
}

/// Every `azure_ai_foundry` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "4bff6526a5362d6d92ab11d55b0eabd262c23cb140cc540c8690f042695968a6",
        source: "azure_ai_foundry",
        name: "keys_to_snake_case",
        run: keys_to_snake_case,
    },
    Entry {
        hash: "8f7295a003cb369f27b9414d90a40dbd1cf7d800343bbdb3f5afd43ea37efd65",
        source: "azure_ai_foundry",
        name: "prompt_filtered_categories",
        run: prompt_filtered_categories,
    },
    Entry {
        hash: "1d8cd7d92318776210c7b6f6b623553e5c8b1be369b55ddb0a3f594368f03a3e",
        source: "azure_ai_foundry",
        name: "response_filtered_categories",
        run: response_filtered_categories,
    },
];
