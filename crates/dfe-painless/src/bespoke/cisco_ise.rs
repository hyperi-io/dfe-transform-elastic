// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cisco_ise`'s two log-detail scripts: the `cisco-av-pair` repeats parsed
//! into a map, and one `key=value` line folded into `log_details`.
//!
//! Only `mdm-tlv` carries sub-keys of its own. Every other pair keeps its value
//! whole, because a value may hold `=` characters -- `FQSubjectName` ships a
//! whole distinguished name -- and splitting on those would shred it.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The map the `kv` processor filled and both scripts write back into.
const LOG_DETAILS: &str = "cisco_ise.log.log_details";

/// The repeated attribute the first script parses.
const AV_PAIR: &str = "cisco_ise.log.log_details.cisco-av-pair";

/// The only top key whose value is itself a `key=value` pair.
const NESTED: &str = "mdm-tlv";

/// `script_parse_av_pairs` in `cisco_ise/log`: `cisco_ise.log.cisco_av_pair`,
/// one entry per `cisco-av-pair` the device repeated.
fn parse_av_pairs(event: &mut Event, _params: &Value) {
    let Some(held) = event.get(AV_PAIR).cloned() else {
        return;
    };
    if held.is_null() {
        return;
    }
    // A lone pair arrives as a string; the script lists it before walking.
    let pairs = match held {
        Value::String(_) => {
            let listed = Value::Array(vec![held]);
            let _ = event.set(AV_PAIR, listed.clone());
            listed
        }
        other => other,
    };
    let Value::Array(pairs) = pairs else {
        return;
    };

    let mut attributes = Map::new();
    for pair in &pairs {
        let Some(text) = pair.as_str() else {
            continue;
        };
        let Some(first_eq) = text.find('=').filter(|at| *at > 0) else {
            continue;
        };
        let top_key = text[..first_eq].trim();
        let rest = &text[first_eq + 1..];
        if top_key != NESTED {
            attributes.insert(top_key.to_owned(), Value::from(rest.trim()));
            continue;
        }
        fold_nested(&mut attributes, rest);
    }

    if !attributes.is_empty() {
        let _ = event.set("cisco_ise.log.cisco_av_pair", Value::Object(attributes));
    }
}

/// One `mdm-tlv` line folded into the shared `mdm-tlv` map.
///
/// The script descends a level at every `=` that has ANOTHER unescaped `=`
/// after it, so a two-part line writes one flat entry and a three-part line
/// nests.
fn fold_nested(attributes: &mut Map<String, Value>, rest: &str) {
    if !attributes.contains_key(NESTED) {
        attributes.insert(NESTED.to_owned(), Value::Object(Map::new()));
    }
    let chars: Vec<char> = rest.chars().collect();
    let count = chars.len();
    let mut in_escape = false;
    let mut start = 0usize;
    let mut key = String::new();
    let mut depth: Vec<String> = Vec::new();

    for at in 0..count {
        let current = chars[at];
        if in_escape {
            in_escape = false;
            continue;
        }
        if current == '\\' {
            in_escape = true;
            continue;
        }
        if current == '=' {
            chars[start..at]
                .iter()
                .collect::<String>()
                .trim()
                .clone_into(&mut key);
            let next_split = chars[at + 1..]
                .iter()
                .position(|held| *held == '=')
                .map(|found| found + at + 1);
            if let Some(split) = next_split
                && chars[split - 1] != '\\'
            {
                let Some(inner) = descend(attributes, &depth, &key) else {
                    return;
                };
                depth.push(inner);
            }
            start = at + 1;
        }
        if at == count - 1 {
            let value = chars[start..count]
                .iter()
                .collect::<String>()
                .trim()
                .to_owned();
            let Some(target) = walk(attributes, &depth) else {
                return;
            };
            target.insert(key.clone(), Value::from(value));
        }
    }
}

/// Open the child map a descent moves into, reporting the key it took.
fn descend(attributes: &mut Map<String, Value>, depth: &[String], key: &str) -> Option<String> {
    let target = walk(attributes, depth)?;
    if !target.contains_key(key) {
        target.insert(key.to_owned(), Value::Object(Map::new()));
    }
    // A key already holding something other than a map is where Painless
    // raises, so nothing more of this line is written.
    target.get(key)?.as_object()?;
    Some(key.to_owned())
}

/// The map at a depth under `mdm-tlv`.
fn walk<'a>(
    attributes: &'a mut Map<String, Value>,
    depth: &[String],
) -> Option<&'a mut Map<String, Value>> {
    let mut current = attributes.get_mut(NESTED)?.as_object_mut()?;
    for segment in depth {
        current = current.get_mut(segment)?.as_object_mut()?;
    }
    Some(current)
}

/// `cisco_ise_log_log_details_raw_split` in `cisco_ise/log`: the single
/// `key=value` line the purge-audit pipeline leaves in `log_details_raw`,
/// folded into `log_details`.
///
/// Only the FIRST `=` splits, so a value carrying more of them stays whole.
fn split_log_details_raw(event: &mut Event, _params: &Value) {
    let Some(raw) = event.get_string("cisco_ise.log.log_details_raw") else {
        return;
    };
    let mut details = event.get_object(LOG_DETAILS).cloned().unwrap_or_default();
    match raw.find('=') {
        None => {
            details.insert(raw, Value::Null);
        }
        Some(at) => {
            details.insert(raw[..at].to_owned(), Value::from(&raw[at + 1..]));
        }
    }
    let _ = event.set(LOG_DETAILS, Value::Object(details));
}

/// Every `cisco_ise` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "b1de5852c63f9fc6b4ee9ae2d791d454c6f72b2ec4dcca8c692794e3ed59cfc4",
        source: "cisco_ise",
        name: "parse_av_pairs",
        run: parse_av_pairs,
    },
    Entry {
        hash: "cd89d5720ba215bd958b9e894188bbfa0d7d8485742cc6963b0914cafbb975a9",
        source: "cisco_ise",
        name: "split_log_details_raw",
        run: split_log_details_raw,
    },
];
