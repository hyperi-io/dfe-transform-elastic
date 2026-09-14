// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! wiz's issue, defend and `defend_v2` scripts, transcribed.

use serde_json::{Map, Value, json};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// The vendor's rule objects, and the names they take under `wiz.issue`.
const RULE_FIELDS: [(&str, &str); 9] = [
    ("__typename", "__typename"),
    ("id", "id"),
    ("name", "name"),
    ("resolutionRecommendation", "resolution_recommendation"),
    ("remediationInstructions", "remediation_instructions"),
    ("securitySubCategories", "security_sub_categories"),
    ("type", "type"),
    ("serviceType", "service_type"),
    ("sourceType", "source_type"),
];

/// The nodes `defend_v2`'s OS mapping walks.
const TRIGGERING_NODES: &str = "wiz.defend_v2.triggering_events.nodes";

/// `map_source_rules_array` in `wiz/issue`: `wiz.issue.source_rules`, and the
/// event's `message` where nothing has set one.
///
/// The message is the FIRST rule description in the array, because the script
/// only fills it while it is still null.
fn map_source_rules(event: &mut Event, _params: &Value) {
    let (rules, description) = {
        let Some(source_rules) = event.get_array("json.sourceRules") else {
            return;
        };
        let mut rules = Vec::with_capacity(source_rules.len());
        let mut description: Option<Value> = None;
        for rule in source_rules {
            let mut mapped = Map::new();
            for (from, to) in RULE_FIELDS {
                if let Some(held) = present(rule, from) {
                    mapped.insert(to.to_string(), held.clone());
                }
            }
            if let Some(text) = present(rule, "description") {
                mapped.insert("description".to_string(), text.clone());
                if description.is_none() {
                    description = Some(text.clone());
                }
            }
            if let Some(Value::Array(risks)) = present(rule, "risks")
                && !risks.is_empty()
            {
                mapped.insert("risks".to_string(), Value::Array(risks.clone()));
            }
            rules.push(Value::Object(mapped));
        }
        (rules, description)
    };
    if let Some(text) = description
        && !event.has_value("message")
    {
        let _ = event.set("message", text);
    }
    let _ = event.set("wiz.issue.source_rules", Value::Array(rules));
}

/// One member of a rule object, absent where the vendor sent an explicit null.
fn present<'r>(rule: &'r Value, key: &str) -> Option<&'r Value> {
    rule.get(key).filter(|held| !held.is_null())
}

/// `script_set_host_os_type_from_triggering_events` in `wiz/defend_v2`:
/// `host.os.type` from every triggering audit record.
///
/// One distinct family stays a string and several become a list, which is what
/// the mapping accepts either way.
fn map_os_type(event: &mut Event, _params: &Value) {
    let families = {
        let Some(nodes) = event.get_array(TRIGGERING_NODES) else {
            return;
        };
        let mut families: Vec<&'static str> = Vec::new();
        for node in nodes {
            let host_info = node
                .get("raw_audit_log_record")
                .filter(|held| held.is_object())
                .and_then(|audit| audit.get("common"))
                .filter(|held| held.is_object())
                .and_then(|common| common.get("host_info"))
                .filter(|held| held.is_object());
            let Some(family) = host_info.and_then(|held| os_family(held.get("os_type"))) else {
                continue;
            };
            if !families.contains(&family) {
                families.push(family);
            }
        }
        families
    };
    let Some((first, rest)) = families.split_first() else {
        return;
    };
    let mapped = if rest.is_empty() {
        json!(first)
    } else {
        json!(families)
    };
    let _ = event.set("host.os.type", mapped);
}

/// The ladder the script's own `mapOsType` walks, in its order -- `mac` is
/// tested before `linux`, so a macOS build string never reads as unix.
fn os_family(os_type: Option<&Value>) -> Option<&'static str> {
    let os_type = os_type.filter(|held| !held.is_null())?;
    let os = painless_to_string(os_type).to_lowercase();
    if os.contains("windows") {
        Some("windows")
    } else if os.contains("mac") || os.contains("darwin") {
        Some("macos")
    } else if os.contains("linux")
        || os.contains("amazon")
        || os.contains("ubuntu")
        || os.contains("debian")
        || os.contains("centos")
        || os.contains("rhel")
        || os.contains("unix")
    {
        Some("linux")
    } else if os.contains("android") {
        Some("android")
    } else if os.contains("ios") {
        Some("ios")
    } else {
        None
    }
}

/// `set_event_outcome` in `wiz/defend`: `event.outcome` from the triggering
/// event's status.
fn event_outcome_from_status(event: &mut Event, _params: &Value) {
    let Some(status) = event
        .get_str("wiz.defend.triggering_event.status")
        .map(str::to_lowercase)
    else {
        return;
    };
    let outcome = if status.contains("success") {
        "success"
    } else if status.contains("fail") {
        "failure"
    } else {
        "unknown"
    };
    let _ = event.set("event.outcome", outcome);
}

/// `script_add_event_original` in `wiz/defend`: `event.original` as the vendor
/// payload's own JSON.
fn event_original_from_json(event: &mut Event, _params: &Value) {
    let Some(dumped) = event.get("json").map(Value::to_string) else {
        return;
    };
    let _ = event.set("event.original", dumped);
}

/// `set_event_severity` in `wiz/defend`: `event.severity` as the ECS score the
/// vendor's label maps to.
fn event_severity_from_severity(event: &mut Event, _params: &Value) {
    let Some(severity) = event.get_str("wiz.defend.severity") else {
        return;
    };
    let score =
        if severity.eq_ignore_ascii_case("low") || severity.eq_ignore_ascii_case("informational") {
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

/// Every wiz script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "9907c0ddf3c1b68f363b8153644f6cd96a23d027297889f19b335f80359f8e80",
        source: "wiz",
        name: "map_source_rules",
        run: map_source_rules,
    },
    Entry {
        hash: "52f979c992f23b6788e0c6a20f99bf33222bbb3b1535bc861b930cb0ff848430",
        source: "wiz",
        name: "map_os_type",
        run: map_os_type,
    },
    Entry {
        hash: "7ccfea48daf32b41d1599c73ce99c34e1a1b65773319c2a1f4df0b0c59214ea2",
        source: "wiz",
        name: "event_outcome_from_status",
        run: event_outcome_from_status,
    },
    Entry {
        hash: "5174210f24fb050e54015608bc8b9b59b0c4704d0a00758b364804c5d7820eb3",
        source: "wiz",
        name: "event_original_from_json",
        run: event_original_from_json,
    },
    Entry {
        hash: "6e9094eeafb41b34b0ed0fc61665542a102fcb92117922d3690ab05b6de3ed18",
        source: "wiz",
        name: "event_severity_from_severity",
        run: event_severity_from_severity,
    },
];
