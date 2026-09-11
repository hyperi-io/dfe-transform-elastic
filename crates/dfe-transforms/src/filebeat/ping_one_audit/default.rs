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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            event.append("event.category", json!("iam"))?;

            let _cond = {
                event.has_value("json.action.type")
                    && (event
                        .get_str("json.action.type")
                        .is_some_and(|s| s.to_lowercase().contains("created"))
                        || event
                            .get_str("json.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("deleted"))
                        || event
                            .get_str("json.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("updated"))
                        || event
                            .get_str("json.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("access_allowed")))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("created"))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("deleted"))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("updated"))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("user"))
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("group"))
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("allowed"))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("denied"))
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("started"))
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("access_allowed"))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("password.check_succeeded"))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event
                    .get_str("json.action.type")
                    .is_some_and(|s| s.to_lowercase().contains("email"))
            };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.recordedAt") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("json.createdAt") && event.get_str("json.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("ping_one.audit.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json._embedded") {
                event.rename("json._embedded", "ping_one.audit.embedded")?;
            }

            if event.has_value("json.tags") {
                event.rename("json.tags", "ping_one.audit.tags")?;
            }

            let _cond = {
                event.has_value("ping_one.audit.tags")
                    && event
                        .get("ping_one.audit.tags")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "ping_one.audit.tags", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "tags",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.id") {
                event.rename("json.id", "ping_one.audit.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.recordedAt") && event.get_str("json.recordedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.recordedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("ping_one.audit.recorded_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.recordedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.recorded_at").cloned() {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.correlationId") {
                event.rename("json.correlationId", "ping_one.audit.correlation.id")?;
            }

            if event.has_value("json.actors.client.id") {
                event.rename("json.actors.client.id", "ping_one.audit.actors.client.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.actors.client.id").cloned() {
                    event.set("client.user.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actors.client.name") {
                event.rename(
                    "json.actors.client.name",
                    "ping_one.audit.actors.client.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.actors.client.name").cloned() {
                    event.set("client.user.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actors.client.environment.id") {
                event.rename(
                    "json.actors.client.environment.id",
                    "ping_one.audit.actors.client.environment.id",
                )?;
            }

            if event.has_value("json.actors.client.href") {
                event.rename(
                    "json.actors.client.href",
                    "ping_one.audit.actors.client.href",
                )?;
            }

            if event.has_value("json.actors.client.type") {
                event.rename(
                    "json.actors.client.type",
                    "ping_one.audit.actors.client.type",
                )?;
            }

            let _cond = { event.has_value("client.user.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("client.user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("client.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("client.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.actors.user.id") {
                event.rename("json.actors.user.id", "ping_one.audit.actors.user.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.actors.user.id").cloned() {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actors.user.name") {
                event.rename("json.actors.user.name", "ping_one.audit.actors.user.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.actors.user.name").cloned() {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

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

            if event.has_value("json.actors.user.population.id") {
                event.rename(
                    "json.actors.user.population.id",
                    "ping_one.audit.actors.user.population.id",
                )?;
            }

            if event.has_value("json.actors.user.environment.id") {
                event.rename(
                    "json.actors.user.environment.id",
                    "ping_one.audit.actors.user.environment.id",
                )?;
            }

            if event.has_value("json.actors.user.href") {
                event.rename("json.actors.user.href", "ping_one.audit.actors.user.href")?;
            }

            if event.has_value("json.actors.user.type") {
                event.rename("json.actors.user.type", "ping_one.audit.actors.user.type")?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.action.type") {
                event.rename("json.action.type", "ping_one.audit.action.type")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("ping_one.audit.action.type").cloned() {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("json.action.description") {
                event.rename(
                    "json.action.description",
                    "ping_one.audit.action.description",
                )?;
            }

            let _cond = {
                event.has_value("json.resources")
                    && event.get("json.resources").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.resources", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            uri_parts(event, "_ingest._value.href", "url", true, false)?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.resources") {
                event.rename("json.resources", "ping_one.audit.resources")?;
            }

            if event.has_value("json.result.id") {
                event.rename("json.result.id", "ping_one.audit.result.id")?;
            }

            if event.has_value("json.result.status") {
                event.rename("json.result.status", "ping_one.audit.result.status")?;
            }

            let _cond = {
                event.get_str("ping_one.audit.result.status") == Some("SUCCESS")
                    || event.get_str("ping_one.audit.result.status") == Some("succeeded")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("ping_one.audit.result.status") == Some("FAILURE")
                    || event.get_str("ping_one.audit.result.status") == Some("failed")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            if event.has_value("json.result.description") {
                event.rename(
                    "json.result.description",
                    "ping_one.audit.result.description",
                )?;
            }

            if event.has_value("json.internalCorrelation.transactionId") {
                event.rename(
                    "json.internalCorrelation.transactionId",
                    "ping_one.audit.internal_correlation.transaction_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.source.ipAddress") {
                    if let Some(val) = event.get("json.source.ipAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.source.ipAddress".into(),
                                message,
                            }
                        })?;
                        event.set("ping_one.audit.source.ip_address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_ip_address_to_ip",
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

            if let Some(v) = event
                .get("ping_one.audit.source.ip_address")
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

            let _cond = { event.has_value("ping_one.audit.source.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("ping_one.audit.source.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.source.userAgent") {
                event.rename("json.source.userAgent", "ping_one.audit.source.user_agent")?;
            }

            if event.has_value("ping_one.audit.source.user_agent") {
                if let Some(ua_str) = event.get_string("ping_one.audit.source.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            event.remove("json");

            let _cond = {
                event.has_value("ping_one.audit.resources")
                    && event
                        .get("ping_one.audit.resources")
                        .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "ping_one.audit.resources", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.remove("_ingest._value.href");
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("ping_one.audit.recorded_at");
                    event.remove("ping_one.audit.tags");
                    event.remove("ping_one.audit.id");
                    event.remove("ping_one.audit.result.status");
                    event.remove("ping_one.audit.action.type");
                    event.remove("ping_one.audit.actors.user.id");
                    event.remove("ping_one.audit.actors.user.name");
                    event.remove("ping_one.audit.actors.client.id");
                    event.remove("ping_one.audit.actors.client.name");
                    event.remove("ping_one.audit.source.ip_address");
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
                        "Processor '{}'\n{}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}'\n",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
