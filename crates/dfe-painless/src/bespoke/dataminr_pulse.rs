// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `dataminr_pulse`'s alert scripts, transcribed.
//!
//! The vendor puts everything an alert found into one `intelAgents` list of
//! discovered entities, and one script sorts that into threat actors,
//! vulnerabilities, malware, URLs and addresses at once. The categorisation
//! script downstream then reads what it wrote, so the two have to agree about
//! what an alert turned up.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Where the vendor's own alert payload sits.
const INTEL_AGENTS: &str = "json.intelAgents";

/// The operating-system spellings each ECS platform name answers to.
///
/// The script carries this as a map literal it walks with a `break` on the
/// first hit, and the spellings are disjoint, so a table read in order is the
/// same answer.
const OS_MAPPING: &[(&str, &[&str])] = &[
    (
        "Windows",
        &["win", "windows", "win32", "win64", "microsoft windows"],
    ),
    ("Linux", &["elf", "linux", "unix"]),
    (
        "macOS",
        &[
            "mac", "macos", "osx", "os x", "darwin", "mac os", "mac os x",
        ],
    ),
    ("AWS", &["aws", "amazon web services"]),
    ("Azure", &["azure", "microsoft azure"]),
    (
        "Azure AD",
        &["azure ad", "azure active directory", "azuread"],
    ),
    ("GCP", &["gcp", "google cloud", "google cloud platform"]),
    ("Network", &["network"]),
    (
        "Office 365",
        &["office 365", "o365", "office365", "microsoft 365", "m365"],
    ),
    ("SaaS", &["saas", "software as a service"]),
];

