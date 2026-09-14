// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `microsoft_defender_cloud`'s two data streams, transcribed.
//!
//! The alert stream arrives in whatever casing the Azure API felt like, so the
//! first script lower-cases every key in the document and every rename after it
//! spells its source in lower case. The assessment stream reads the worst CVE
//! out of a finding's list and answers the ECS `vulnerability.*` fields from it.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// The finding's own `additional_data`.
const ASSESSMENT_DATA: &str = "microsoft_defender_cloud.assessment.additional_data";

/// The sub-assessment's `additional_data`, which is where a CVE list sits.
const SUB_ASSESSMENT_DATA: &str =
    "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data";

/// `microsoft_defender_cloud/event`, the untagged lower-casing script: every
/// key in the document, recursively.
///
/// The walk is depth first, the way the vendor wrote it -- a nested map is
/// rewritten before the key holding it is.
fn lowercase_every_key(event: &mut Event, _params: &Value) {
    lower_value(event.as_value_mut());
}

/// One value's keys, and its children's.
fn lower_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for child in map.values_mut() {
                lower_value(child);
            }
            lower_keys(map);
        }
        Value::Array(items) => {
            for item in items {
                lower_value(item);
            }
        }
        _ => {}
    }
}

/// One map's own keys, lower-cased in place.
fn lower_keys(map: &mut Map<String, Value>) {
    let keys: Vec<String> = map.keys().cloned().collect();
    for key in keys {
        let lower = key.to_lowercase();
        if lower == key {
            continue;
        }
        if let Some(value) = map.shift_remove(&key) {
            map.insert(lower, value);
        }
    }
}

/// `microsoft_defender_cloud/event`, the untagged entities script: the alert's
/// entity list with every key the package knows given its snake-case name.
///
/// A `location` that is a plain value moves to `location_value`, because the
/// entity's own `location` is elsewhere an object and the two cannot share a
/// mapping.
fn rename_entity_keys(event: &mut Event, params: &Value) {
    let Some(Value::Array(entities)) = event.get("json.entities").cloned() else {
        return;
    };
    let renamed = entities
        .iter()
        .filter_map(Value::as_object)
        .map(|entity| Value::Object(rename_keys(entity, params, true)))
        .collect();
    let _ = event.set("entities_obj", Value::Array(renamed));
}

/// `microsoft_defender_cloud/event`, the untagged resource-identifier script:
/// the alert's resource identifiers under their snake-case names.
fn rename_resource_identifier_keys(event: &mut Event, params: &Value) {
    let Some(Value::Array(identifiers)) = event.get("json.resourceidentifiers").cloned() else {
        return;
    };
    let renamed = identifiers
        .iter()
        .filter_map(Value::as_object)
        .map(|identifier| Value::Object(rename_keys(identifier, params, false)))
        .collect();
    let _ = event.set("resource_identifier_obj", Value::Array(renamed));
}

/// One map with every key the table names replaced, recursively.
///
/// `lift_location` is the entity script's extra arm and is off for the
/// resource identifiers, which spell no `location`.
fn rename_keys(
    source: &Map<String, Value>,
    table: &Value,
    lift_location: bool,
) -> Map<String, Value> {
    let mut renamed = Map::new();
    for (key, value) in source {
        let name = mapped(table, key).unwrap_or_else(|| key.clone());
        match value {
            Value::Object(nested) => {
                renamed.insert(
                    name,
                    Value::Object(rename_keys(nested, table, lift_location)),
                );
            }
            Value::Array(items) => {
                let walked: Vec<Value> = items
                    .iter()
                    .map(|item| match item {
                        Value::Object(nested) => {
                            Value::Object(rename_keys(nested, table, lift_location))
                        }
                        other => other.clone(),
                    })
                    .collect();
                renamed.insert(name, Value::Array(walked));
            }
            scalar => {
                renamed.insert(name, scalar.clone());
                if lift_location && key == "location" {
                    renamed.insert("location_value".to_owned(), scalar.clone());
                    renamed.shift_remove("location");
                }
            }
        }
    }
    renamed
}

/// The name a key takes, absent where the table does not name it.
fn mapped(table: &Value, key: &str) -> Option<String> {
    table.get(key).and_then(Value::as_str).map(str::to_owned)
}

/// `microsoft_defender_cloud/assessment`,
/// `script_map_vulnerability_score_base_vulnerability_score_version_vulnerability_severity`:
/// the worst CVE in the sub-assessment's list, as `vulnerability.score.base`,
/// `vulnerability.severity` and `vulnerability.score.version`.
fn vulnerability_from_cve_list(event: &mut Event, _params: &Value) {
    let path = format!("{SUB_ASSESSMENT_DATA}.cve_list");
    let Some(Value::Array(cves)) = event.get(&path).cloned() else {
        return;
    };
    let mut max_score = 0.0f64;
    let mut severity = Value::Null;
    let mut version = Value::Null;
    for cve in &cves {
        let Some(score) = widened_score(cve, "cvss_score") else {
            continue;
        };
        if max_score < score {
            max_score = score;
            severity = member(cve, "severity");
            version = member(cve, "cvss_version");
        }
    }
    let _ = event.set("vulnerability.score.base", max_score);
    let _ = event.set("vulnerability.severity", severity);
    let _ = event.set("vulnerability.score.version", version);
}

