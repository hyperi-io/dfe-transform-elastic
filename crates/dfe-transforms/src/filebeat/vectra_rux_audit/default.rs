// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_event_original_a68ecd77",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("observer.product", json!("Vectra RUX"))?;

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("logout")),
                    serde_json::Value::String(s) => s.contains("logout"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.category", json!("session"))?;
            }

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("login")),
                    serde_json::Value::String(s) => s.contains("login"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("created")),
                    serde_json::Value::String(s) => s.contains("created"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("deleted")),
                    serde_json::Value::String(s) => s.contains("deleted"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("updated")),
                    serde_json::Value::String(s) => s.contains("updated"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("lock")),
                    serde_json::Value::String(s) => s.contains("lock"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("unlock")),
                    serde_json::Value::String(s) => s.contains("unlock"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("marked_as_fixed"))
                    }
                    serde_json::Value::String(s) => s.contains("marked_as_fixed"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("unmarked_as_fixed"))
                    }
                    serde_json::Value::String(s) => s.contains("unmarked_as_fixed"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("lock")),
                    serde_json::Value::String(s) => s.contains("lock"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("unlock")),
                    serde_json::Value::String(s) => s.contains("unlock"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.category", json!("access"))?;
            }

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("updated")),
                    serde_json::Value::String(s) => s.contains("updated"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("marked_as_fixed"))
                    }
                    serde_json::Value::String(s) => s.contains("marked_as_fixed"),
                    _ => false,
                }) || event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("unmarked_as_fixed"))
                    }
                    serde_json::Value::String(s) => s.contains("unmarked_as_fixed"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.type", json!("change"))?;
            }

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("created")),
                    serde_json::Value::String(s) => s.contains("created"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get("json.event_action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("deleted")),
                    serde_json::Value::String(s) => s.contains("deleted"),
                    _ => false,
                })
            };
            if _cond {
                event.append_unique("event.type", json!("deletion"))?;
            }

            if event.has_value("json.api_client_id") {
                event.rename("json.api_client_id", "vectra_rux.audit.api_client_id")?;
            }

            if event.has_value("json.event_action") {
                event.rename("json.event_action", "vectra_rux.audit.event.action")?;
            }

            if let Some(v) = event
                .get("vectra_rux.audit.event.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("event.action") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "join_event_action_1b3a653e",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.event_data") {
                event.rename("json.event_data", "vectra_rux.audit.event.data")?;
            }

            if event.has_value("json.event_object") {
                event.rename("json.event_object", "vectra_rux.audit.event.object")?;
            }

            let _cond = {
                event.has_value("json.event_timestamp")
                    && event.get_str("json.event_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.event_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("vectra_rux.audit.event.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.event_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_event_timestamp_67af833d",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("vectra_rux.audit.event.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.id") {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        }
                    })?;
                    event.set("vectra_rux.audit.id", converted)?;
                }
            }

            if event.has_value("json.message") {
                event.rename("json.message", "vectra_rux.audit.message")?;
            }

            if let Some(v) = event
                .get("vectra_rux.audit.message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.result_status") {
                event.rename("json.result_status", "vectra_rux.audit.result_status")?;
            }

            if let Some(v) = event
                .get("vectra_rux.audit.result_status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.outcome", v)?;
            }

            let _cond = { event.get_str("json.source_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.source_ip") {
                        if let Some(val) = event.get("json.source_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.source_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("vectra_rux.audit.source_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_ip_to_ip_9f448c50",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("vectra_rux.audit.source_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { event.has_value("vectra_rux.audit.source_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("vectra_rux.audit.source_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.user_id") {
                if let Some(val) = event.get("json.user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.user_id".into(),
                            message,
                        }
                    })?;
                    event.set("vectra_rux.audit.user.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("vectra_rux.audit.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("vectra_rux.audit.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("vectra_rux.audit.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.username") {
                event.rename("json.username", "vectra_rux.audit.user.name")?;
            }

            if let Some(v) = event
                .get("vectra_rux.audit.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.user_role") {
                event.rename("json.user_role", "vectra_rux.audit.user.role")?;
            }

            let _cond = { event.has_value("vectra_rux.audit.user.role") };
            if _cond {
                event.append_unique(
                    "user.roles",
                    json!(
                        event
                            .get("vectra_rux.audit.user.role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.role") {
                event.rename("json.role", "vectra_rux.audit.role")?;
            }

            if event.has_value("json.user_type") {
                event.rename("json.user_type", "vectra_rux.audit.user.type")?;
            }

            if event.has_value("json.version") {
                event.rename("json.version", "vectra_rux.audit.version")?;
            }

            if let Some(v) = event
                .get("vectra_rux.audit.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("vectra_rux.audit.event.timestamp");
                event.remove("vectra_rux.audit.message");
                event.remove("vectra_rux.audit.result_status");
                event.remove("vectra_rux.audit.source_ip");
                event.remove("vectra_rux.audit.user.id");
                event.remove("vectra_rux.audit.user.name");
                event.remove("vectra_rux.audit.user.role");
                event.remove("vectra_rux.audit.version");
            }

            event.remove("json");

            // Painless script
            // Source: def filterMassive(def src) {\n  if (src instanceof Map) {\n    for (def entry: src.entrySet()) {\n      entry.setValue(filterMassive(entry.getValue()));\n    }\n    return src;\n  } else if (src instanceof List) {\n    for (int i = 0; i < src.length; i++) {\n      src[i] = filterMassive(src[i]);\n    }\n    return src;\n  } else if (src instanceof String && src.length() > 32766) {\n    return src.substring(0, 32700)+' (truncated)';\n  }\n  return src;\n}\nfilterMassive(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def filterMassive(def src) {\n  if (src instanceof Map) {\n    for (def entry: src.entrySet()) {\n      entry.setValue(filterMassive(entry.getValue()));\n    }\n    return src;\n  } else if (src instanceof List) {\n    for (int i = 0; i < src.length; i++) {\n      src[i] = filterMassive(src[i]);\n    }\n    return src;\n  } else if (src instanceof String && src.length() > 32766) {\n    return src.substring(0, 32700)+' (truncated)';\n  }\n  return src;\n}\nfilterMassive(ctx);"#
                ),
            )?;

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
