// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `stormshield`'s statistics reshaping: the counters a firewall reports as
//! one flat key=value line turned into the nested `metadata` the package
//! declares.
//!
//! The vendor packs several numbers into one key or one value -- `Byte(i/o)`
//! against `10592/14285`, `Ethernet0` against a seven-field CSV, `Rule1:3`
//! against a byte count -- and each `logtype` gets the script that unpacks its
//! own spelling. Every one of them assembles its answer and then writes, so a
//! counter the script's own `Long.parseLong` would raise on leaves the event
//! as it was.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The namespace all four scripts read and write.
const ROOT: &str = "stormshield";

/// `script_split_composite_pairs` in `compat-stormshield-log-filterstat`:
/// every capitalised counter into `metadata`, and a `Name(a/b)` key split
/// into a `Name` holding one member per half.
fn split_composite_pairs(event: &mut Event, params: &Value) {
    let Some(mut namespace) = event.get_object(ROOT).cloned() else {
        return;
    };

    let mut composites: Vec<(String, String)> = Vec::new();
    let mut moved: Vec<String> = Vec::new();
    for (key, value) in &namespace {
        if key.contains('(') {
            composites.push((key.clone(), painless_string(value)));
        } else if key.chars().next().is_some_and(char::is_uppercase) {
            moved.push(key.clone());
        }
    }

    let mut gathered: Vec<(String, Value)> = Vec::new();
    for key in &moved {
        let value = namespace.get(key).cloned().unwrap_or(Value::Null);
        gathered.push((key.clone(), value));
    }
    let mut consumed: Vec<String> = moved;

    for (key, value) in composites {
        let Some((name, first_half, second_half)) = composite_key(&key) else {
            continue;
        };
        let Some((first, second)) = composite_value(&value) else {
            continue;
        };
        let (Some(first), Some(second)) = (parse_long(first), parse_long(second)) else {
            return;
        };

        let mut halves = Map::with_capacity(2);
        halves.insert(renamed(params, first_half), Value::from(first));
        halves.insert(renamed(params, second_half), Value::from(second));
        gathered.push((name.to_owned(), Value::Object(halves)));
        consumed.push(key);
    }

    for key in &consumed {
        namespace.shift_remove(key);
    }
    let Some(metadata) = metadata_of(&mut namespace) else {
        return;
    };
    for (key, value) in gathered {
        metadata.insert(key, value);
    }

    event.update(ROOT, Value::Object(namespace));
}

/// `script_process_ethernet` in `compat-stormshield-log-monitor`: one
/// `metadata` member per port, and the port names beside them.
fn process_ethernet(event: &mut Event, params: &Value) {
    let Some(mut namespace) = event.get_object(ROOT).cloned() else {
        return;
    };
    let stats = params.get("stats").and_then(Value::as_array);
    let found = device_stats(&namespace, params);
    if found.is_empty() {
        return;
    }

    let mut ports: Vec<String> = Vec::with_capacity(found.len());
    let mut gathered: Vec<(String, Value)> = Vec::with_capacity(found.len());
    for (key, value, _) in found {
        ports.push(key.clone());
        let Some(item) = stat_item(&key, &value, stats) else {
            return;
        };
        namespace.shift_remove(&key);
        gathered.push((key, item));
    }

    let Some(metadata) = metadata_of(&mut namespace) else {
        return;
    };
    for (key, item) in gathered {
        metadata.insert(key, item);
    }

    ports.sort();
    namespace.insert(
        "ports".to_owned(),
        Value::Array(ports.into_iter().map(Value::from).collect()),
    );

    event.update(ROOT, Value::Object(namespace));
}

/// `script_process_devices` in `compat-stormshield-log-monitor`: the per-slot
/// counters gathered under `device_stats`, one list per device stem, plus the
/// devices whose counters the params table names one by one.
fn process_devices(event: &mut Event, params: &Value) {
    let Some(mut namespace) = event.get_object(ROOT).cloned() else {
        return;
    };
    let stats = params.get("stats").and_then(Value::as_array);
    let found = device_stats(&namespace, params);
    let special = special_cases(&namespace, params);
    if found.is_empty() && special.is_empty() {
        return;
    }

    let mut gathered: Vec<(String, Vec<Value>)> = Vec::new();
    let mut consumed: Vec<String> = Vec::new();
    for (key, value, stem) in found {
        let Some(item) = stat_item(&key, &value, stats) else {
            return;
        };
        match gathered.iter_mut().find(|(held, _)| *held == stem) {
            Some((_, items)) => items.push(item),
            None => gathered.push((stem, vec![item])),
        }
        consumed.push(key);
    }

    let mut devices = Map::with_capacity(gathered.len() + special.len());
    for (stem, mut items) in gathered {
        items.sort_by(|left, right| original_of(left).cmp(original_of(right)));
        devices.insert(stem, Value::Array(items));
    }
    for (key, counters) in special {
        devices.insert(key.clone(), counters);
        consumed.push(key);
    }

    for key in &consumed {
        namespace.shift_remove(key);
    }
    let Some(metadata) = metadata_of(&mut namespace) else {
        return;
    };
    metadata.insert("device_stats".to_owned(), Value::Object(devices));

    event.update(ROOT, Value::Object(namespace));
}