/// `dataminr_pulse/alerts`, `extract_entities_and_map_ecs`: every discovered
/// entity sorted onto the vendor's own namespace and onto ECS.
#[allow(clippy::too_many_lines)] // One script, transcribed whole.
fn extract_entities(event: &mut Event, _params: &Value) {
    let Some(agents) = event.get_array(INTEL_AGENTS).cloned() else {
        return;
    };

    let mut actor_names: Vec<Value> = Vec::new();
    let mut actor_aliases: Vec<Value> = Vec::new();
    let mut actor_countries: Vec<Value> = Vec::new();
    let mut vulnerability_names: Vec<Value> = Vec::new();
    let mut malware: Vec<Value> = Vec::new();
    let mut platforms: Vec<Value> = Vec::new();
    let mut urls: Vec<Value> = Vec::new();
    let mut ips: Vec<Value> = Vec::new();
    let mut enrichments: Vec<Value> = Vec::new();
    let mut first_cvss: Option<Value> = None;
    let mut first_description: Option<Value> = None;

    for agent in &agents {
        let Some(entities) = agent.get("discoveredEntities").and_then(Value::as_array) else {
            continue;
        };
        for entity in entities {
            let kind = entity
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or_default();
            match kind {
                "threatActor" if non_empty(entity, "name").is_some() => {
                    let name = non_empty(entity, "name").unwrap_or(&Value::Null).clone();
                    push_unique(&mut actor_names, name);
                    if let Some(aliases) = entity.get("aliases").and_then(Value::as_array) {
                        for alias in aliases {
                            push_unique(&mut actor_aliases, alias.clone());
                        }
                    }
                    match entity.get("countryOfOrigin") {
                        None | Some(Value::Null) => {}
                        Some(Value::Array(countries)) => {
                            for country in countries {
                                if country.is_string() {
                                    push_unique(&mut actor_countries, country.clone());
                                }
                            }
                        }
                        Some(country) if country.is_string() => {
                            push_unique(&mut actor_countries, country.clone());
                        }
                        Some(_) => {}
                    }
                }
                "vulnerability" => {
                    let named = non_empty(entity, "cve")
                        .or_else(|| non_empty(entity, "name"))
                        .or_else(|| non_empty(entity, "id"));
                    if let Some(named) = named {
                        push_unique(&mut vulnerability_names, named.clone());
                    }
                    if first_cvss.is_none()
                        && let Some(cvss) = entity.get("cvss").filter(|value| !value.is_null())
                    {
                        first_cvss = Some(cvss.clone());
                    }
                    if first_description.is_none()
                        && let Some(summary) = non_empty(entity, "summary")
                    {
                        first_description = Some(summary.clone());
                    }
                }
                "malware" if non_empty(entity, "name").is_some() => {
                    let name = non_empty(entity, "name").unwrap_or(&Value::Null).clone();
                    push_unique(&mut malware, name);
                    if let Some(systems) = entity
                        .get("affectedOperatingSystems")
                        .and_then(Value::as_array)
                    {
                        for system in systems {
                            let Some(text) = system.as_str() else {
                                continue;
                            };
                            let lowered = text.to_lowercase();
                            // A system nothing matches contributes a NULL,
                            // which the set keeps and the shipped list shows.
                            let normalised = OS_MAPPING
                                .iter()
                                .find(|(_, spellings)| {
                                    spellings.iter().any(|spelling| *spelling == lowered)
                                })
                                .map_or(Value::Null, |(name, _)| Value::from(*name));
                            push_unique(&mut platforms, normalised);
                        }
                    }
                }
                "url" if non_empty(entity, "name").is_some() => {
                    let name = non_empty(entity, "name").unwrap_or(&Value::Null).clone();
                    let refanged = refang(&name);
                    push_unique(&mut urls, Value::from(refanged.clone()));
                    let mut indicator = Map::new();
                    indicator.insert("type".to_owned(), Value::from("url"));
                    indicator.insert("name".to_owned(), name);
                    let mut url = Map::new();
                    url.insert("original".to_owned(), Value::from(refanged));
                    indicator.insert("url".to_owned(), Value::Object(url));
                    enrichments.push(wrap_indicator(indicator));
                }
                "ipAddress" if non_empty(entity, "ip").is_some() => {
                    let address = non_empty(entity, "ip").unwrap_or(&Value::Null).clone();
                    let refanged = refang(&address);
                    push_unique(&mut ips, Value::from(refanged.clone()));
                    let mut indicator = Map::new();
                    indicator.insert("name".to_owned(), address);
                    indicator.insert("ip".to_owned(), Value::from(refanged.clone()));
                    let family = if refanged.contains(':') {
                        "ipv6-addr"
                    } else {
                        "ipv4-addr"
                    };
                    indicator.insert("type".to_owned(), Value::from(family));
                    if let Some(ports) = entity.get("ports").and_then(Value::as_array)
                        && !ports.is_empty()
                    {
                        indicator.insert("port".to_owned(), Value::Array(ports.clone()));
                    }
                    enrichments.push(wrap_indicator(indicator));
                }
                _ => {}
            }
        }
    }

    if !event.has("dataminr_pulse") {
        let _ = event.set("dataminr_pulse", Value::Object(Map::new()));
    }

    if !actor_names.is_empty() {
        let _ = event.set("dataminr_pulse.threatactor", Value::Object(Map::new()));
        let _ = event.set(
            "dataminr_pulse.threatactor.name",
            Value::Array(actor_names.clone()),
        );
        if !actor_aliases.is_empty() {
            let _ = event.set(
                "dataminr_pulse.threatactor.alias",
                Value::Array(actor_aliases.clone()),
            );
        }
        if !actor_countries.is_empty() {
            let _ = event.set(
                "dataminr_pulse.threatactor.country_of_origin",
                Value::Array(actor_countries.clone()),
            );
        }
        if !event.has("threat") {
            let _ = event.set("threat", Value::Object(Map::new()));
        }
        let _ = event.set("threat.group", Value::Object(Map::new()));
        let _ = event.set("threat.group.name", actor_names[0].clone());
        if !actor_aliases.is_empty() {
            let _ = event.set("threat.group.alias", Value::Array(actor_aliases));
        }
        if !actor_countries.is_empty() {
            let _ = event.set("threat.indicator", Value::Object(Map::new()));
            let _ = event.set("threat.indicator.geo", Value::Object(Map::new()));
            let _ = event.set(
                "threat.indicator.geo.country_iso_code",
                Value::Array(actor_countries),
            );
        }
        let _ = event.set("threat.framework", Value::from("MITRE ATT&CK"));
    }

    if !vulnerability_names.is_empty() {
        let _ = event.set("dataminr_pulse.vulnerability", Value::Object(Map::new()));
        let _ = event.set(
            "dataminr_pulse.vulnerability.name",
            Value::Array(vulnerability_names.clone()),
        );
        let _ = event.set("vulnerability", Value::Object(Map::new()));
        let _ = event.set("vulnerability.id", vulnerability_names[0].clone());
        if let Some(cvss) = first_cvss {
            let _ = event.set("vulnerability.score", Value::Object(Map::new()));
            let _ = event.set("vulnerability.score.base", cvss);
        }
        if let Some(description) = first_description {
            let _ = event.set("vulnerability.description", description);
        }
    }

    if !malware.is_empty() {
        let _ = event.set("dataminr_pulse.malware", Value::Array(malware.clone()));
        if !event.has("threat") {
            let _ = event.set("threat", Value::Object(Map::new()));
        }
        let _ = event.set("threat.software", Value::Object(Map::new()));
        let _ = event.set("threat.software.name", malware[0].clone());
        let _ = event.set("threat.software.type", Value::from("Malware"));
        if !platforms.is_empty() {
            let _ = event.set("dataminr_pulse.platforms", Value::Array(platforms.clone()));
            let _ = event.set("threat.software.platforms", Value::Array(platforms));
        }
    }

    if !urls.is_empty() {
        let _ = event.set("dataminr_pulse.url", Value::Array(urls));
    }

    if !ips.is_empty() {
        let _ = event.set("dataminr_pulse.ip", Value::Array(ips.clone()));
        if !event.has("related") {
            let _ = event.set("related", Value::Object(Map::new()));
        }
        let _ = event.set("related.ip", Value::Array(ips));
    }

    if !enrichments.is_empty() {
        if !event.has("threat") {
            let _ = event.set("threat", Value::Object(Map::new()));
        }
        let _ = event.set("threat.enrichments", Value::Array(enrichments));
    }
}

