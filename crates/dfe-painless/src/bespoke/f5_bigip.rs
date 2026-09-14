// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `f5_bigip`'s two iHealth diagnostics scripts, the bot-and-dos request cut,
//! and the ASM directory-query names: each diagnostic's version numbers
//! rendered as strings, then every diagnostic's CVEs, severity, summary, rule
//! name and solution links gathered into the ECS fields with its own `cveIds`
//! key renamed to `cve_ids`, the raw HTTP request lifted out of the vendor's
//! line by index, and the LDAP names read off the query string.
//!
//! Every gather is a `HashSet`, so the lists come out in Java's bucket order
//! rather than the order the diagnostics spell them.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_set_order, painless_to_string};

/// The vendor's diagnostics, still under `json` when this runs.
const DIAGNOSTICS: &str = "json.diagnostics";

/// The version members the vendor sends as numbers and the mapping wants as
/// strings.
const VERSION_KEYS: [&str; 5] = ["minor", "major", "maintenance", "fix", "point"];

/// `script_to_convert_version_keys_to_string` in `f5_bigip/log`: the five
/// version members of every diagnostic, rendered as strings.
///
/// Nothing else in each version map is touched, and a diagnostic with no
/// version list is left alone.
fn stringify_version_keys(event: &mut Event, _params: &Value) {
    let Some(mut diagnostics) = event.take_array(DIAGNOSTICS) else {
        return;
    };
    for diagnostic in &mut diagnostics {
        let Some(versions) = diagnostic
            .as_object_mut()
            .and_then(|entry| entry.get_mut("version"))
            .and_then(Value::as_array_mut)
        else {
            continue;
        };
        for version in versions {
            let Some(members) = version.as_object_mut() else {
                continue;
            };
            for (key, held) in members.iter_mut() {
                if VERSION_KEYS.contains(&key.as_str()) {
                    *held = Value::from(painless_to_string(held));
                }
            }
        }
    }
    let _ = event.set(DIAGNOSTICS, Value::Array(diagnostics));
}

/// `script_to_append_diagnostics_objects_into_ecs_fields` in `f5_bigip/log`:
/// `vulnerability.*`, `rule.*` and `threat.enrichments.indicator.reference`.
///
/// `vulnerability.description` is written twice and the summaries win, so the
/// headers the script also gathers reach no field.
fn append_diagnostics_to_ecs(event: &mut Event, _params: &Value) {
    for container in [
        "vulnerability",
        "rule",
        "threat",
        "threat.enrichments",
        "threat.enrichments.indicator",
    ] {
        if !event.has_value(container) {
            let _ = event.set(container, Value::Object(Map::new()));
        }
    }

    let mut cve_ids = Gathered::default();
    let mut importance = Gathered::default();
    let mut summary = Gathered::default();
    let mut rule_name = Gathered::default();
    let mut rule_ref = Gathered::default();
    let mut threat_ref = Gathered::default();

    if let Some(mut diagnostics) = event.take_array(DIAGNOSTICS) {
        for diagnostic in &mut diagnostics {
            let Some(entry) = diagnostic.as_object_mut() else {
                continue;
            };
            let listed = entry.get("cveIds").cloned().unwrap_or(Value::Null);
            if let Some(members) = listed.as_array() {
                for member in members {
                    cve_ids.add(member);
                }
                rename_cve_ids(entry, &listed);
            }
            if let Some(solutions) = entry.get("solution").and_then(Value::as_array).cloned() {
                for solution in &solutions {
                    if let Some(id) = solution.get("id").filter(|held| !held.is_null()) {
                        rule_ref.add(id);
                    }
                    if let Some(value) = solution.get("value").filter(|held| !held.is_null()) {
                        threat_ref.add(value);
                    }
                }
                // The script repeats the rename here with the same variable, so
                // a diagnostic with no `cveIds` list gets a null one.
                rename_cve_ids(entry, &listed);
            }
            for (gathered, key) in [
                (&mut importance, "importance"),
                (&mut summary, "summary"),
                (&mut rule_name, "name"),
            ] {
                if let Some(held) = entry.get(key).filter(|held| !held.is_null()) {
                    gathered.add(held);
                }
            }
        }
        let _ = event.set(DIAGNOSTICS, Value::Array(diagnostics));
    }

    let _ = event.set("vulnerability.id", cve_ids.into_value());
    let _ = event.set("vulnerability.severity", importance.into_value());
    let _ = event.set("vulnerability.description", summary.into_value());
    let _ = event.set("rule.name", rule_name.into_value());
    let _ = event.set("rule.reference", rule_ref.into_value());
    let _ = event.set(
        "threat.enrichments.indicator.reference",
        threat_ref.into_value(),
    );
}

