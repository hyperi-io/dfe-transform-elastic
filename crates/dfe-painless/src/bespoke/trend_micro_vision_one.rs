// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `trend_micro_vision_one`'s alert payload, severity and indicator fields,
//! transcribed.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// The indicator list the alert carries.
const INDICATORS: &str = "trend_micro_vision_one.alert.indicators";

/// `set_event_severity` in `trend_micro_vision_one/alert`: `event.severity` as
/// the ECS score the vendor's label maps to.
fn severity_from_alert_severity(event: &mut Event, _params: &Value) {
    let Some(severity) = event.get_string("trend_micro_vision_one.alert.severity") else {
        return;
    };
    let score = if severity.eq_ignore_ascii_case("low") {
        21
    } else if severity.eq_ignore_ascii_case("medium") {
        47
    } else if severity.eq_ignore_ascii_case("high") {
        73
    } else if severity.eq_ignore_ascii_case("critical") {
        99
    } else {
        return;
    };
    let _ = event.set("event.severity", score);
}

/// `script_convert_camelcase_to_snake_case` in `trend_micro_vision_one/alert`:
/// the whole vendor payload re-keyed under `trend_micro_vision_one.alert`.
fn alert_from_json(event: &mut Event, _params: &Value) {
    let converted = {
        let Some(payload) = event.get("json") else {
            return;
        };
        snake_case(payload)
    };
    let _ = event.set("trend_micro_vision_one.alert", converted);
}

/// The vendor's own `convertToSnakeCase`, which also reads an empty string as
/// an absent value so the module's closing prune takes it away.
fn snake_case(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = Map::with_capacity(map.len());
            for (key, held) in map {
                out.insert(camel_to_snake(key), snake_case(held));
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(snake_case).collect()),
        Value::String(text) if text.is_empty() => Value::Null,
        other => other.clone(),
    }
}

/// One key from camelCase to snake case.
///
/// A RUN of capitals takes one separator between them all, so `objectIPs`
/// becomes `object_ips` rather than `object_i_ps`.
fn camel_to_snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    let mut last_was_upper = false;
    for (index, letter) in key.chars().enumerate() {
        if letter.is_uppercase() {
            if index > 0 && !last_was_upper {
                out.push('_');
            }
            out.extend(letter.to_lowercase());
            last_was_upper = true;
        } else {
            out.push(letter);
            last_was_upper = false;
        }
    }
    out
}

/// The untagged field flattener in `trend_micro_vision_one/alert`: an
/// indicator's `fields` taken from a list of lists to one sorted list.
///
/// The vendor reports the fields that matched per detection, so an indicator
/// matched twice arrives as two lists of the same names.
fn flatten_indicator_fields(event: &mut Event, _params: &Value) {
    let Some(mut indicators) = event.take_array(INDICATORS) else {
        return;
    };
    for indicator in &mut indicators {
        let Some(slot) = indicator.as_object_mut() else {
            continue;
        };
        let replacement = match slot.get("fields") {
            None | Some(Value::Null | Value::String(_)) => continue,
            Some(Value::Array(groups)) => match groups.first() {
                Some(Value::Array(first)) => Value::Array(sorted(groups, first)),
                _ => continue,
            },
            // Anything that is not a list is not a field set either, and the
            // script nulls it out for the closing prune.
            Some(_) => Value::Null,
        };
        slot.insert("fields".to_string(), replacement);
    }
    let _ = event.update(INDICATORS, Value::Array(indicators));
}

/// The field names across every group, deduplicated and sorted.
///
/// One group is taken as it stands, because the script sorts that list alone
/// rather than gathering it.
fn sorted(groups: &[Value], first: &[Value]) -> Vec<Value> {
    let mut names: Vec<Value> = if groups.len() == 1 {
        first.to_vec()
    } else {
        let mut gathered: Vec<Value> = Vec::new();
        for group in groups {
            let Some(items) = group.as_array() else {
                continue;
            };
            for item in items {
                if !gathered.contains(item) {
                    gathered.push(item.clone());
                }
            }
        }
        gathered
    };
    names.sort_by_cached_key(painless_to_string);
    names
}

/// Every `trend_micro_vision_one` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "d60d35ac2a0e7913dc5ea001d2d462b6b5e884cd3857d91584fb143af41e4d22",
        source: "trend_micro_vision_one",
        name: "severity_from_alert_severity",
        run: severity_from_alert_severity,
    },
    Entry {
        hash: "24cb98ee25c187322525fc09a950953cd7ecdace2bb7f407300ebe2943e8c69c",
        source: "trend_micro_vision_one",
        name: "alert_from_json",
        run: alert_from_json,
    },
    Entry {
        hash: "0ed8c5787991d96521adc68c5d835954526672e64b1e6c403151948117768cff",
        source: "trend_micro_vision_one",
        name: "flatten_indicator_fields",
        run: flatten_indicator_fields,
    },
];