/// `script_process_devices` in `compat-stormshield-log-count`: a `RuleN:M`
/// counter read as a rule number and a byte count under the category the
/// params table gives `N`.
fn process_rule_stats(event: &mut Event, params: &Value) {
    let Some(mut namespace) = event.get_object(ROOT).cloned() else {
        return;
    };
    let categories = params.get("stats").and_then(Value::as_array);

    let mut rules: Vec<Value> = Vec::new();
    let mut consumed: Vec<String> = Vec::new();
    for (key, value) in &namespace {
        let Some(suffix) = key.strip_prefix("Rule") else {
            continue;
        };
        // The colon has to sit BEYOND the prefix, so a bare `Rule:` is not one.
        if !suffix.contains(':') || suffix.starts_with(':') {
            continue;
        }
        consumed.push(key.clone());

        let value = painless_string(value);
        let mut parts = suffix.split(':');
        let (Some(category), Some(number), None) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        if !is_numeric(&value)
            || !is_numeric(number)
            || category.len() != 1
            || !is_numeric(category)
        {
            continue;
        }
        let (Ok(index), Some(byte_count), Some(number)) = (
            category.parse::<usize>(),
            parse_long(&value),
            parse_long(number),
        ) else {
            continue;
        };
        let Some(name) = categories.and_then(|held| held.get(index)) else {
            continue;
        };

        let mut item = Map::with_capacity(4);
        item.insert("original".to_owned(), Value::from(key.clone()));
        item.insert("byte_count".to_owned(), Value::from(byte_count));
        item.insert("category".to_owned(), name.clone());
        item.insert("rule_number".to_owned(), Value::from(number));
        rules.push(Value::Object(item));
    }

    for key in &consumed {
        namespace.shift_remove(key);
    }
    if rules.is_empty() {
        event.update(ROOT, Value::Object(namespace));
        return;
    }

    rules.sort_by(|left, right| original_of(left).cmp(original_of(right)));
    let Some(metadata) = metadata_of(&mut namespace) else {
        return;
    };
    metadata.insert("rule_stats".to_owned(), Value::Array(rules));

    event.update(ROOT, Value::Object(namespace));
}

/// The `metadata` child, created empty where the namespace has none.
///
/// A `metadata` that is not a map raises in Painless on the first write, and
/// answers None here so the caller writes nothing.
fn metadata_of(namespace: &mut Map<String, Value>) -> Option<&mut Map<String, Value>> {
    if !namespace.contains_key("metadata") {
        namespace.insert("metadata".to_owned(), Value::Object(Map::new()));
    }
    namespace.get_mut("metadata")?.as_object_mut()
}

/// Every counter whose key starts with one of the device stems the params
/// table names, with the stem that claimed it.
///
/// A key matching two stems takes the LAST, which is the map assignment the
/// script's own nested loop leaves behind.
fn device_stats(namespace: &Map<String, Value>, params: &Value) -> Vec<(String, String, String)> {
    let Some(devices) = params.get("devices").and_then(Value::as_array) else {
        return Vec::new();
    };

    let mut found = Vec::new();
    for (key, value) in namespace {
        let mut claimed: Option<&str> = None;
        for device in devices {
            if let Some(stem) = device.as_str()
                && key.starts_with(stem)
            {
                claimed = Some(stem);
            }
        }
        if let Some(stem) = claimed {
            found.push((key.clone(), painless_string(value), stem.to_owned()));
        }
    }
    found
}

