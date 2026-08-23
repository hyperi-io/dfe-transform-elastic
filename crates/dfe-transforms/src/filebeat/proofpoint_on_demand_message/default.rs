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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.guid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.ts") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.msg.header.message-id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Proofpoint"))?;

            event.set("observer.product", json!("Proofpoint On Demand"))?;

            event.set("observer.type", json!("mail-gateway"))?;

            if event.has("json.action_dkimv") {
                event.rename(
                    "json.action_dkimv",
                    "proofpoint_on_demand.message.action_dkimv",
                )?;
            }

            if event.has("json.action_dmarc") {
                event.rename(
                    "json.action_dmarc",
                    "proofpoint_on_demand.message.action_dmarc",
                )?;
            }

            if event.has("json.action_spf") {
                event.rename("json.action_spf", "proofpoint_on_demand.message.action_spf")?;
            }

            if event.has("json.connection.country") {
                event.rename(
                    "json.connection.country",
                    "proofpoint_on_demand.message.connection.country",
                )?;
            }

            let _cond = { event.get_str("json.connection.ip") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.connection.helo") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.connection.helo".into(),
                                message,
                            }
                        })?;
                        event.set("json.connection.helo_ip", converted)?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.connection.helo") {
                event.rename(
                    "json.connection.helo",
                    "proofpoint_on_demand.message.connection.helo",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.connection.helo")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.address", v)?;
            }

            let _cond = {
                event.has_value("proofpoint_on_demand.message.connection.helo")
                    && !event.has_value("json.connection.helo_ip")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.connection.helo")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("proofpoint_on_demand.message.connection.helo")
                    && event.has_value("json.connection.helo_ip")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.connection.helo")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.connection.host") {
                event.rename(
                    "json.connection.host",
                    "proofpoint_on_demand.message.connection.host",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.connection.host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.message.connection.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.connection.host")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.connection.ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.connection.ip") {
                        if let Some(val) = event.get("json.connection.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.connection.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("proofpoint_on_demand.message.connection.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_connection_ip_to_ip",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                .get("proofpoint_on_demand.message.connection.ip")
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

            let _cond = { event.has_value("proofpoint_on_demand.message.connection.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.connection.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.connection.protocol") {
                event.rename(
                    "json.connection.protocol",
                    "proofpoint_on_demand.message.connection.protocol",
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.message.connection.protocol")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("proofpoint_on_demand.message.connection.protocol")
                        .is_some_and(|s| s.to_lowercase().contains("smtp"))
            };
            if _cond {
                event.set("network.protocol", json!("smtp"))?;
            }

            if event.has("json.connection.resolveStatus") {
                event.rename(
                    "json.connection.resolveStatus",
                    "proofpoint_on_demand.message.connection.resolve_status",
                )?;
            }

            if event.has("json.connection.sid") {
                event.rename(
                    "json.connection.sid",
                    "proofpoint_on_demand.message.connection.sid",
                )?;
            }

            if event.has("json.connection.tls.inbound.cipher") {
                event.rename(
                    "json.connection.tls.inbound.cipher",
                    "proofpoint_on_demand.message.connection.tls.inbound.cipher",
                )?;
            }

            let _cond = {
                event.get_str("proofpoint_on_demand.message.connection.tls.inbound.cipher")
                    != Some("NONE")
            };
            if _cond {
                if let Some(v) = event
                    .get("proofpoint_on_demand.message.connection.tls.inbound.cipher")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("tls.cipher", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.connection.tls.inbound.cipherBits") {
                    if let Some(val) = event.get("json.connection.tls.inbound.cipherBits") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.connection.tls.inbound.cipherBits".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.connection.tls.inbound.cipher_bits",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_connection_tls_inbound_cipherBits_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.connection.tls.inbound.policy") {
                event.rename(
                    "json.connection.tls.inbound.policy",
                    "proofpoint_on_demand.message.connection.tls.inbound.policy",
                )?;
            }

            if event.has("json.connection.tls.inbound.version") {
                event.rename(
                    "json.connection.tls.inbound.version",
                    "proofpoint_on_demand.message.connection.tls.inbound.version",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) =
                    event.get_string("proofpoint_on_demand.message.connection.tls.inbound.version")
                {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("v") else {
                            break 'dissect false;
                        };
                        captured.push(("tls.version_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("v") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("tls.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("tls.version_protocol") {
                if let Some(s) = event.get_string("tls.version_protocol") {
                    let lowered = s.to_lowercase();
                    event.set("tls.version_protocol", lowered)?;
                }
            }

            if event.has("json.envelope.from") {
                event.rename(
                    "json.envelope.from",
                    "proofpoint_on_demand.message.envelope.from",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.envelope.from")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.sender.address", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.message.envelope.from") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.envelope.from")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.envelope.fromHashed") {
                event.rename(
                    "json.envelope.fromHashed",
                    "proofpoint_on_demand.message.envelope.from_hashed",
                )?;
            }

            let _cond = {
                event
                    .get("json.envelope.rcpts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.envelope.rcpts").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.envelope.rcpts", Value::Array(out))?;
                }
            }

            if event.has("json.envelope.rcpts") {
                event.rename(
                    "json.envelope.rcpts",
                    "proofpoint_on_demand.message.envelope.rcpts",
                )?;
            }

            if event.has("json.envelope.rcptsHashed") {
                event.rename(
                    "json.envelope.rcptsHashed",
                    "proofpoint_on_demand.message.envelope.rcpts_hashed",
                )?;
            }

            let _cond = {
                event
                    .get("json.filter.actions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.filter.actions").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.isFinal") {
                                if let Some(val) = event.get("_ingest._value.isFinal") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.isFinal".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_final", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_filter_actions_isFinal_to_boolean",
                            )?;
                            event.remove("_ingest._value.isFinal");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.actions", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.actions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.filter.actions").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.remove("_ingest._value.isFinal");
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.actions", Value::Array(out))?;
                }
            }

            if event.has("json.filter.actions") {
                event.rename(
                    "json.filter.actions",
                    "proofpoint_on_demand.message.filter.actions",
                )?;
            }

            if event.has("json.filter.currentFolder") {
                event.rename(
                    "json.filter.currentFolder",
                    "proofpoint_on_demand.message.filter.current_folder",
                )?;
            }

            if event.has("json.filter.disposition") {
                event.rename(
                    "json.filter.disposition",
                    "proofpoint_on_demand.message.filter.disposition",
                )?;
            }

            let _cond = {
                event.has_value("json.filter.startTime")
                    && event.get_str("json.filter.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.filter.startTime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("proofpoint_on_demand.message.filter.start_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_filter_startTime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                .get("proofpoint_on_demand.message.filter.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.durationSecs") {
                    if let Some(val) = event.get("json.filter.durationSecs") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.durationSecs".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.duration_secs",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_durationSecs_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("proofpoint_on_demand.message.filter.duration_secs") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event.duration = (int) (ctx.proofpoint_on_demand.message.filter.duration_secs * 1000000000);\nif (ctx.event?.start != null) {\n  ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n  ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event.duration = (int) (ctx.proofpoint_on_demand.message.filter.duration_secs * 1000000000);\nif (ctx.event?.start != null) {\n  ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n  ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_event_duration_s_to_ns",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.isMsgEncrypted") {
                    if let Some(val) = event.get("json.filter.isMsgEncrypted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.isMsgEncrypted".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.is_msg_encrypted",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_isMsgEncrypted_to_boolean",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.isMsgReinjected") {
                    if let Some(val) = event.get("json.filter.isMsgReinjected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.isMsgReinjected".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.is_msg_reinjected",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_isMsgReinjected_to_boolean",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.filter.mid") {
                event.rename("json.filter.mid", "proofpoint_on_demand.message.filter.mid")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.filter.mid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.local_id", v)?;
            }

            if event.has("json.filter.modules.av.virusNames") {
                event.rename(
                    "json.filter.modules.av.virusNames",
                    "proofpoint_on_demand.message.filter.modules.av.virus_names",
                )?;
            }

            let _cond = {
                event
                    .get("json.filter.modules.dkimv")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.filter.modules.dkimv").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.domain")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dkimv", Value::Array(out))?;
                }
            }

            if event.has("json.filter.modules.dkimv") {
                event.rename(
                    "json.filter.modules.dkimv",
                    "proofpoint_on_demand.message.filter.modules.dkimv",
                )?;
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.alignment")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.alignment").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has("_ingest._value.fromDomain") {
                            event.rename(
                                "_ingest._value.fromDomain",
                                "_ingest._value.from_domain",
                            )?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.alignment", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.alignment")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.alignment").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.from_domain")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.alignment", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.alignment")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.alignment").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if let Some(Value::Array(items)) =
                            event.get("_ingest._value.results").cloned()
                        {
                            let mut out = Vec::with_capacity(items.len());
                            for item in items {
                                event.set("_ingest._value", item)?;
                                if event.has("_ingest._value.identityOrg") {
                                    event.rename(
                                        "_ingest._value.identityOrg",
                                        "_ingest._value.identity_org",
                                    )?;
                                }
                                out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                            }
                            event.remove("_ingest");
                            event.set("_ingest._value.results", Value::Array(out))?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.alignment", Value::Array(out))?;
                }
            }

            if event.has("json.filter.modules.dmarc.alignment") {
                event.rename(
                    "json.filter.modules.dmarc.alignment",
                    "proofpoint_on_demand.message.filter.modules.dmarc.alignment",
                )?;
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.authResults")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.authResults").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            dot_expand(event, "_ingest._value.emailIdentities", "*")?;
                            Ok(())
                        })();
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.authResults", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.authResults")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.authResults").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has("_ingest._value.emailIdentities") {
                            event.rename(
                                "_ingest._value.emailIdentities",
                                "_ingest._value.email_identities",
                            )?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.authResults", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.authResults")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.authResults").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has("_ingest._value.email_identities.smtp.mailfromHashed") {
                            event.rename(
                                "_ingest._value.email_identities.smtp.mailfromHashed",
                                "_ingest._value.email_identities.smtp.mailfrom_hashed",
                            )?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.authResults", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.authResults")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.authResults").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.email_identities.header.from")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.authResults", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.filter.modules.dmarc.authResults")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.filter.modules.dmarc.authResults").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            dot_expand(event, "_ingest._value.propspec", "*")?;
                            Ok(())
                        })();
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.modules.dmarc.authResults", Value::Array(out))?;
                }
            }

            if event.has("json.filter.modules.dmarc.authResults") {
                event.rename(
                    "json.filter.modules.dmarc.authResults",
                    "proofpoint_on_demand.message.filter.modules.dmarc.auth_results",
                )?;
            }

            if event.has("json.filter.modules.dmarc.filterdResult") {
                event.rename(
                    "json.filter.modules.dmarc.filterdResult",
                    "proofpoint_on_demand.message.filter.modules.dmarc.filterd_result",
                )?;
            }

            if event.has("json.filter.modules.dmarc.records") {
                event.rename(
                    "json.filter.modules.dmarc.records",
                    "proofpoint_on_demand.message.filter.modules.dmarc.records",
                )?;
            }

            if event.has("json.filter.modules.dmarc.srvid") {
                event.rename(
                    "json.filter.modules.dmarc.srvid",
                    "proofpoint_on_demand.message.filter.modules.dmarc.srvid",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.pdr.v1.rscore") {
                    if let Some(val) = event.get("json.filter.modules.pdr.v1.rscore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.pdr.v1.rscore".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.modules.pdr.v1.rscore",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_pdr_v1_rscore_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.pdr.v1.spamscore") {
                    if let Some(val) = event.get("json.filter.modules.pdr.v1.spamscore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.pdr.v1.spamscore".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.modules.pdr.v1.spamscore",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_pdr_v1_spamscore_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.pdr.v1.virusscore") {
                    if let Some(val) = event.get("json.filter.modules.pdr.v1.virusscore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.pdr.v1.virusscore".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.modules.pdr.v1.virusscore",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_pdr_v1_virusscore_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.filter.modules.pdr.v2.response") {
                event.rename(
                    "json.filter.modules.pdr.v2.response",
                    "proofpoint_on_demand.message.filter.modules.pdr.v2.response",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.pdr.v2.rscore") {
                    if let Some(val) = event.get("json.filter.modules.pdr.v2.rscore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.pdr.v2.rscore".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.modules.pdr.v2.rscore",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_pdr_v2_rscore_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.filter.modules.sandbox.errorStatus") {
                event.rename(
                    "json.filter.modules.sandbox.errorStatus",
                    "proofpoint_on_demand.message.filter.modules.sandbox.error_status",
                )?;
            }

            if event.has("json.filter.modules.spam.triggeredClassifier") {
                event.rename(
                    "json.filter.modules.spam.triggeredClassifier",
                    "json.filter.modules.spam.triggered_classifier",
                )?;
            }

            if event.has("json.filter.modules.spam") {
                event.rename(
                    "json.filter.modules.spam",
                    "proofpoint_on_demand.message.filter.modules.spam",
                )?;
            }

            if event.has("json.filter.modules.spf.domain") {
                event.rename(
                    "json.filter.modules.spf.domain",
                    "proofpoint_on_demand.message.filter.modules.spf.domain",
                )?;
            }

            let _cond =
                { event.has_value("proofpoint_on_demand.message.filter.modules.spf.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.filter.modules.spf.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.filter.modules.spf.result") {
                event.rename(
                    "json.filter.modules.spf.result",
                    "proofpoint_on_demand.message.filter.modules.spf.result",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.urldefense.counts.maxLimit") {
                    if let Some(val) = event.get("json.filter.modules.urldefense.counts.maxLimit") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.urldefense.counts.maxLimit".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.max_limit", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_maxLimit_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.filter.modules.urldefense.counts.noRewriteIsContentTypeText")
                {
                    if let Some(val) = event
                        .get("json.filter.modules.urldefense.counts.noRewriteIsContentTypeText")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.filter.modules.urldefense.counts.noRewriteIsContentTypeText".into(),
                            message,
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_content_type_text", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsContentTypeText_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.urldefense.counts.noRewriteIsEmail") {
                    if let Some(val) =
                        event.get("json.filter.modules.urldefense.counts.noRewriteIsEmail")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.urldefense.counts.noRewriteIsEmail"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_email", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsEmail_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.filter.modules.urldefense.counts.noRewriteIsExcludedDomain")
                {
                    if let Some(val) =
                        event.get("json.filter.modules.urldefense.counts.noRewriteIsExcludedDomain")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.filter.modules.urldefense.counts.noRewriteIsExcludedDomain".into(),
                            message,
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_excluded_domain", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsExcludedDomain_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.filter.modules.urldefense.counts.noRewriteIsLargeMsgPartSize")
                {
                    if let Some(val) = event
                        .get("json.filter.modules.urldefense.counts.noRewriteIsLargeMsgPartSize")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.filter.modules.urldefense.counts.noRewriteIsLargeMsgPartSize".into(),
                            message,
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_large_msgpart_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsLargeMsgPartSize_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.filter.modules.urldefense.counts.noRewriteIsMaxLengthExceeded")
                {
                    if let Some(val) = event
                        .get("json.filter.modules.urldefense.counts.noRewriteIsMaxLengthExceeded")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.filter.modules.urldefense.counts.noRewriteIsMaxLengthExceeded".into(),
                            message,
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_maxlength_exceeded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsMaxLengthExceeded_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.urldefense.counts.noRewriteIsSchemeless") {
                    if let Some(val) =
                        event.get("json.filter.modules.urldefense.counts.noRewriteIsSchemeless")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.urldefense.counts.noRewriteIsSchemeless"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_schemeless", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsSchemeless_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.filter.modules.urldefense.counts.noRewriteIsUnsupportedScheme")
                {
                    if let Some(val) = event
                        .get("json.filter.modules.urldefense.counts.noRewriteIsUnsupportedScheme")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.filter.modules.urldefense.counts.noRewriteIsUnsupportedScheme".into(),
                            message,
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.no_rewrite.is_unsupported_scheme", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_noRewriteIsUnsupportedScheme_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.urldefense.counts.rewritten") {
                    if let Some(val) = event.get("json.filter.modules.urldefense.counts.rewritten")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.urldefense.counts.rewritten".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.message.filter.modules.urldefense.counts.rewritten", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_rewritten_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.urldefense.counts.total") {
                    if let Some(val) = event.get("json.filter.modules.urldefense.counts.total") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.urldefense.counts.total".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.modules.urldefense.counts.total",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_total_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.modules.urldefense.counts.unique") {
                    if let Some(val) = event.get("json.filter.modules.urldefense.counts.unique") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.modules.urldefense.counts.unique".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.modules.urldefense.counts.unique",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_modules_urldefense_counts_unique_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.filter.modules.urldefense.rewrittenUrls") {
                event.rename(
                    "json.filter.modules.urldefense.rewrittenUrls",
                    "proofpoint_on_demand.message.filter.modules.urldefense.rewritten_urls",
                )?;
            }

            if event.has_value("json.filter.modules.urldefense.version.engine") {
                if let Some(val) = event.get("json.filter.modules.urldefense.version.engine") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.filter.modules.urldefense.version.engine".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "proofpoint_on_demand.message.filter.modules.urldefense.version.engine",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.filter.modules.zerohour.score") {
                if let Some(val) = event.get("json.filter.modules.zerohour.score") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.filter.modules.zerohour.score".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "proofpoint_on_demand.message.filter.modules.zerohour.score",
                        converted,
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filter.msgSizeBytes") {
                    if let Some(val) = event.get("json.filter.msgSizeBytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filter.msgSizeBytes".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_on_demand.message.filter.msg_size_bytes",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filter_msgSizeBytes_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.filter.origGuid") {
                event.rename(
                    "json.filter.origGuid",
                    "proofpoint_on_demand.message.filter.orig_guid",
                )?;
            }

            let _cond = {
                event
                    .get("json.filter.pe.rcpts")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                if event.has("json.filter.pe.rcpts") {
                    event.rename(
                        "json.filter.pe.rcpts",
                        "proofpoint_on_demand.message.filter.pe.rcpts_object",
                    )?;
                }
            }

            if event.has_value("json.filter.pe.rcpts") {
                if let Some(val) = event.get("json.filter.pe.rcpts") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.filter.pe.rcpts".into(),
                            message,
                        }
                    })?;
                    event.set("proofpoint_on_demand.message.filter.pe.rcpts", converted)?;
                }
            }

            if event.has("json.filter.qid") {
                event.rename("json.filter.qid", "proofpoint_on_demand.message.filter.qid")?;
            }

            if event.has("json.filter.quarantine.folder") {
                event.rename(
                    "json.filter.quarantine.folder",
                    "proofpoint_on_demand.message.filter.quarantine.folder",
                )?;
            }

            if event.has("json.filter.quarantine.rule") {
                event.rename(
                    "json.filter.quarantine.rule",
                    "proofpoint_on_demand.message.filter.quarantine.rule",
                )?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.message.filter.quarantine.rule") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.filter.quarantine.rule")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.filter.routeDirection") {
                event.rename(
                    "json.filter.routeDirection",
                    "proofpoint_on_demand.message.filter.route_direction",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.filter.route_direction")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.direction", v)?;
            }

            if event.has_value("network.direction") {
                if let Some(s) = event.get_string("network.direction") {
                    let lowered = s.to_lowercase();
                    event.set("network.direction", lowered)?;
                }
            }

            if event.has("json.filter.routes") {
                event.rename(
                    "json.filter.routes",
                    "proofpoint_on_demand.message.filter.routes",
                )?;
            }

            if event.has("json.filter.smime.rcpts") {
                event.rename(
                    "json.filter.smime.rcpts",
                    "proofpoint_on_demand.message.filter.smime.rcpts",
                )?;
            }

            if event.has("json.filter.smime.signedRcpts") {
                event.rename(
                    "json.filter.smime.signedRcpts",
                    "proofpoint_on_demand.message.filter.smime.signed_rcpts",
                )?;
            }

            if event.has_value("json.filter.suborgs.rcpts") {
                if let Some(val) = event.get("json.filter.suborgs.rcpts") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.filter.suborgs.rcpts".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "proofpoint_on_demand.message.filter.suborgs.rcpts",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.filter.suborgs.sender") {
                if let Some(val) = event.get("json.filter.suborgs.sender") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.filter.suborgs.sender".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "proofpoint_on_demand.message.filter.suborgs.sender",
                        converted,
                    )?;
                }
            }

            let _cond = { event.get_str("json.filter.throttleIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.filter.throttleIp") {
                        if let Some(val) = event.get("json.filter.throttleIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.filter.throttleIp".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "proofpoint_on_demand.message.filter.throttle_ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_filter_throttleIp_to_ip",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.has_value("proofpoint_on_demand.message.filter.throttle_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.filter.throttle_ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("json.filter.verified.rcpts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.filter.verified.rcpts").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.filter.verified.rcpts", Value::Array(out))?;
                }
            }

            if event.has("json.filter.verified.rcpts") {
                event.rename(
                    "json.filter.verified.rcpts",
                    "proofpoint_on_demand.message.filter.verified.rcpts",
                )?;
            }

            if event.has("json.filter.verified.rcptsHashed") {
                event.rename(
                    "json.filter.verified.rcptsHashed",
                    "proofpoint_on_demand.message.filter.verified.rcpts_hashed",
                )?;
            }

            if event.has("json.final_action") {
                event.rename(
                    "json.final_action",
                    "proofpoint_on_demand.message.final_action",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.final_action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let lowered = s.to_lowercase();
                    event.set("event.action", lowered)?;
                }
            }

            if event.has("json.final_module") {
                event.rename(
                    "json.final_module",
                    "proofpoint_on_demand.message.final_module",
                )?;
            }

            if event.has("json.final_rule") {
                event.rename("json.final_rule", "proofpoint_on_demand.message.final_rule")?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.message.final_rule") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.final_rule")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.guid") {
                event.rename("json.guid", "proofpoint_on_demand.message.guid")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.guid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.metadata.origin.data.agent") {
                event.rename(
                    "json.metadata.origin.data.agent",
                    "proofpoint_on_demand.message.metadata.origin.data.agent",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.metadata.origin.data.agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond =
                { event.has_value("proofpoint_on_demand.message.metadata.origin.data.agent") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.metadata.origin.data.agent")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.metadata.origin.data.cid") {
                event.rename(
                    "json.metadata.origin.data.cid",
                    "proofpoint_on_demand.message.metadata.origin.data.cid",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.metadata.origin.data.cid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has("json.metadata.origin.data.version") {
                event.rename(
                    "json.metadata.origin.data.version",
                    "proofpoint_on_demand.message.metadata.origin.data.version",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.metadata.origin.data.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has("json.pps.agent") {
                event.rename("json.pps.agent", "proofpoint_on_demand.message.pps.agent")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.pps.agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.message.pps.agent") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.message.pps.agent")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.pps.cid") {
                event.rename("json.pps.cid", "proofpoint_on_demand.message.pps.cid")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.pps.cid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has("json.pps.version") {
                event.rename(
                    "json.pps.version",
                    "proofpoint_on_demand.message.pps.version",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.message.pps.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            let _cond = {
                event
                    .get("json.msg.header.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.cc").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.cc.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.cc", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.msg.header.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.cc").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.cc", Value::Array(out))?;
                }
            }

            if event.has("json.msg.header.cc") {
                event.rename(
                    "json.msg.header.cc",
                    "proofpoint_on_demand.message.msg.header.cc",
                )?;
            }

            let _cond = {
                event
                    .get("json.msg.header.from")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.from").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.from.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.from", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.msg.header.from")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.from").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.from", Value::Array(out))?;
                }
            }

            if event.has("json.msg.header.from") {
                event.rename(
                    "json.msg.header.from",
                    "proofpoint_on_demand.message.msg.header.from",
                )?;
            }

            if event.has("json.msg.header.fromHashed") {
                event.rename(
                    "json.msg.header.fromHashed",
                    "proofpoint_on_demand.message.msg.header.from_hashed",
                )?;
            }

            if event.has("json.msg.header.message-id") {
                event.rename(
                    "json.msg.header.message-id",
                    "proofpoint_on_demand.message.msg.header.message_id",
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.message.msg.header.message_id")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("proofpoint_on_demand.message.msg.header.message_id")
                    .cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.message_id",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set(
                        "proofpoint_on_demand.message.msg.header.message_id",
                        Value::Array(out),
                    )?;
                }
            }

            if event.has("json.msg.header.reply-to") {
                event.rename(
                    "json.msg.header.reply-to",
                    "proofpoint_on_demand.message.msg.header.reply_to",
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.message.msg.header.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("proofpoint_on_demand.message.msg.header.reply_to")
                    .cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.reply_to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set(
                        "proofpoint_on_demand.message.msg.header.reply_to",
                        Value::Array(out),
                    )?;
                }
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.message.msg.header.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("proofpoint_on_demand.message.msg.header.reply_to")
                    .cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set(
                        "proofpoint_on_demand.message.msg.header.reply_to",
                        Value::Array(out),
                    )?;
                }
            }

            if event.has("json.msg.header.return-path") {
                event.rename(
                    "json.msg.header.return-path",
                    "proofpoint_on_demand.message.msg.header.return_path",
                )?;
            }

            let _cond = {
                event
                    .get("json.msg.header.subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.subject").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.subject",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.subject", Value::Array(out))?;
                }
            }

            if event.has("json.msg.header.subject") {
                event.rename(
                    "json.msg.header.subject",
                    "proofpoint_on_demand.message.msg.header.subject",
                )?;
            }

            let _cond = {
                event
                    .get("json.msg.header.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.to").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.to", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.msg.header.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.header.to").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.header.to", Value::Array(out))?;
                }
            }

            if event.has("json.msg.header.to") {
                event.rename(
                    "json.msg.header.to",
                    "proofpoint_on_demand.message.msg.header.to",
                )?;
            }

            if event.has("json.msg.header.toHashed") {
                event.rename(
                    "json.msg.header.toHashed",
                    "proofpoint_on_demand.message.msg.header.to_hashed",
                )?;
            }

            if event.has("json.msg.lang") {
                event.rename("json.msg.lang", "proofpoint_on_demand.message.msg.lang")?;
            }

            if event.has("json.msg.normalizedHeader.fromHashed") {
                event.rename(
                    "json.msg.normalizedHeader.fromHashed",
                    "json.msg.normalizedHeader.from_hashed",
                )?;
            }

            if event.has("json.msg.normalizedHeader.message-id") {
                event.rename(
                    "json.msg.normalizedHeader.message-id",
                    "json.msg.normalizedHeader.message_id",
                )?;
            }

            if event.has("json.msg.normalizedHeader.reply-to") {
                event.rename(
                    "json.msg.normalizedHeader.reply-to",
                    "json.msg.normalizedHeader.reply_to",
                )?;
            }

            if event.has("json.msg.normalizedHeader.return-path") {
                event.rename(
                    "json.msg.normalizedHeader.return-path",
                    "json.msg.normalizedHeader.return_path",
                )?;
            }

            if event.has("json.msg.normalizedHeader.toHashed") {
                event.rename(
                    "json.msg.normalizedHeader.toHashed",
                    "json.msg.normalizedHeader.to_hashed",
                )?;
            }

            if event.has("json.msg.normalizedHeader") {
                event.rename(
                    "json.msg.normalizedHeader",
                    "proofpoint_on_demand.message.msg.normalized_header",
                )?;
            }

            let _cond = {
                event
                    .get("json.msg.parsedAddresses.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.parsedAddresses.cc").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.parsedAddresses.cc", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("json.msg.parsedAddresses.from")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("json.msg.parsedAddresses.from").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.parsedAddresses.from", Value::Array(out))?;
                }
            }

            if event.has("json.msg.parsedAddresses.fromHashed") {
                event.rename(
                    "json.msg.parsedAddresses.fromHashed",
                    "json.msg.parsedAddresses.from_hashed",
                )?;
            }

            let _cond = {
                event
                    .get("json.msg.parsedAddresses.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.msg.parsedAddresses.to").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.msg.parsedAddresses.to", Value::Array(out))?;
                }
            }

            if event.has("json.msg.parsedAddresses.toHashed") {
                event.rename(
                    "json.msg.parsedAddresses.toHashed",
                    "json.msg.parsedAddresses.to_hashed",
                )?;
            }

            if event.has("json.msg.parsedAddresses") {
                event.rename(
                    "json.msg.parsedAddresses",
                    "proofpoint_on_demand.message.msg.parsed_addresses",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.msg.sizeBytes") {
                    if let Some(val) = event.get("json.msg.sizeBytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.msg.sizeBytes".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.message.msg.size_bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_msg_sizeBytes_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get("json.msgParts").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def convertToLong(def value) {\n  if (value instanceof String) {\n    return Long.parseLong(value);\n  } else if (value instanceof Number) {\n    return ((long) value).longValue();\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef convertToBoolean(def value) {\n  if (value instanceof String) {\n    return Boolean.parseBoolean(value);\n  } else if (value instanceof Boolean) {\n    return (Boolean) value;\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        if (key == '') {\n          key = \"MISSING_KEY\";\n        }\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        if (key == '') {\n          key = \"MISSING_KEY\";\n        }\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        if (['detected_size_bytes', 'size_decoded_bytes'].contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToLong(value);\n        } else if (['is_archive', 'is_corrupted', 'is_deleted', 'is_protected', 'is_timed_out', 'is_virtual', 'is_rewritten'].contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToBoolean(value);\n        } else {\n          updatedJson[keyMap[key]] = value;\n        }\n      } else {\n        if (key == '') {\n          key = \"MISSING_KEY\";\n        }\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\nctx.proofpoint_on_demand.message.put('msg_parts', new ArrayList());\nif (ctx.related == null) {\n  ctx.put('related', new HashMap());\n}\nctx.related.put('hash', new ArrayList());\nif (ctx.url == null) {\n  ctx.put('url', new HashMap());\n}\nctx.url.put('full', new ArrayList());\nif(ctx.email == null){\n  ctx.put('email', new HashMap());\n}\nctx.email.put('attachments', new ArrayList());\nfor (part in ctx.json.msgParts) {\n  def msg_part = renameKeys(part, params);\n  ctx.proofpoint_on_demand.message.msg_parts.add(msg_part);\n  ctx.related.hash.add(msg_part.sha256);\n  ctx.related.hash.add(msg_part.md5);\n  if (msg_part.urls instanceof List) {\n    for (url in msg_part.urls) {\n      ctx.url.full.add(url.url);\n    }\n  }\n  def attachment = new HashMap();\n  attachment.put('file', new HashMap());\n  attachment.file.put('name', msg_part.detected_name);\n  attachment.file.put('extension', msg_part.detected_ext);\n  attachment.file.put('mime_type', msg_part.detected_mime);\n  attachment.file.put('size', msg_part.detected_size_bytes);\n  attachment.file.put('hash', new HashMap());\n  attachment.file.hash.put('md5', msg_part.md5);\n  attachment.file.hash.put('sha256', msg_part.sha256);\n  ctx.email.attachments.add(attachment);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def convertToLong(def value) {\n  if (value instanceof String) {\n    return Long.parseLong(value);\n  } else if (value instanceof Number) {\n    return ((long) value).longValue();\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef convertToBoolean(def value) {\n  if (value instanceof String) {\n    return Boolean.parseBoolean(value);\n  } else if (value instanceof Boolean) {\n    return (Boolean) value;\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        if (key == '') {\n          key = \"MISSING_KEY\";\n        }\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        if (key == '') {\n          key = \"MISSING_KEY\";\n        }\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        if (['detected_size_bytes', 'size_decoded_bytes'].contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToLong(value);\n        } else if (['is_archive', 'is_corrupted', 'is_deleted', 'is_protected', 'is_timed_out', 'is_virtual', 'is_rewritten'].contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToBoolean(value);\n        } else {\n          updatedJson[keyMap[key]] = value;\n        }\n      } else {\n        if (key == '') {\n          key = \"MISSING_KEY\";\n        }\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\nctx.proofpoint_on_demand.message.put('msg_parts', new ArrayList());\nif (ctx.related == null) {\n  ctx.put('related', new HashMap());\n}\nctx.related.put('hash', new ArrayList());\nif (ctx.url == null) {\n  ctx.put('url', new HashMap());\n}\nctx.url.put('full', new ArrayList());\nif(ctx.email == null){\n  ctx.put('email', new HashMap());\n}\nctx.email.put('attachments', new ArrayList());\nfor (part in ctx.json.msgParts) {\n  def msg_part = renameKeys(part, params);\n  ctx.proofpoint_on_demand.message.msg_parts.add(msg_part);\n  ctx.related.hash.add(msg_part.sha256);\n  ctx.related.hash.add(msg_part.md5);\n  if (msg_part.urls instanceof List) {\n    for (url in msg_part.urls) {\n      ctx.url.full.add(url.url);\n    }\n  }\n  def attachment = new HashMap();\n  attachment.put('file', new HashMap());\n  attachment.file.put('name', msg_part.detected_name);\n  attachment.file.put('extension', msg_part.detected_ext);\n  attachment.file.put('mime_type', msg_part.detected_mime);\n  attachment.file.put('size', msg_part.detected_size_bytes);\n  attachment.file.put('hash', new HashMap());\n  attachment.file.hash.put('md5', msg_part.md5);\n  attachment.file.hash.put('sha256', msg_part.sha256);\n  ctx.email.attachments.add(attachment);\n}\n"#
                        ),
                        cached_params!(
                            "{\"dataBase64\":\"database64\",\"detectedCharset\":\"detected_charset\",\"detectedExt\":\"detected_ext\",\"detectedMime\":\"detected_mime\",\"detectedName\":\"detected_name\",\"detectedSizeBytes\":\"detected_size_bytes\",\"isArchive\":\"is_archive\",\"isCorrupted\":\"is_corrupted\",\"isDeleted\":\"is_deleted\",\"isProtected\":\"is_protected\",\"isRewritten\":\"is_rewritten\",\"isTimedOut\":\"is_timed_out\",\"isVirtual\":\"is_virtual\",\"labeledCharset\":\"labeled_charset\",\"labeledExt\":\"labeled_ext\",\"labeledMime\":\"labeled_mime\",\"labeledName\":\"labeled_name\",\"notRewrittenReason\":\"not_rewritten_reason\",\"sandboxStatus\":\"sandbox_status\",\"sizeDecodedBytes\":\"size_decoded_bytes\",\"structureId\":\"structure_id\",\"textExtracted\":\"text_extracted\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_map_fields_under_msgParts_object",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("email.attachments").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.file.extension") {
                            if let Some(s) = event.get_string("_ingest._value.file.extension") {
                                let lowered = s.to_lowercase();
                                event.set("_ingest._value.file.extension", lowered)?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("email.attachments", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("json.ts") && event.get_str("json.ts") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ts") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("proofpoint_on_demand.message.ts", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ts")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                .get("proofpoint_on_demand.message.ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.message.msg_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("proofpoint_on_demand.message.msg_parts").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if let Some(Value::Array(items)) = event.get("_ingest._value.urls").cloned()
                        {
                            let mut out = Vec::with_capacity(items.len());
                            for item in items {
                                event.set("_ingest._value", item)?;
                                let _cond = {
                                    !event.has_value("tags")
                                        || !(event.get("tags").is_some_and(|v| match v {
                                            serde_json::Value::Array(a) => a.iter().any(|x| {
                                                x.as_str()
                                                    == Some("preserve_duplicate_custom_fields")
                                            }),
                                            serde_json::Value::String(s) => {
                                                s.contains("preserve_duplicate_custom_fields")
                                            }
                                            _ => false,
                                        }))
                                };
                                if _cond {
                                    event.remove("_ingest._value.url");
                                }
                                out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                            }
                            event.remove("_ingest");
                            event.set("_ingest._value.urls", Value::Array(out))?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("proofpoint_on_demand.message.msg_parts", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.message.msg_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("proofpoint_on_demand.message.msg_parts").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.detected_ext");
                            event.remove("_ingest._value.detected_mime");
                            event.remove("_ingest._value.detected_name");
                            event.remove("_ingest._value.detected_size_bytes");
                            event.remove("_ingest._value.md5");
                            event.remove("_ingest._value.sha256");
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("proofpoint_on_demand.message.msg_parts", Value::Array(out))?;
                }
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
                event.remove("proofpoint_on_demand.message.connection.helo");
                event.remove("proofpoint_on_demand.message.connection.host");
                event.remove("proofpoint_on_demand.message.connection.ip");
                event.remove("proofpoint_on_demand.message.connection.tls.inbound.cipher");
                event.remove("proofpoint_on_demand.message.envelope.from");
                event.remove("proofpoint_on_demand.message.filter.mid");
                event.remove("proofpoint_on_demand.message.filter.route_direction");
                event.remove("proofpoint_on_demand.message.filter.start_time");
                event.remove("proofpoint_on_demand.message.final_action");
                event.remove("proofpoint_on_demand.message.guid");
                event.remove("proofpoint_on_demand.message.msg.header.cc");
                event.remove("proofpoint_on_demand.message.msg.header.from");
                event.remove("proofpoint_on_demand.message.msg.header.message_id");
                event.remove("proofpoint_on_demand.message.msg.header.reply_to");
                event.remove("proofpoint_on_demand.message.msg.header.to");
                event.remove("proofpoint_on_demand.message.msg.header.subject");
                event.remove("proofpoint_on_demand.message.ts");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '' || o == '**' || o == '0' || (o instanceof String && ((String) o).trim() == '')) {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).entrySet().removeIf(e -> e.getKey().trim() == '' || drop(e.getValue()));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '' || o == '**' || o == '0' || (o instanceof String && ((String) o).trim() == '')) {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).entrySet().removeIf(e -> e.getKey().trim() == '' || drop(e.getValue()));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
