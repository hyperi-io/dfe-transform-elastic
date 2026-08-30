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

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.remove("message");

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("infoblox_threat_defense.event.created", v)?;
            }

            if event.has_value("log.syslog") {
                event.rename("log.syslog", "infoblox_threat_defense.event.syslog")?;
            }

            if event.has_value("cef.name") {
                event.rename("cef.name", "infoblox_threat_defense.event.name")?;
            }

            if event.has_value("cef.version") {
                event.rename("cef.version", "infoblox_threat_defense.event.version")?;
            }

            if event.has_value("cef.device") {
                event.rename("cef.device", "infoblox_threat_defense.event.device")?;
            }

            let _cond = { event.get_str("cef.severity") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.severity") {
                        if let Some(val) = event.get("cef.severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.severity".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_threat_defense.event.severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_severity_to_long",
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

            let _cond = { event.has_value("infoblox_threat_defense.event.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nif (ctx.infoblox_threat_defense.event.severity >= 0 && ctx.infoblox_threat_defense.event.severity <= 3 ) { // Severity level - 0,1,2,3 denotes Low severity\n  ctx.event.severity = 21;\n} else if (ctx.infoblox_threat_defense.event.severity >= 4 && ctx.infoblox_threat_defense.event.severity <= 6) { // Severity level - 4,5,6 denotes Medium severity\n  ctx.event.severity = 47;\n} else if (ctx.infoblox_threat_defense.event.severity == 7 || ctx.infoblox_threat_defense.event.severity == 8) { // Severity level - 7 and 8 denotes High severity\n  ctx.event.severity = 73;\n} else if (ctx.infoblox_threat_defense.event.severity == 9 || ctx.infoblox_threat_defense.event.severity == 10) { // Severity level - 9 and 10 denotes Critical severity\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nif (ctx.infoblox_threat_defense.event.severity >= 0 && ctx.infoblox_threat_defense.event.severity <= 3 ) { // Severity level - 0,1,2,3 denotes Low severity\n  ctx.event.severity = 21;\n} else if (ctx.infoblox_threat_defense.event.severity >= 4 && ctx.infoblox_threat_defense.event.severity <= 6) { // Severity level - 4,5,6 denotes Medium severity\n  ctx.event.severity = 47;\n} else if (ctx.infoblox_threat_defense.event.severity == 7 || ctx.infoblox_threat_defense.event.severity == 8) { // Severity level - 7 and 8 denotes High severity\n  ctx.event.severity = 73;\n} else if (ctx.infoblox_threat_defense.event.severity == 9 || ctx.infoblox_threat_defense.event.severity == 10) { // Severity level - 9 and 10 denotes Critical severity\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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
                                .get("_ingest.pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.cef.extensions.containsKey('message') && ctx.cef.extensions.message != null && ctx.cef.extensions.message instanceof String) {\n  if (ctx.cef.extensions.message.startsWith('\"') && ctx.cef.extensions.message.endsWith('\"') && ctx.cef.extensions.message.length() >= 2) {\n    ctx.cef.extensions.message = ctx.cef.extensions.message.substring(1, ctx.cef.extensions.message.length() - 1);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.cef.extensions.containsKey('message') && ctx.cef.extensions.message != null && ctx.cef.extensions.message instanceof String) {\n  if (ctx.cef.extensions.message.startsWith('\"') && ctx.cef.extensions.message.endsWith('\"') && ctx.cef.extensions.message.length() >= 2) {\n    ctx.cef.extensions.message = ctx.cef.extensions.message.substring(1, ctx.cef.extensions.message.length() - 1);\n  }\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_remove_quotes_from_begining_and_end_from_message",
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
                            .get("_ingest.pipeline")
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
                .get("cef.extensions.message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("event.code") == Some("BloxOne-Audit-Log") };
            if _cond {
                // Begin nested pipeline: "pipeline_audit"
                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename(
                        "cef.extensions.applicationProtocol",
                        "infoblox_threat_defense.event.application_protocol",
                    )?;
                }
                if event.has_value("network.application") {
                    map_strings(
                        event,
                        "network.application",
                        "network.application",
                        str::to_lowercase,
                    )?;
                }
                if event.has_value("cef.extensions.deviceAction") {
                    event.rename(
                        "cef.extensions.deviceAction",
                        "infoblox_threat_defense.event.device.action",
                    )?;
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
                let _cond = {
                    event.has_value("event.action") && event.get_str("event.action") != Some("")
                };
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
                        event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename(
                        "cef.extensions.deviceEventCategory",
                        "infoblox_threat_defense.event.device.event_category",
                    )?;
                }
                if event.has_value("cef.extensions.eventOutcome") {
                    event.rename(
                        "cef.extensions.eventOutcome",
                        "infoblox_threat_defense.event.outcome",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("infoblox_threat_defense.event.outcome")
                        .is_some_and(|s| s.to_lowercase().contains("success"))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event
                        .get_str("infoblox_threat_defense.event.outcome")
                        .is_some_and(|s| s.to_lowercase().contains("fail"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if event.has_value("cef.extensions.InfobloxEventVersion") {
                    event.rename(
                        "cef.extensions.InfobloxEventVersion",
                        "infoblox_threat_defense.event.infoblox.event.version",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxHTTPReqBody") {
                    event.rename(
                        "cef.extensions.InfobloxHTTPReqBody",
                        "infoblox_threat_defense.event.infoblox.http.req_body",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.http.req_body")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.body.content", v)?;
                }
                let _cond = {
                    event.has_value("infoblox_threat_defense.event.infoblox.http.req_body")
                        && event.get_str("infoblox_threat_defense.event.infoblox.http.req_body")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "infoblox_threat_defense.event.infoblox.http.req_body",
                            "infoblox_threat_defense.event.infoblox.http.req_body",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set("_ingest.on_failure_processor_tag", "json_http_req_body")?;
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
                if event.has_value("cef.extensions.InfobloxHTTPRespBody") {
                    event.rename(
                        "cef.extensions.InfobloxHTTPRespBody",
                        "infoblox_threat_defense.event.infoblox.http.resp_body",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.http.resp_body")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.response.body.content", v)?;
                }
                let _cond = {
                    event.has_value("infoblox_threat_defense.event.infoblox.http.resp_body")
                        && event.get_str("infoblox_threat_defense.event.infoblox.http.resp_body")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "infoblox_threat_defense.event.infoblox.http.resp_body",
                            "infoblox_threat_defense.event.infoblox.http.resp_body",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set("_ingest.on_failure_processor_tag", "json_http_resp_body")?;
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
                if event.has_value("cef.extensions.InfobloxResourceDesc") {
                    event.rename(
                        "cef.extensions.InfobloxResourceDesc",
                        "infoblox_threat_defense.event.infoblox.resource.desc",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxResourceId") {
                    event.rename(
                        "cef.extensions.InfobloxResourceId",
                        "infoblox_threat_defense.event.infoblox.resource.id",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.resource.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if event.has_value("cef.extensions.InfobloxResourceType") {
                    event.rename(
                        "cef.extensions.InfobloxResourceType",
                        "infoblox_threat_defense.event.infoblox.resource.type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxSubjectGroups") {
                    event.rename(
                        "cef.extensions.InfobloxSubjectGroups",
                        "infoblox_threat_defense.event.infoblox.subject.groups",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxSubjectType") {
                    event.rename(
                        "cef.extensions.InfobloxSubjectType",
                        "infoblox_threat_defense.event.infoblox.subject.type",
                    )?;
                }
                if event.has_value("cef.extensions.message") {
                    event.rename(
                        "cef.extensions.message",
                        "infoblox_threat_defense.event.message",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.sourceAddress") {
                            if let Some(val) = event.get("cef.extensions.sourceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.sourceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.source.address",
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
                            "convert_cef_extensions_sourceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.source.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.sourceUserName") {
                    event.rename(
                        "cef.extensions.sourceUserName",
                        "infoblox_threat_defense.event.source.user_name",
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.source.user_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.user_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline_audit"
            }

            let _cond = { event.get_str("event.code") == Some("BloxOne-Service-Log") };
            if _cond {
                // Begin nested pipeline: "pipeline_service"
                if event.has_value("cef.extensions.InfobloxLogName") {
                    event.rename(
                        "cef.extensions.InfobloxLogName",
                        "infoblox_threat_defense.event.infoblox.log_name",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxPoolId") {
                    event.rename(
                        "cef.extensions.InfobloxPoolId",
                        "infoblox_threat_defense.event.infoblox.pool_id",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxServiceId") {
                    event.rename(
                        "cef.extensions.InfobloxServiceId",
                        "infoblox_threat_defense.event.infoblox.service_id",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.service_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if event.has_value("cef.extensions.message") {
                    event.rename(
                        "cef.extensions.message",
                        "infoblox_threat_defense.event.message",
                    )?;
                }
                // End nested pipeline: "pipeline_service"
            }

            let _cond = {
                event.has_value("event.code")
                    && event.get("event.code").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCP-LEASE"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCP-LEASE"),
                        _ => false,
                    })
            };
            if _cond {
                // Begin nested pipeline: "pipeline_dhcp_lease"
                event.append("event.type", json!("info"))?;
                event.append("event.category", json!("network"))?;
                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename(
                        "cef.extensions.applicationProtocol",
                        "infoblox_threat_defense.event.application_protocol",
                    )?;
                }
                if event.has_value("network.application") {
                    map_strings(
                        event,
                        "network.application",
                        "network.application",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.destinationAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.destinationAddress") {
                            if let Some(val) = event.get("cef.extensions.destinationAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.destinationAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.destination.address",
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
                            "convert_cef_extensions_destinationAddress_to_ip",
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
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.destination.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.destination.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename(
                        "cef.extensions.deviceEventCategory",
                        "infoblox_threat_defense.event.device.event_category",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxClientID") {
                    event.rename(
                        "cef.extensions.InfobloxClientID",
                        "infoblox_threat_defense.event.infoblox.client_id",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxDHCPOptions") {
                    event.rename(
                        "cef.extensions.InfobloxDHCPOptions",
                        "infoblox_threat_defense.event.infoblox.dhcp_options",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxDUID") {
                    event.rename(
                        "cef.extensions.InfobloxDUID",
                        "infoblox_threat_defense.event.infoblox.duid",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxFingerprint") {
                    event.rename(
                        "cef.extensions.InfobloxFingerprint",
                        "infoblox_threat_defense.event.infoblox.fingerprint",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.InfobloxFingerprintPr") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxFingerprintPr") {
                            if let Some(val) = event.get("cef.extensions.InfobloxFingerprintPr") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "cef.extensions.InfobloxFingerprintPr".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.fingerprint_pr",
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
                            "convert_cef_extensions_InfobloxFingerprintPr_to_boolean",
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
                if event.has_value("cef.extensions.InfobloxHost") {
                    event.rename(
                        "cef.extensions.InfobloxHost",
                        "infoblox_threat_defense.event.infoblox.host_name",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.host_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.host_name") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.host_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxHostID") {
                    event.rename(
                        "cef.extensions.InfobloxHostID",
                        "infoblox_threat_defense.event.infoblox.host_id",
                    )?;
                }
                let _cond =
                    { event.get_str("infoblox_threat_defense.event.infoblox.host_id") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_threat_defense.event.infoblox.host_id") {
                            if let Some(input) =
                                event.get_string("infoblox_threat_defense.event.infoblox.host_id")
                            {
                                let mut remaining: &str = &input;
                                let mut captured: Vec<(&str, &str)> = Vec::new();
                                let matched = 'dissect: {
                                    let Some(rest) = remaining.strip_prefix("dhcp/host/") else {
                                        break 'dissect false;
                                    };
                                    remaining = rest;
                                    captured.push(("host.id", remaining));
                                    true
                                };
                                if matched {
                                    for (path, value) in captured {
                                        event.set(path, value)?;
                                    }
                                } else {
                                    return Err(TransformError::ParseError {
                                        path: "infoblox_threat_defense.event.infoblox.host_id"
                                            .into(),
                                        message: "dissect pattern did not match".into(),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "dissect_event_infoblox_host_id",
                        )?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("host.id") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxIPSpaceName") {
                    event.rename(
                        "cef.extensions.InfobloxIPSpaceName",
                        "infoblox_threat_defense.event.infoblox.ip_space.name",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxIPSpace") {
                    event.rename(
                        "cef.extensions.InfobloxIPSpace",
                        "infoblox_threat_defense.event.infoblox.ip_space.value",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxLeaseOp") {
                    event.rename(
                        "cef.extensions.InfobloxLeaseOp",
                        "infoblox_threat_defense.event.infoblox.lease.op",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.lease.op")
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
                let _cond = {
                    event.has_value("event.action") && event.get_str("event.action") != Some("")
                };
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
                        event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
                if event.has_value("cef.extensions.InfobloxLeaseUUID") {
                    event.rename(
                        "cef.extensions.InfobloxLeaseUUID",
                        "infoblox_threat_defense.event.infoblox.lease.uuid",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.InfobloxLifetime") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxLifetime") {
                            if let Some(val) = event.get("cef.extensions.InfobloxLifetime") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxLifetime".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.lifetime",
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
                            "convert_cef_extensions_InfobloxLifetime_to_long",
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
                let _cond = { event.get_str("cef.extensions.InfobloxRangeEnd") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxRangeEnd") {
                            if let Some(val) = event.get("cef.extensions.InfobloxRangeEnd") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxRangeEnd".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.range.end",
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
                            "convert_cef_extensions_InfobloxRangeEnd_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.range.end") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.range.end")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.InfobloxRangeStart") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxRangeStart") {
                            if let Some(val) = event.get("cef.extensions.InfobloxRangeStart") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxRangeStart".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.range.start",
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
                            "convert_cef_extensions_InfobloxRangeStart_to_ip",
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
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.infoblox.range.start") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.range.start")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxSubnet") {
                    event.rename(
                        "cef.extensions.InfobloxSubnet",
                        "infoblox_threat_defense.event.infoblox.subnet",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.sourceAddress") {
                            if let Some(val) = event.get("cef.extensions.sourceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.sourceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.source.address",
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
                            "convert_cef_extensions_sourceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.source.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.sourceHostName") {
                    event.rename(
                        "cef.extensions.sourceHostName",
                        "infoblox_threat_defense.event.source.hostname",
                    )?;
                }
                if event.has_value("cef.extensions.sourceMacAddress") {
                    event.rename(
                        "cef.extensions.sourceMacAddress",
                        "infoblox_threat_defense.event.source.mac_address",
                    )?;
                }
                // End nested pipeline: "pipeline_dhcp_lease"
            }

            let _cond = { event.get_str("event.code") == Some("DNS Response") };
            if _cond {
                // Begin nested pipeline: "pipeline_dns_response"
                event.append("event.type", json!("info"))?;
                event.append("event.category", json!("network"))?;
                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename(
                        "cef.extensions.applicationProtocol",
                        "infoblox_threat_defense.event.application_protocol",
                    )?;
                }
                if event.has_value("network.application") {
                    map_strings(
                        event,
                        "network.application",
                        "network.application",
                        str::to_lowercase,
                    )?;
                }
                if event.has_value("cef.extensions.destinationDnsDomain") {
                    event.rename(
                        "cef.extensions.destinationDnsDomain",
                        "infoblox_threat_defense.event.destination.dns_domain",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.deviceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.deviceAddress") {
                            if let Some(val) = event.get("cef.extensions.deviceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.deviceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.device.address",
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
                            "convert_cef_extensions_deviceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.deviceHostName") != Some("") };
                if _cond {
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.deviceHostName") {
                            if let Some(val) = event.get("cef.extensions.deviceHostName") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.deviceHostName".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.device.host_ip",
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
                            "convert_cef_extensions_deviceHostName_to_ip",
                        )?;
                        if event.has_value("cef.extensions.deviceHostName") {
                            event.rename(
                                "cef.extensions.deviceHostName",
                                "infoblox_threat_defense.event.device.host_name",
                            )?;
                        }
                        if let Some(v) = event
                            .get("infoblox_threat_defense.event.device.host_name")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("host.hostname", v)?;
                        }
                        let _cond =
                            { event.has_value("infoblox_threat_defense.event.device.host_name") };
                        if _cond {
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("infoblox_threat_defense.event.device.host_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.host_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.host_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.host_ip") };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.host_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxAnCount") {
                    event.rename(
                        "cef.extensions.InfobloxAnCount",
                        "infoblox_threat_defense.event.infoblox.an_count",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxArCount") {
                    event.rename(
                        "cef.extensions.InfobloxArCount",
                        "infoblox_threat_defense.event.infoblox.ar_count",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1ConnectionType") {
                    event.rename(
                        "cef.extensions.InfobloxB1ConnectionType",
                        "infoblox_threat_defense.event.infoblox.b1.connection_type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1DHCPFingerprint") {
                    event.rename(
                        "cef.extensions.InfobloxB1DHCPFingerprint",
                        "infoblox_threat_defense.event.infoblox.b1.dhcp_fingerprint",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1DNSTags") {
                    event.rename(
                        "cef.extensions.InfobloxB1DNSTags",
                        "infoblox_threat_defense.event.infoblox.b1.dns_tags",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1Network") {
                    event.rename(
                        "cef.extensions.InfobloxB1Network",
                        "infoblox_threat_defense.event.infoblox.b1.network",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.InfobloxB1OPHIPAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxB1OPHIPAddress") {
                            if let Some(val) = event.get("cef.extensions.InfobloxB1OPHIPAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxB1OPHIPAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.b1.oph.ip_address",
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
                            "convert_cef_extensions_InfobloxB1OPHIPAddress_to_ip",
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
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.infoblox.b1.oph.ip_address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.b1.oph.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1OPHName") {
                    event.rename(
                        "cef.extensions.InfobloxB1OPHName",
                        "infoblox_threat_defense.event.infoblox.b1.oph.name",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1Region") {
                    event.rename(
                        "cef.extensions.InfobloxB1Region",
                        "infoblox_threat_defense.event.infoblox.b1.region",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1SrcOSVersion") {
                    event.rename(
                        "cef.extensions.InfobloxB1SrcOSVersion",
                        "infoblox_threat_defense.event.infoblox.b1.src_os_version",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.b1.src_os_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.full", v)?;
                }
                if event.has_value("cef.extensions.InfobloxDNSQClass") {
                    event.rename(
                        "cef.extensions.InfobloxDNSQClass",
                        "infoblox_threat_defense.event.infoblox.dns_q.class",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.dns_q.class")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("dns.question.class", v)?;
                }
                if event.has_value("cef.extensions.InfobloxDNSQFlags") {
                    event.rename(
                        "cef.extensions.InfobloxDNSQFlags",
                        "infoblox_threat_defense.event.infoblox.dns_q.flags",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxDNSQType") {
                    event.rename(
                        "cef.extensions.InfobloxDNSQType",
                        "infoblox_threat_defense.event.infoblox.dns_q.type",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.dns_q.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("dns.question.type", v)?;
                }
                if event.has_value("cef.extensions.InfobloxDNSRCode") {
                    event.rename(
                        "cef.extensions.InfobloxDNSRCode",
                        "infoblox_threat_defense.event.infoblox.dns_r_code",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.dns_r_code")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("dns.response_code", v)?;
                }
                if event.has_value("cef.extensions.InfobloxNsCount") {
                    event.rename(
                        "cef.extensions.InfobloxNsCount",
                        "infoblox_threat_defense.event.infoblox.ns_count",
                    )?;
                }
                if event.has_value("cef.extensions.message") {
                    event.rename(
                        "cef.extensions.message",
                        "infoblox_threat_defense.event.message",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.sourceAddress") {
                            if let Some(val) = event.get("cef.extensions.sourceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.sourceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.source.address",
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
                            "convert_cef_extensions_sourceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.source.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                let _cond = { event.get_str("cef.extensions.sourcePort") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.sourcePort") {
                            if let Some(val) = event.get("cef.extensions.sourcePort") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.sourcePort".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("infoblox_threat_defense.event.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_cef_extensions_sourcePort_to_long",
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
                if event.has_value("cef.extensions.InfobloxDNSView") {
                    event.rename(
                        "cef.extensions.InfobloxDNSView",
                        "infoblox_threat_defense.event.infoblox.dns_view",
                    )?;
                }
                if event.has_value("cef.extensions.sourceMacAddress") {
                    event.rename(
                        "cef.extensions.sourceMacAddress",
                        "infoblox_threat_defense.event.source.mac_address",
                    )?;
                }
                if event.has_value("cef.extensions.sourceUserName") {
                    event.rename(
                        "cef.extensions.sourceUserName",
                        "infoblox_threat_defense.event.source.user_name",
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.source.user_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.user_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.destinationAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.destinationAddress") {
                            if let Some(val) = event.get("cef.extensions.destinationAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.destinationAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.destination.address",
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
                            "convert_cef_extensions_destinationAddress_to_ip",
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
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.destination.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.destination.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.transportProtocol") {
                    event.rename(
                        "cef.extensions.transportProtocol",
                        "infoblox_threat_defense.event.transport_protocol",
                    )?;
                }
                if event.has_value("network.transport") {
                    map_strings(
                        event,
                        "network.transport",
                        "network.transport",
                        str::to_lowercase,
                    )?;
                }
                // End nested pipeline: "pipeline_dns_response"
            }

            let _cond = { event.get_str("event.code") == Some("BloxOne-Notifications-Log") };
            if _cond {
                // Begin nested pipeline: "pipeline_atlas_notification"
                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename(
                        "cef.extensions.deviceEventCategory",
                        "infoblox_threat_defense.event.device.event_category",
                    )?;
                }
                if event.has_value("cef.extensions.message") {
                    event.rename(
                        "cef.extensions.message",
                        "infoblox_threat_defense.event.message",
                    )?;
                }
                if event.has_value("cef.extensions.status") {
                    event.rename(
                        "cef.extensions.status",
                        "infoblox_threat_defense.event.status",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxNotificationType") {
                    event.rename(
                        "cef.extensions.InfobloxNotificationType",
                        "infoblox_threat_defense.event.infoblox.notification.type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxNotificationSubType") {
                    event.rename(
                        "cef.extensions.InfobloxNotificationSubType",
                        "infoblox_threat_defense.event.infoblox.notification.sub_type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxOnPremHostName") {
                    event.rename(
                        "cef.extensions.InfobloxOnPremHostName",
                        "infoblox_threat_defense.event.infoblox.on_prem_host_name",
                    )?;
                }
                let _cond = {
                    event.get_str("infoblox_threat_defense.event.infoblox.on_prem_host_name")
                        != Some("(none)")
                };
                if _cond {
                    if let Some(v) = event
                        .get("infoblox_threat_defense.event.infoblox.on_prem_host_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("host.hostname", v)?;
                    }
                }
                let _cond = {
                    event.has_value("infoblox_threat_defense.event.infoblox.on_prem_host_name")
                        && event.get_str("infoblox_threat_defense.event.infoblox.on_prem_host_name")
                            != Some("(none)")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.on_prem_host_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("cef.extensions.InfobloxEventOccurredTime")
                        && event.get_str("cef.extensions.InfobloxEventOccurredTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("cef.extensions.InfobloxEventOccurredTime")
                        {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set(
                                    "infoblox_threat_defense.event.infoblox.event.occurred_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "cef.extensions.InfobloxEventOccurredTime".into(),
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
                            "date_cef_extensions_InfobloxEventOccurredTime",
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
                    .get("infoblox_threat_defense.event.infoblox.event.occurred_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                // End nested pipeline: "pipeline_atlas_notification"
            }

            let _cond = { event.get_str("event.code") == Some("BloxOne-InsightsNotification-Log") };
            if _cond {
                // Begin nested pipeline: "pipeline_soc_insight"
                event.append("event.type", json!("indicator"))?;
                event.append("event.category", json!("threat"))?;
                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename(
                        "cef.extensions.deviceEventCategory",
                        "infoblox_threat_defense.event.device.event_category",
                    )?;
                }
                let _cond = {
                    event.has_value("cef.extensions.InfobloxEventOccurredTime")
                        && event.get_str("cef.extensions.InfobloxEventOccurredTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("cef.extensions.InfobloxEventOccurredTime")
                        {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set(
                                    "infoblox_threat_defense.event.infoblox.event.occurred_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "cef.extensions.InfobloxEventOccurredTime".into(),
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
                            "date_cef_extensions_InfobloxEventOccurredTime",
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
                    .get("infoblox_threat_defense.event.infoblox.event.occurred_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
                if event.has_value("cef.extensions.InfobloxInsightDescription") {
                    event.rename(
                        "cef.extensions.InfobloxInsightDescription",
                        "infoblox_threat_defense.event.infoblox.insight.description",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxInsightFeedSource") {
                    event.rename(
                        "cef.extensions.InfobloxInsightFeedSource",
                        "infoblox_threat_defense.event.infoblox.insight.feed_source",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxInsightId") {
                    event.rename(
                        "cef.extensions.InfobloxInsightId",
                        "infoblox_threat_defense.event.infoblox.insight.id",
                    )?;
                }
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.infoblox.insight.id") };
                if _cond {
                    event.append_unique(
                        "threat.indicator.id",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.insight.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("cef.extensions.InfobloxInsightStatus")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("infoblox_threat_defense.event.infoblox.insight.status", v)?;
                }
                if let Some(v) = event
                    .get("cef.extensions.status")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("infoblox_threat_defense.event.infoblox.insight.status") {
                        event.set("infoblox_threat_defense.event.infoblox.insight.status", v)?;
                    }
                }
                if event.has_value("cef.extensions.InfobloxInsightThreatType") {
                    event.rename(
                        "cef.extensions.InfobloxInsightThreatType",
                        "infoblox_threat_defense.event.infoblox.insight.threat_type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxInsightUserComment") {
                    event.rename(
                        "cef.extensions.InfobloxInsightUserComment",
                        "infoblox_threat_defense.event.infoblox.insight.user_comment",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxEventsBlockedCount") {
                    event.rename(
                        "cef.extensions.InfobloxEventsBlockedCount",
                        "infoblox_threat_defense.event.infoblox.stats.events_blocked_count",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxEventsNotBlockedCount") {
                    event.rename(
                        "cef.extensions.InfobloxEventsNotBlockedCount",
                        "infoblox_threat_defense.event.infoblox.stats.events_not_blocked_count",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxThreatClass") {
                    event.rename(
                        "cef.extensions.InfobloxThreatClass",
                        "infoblox_threat_defense.event.infoblox.threat.class",
                    )?;
                }
                let _cond =
                    { event.get_str("cef.extensions.InfobloxThreatConfidence") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxThreatConfidence") {
                            if let Some(val) = event.get("cef.extensions.InfobloxThreatConfidence")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxThreatConfidence".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.threat.confidence",
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
                            "convert_cef_extensions_InfobloxThreatConfidence_to_long",
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
                let _cond = { event.get_str("cef.extensions.InfobloxThreatLevel") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxThreatLevel") {
                            if let Some(val) = event.get("cef.extensions.InfobloxThreatLevel") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxThreatLevel".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.threat.level",
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
                            "convert_cef_extensions_InfobloxThreatLevel_to_long",
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
                if event.has_value("cef.extensions.InfobloxThreatFamily") {
                    event.rename(
                        "cef.extensions.InfobloxThreatFamily",
                        "infoblox_threat_defense.event.infoblox.threat.family",
                    )?;
                }
                if event.has_value("cef.extensions.message") {
                    event.rename(
                        "cef.extensions.message",
                        "infoblox_threat_defense.event.message",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.description", v)?;
                }
                let _cond = { event.get_str("cef.extensions.baseEventCount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.baseEventCount") {
                            if let Some(val) = event.get("cef.extensions.baseEventCount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.baseEventCount".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.stats.base_event_count",
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
                            "convert_cef_extensions_baseEventCount_to_long",
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
                // End nested pipeline: "pipeline_soc_insight"
            }

            let _cond = {
                event.has_value("event.code")
                    && event.get("event.code").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("RPZ")),
                        serde_json::Value::String(s) => s.contains("RPZ"),
                        _ => false,
                    })
            };
            if _cond {
                // Begin nested pipeline: "pipeline_rpz"
                event.append("event.type", json!("indicator"))?;
                event.append("event.category", json!("threat"))?;
                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename(
                        "cef.extensions.applicationProtocol",
                        "infoblox_threat_defense.event.application_protocol",
                    )?;
                }
                if event.has_value("network.application") {
                    map_strings(
                        event,
                        "network.application",
                        "network.application",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.deviceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.deviceAddress") {
                            if let Some(val) = event.get("cef.extensions.deviceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.deviceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.device.address",
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
                            "convert_cef_extensions_deviceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1ConnectionType") {
                    event.rename(
                        "cef.extensions.InfobloxB1ConnectionType",
                        "infoblox_threat_defense.event.infoblox.b1.connection_type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1DHCPFingerprint") {
                    event.rename(
                        "cef.extensions.InfobloxB1DHCPFingerprint",
                        "infoblox_threat_defense.event.infoblox.b1.dhcp_fingerprint",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1DNSTags") {
                    event.rename(
                        "cef.extensions.InfobloxB1DNSTags",
                        "infoblox_threat_defense.event.infoblox.b1.dns_tags",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1FeedName") {
                    event.rename(
                        "cef.extensions.InfobloxB1FeedName",
                        "infoblox_threat_defense.event.infoblox.b1.feed.name",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1FeedType") {
                    event.rename(
                        "cef.extensions.InfobloxB1FeedType",
                        "infoblox_threat_defense.event.infoblox.b1.feed.type",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1Network") {
                    event.rename(
                        "cef.extensions.InfobloxB1Network",
                        "infoblox_threat_defense.event.infoblox.b1.network",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1OPHName") {
                    event.rename(
                        "cef.extensions.InfobloxB1OPHName",
                        "infoblox_threat_defense.event.infoblox.b1.oph.name",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.InfobloxB1OPHIPAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxB1OPHIPAddress") {
                            if let Some(val) = event.get("cef.extensions.InfobloxB1OPHIPAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxB1OPHIPAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.b1.oph.ip_address",
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
                            "convert_cef_extensions_InfobloxB1OPHIPAddress_to_ip",
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
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.infoblox.b1.oph.ip_address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.infoblox.b1.oph.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1PolicyAction") {
                    event.rename(
                        "cef.extensions.InfobloxB1PolicyAction",
                        "infoblox_threat_defense.event.infoblox.b1.policy.action",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1PolicyName") {
                    event.rename(
                        "cef.extensions.InfobloxB1PolicyName",
                        "infoblox_threat_defense.event.infoblox.b1.policy.name",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxB1Region") {
                    event.rename(
                        "cef.extensions.InfobloxB1Region",
                        "infoblox_threat_defense.event.infoblox.b1.region",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.b1.region")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo.region_name", v)?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.b1.region")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.region", v)?;
                }
                if event.has_value("cef.extensions.InfobloxB1SrcOSVersion") {
                    event.rename(
                        "cef.extensions.InfobloxB1SrcOSVersion",
                        "infoblox_threat_defense.event.infoblox.b1.src_os_version",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.b1.src_os_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
                if event.has_value("cef.extensions.InfobloxB1ThreatIndicator") {
                    event.rename(
                        "cef.extensions.InfobloxB1ThreatIndicator",
                        "infoblox_threat_defense.event.infoblox.b1.threat.indicator",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.b1.threat.indicator")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.reference", v)?;
                }
                if event.has_value("cef.extensions.InfobloxCSiteId") {
                    event.rename(
                        "cef.extensions.InfobloxCSiteId",
                        "infoblox_threat_defense.event.infoblox.c_site_id",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.destinationAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.destinationAddress") {
                            if let Some(val) = event.get("cef.extensions.destinationAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.destinationAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.destination.address",
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
                            "convert_cef_extensions_destinationAddress_to_ip",
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
                let _cond =
                    { event.has_value("infoblox_threat_defense.event.destination.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.destination.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                if event.has_value("cef.extensions.destinationDnsDomain") {
                    event.rename(
                        "cef.extensions.destinationDnsDomain",
                        "infoblox_threat_defense.event.destination.dns_domain",
                    )?;
                }
                if event.has_value("cef.extensions.deviceAction") {
                    event.rename(
                        "cef.extensions.deviceAction",
                        "infoblox_threat_defense.event.device.action",
                    )?;
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
                let _cond = {
                    event.has_value("event.action") && event.get_str("event.action") != Some("")
                };
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
                        event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
                let _cond = { event.get_str("cef.extensions.deviceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.deviceAddress") {
                            if let Some(val) = event.get("cef.extensions.deviceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.deviceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.device.address",
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
                            "convert_cef_extensions_deviceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxDNSQType") {
                    event.rename(
                        "cef.extensions.InfobloxDNSQType",
                        "infoblox_threat_defense.event.infoblox.dns_q.type",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.dns_q.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("dns.question.type", v)?;
                }
                if event.has_value("cef.extensions.InfobloxDNSView") {
                    event.rename(
                        "cef.extensions.InfobloxDNSView",
                        "infoblox_threat_defense.event.infoblox.dns_view",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxDomainCat") {
                    event.rename(
                        "cef.extensions.InfobloxDomainCat",
                        "infoblox_threat_defense.event.infoblox.domain.cat",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.deviceHostName") != Some("") };
                if _cond {
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.deviceHostName") {
                            if let Some(val) = event.get("cef.extensions.deviceHostName") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.deviceHostName".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.device.host_ip",
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
                            "convert_cef_extensions_deviceHostName_to_ip",
                        )?;
                        if event.has_value("cef.extensions.deviceHostName") {
                            event.rename(
                                "cef.extensions.deviceHostName",
                                "infoblox_threat_defense.event.device.host_name",
                            )?;
                        }
                        if let Some(v) = event
                            .get("infoblox_threat_defense.event.device.host_name")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("host.hostname", v)?;
                        }
                        let _cond =
                            { event.has_value("infoblox_threat_defense.event.device.host_name") };
                        if _cond {
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("infoblox_threat_defense.event.device.host_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.host_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.host_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.device.host_ip") };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.device.host_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxPolicyID") {
                    event.rename(
                        "cef.extensions.InfobloxPolicyID",
                        "infoblox_threat_defense.event.infoblox.policy.id",
                    )?;
                }
                if event.has_value("cef.extensions.InfobloxRPZRule") {
                    event.rename(
                        "cef.extensions.InfobloxRPZRule",
                        "infoblox_threat_defense.event.infoblox.rpz.rule",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.infoblox.rpz.rule")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
                if event.has_value("cef.extensions.InfobloxRPZ") {
                    event.rename(
                        "cef.extensions.InfobloxRPZ",
                        "infoblox_threat_defense.event.infoblox.rpz.value",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.sourceAddress") {
                            if let Some(val) = event.get("cef.extensions.sourceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.sourceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.source.address",
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
                            "convert_cef_extensions_sourceAddress_to_ip",
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
                let _cond = { event.has_value("infoblox_threat_defense.event.source.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                let _cond =
                    { event.get_str("cef.extensions.InfobloxThreatConfidence") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxThreatConfidence") {
                            if let Some(val) = event.get("cef.extensions.InfobloxThreatConfidence")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxThreatConfidence".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.threat.confidence",
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
                            "convert_cef_extensions_InfobloxThreatConfidence_to_long",
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
                let _cond = { event.get_str("cef.extensions.InfobloxThreatLevel") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.InfobloxThreatLevel") {
                            if let Some(val) = event.get("cef.extensions.InfobloxThreatLevel") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.InfobloxThreatLevel".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "infoblox_threat_defense.event.infoblox.threat.level",
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
                            "convert_cef_extensions_InfobloxThreatLevel_to_long",
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
                if event.has_value("cef.extensions.InfobloxThreatProperty") {
                    event.rename(
                        "cef.extensions.InfobloxThreatProperty",
                        "infoblox_threat_defense.event.infoblox.threat.property",
                    )?;
                }
                if event.has_value("cef.extensions.message") {
                    event.rename(
                        "cef.extensions.message",
                        "infoblox_threat_defense.event.message",
                    )?;
                }
                if event.has_value("cef.extensions.sourceMacAddress") {
                    event.rename(
                        "cef.extensions.sourceMacAddress",
                        "infoblox_threat_defense.event.source.mac_address",
                    )?;
                }
                let _cond = { event.get_str("cef.extensions.sourcePort") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cef.extensions.sourcePort") {
                            if let Some(val) = event.get("cef.extensions.sourcePort") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cef.extensions.sourcePort".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("infoblox_threat_defense.event.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_cef_extensions_sourcePort_to_long",
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
                if event.has_value("cef.extensions.sourceUserName") {
                    event.rename(
                        "cef.extensions.sourceUserName",
                        "infoblox_threat_defense.event.source.user_name",
                    )?;
                }
                if let Some(v) = event
                    .get("infoblox_threat_defense.event.source.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.name", v)?;
                }
                let _cond = { event.has_value("infoblox_threat_defense.event.source.user_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("infoblox_threat_defense.event.source.user_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline_rpz"
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
                event.remove("infoblox_threat_defense.event.application_protocol");
                event.remove("infoblox_threat_defense.event.destination.address");
                event.remove("infoblox_threat_defense.event.device.address");
                event.remove("infoblox_threat_defense.event.device.event_class_id");
                event.remove("infoblox_threat_defense.event.device.product");
                event.remove("infoblox_threat_defense.event.device.vendor");
                event.remove("infoblox_threat_defense.event.device.version");
                event.remove("infoblox_threat_defense.event.infoblox.b1.region");
                event.remove("infoblox_threat_defense.event.infoblox.b1.src_os_version");
                event.remove("infoblox_threat_defense.event.infoblox.b1.threat.indicator");
                event.remove("infoblox_threat_defense.event.infoblox.dns_q.class");
                event.remove("infoblox_threat_defense.event.infoblox.dns_q.type");
                event.remove("infoblox_threat_defense.event.infoblox.dns_r_code");
                event.remove("infoblox_threat_defense.event.infoblox.event.occurred_time");
                event.remove("infoblox_threat_defense.event.infoblox.host_id");
                event.remove("infoblox_threat_defense.event.infoblox.insight.id");
                event.remove("infoblox_threat_defense.event.infoblox.lease.op");
                event.remove("infoblox_threat_defense.event.infoblox.on_prem_host_name");
                event.remove("infoblox_threat_defense.event.infoblox.resource.id");
                event.remove("infoblox_threat_defense.event.infoblox.rpz.rule");
                event.remove("infoblox_threat_defense.event.infoblox.service_id");
                event.remove("infoblox_threat_defense.event.message");
                event.remove("infoblox_threat_defense.event.source.address");
                event.remove("infoblox_threat_defense.event.source.mac_address");
                event.remove("infoblox_threat_defense.event.source.port");
                event.remove("infoblox_threat_defense.event.source.user_name");
                event.remove("infoblox_threat_defense.event.transport_protocol");
            }

            event.remove("cef");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
                        "Processor '{}' {}failed with message '{}'",
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
                                "with tag '{}' ",
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