/// The devices whose counters the params table names one by one, each read as
/// that many numbers.
fn special_cases(namespace: &Map<String, Value>, params: &Value) -> Vec<(String, Value)> {
    let Some(cases) = params.get("special_cases").and_then(Value::as_array) else {
        return Vec::new();
    };

    let mut found: Vec<(String, Value)> = Vec::new();
    for (key, value) in namespace {
        for case in cases {
            let Some(prefix) = case.get("key").and_then(Value::as_str) else {
                continue;
            };
            let Some(names) = case.get("value").and_then(Value::as_array) else {
                continue;
            };
            if !key.starts_with(prefix) {
                continue;
            }

            let value = painless_string(value);
            let parts = split_on_token(&value, ',', 3);
            if parts.len() < names.len() {
                continue;
            }
            let mut counters = Map::with_capacity(names.len());
            for (name, part) in names.iter().zip(parts) {
                let (Some(name), Some(count)) = (name.as_str(), parse_long(part)) else {
                    continue;
                };
                counters.insert(name.to_owned(), Value::from(count));
            }
            match found.iter_mut().find(|(held, _)| held == prefix) {
                Some((_, held)) => *held = Value::Object(counters),
                None => found.push((prefix.to_owned(), Value::Object(counters))),
            }
        }
    }
    found
}

/// One device's CSV counters under the names the params table gives them.
///
/// The first is the port's own name and stays a string; every other is a
/// count. None where one of those counts does not parse, which is the raise.
fn stat_item(key: &str, value: &str, stats: Option<&Vec<Value>>) -> Option<Value> {
    let stats = stats?;
    let mut item = Map::with_capacity(stats.len() + 1);
    item.insert("original".to_owned(), Value::from(key.to_owned()));
    for (position, part) in split_on_token(value, ',', 7).into_iter().enumerate() {
        let name = stats.get(position)?.as_str()?;
        if position == 0 {
            item.insert(name.to_owned(), Value::from(part.to_owned()));
        } else {
            item.insert(name.to_owned(), Value::from(parse_long(part)?));
        }
    }
    Some(Value::Object(item))
}

/// A gathered item's `original`, which is what every one of these lists sorts
/// on.
fn original_of(item: &Value) -> &str {
    item.get("original").and_then(Value::as_str).unwrap_or("")
}

/// `^(\w+)\((\w+)/(\w+)\)$` over a composite key.
fn composite_key(key: &str) -> Option<(&str, &str, &str)> {
    let (name, rest) = key.split_once('(')?;
    let (first, second) = rest.strip_suffix(')')?.split_once('/')?;
    (is_word(name) && is_word(first) && is_word(second)).then_some((name, first, second))
}

/// `^(\w+)/(\w+)$` over a composite value.
fn composite_value(value: &str) -> Option<(&str, &str)> {
    let (first, second) = value.split_once('/')?;
    (is_word(first) && is_word(second)).then_some((first, second))
}

/// The name the params table gives a half of a composite key, or the half
/// itself where it names none.
fn renamed(params: &Value, half: &str) -> String {
    params
        .get(half)
        .and_then(Value::as_str)
        .unwrap_or(half)
        .to_owned()
}

/// Java's `String.splitOnToken`: at most `limit` parts, the last holding
/// everything after the last split.
fn split_on_token(text: &str, token: char, limit: usize) -> Vec<&str> {
    text.splitn(limit, token).collect()
}

/// Java's `\w`, which is ASCII.
fn is_word(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// The script's own `isNumeric`: a non-empty run of digits no longer than a
/// `long`.
fn is_numeric(text: &str) -> bool {
    !text.is_empty() && text.len() <= 18 && text.bytes().all(|b| b.is_ascii_digit())
}

/// `Long.parseLong`, which takes a string and nothing else.
fn parse_long(text: &str) -> Option<i64> {
    text.parse::<i64>().ok()
}

/// A kv value as the script reads it, which is a string for everything the
/// parse produced.
fn painless_string(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), ToOwned::to_owned)
}

/// Every `stormshield` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "7fcd8ae0b46f1a0ca4efa3dbe45ced648c260e32a6eea40f42d3977956295d6a",
        source: "stormshield",
        name: "split_composite_pairs",
        run: split_composite_pairs,
    },
    Entry {
        hash: "57d8c00a8f8fc6eefec44c7e8583b6698434881ce735691fed9a13d45f50f5f7",
        source: "stormshield",
        name: "process_ethernet",
        run: process_ethernet,
    },
    Entry {
        hash: "66d4731050fbad6680e35840d458c4a057a7291a15ae255899f3f5e8110fa790",
        source: "stormshield",
        name: "process_devices",
        run: process_devices,
    },
    Entry {
        hash: "f113bc85d14c963b92872407b6f80279c89ce329e397074a28fc4a35a6f00050",
        source: "stormshield",
        name: "process_rule_stats",
        run: process_rule_stats,
    },
];
