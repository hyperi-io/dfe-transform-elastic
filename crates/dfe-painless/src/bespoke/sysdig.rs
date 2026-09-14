// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The three `sysdig/event` scripts, transcribed.

use serde_json::{Value, json};

use dfe_core::Event;

use super::Entry;

/// The rule tags the MITRE mapping walks.
const RULE_TAGS: &str = "sysdig.event.content.rule_tags";

/// `script_set_threat_*` in `sysdig/event`: the ATT&CK tactic, technique and
/// subtechnique names the vendor spells into its rule tags.
///
/// The empty lists are written whatever the tags hold, which is what the
/// module's closing prune then takes away again for a rule with no MITRE tag.
fn threat_from_rule_tags(event: &mut Event, _params: &Value) {
    ensure_container(event, "threat", &json!({}));
    ensure_container(event, "threat.tactic", &json!({}));
    ensure_container(event, "threat.technique", &json!({}));
    ensure_container(event, "threat.technique.subtechnique", &json!({}));
    ensure_container(event, "threat.tactic.id", &json!([]));
    ensure_container(event, "threat.technique.id", &json!([]));
    ensure_container(event, "threat.technique.subtechnique.id", &json!([]));
    ensure_container(event, "threat.tactic.name", &json!([]));
    ensure_container(event, "threat.technique.name", &json!([]));
    ensure_container(event, "threat.technique.subtechnique.name", &json!([]));

    let found = {
        // The tactic branch is tried first, the way the script's own
        // alternation orders it.
        let Some(pattern) =
            dfe_core::cached_regex!(r"MITRE_(T[A-Z]\d{4})_(\w+)|MITRE_(T\d{4}(?:\.\d{3})?)_(\w+)")
                .fast()
        else {
            return;
        };
        let Some(tags) = event.get_array(RULE_TAGS) else {
            return;
        };
        let mut found: Vec<(&'static str, &'static str, String, String)> = Vec::new();
        for tag in tags {
            let Some(tag) = tag.as_str() else {
                continue;
            };
            let Some(caught) = pattern.captures(tag) else {
                continue;
            };
            if let Some((id, name)) = caught.get(1).zip(caught.get(2)) {
                found.push((
                    "threat.tactic.id",
                    "threat.tactic.name",
                    id.as_str().to_owned(),
                    name.as_str().to_owned(),
                ));
                continue;
            }
            let Some((id, name)) = caught.get(3).zip(caught.get(4)) else {
                continue;
            };
            let (id, name) = (id.as_str().to_owned(), name.as_str().to_owned());
            if id.contains('.') {
                found.push((
                    "threat.technique.subtechnique.id",
                    "threat.technique.subtechnique.name",
                    id,
                    name,
                ));
            } else {
                found.push(("threat.technique.id", "threat.technique.name", id, name));
            }
        }
        found
    };

    for (id_path, name_path, id, name) in found {
        let _ = event.append(id_path, id);
        let _ = event.append(name_path, name);
    }
}

/// `script_set_severity_value` in `sysdig/event`: the vendor's own word for the
/// numeric severity.
///
/// The ladder leaves 8 and above unnamed, and the script writes nothing there.
fn severity_value_from_severity(event: &mut Event, _params: &Value) {
    let Some(severity) = event.get_as_i64("event.severity") else {
        return;
    };
    let named = match severity {
        0..=3 => "High",
        4..=5 => "Medium",
        6 => "Low",
        7 => "Info",
        _ => return,
    };
    // The script writes through `ctx.sysdig.event`, which raises where the
    // vendor block is absent rather than building one.
    if !event.has("sysdig.event") {
        return;
    }
    let _ = event.set("sysdig.event.severity_value", named);
}

/// `script_append_container_image_hash_to_related_hash` in `sysdig/event`: the
/// container image digest, with its algorithm prefix cut off.
fn related_hash_from_image_digest(event: &mut Event, _params: &Value) {
    ensure_container(event, "related", &json!({}));
    ensure_container(event, "related.hash", &json!([]));
    let Some(digest) = event.get_string("sysdig.event.labels.container.image.digest") else {
        return;
    };
    let hash = match digest.find(':') {
        Some(at) => digest[at + 1..].to_string(),
        None => digest,
    };
    let _ = event.append("related.hash", hash);
}

/// One `ctx.x = ctx.x ?: <empty>`, which keeps whatever is already there.
fn ensure_container(event: &mut Event, path: &str, empty: &Value) {
    if event.has_value(path) {
        return;
    }
    let _ = event.set(path, empty.clone());
}

/// Every `sysdig` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "1351179af7e0115ddf3cc96bcd16c058234ca72dd649898c9459c2a057ede0ed",
        source: "sysdig",
        name: "threat_from_rule_tags",
        run: threat_from_rule_tags,
    },
    Entry {
        hash: "9b284f325fd5faea5f7cb55803eeca90917ee9d1abae987e6ab9c2fada863b8e",
        source: "sysdig",
        name: "severity_value_from_severity",
        run: severity_value_from_severity,
    },
    Entry {
        hash: "6b81fdc103933f8f1b89b0e60ab29eefa3869225f6bcf1f9a1eb86af7fe506ea",
        source: "sysdig",
        name: "related_hash_from_image_digest",
        run: related_hash_from_image_digest,
    },
];