/// `microsoft_defender_cloud/assessment`,
/// `script_map_vulnerability_score_base_vulnerability_severity`: the same two
/// fields for a finding whose CVEs sit on the assessment itself.
fn vulnerability_from_cves(event: &mut Event, _params: &Value) {
    let path = format!("{ASSESSMENT_DATA}.cves");
    let Some(Value::Array(cves)) = event.get(&path).cloned() else {
        return;
    };
    let mut max_score = 0.0f64;
    let mut severity = Value::Null;
    for cve in &cves {
        let Some(score) = widened_score(cve, "base_score") else {
            continue;
        };
        if max_score < score {
            max_score = score;
            severity = member(cve, "severity");
        }
    }
    let _ = event.set("vulnerability.score.base", max_score);
    let _ = event.set("vulnerability.severity", severity);
}

/// One CVE's score as the `double` the script compares and stores.
///
/// The pipeline converts the score to a `float` first, so the document holds
/// the shortest decimal that names a single. Assigning it to a `double` widens
/// the SINGLE, not that decimal -- which is why the stored maximum reads
/// `9.100000381469728` and not `9.1`.
fn widened_score(cve: &Value, member: &str) -> Option<f64> {
    let score = cve.get(member).filter(|value| !value.is_null())?.as_f64()?;
    #[allow(clippy::cast_possible_truncation)]
    let single = score as f32;
    Some(f64::from(single))
}

/// One member of a CVE, null where it carries none.
fn member(cve: &Value, name: &str) -> Value {
    cve.get(name).cloned().unwrap_or(Value::Null)
}

/// `microsoft_defender_cloud/assessment`,
/// `script_to_parse_vulnerability_details_cvss`: the CVSS block the vendor
/// keys by version number, given a field name per version.
fn split_cvss_by_version(event: &mut Event, _params: &Value) {
    let base = format!("{SUB_ASSESSMENT_DATA}.vulnerability_details");
    let Some(Value::Object(cvss)) = event.get(&format!("{base}.cvss")).cloned() else {
        return;
    };
    for (version, field) in [("2.0", "cvss_v2"), ("3.0", "cvss_v3"), ("4.0", "cvss_v4")] {
        if let Some(value) = cvss.get(version) {
            let _ = event.set(&format!("{base}.{field}"), value.clone());
        }
    }
}

/// `microsoft_defender_cloud/assessment`, `script_map_host_os_type`:
/// `host.os.type`, from whichever of the two places the finding reports the
/// operating system.
fn host_os_type(event: &mut Event, params: &Value) {
    let details = format!("{SUB_ASSESSMENT_DATA}.software_details.os_details.os_platform");
    let platform = event
        .get_str(&details)
        .or_else(|| event.get_str(&format!("{ASSESSMENT_DATA}.os_type")))
        .map(str::to_lowercase);
    let Some(platform) = platform else {
        return;
    };
    // `ctx.host.os.put` reaches through a map an earlier processor made.
    if !event.has("host.os") {
        return;
    }
    let Some(Value::Array(known)) = params.get("os_type") else {
        return;
    };
    for os in known {
        let Some(os) = os.as_str() else {
            continue;
        };
        if !platform.contains(os) {
            continue;
        }
        let _ = event.set("host.os.type", if os == "mac" { "macos" } else { os });
        return;
    }
}

/// Every `microsoft_defender_cloud` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "476f0bc89929d00b14ed03099f6460e621ea6e79f7aa83f7c5b7159979b2809d",
        source: "microsoft_defender_cloud",
        name: "lowercase_every_key",
        run: lowercase_every_key,
    },
    Entry {
        hash: "d7fa430067d0f2aa0271bcf4fd8842525f912727daeaa5fd940944ca62762217",
        source: "microsoft_defender_cloud",
        name: "rename_entity_keys",
        run: rename_entity_keys,
    },
    Entry {
        hash: "364a67a2f9ad956db68e5283402875bd9aa02b33bb7df42796f71104d12c6c5b",
        source: "microsoft_defender_cloud",
        name: "rename_resource_identifier_keys",
        run: rename_resource_identifier_keys,
    },
    Entry {
        hash: "61433c0df39d2c5a773e6eaf94faa3d80525cfd52cba8f28b9bd747f967c95f5",
        source: "microsoft_defender_cloud",
        name: "vulnerability_from_cve_list",
        run: vulnerability_from_cve_list,
    },
    Entry {
        hash: "9450f1fd50c0be068de552617aeec8b648a237294639a4476a79f5f7ebaf605d",
        source: "microsoft_defender_cloud",
        name: "vulnerability_from_cves",
        run: vulnerability_from_cves,
    },
    Entry {
        hash: "110155c37069bdf5b0d4696e120b36f11797add43b976344d517fcc50560e075",
        source: "microsoft_defender_cloud",
        name: "split_cvss_by_version",
        run: split_cvss_by_version,
    },
    Entry {
        hash: "e93ccff5c9be7eeb5a48b4faff90f65398e6f47abe9ef25ee8c9ce2141325aa2",
        source: "microsoft_defender_cloud",
        name: "host_os_type",
        run: host_os_type,
    },
];