/// Swap one diagnostic's `cveIds` key for `cve_ids`, keeping whatever the
/// original held.
fn rename_cve_ids(entry: &mut Map<String, Value>, listed: &Value) {
    entry.shift_remove("cveIds");
    entry.insert("cve_ids".to_owned(), listed.clone());
}

/// One of the script's `HashSet`s: distinct members, in the order they arrived.
#[derive(Default)]
struct Gathered {
    members: Vec<Value>,
}

impl Gathered {
    /// Add a member the set does not already hold.
    fn add(&mut self, member: &Value) {
        if !self.members.contains(member) {
            self.members.push(member.clone());
        }
    }

    /// The set as the field receives it, in Java's own iteration order.
    fn into_value(self) -> Value {
        Value::Array(java_set_order(self.members))
    }
}

/// The marker the request cut looks for, quote included.
const REQUEST_MARKER: &str = "http_request=\"";

/// The LDAP names the ASM query script reads, and where each lands.
const QUERY_NAMES: [(&str, &str); 3] = [
    ("sAMAccountName=", "f5_bigip.log.sam_account_name"),
    ("UserPrincipleName=", "f5_bigip.log.user_principle_name"),
    ("User_Name=", "f5_bigip.log.user_name"),
];

/// `(no tag)` in `f5_bigip/log`: the three LDAP names the ASM directory query
/// carries, each read off the raw line.
///
/// The script's `[^\)]+` stops at the first backslash or closing bracket, which
/// is what ends a term of the vendor's filter.
fn read_query_names(event: &mut Event, _params: &Value) {
    let Some(log) = event
        .get_string("event.original")
        .or_else(|| event.get_string("json.originalRawData"))
    else {
        return;
    };
    if !event.has("f5_bigip.log") {
        return;
    }
    for (marker, target) in QUERY_NAMES {
        if let Some(name) = term_after(&log, marker) {
            let _ = event.set(target, Value::from(name));
        }
    }
}

/// The filter term that follows a marker, or nothing where no occurrence of the
/// marker is followed by one.
fn term_after<'a>(log: &'a str, marker: &str) -> Option<&'a str> {
    let mut from = 0usize;
    while let Some(at) = log[from..].find(marker) {
        let start = from + at + marker.len();
        let term = log[start..].split(['\\', ')']).next().unwrap_or_default();
        if !term.is_empty() {
            return Some(term);
        }
        from = start;
    }
    None
}

/// `(no tag)` in `f5_bigip/log`: `kv.http_request`, cut out of the vendor's own
/// line between the marker and the next quote.
///
/// The script adds the marker's length to `indexOf` WITHOUT checking for -1
/// first, so a line carrying no marker starts the cut at 13 -- and
/// Elasticsearch writes that, which is why the cut is reproduced rather than
/// corrected.
fn cut_http_request(event: &mut Event, _params: &Value) {
    if !event.has("kv") {
        return;
    }
    let Some(message) = event.get_string("event.original") else {
        return;
    };
    let start = if let Some(at) = message.find(REQUEST_MARKER) {
        at + REQUEST_MARKER.len()
    } else {
        // `indexOf` answered -1 and the script added the marker's length anyway.
        let Some(at) = utf16_offset(&message, REQUEST_MARKER.chars().count() - 1) else {
            return;
        };
        at
    };
    let Some(rest) = message.get(start..) else {
        return;
    };
    let Some(end) = rest.find('"') else {
        return;
    };
    let _ = event.set("kv.http_request", Value::from(&rest[..end]));
}

/// The byte offset of a UTF-16 index, or nothing where the text is shorter.
fn utf16_offset(text: &str, units: usize) -> Option<usize> {
    let mut seen = 0usize;
    for (at, held) in text.char_indices() {
        if seen == units {
            return Some(at);
        }
        seen += held.len_utf16();
    }
    (seen == units).then_some(text.len())
}

/// Every `f5_bigip` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "5cba10b16fc0476a923fd96b661a0de1119cf75cee861f937583481a1214cc57",
        source: "f5_bigip",
        name: "read_query_names",
        run: read_query_names,
    },
    Entry {
        hash: "f8868a7496d5ae6459687fd029db4584f0b9ca68646270601ee3bad5e17096a7",
        source: "f5_bigip",
        name: "cut_http_request",
        run: cut_http_request,
    },
    Entry {
        hash: "46278489e80d12311abb9786376c1086fcb03f2aa990304e73741deb037fff9d",
        source: "f5_bigip",
        name: "stringify_version_keys",
        run: stringify_version_keys,
    },
    Entry {
        hash: "146defcaf3c298630cd40d7f1530c2c37a2e85c048c89ecf7cebee38e2a320c2",
        source: "f5_bigip",
        name: "append_diagnostics_to_ecs",
        run: append_diagnostics_to_ecs,
    },
];