/// One enrichment, which is an indicator under its own key.
fn wrap_indicator(indicator: Map<String, Value>) -> Value {
    let mut wrapper = Map::new();
    wrapper.insert("indicator".to_owned(), Value::Object(indicator));
    Value::Object(wrapper)
}

/// The member at `key`, unless it is absent, null or the empty string.
fn non_empty<'a>(entity: &'a Value, key: &str) -> Option<&'a Value> {
    entity
        .get(key)
        .filter(|value| !value.is_null() && value.as_str() != Some(""))
}

/// The vendor defangs an indicator by bracketing its dots; this puts them back.
fn refang(value: &Value) -> String {
    value.as_str().unwrap_or_default().replace("[.]", ".")
}

/// Append unless the list already holds it.
fn push_unique(list: &mut Vec<Value>, value: Value) {
    if !list.contains(&value) {
        list.push(value);
    }
}

/// `dataminr_pulse/alerts`, `convert_intelAgents_summary`: the first intel
/// agent's summary as one string, whether the vendor sent a string or a list
/// of titled sections.
fn intel_agents_summary(event: &mut Event, _params: &Value) {
    let Some(summary) = event
        .get_array(INTEL_AGENTS)
        .and_then(|agents| agents.first())
        .and_then(|agent| agent.get("summary"))
        .cloned()
    else {
        return;
    };

    let joined = match &summary {
        Value::String(text) => text.clone(),
        Value::Array(items) => {
            let mut parts: Vec<String> = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Value::Object(section) => {
                        let title = section
                            .get("title")
                            .and_then(Value::as_str)
                            .unwrap_or_default();
                        let content = match section.get("content") {
                            Some(Value::Array(lines)) => lines
                                .iter()
                                .map(|line| line.as_str().unwrap_or_default())
                                .collect::<Vec<&str>>()
                                .join(" "),
                            _ => String::new(),
                        };
                        if !title.is_empty() && !content.is_empty() {
                            parts.push(format!("{title}: {content}"));
                        } else if !content.is_empty() {
                            parts.push(content);
                        } else if !title.is_empty() {
                            parts.push(title.to_owned());
                        }
                    }
                    Value::String(text) => parts.push(text.clone()),
                    _ => {}
                }
            }
            if parts.is_empty() {
                return;
            }
            parts.join(" | ")
        }
        _ => return,
    };

    if !event.has("dataminr_pulse") {
        let _ = event.set("dataminr_pulse", Value::Object(Map::new()));
    }
    if !event.has("dataminr_pulse.intel_agents") {
        let _ = event.set("dataminr_pulse.intel_agents", Value::Object(Map::new()));
    }
    let _ = event.set("dataminr_pulse.intel_agents.summary", Value::from(joined));
}

