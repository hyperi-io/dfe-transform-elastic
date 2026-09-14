// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `tenable_sc`'s asset, plugin and vulnerability scripts, transcribed.
//!
//! Two of these matter more than they look: the vendor names the fields its
//! own record is keyed by in a `uniqueness` string, so the key is assembled by
//! reading whatever fields that string lists, and the fingerprint downstream
//! hashes the result. A record with no key gets a different fingerprint, which
//! is why one missing script moves two fields.

use std::collections::HashSet;

use chrono::DateTime;
use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_string_set_order, painless_to_string};

/// `tenable_sc/vulnerability`: the record key the vendor's `uniqueness` names,
/// written to `tenable_sc.vulnerability.id`.
fn vulnerability_id(event: &mut Event, _params: &Value) {
    let Some(key) = unique_key(event) else {
        return;
    };
    // Painless raises on the assignment where the parent map is absent.
    if !event.has("tenable_sc.vulnerability") {
        return;
    }
    let _ = event.set("tenable_sc.vulnerability.id", Value::from(key));
}

/// `tenable_sc/asset`: the same key, written to `tenable_sc.asset.custom_hash`
/// after the script has REPLACED `ctx.tenable_sc` with a fresh map.
fn asset_custom_hash(event: &mut Event, _params: &Value) {
    let Some(key) = unique_key(event) else {
        return;
    };
    let _ = event.set("tenable_sc", Value::Object(Map::new()));
    let _ = event.set("tenable_sc.asset", Value::Object(Map::new()));
    let _ = event.set("tenable_sc.asset.custom_hash", Value::from(key));
}

/// The key itself: each field the `uniqueness` string names, joined by
/// underscores, with `repositoryID` standing for `json.repository.id`.
///
/// A named field the record does not carry contributes Java's own `null`,
/// which is part of the key rather than a reason to skip it.
fn unique_key(event: &Event) -> Option<String> {
    let uniqueness = event.get("json.uniqueness").map(painless_to_string)?;
    let keys: Vec<&str> = uniqueness.split(',').collect();
    let mut key = String::new();
    for (index, name) in keys.iter().enumerate() {
        let path = if *name == "repositoryID" {
            "json.repository.id".to_owned()
        } else {
            format!("json.{name}")
        };
        match event.get(&path) {
            Some(value) => key.push_str(&painless_to_string(value)),
            None => key.push_str("null"),
        }
        if index != keys.len() - 1 {
            key.push('_');
        }
    }
    Some(key)
}

/// `tenable_sc/vulnerability`: the vendor's VPR context list folded into a map
/// keyed by each part's `id`, with the list kept beside it as `_original`.
fn vulnerability_vpr_context(event: &mut Event, _params: &Value) {
    vpr_context(event, "tenable_sc.vulnerability.vpr");
}

/// `tenable_sc/plugin`: the same fold, under the plugin's own subtree.
fn plugin_vpr_context(event: &mut Event, _params: &Value) {
    vpr_context(event, "tenable_sc.plugin.vpr");
}

/// The fold itself.
fn vpr_context(event: &mut Event, subtree: &str) {
    let Some(parts) = event.get_array("json.vprContext").cloned() else {
        return;
    };
    if parts.is_empty() {
        return;
    }
    // Painless raises on the assignment where the parent map is absent.
    if !event.has(subtree) {
        return;
    }

    let mut context = Map::with_capacity(parts.len() + 1);
    for part in &parts {
        let Some(id) = part.get("id").and_then(Value::as_str) else {
            continue;
        };
        let value = part.get("value").cloned().unwrap_or(Value::Null);
        context.insert(id.to_owned(), value);
    }
    context.insert("_original".to_owned(), Value::Array(parts));
    let _ = event.set(&format!("{subtree}.context"), Value::Object(context));
}

/// `tenable_sc/vulnerability`: the whole days between the record's first and
/// last sighting.
///
/// `ChronoUnit.DAYS.between` counts COMPLETE days, so a last sighting earlier
/// in the day than the first takes one off the calendar difference.
fn vulnerability_age(event: &mut Event, _params: &Value) {
    let (Some(first), Some(last)) = (
        event.get_str("tenable_sc.vulnerability.first_seen"),
        event.get_str("tenable_sc.vulnerability.last_seen"),
    ) else {
        return;
    };
    let (Ok(first), Ok(last)) = (
        DateTime::parse_from_rfc3339(first),
        DateTime::parse_from_rfc3339(last),
    ) else {
        return;
    };
    let days = (last - first).num_days();
    if !event.has("tenable_sc.vulnerability") {
        return;
    }
    let _ = event.set("tenable_sc.vulnerability.age", days);
}

/// `tenable_sc/vulnerability`: the reference links, the CVE lookup URLs the
/// record's own CVE ids build plus whatever `seeAlso` carries.
///
/// The script collects into a `HashSet`, so the list comes out in Java bucket
/// order rather than the order the links were read in.
fn vulnerability_reference(event: &mut Event, _params: &Value) {
    let mut links: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    if let Some(cve) = event.get("json.cve").map(painless_to_string)
        && !cve.is_empty()
    {
        for id in cve.split(',') {
            let link = format!("https://cve.mitre.org/cgi-bin/cvename.cgi?name={id}");
            if seen.insert(link.clone()) {
                links.push(link);
            }
        }
    }
    if let Some(see_also) = event.get_array("json.seeAlso") {
        for link in see_also {
            let link = painless_to_string(link);
            if seen.insert(link.clone()) {
                links.push(link);
            }
        }
    }

    // Painless raises on the assignment where the parent map is absent.
    if !event.has("vulnerability") {
        return;
    }
    let _ = event.set(
        "vulnerability.reference",
        Value::Array(java_string_set_order(links)),
    );
}

/// Every `tenable_sc` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "6b96f6739e95ba65ba9ddc8164987343fed09cc9933a23512f6404cebcabe74e",
        source: "tenable_sc",
        name: "vulnerability_id",
        run: vulnerability_id,
    },
    Entry {
        hash: "8917c851e5d0dbbd17f86b28ea3955453ff222646cc43fb53e0195923412fd2b",
        source: "tenable_sc",
        name: "asset_custom_hash",
        run: asset_custom_hash,
    },
    Entry {
        hash: "b562be4c92e80fbc38bd62c7d186fbdb15005475b40dad22c5c74449e42b8268",
        source: "tenable_sc",
        name: "vulnerability_vpr_context",
        run: vulnerability_vpr_context,
    },
    Entry {
        hash: "3dd2a3ec1187688c2694aab822d0fbfd54c9934202db28d000876a74022b87a0",
        source: "tenable_sc",
        name: "plugin_vpr_context",
        run: plugin_vpr_context,
    },
    Entry {
        hash: "77c093d2fb4c4ada6b3d4e02b7899bb72475f1faf429f8e319a7c404e49fa5c8",
        source: "tenable_sc",
        name: "vulnerability_age",
        run: vulnerability_age,
    },
    Entry {
        hash: "d5a5a092d9d3d4dd4a026da5d1b2878d27db9b92bc596bd8ae65592898101ce5",
        source: "tenable_sc",
        name: "vulnerability_reference",
        run: vulnerability_reference,
    },
];