/// `dataminr_pulse/alerts`, `set_event_category_based_on_entities`: the ECS
/// categories and types the entities the alert turned up imply.
///
/// `web` and `info` are the fallback where the alert turned NOTHING up, which
/// is what makes this script depend on the entity extraction above having run.
fn event_category(event: &mut Event, _params: &Value) {
    if !event.has("event") {
        let _ = event.set("event", Value::Object(Map::new()));
    }
    let mut category: Vec<Value> = event
        .get_array("event.category")
        .cloned()
        .unwrap_or_default();
    let mut kinds: Vec<Value> = event.get_array("event.type").cloned().unwrap_or_default();

    let mut found = false;
    if populated(event, "dataminr_pulse.vulnerability.name") {
        push_unique(&mut category, Value::from("vulnerability"));
        push_unique(&mut kinds, Value::from("info"));
        found = true;
    }
    if populated(event, "dataminr_pulse.threatactor.name") {
        push_unique(&mut category, Value::from("threat"));
        push_unique(&mut kinds, Value::from("indicator"));
        found = true;
    }
    if populated(event, "dataminr_pulse.malware") {
        push_unique(&mut category, Value::from("malware"));
        push_unique(&mut kinds, Value::from("info"));
        found = true;
    }
    if populated(event, "dataminr_pulse.ip") || populated(event, "dataminr_pulse.url") {
        push_unique(&mut category, Value::from("threat"));
        push_unique(&mut kinds, Value::from("indicator"));
        found = true;
    }
    if !found {
        // The fallback APPENDS without checking, which is the one place the
        // script does not dedup.
        category.push(Value::from("web"));
        push_unique(&mut kinds, Value::from("info"));
    }

    let _ = event.set("event.category", Value::Array(category));
    let _ = event.set("event.type", Value::Array(kinds));
}

/// Whether a list the extraction may have written holds anything.
fn populated(event: &Event, path: &str) -> bool {
    event.get_array(path).is_some_and(|list| !list.is_empty())
}

/// `dataminr_pulse/alerts`, `script_map_hash_values`: the vendor's typed hash
/// list onto `file.hash.*`, with every value gathered into `related.hash`.
fn map_hash_values(event: &mut Event, _params: &Value) {
    let Some(hashes) = event.get_array("json.metadata.cyber.hashValues").cloned() else {
        return;
    };
    if hashes.is_empty() {
        return;
    }

    if !event.has("file") {
        let _ = event.set("file", Value::Object(Map::new()));
    }
    if !event.has("file.hash") {
        let _ = event.set("file.hash", Value::Object(Map::new()));
    }
    if !event.has("related") {
        let _ = event.set("related", Value::Object(Map::new()));
    }
    let mut related: Vec<Value> = event.get_array("related.hash").cloned().unwrap_or_default();

    for hash in &hashes {
        let (Some(kind), Some(value)) = (
            hash.get("type").filter(|value| !value.is_null()),
            hash.get("value").filter(|value| !value.is_null()),
        ) else {
            continue;
        };
        let kind = kind.as_str().unwrap_or_default().to_lowercase();
        if !matches!(kind.as_str(), "md5" | "sha1" | "sha256" | "sha512") {
            continue;
        }
        let _ = event.set(&format!("file.hash.{kind}"), value.clone());
        push_unique(&mut related, value.clone());
    }

    let _ = event.set("related.hash", Value::Array(related));
}

/// `dataminr_pulse/alerts`, `script_map_alert_severity`: the vendor's alert
/// class as an ECS severity, with 30 for anything it does not name.
fn map_alert_severity(event: &mut Event, _params: &Value) {
    let severity = match event.get_str("json.alertType.name") {
        Some("Flash") => 90,
        Some("Urgent") => 50,
        _ => 30,
    };
    // Painless raises on the assignment where the parent map is absent.
    if !event.has("event") {
        return;
    }
    let _ = event.set("event.severity", severity);
}

/// Every `dataminr_pulse` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "e5327e43b17eb33e6a016d3e815073d4239330cbe3191e05439eb938ad2e069e",
        source: "dataminr_pulse",
        name: "extract_entities",
        run: extract_entities,
    },
    Entry {
        hash: "eda0a39ad823655387b24aecefcc613ea671c26d44659fb3c0e98d62d6b16420",
        source: "dataminr_pulse",
        name: "intel_agents_summary",
        run: intel_agents_summary,
    },
    Entry {
        hash: "47731f70bb857e68bdc31540e6d157a95ee4ad398ff014dd975c6c508128b86c",
        source: "dataminr_pulse",
        name: "event_category",
        run: event_category,
    },
    Entry {
        hash: "2dd2083d8dc6301e4ee346f0c6505deed16f62e139cd309dd3e8a338f90cbddd",
        source: "dataminr_pulse",
        name: "map_hash_values",
        run: map_hash_values,
    },
    Entry {
        hash: "00f3cc5090c6706837f305961cd305a3671c23fa71d7243f89d472f709cb3691",
        source: "dataminr_pulse",
        name: "map_alert_severity",
        run: map_alert_severity,
    },
];
