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

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.append_unique(
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

            if event.has_value("json.Alert Id") {
                event.rename("json.Alert Id", "tanium.threat_response.alert_id")?;
            }

            let _cond = {
                event.has_value("tanium.threat_response.alert_id")
                    && event.get_str("tanium.threat_response.alert_id") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("json.Intel Id") {
                event.rename("json.Intel Id", "tanium.threat_response.intel_id")?;
            }

            if event.has_value("json.Intel Type") {
                event.rename("json.Intel Type", "tanium.threat_response.intel_type")?;
            }

            if event.has_value("json.Intel Name") {
                event.rename("json.Intel Name", "tanium.threat_response.intel_name")?;
            }

            if event.has_value("json.Intel Labels") {
                event.rename("json.Intel Labels", "tanium.threat_response.intel_labels")?;
            }

            if event.has_value("json.Impact Score") {
                event.rename("json.Impact Score", "tanium.threat_response.impact_score")?;
            }

            if event.has_value("json.Link") {
                event.rename("json.Link", "tanium.threat_response.link")?;
            }

            let _cond = {
                event.has_value("json.updatedAt") && event.get_str("json.updatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedAt") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss.SX",
                                "yyyy-MM-dd'T'HH:mm:ss.SZ",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("tanium.threat_response.updated_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updatedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_updatedat")?;
                    event.append_unique(
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

            if event.has_value("json.id") {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        }
                    })?;
                    event.set("json.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("json.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tanium.threat_response.id", v)?;
            }

            if let Some(v) = event
                .get("json.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.action") {
                event.rename("json.action", "tanium.threat_response.action")?;
            }

            if let Some(v) = event
                .get("tanium.threat_response.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("json.userId") {
                if let Some(val) = event.get("json.userId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.userId".into(),
                            message,
                        }
                    })?;
                    event.set("tanium.threat_response.user.id", converted)?;
                }
            }

            if event.has_value("json.userName") {
                event.rename("json.userName", "tanium.threat_response.user.name")?;
            }

            if event.has_value("json.table") {
                event.rename("json.table", "tanium.threat_response.table")?;
            }

            if event.has_value("json.rowId") {
                if let Some(val) = event.get("json.rowId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.rowId".into(),
                            message,
                        }
                    })?;
                    event.set("tanium.threat_response.row_id", converted)?;
                }
            }

            if event.has_value("json.revision") {
                event.rename("json.revision", "tanium.threat_response.revision")?;
            }

            let _cond = { event.get("json.state").is_some_and(|v| v.is_object()) };
            if _cond {
                event.rename("json.state", "tanium.threat_response.state")?;
            }

            let _cond =
                { event.has_value("json.state") && event.get_str("json.state") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "json.state", "tanium.threat_response.state")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_json_state")?;
                    event.append_unique(
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

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif(ctx.tanium?.threat_response?.state != null) {\n  ctx.tanium.threat_response.state = keysToSnakeCase(ctx.tanium.threat_response.state);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif(ctx.tanium?.threat_response?.state != null) {\n  ctx.tanium.threat_response.state = keysToSnakeCase(ctx.tanium.threat_response.state);\n}\n"#
                ),
            )?;

            if let Some(v) = event
                .get("tanium.threat_response.state.target.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
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

            let _cond = {
                event.has_value("json.createdAt") && event.get_str("json.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss.SX",
                                "yyyy-MM-dd'T'HH:mm:ss.SZ",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("tanium.threat_response.created_at", parsed)?
                            }
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
                    event.set("_ingest.on_failure_processor_tag", "date_json_createdat")?;
                    event.append_unique(
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

            let _cond = { event.get_str("json.Computer IP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Computer IP") {
                        if let Some(val) = event.get("json.Computer IP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Computer IP".into(),
                                    message,
                                }
                            })?;
                            event.set("tanium.threat_response.computer.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_computer_ip",
                    )?;
                    event.append_unique(
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

            if let Some(v) = event
                .get("tanium.threat_response.computer.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.Computer Name") {
                event.rename("json.Computer Name", "tanium.threat_response.computer.name")?;
            }

            if let Some(v) = event
                .get("tanium.threat_response.computer.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
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

            if event.has_value("json.Event Id") {
                event.rename("json.Event Id", "tanium.threat_response.event.id")?;
            }

            if let Some(v) = event
                .get("tanium.threat_response.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.Event Name") {
                event.rename("json.Event Name", "tanium.threat_response.event.name")?;
            }

            if event.has_value("json.Priority") {
                event.rename("json.Priority", "tanium.threat_response.priority")?;
            }

            if event.has_value("json.Severity") {
                event.rename("json.Severity", "tanium.threat_response.severity")?;
            }

            let _cond = { event.has_value("json.Timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("tanium.threat_response.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_timestamp")?;
                    event.append_unique(
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

            if let Some(v) = event
                .get("tanium.threat_response.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.User Domain") {
                event.rename("json.User Domain", "tanium.threat_response.user.domain")?;
            }

            if let Some(v) = event
                .get("tanium.threat_response.user.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if event.has_value("json.User Id") {
                if let Some(val) = event.get("json.User Id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.User Id".into(),
                            message,
                        }
                    })?;
                    event.set("tanium.threat_response.user.id", converted)?;
                }
            }

            if event.has_value("json.User Name") {
                event.rename("json.User Name", "tanium.threat_response.user.name")?;
            }

            if let Some(v) = event
                .get("tanium.threat_response.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("tanium.threat_response.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.Other Parameters") {
                    if let Some(kv_str) = event.get_string("json.Other Parameters") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("\\|").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "json.Other Parameters".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("tanium.threat_response.other_parameters.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "kv_json_other_parameters",
                )?;
                event.append_unique(
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

            let _cond = { event.has_value("tanium.threat_response.other_parameters.payload") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.tanium.threat_response.other_parameters.payload_json = ctx.tanium.threat_response.other_parameters.payload.decodeBase64()
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.tanium.threat_response.other_parameters.payload_json = ctx.tanium.threat_response.other_parameters.payload.decodeBase64()"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_decode_base64_json_field",
                    )?;
                    event.append_unique(
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

            let _cond = {
                event
                    .get("tanium.threat_response.other_parameters.payload_json")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "tanium.threat_response.other_parameters.payload_json",
                        "tanium.threat_response.match_details",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_tanium_threat_response_other_parameters_payload_json",
                    )?;
                    event.append_unique(
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

            event.remove("tanium.threat_response.other_parameters.payload");
            event.remove("tanium.threat_response.other_parameters.payload_json");

            let _cond = {
                event
                    .get("json.Match Details")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: ctx.tanium.threat_response.match_details = ctx.tanium.threat_response.match_details ?: [:];\nctx.tanium.threat_response.match_details.putAll(ctx.json['Match Details']);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.tanium.threat_response.match_details = ctx.tanium.threat_response.match_details ?: [:];\nctx.tanium.threat_response.match_details.putAll(ctx.json['Match Details']);\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("tanium.threat_response.match_details")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Begin nested pipeline: "match_details"
                // Painless script, resolved to its runners at generation time
                // Source: boolean dropEmptyMapsAndLists(Object object) {\n  if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyMapsAndLists(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyMapsAndLists(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyMapsAndLists(ctx.tanium.threat_response.match_details);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        empty_collections: true,
                        prune_lists: true,
                        ..DropPolicy::none()
                    },
                    Some("tanium.threat_response.match_details"),
                );
                if event.has_value(
                    "tanium.threat_response.match_details.match.properties.file.fullpath",
                ) {
                    event.rename(
                        "tanium.threat_response.match_details.match.properties.file.fullpath",
                        "tanium.threat_response.match_details.match.properties.file.full_path",
                    )?;
                }
                if event.has_value(
                    "tanium.threat_response.match_details.match.properties.parent.file.fullpath",
                ) {
                    event.rename("tanium.threat_response.match_details.match.properties.parent.file.fullpath", "tanium.threat_response.match_details.match.properties.parent.file.full_path")?;
                }
                if event.has_value("tanium.threat_response.match_details.finding.finding_id") {
                    event.rename(
                        "tanium.threat_response.match_details.finding.finding_id",
                        "tanium.threat_response.match_details.finding.id",
                    )?;
                }
                if event.has_value("tanium.threat_response.match_details.match.properties.parent.parent.file.fullpath") {
                event.rename("tanium.threat_response.match_details.match.properties.parent.parent.file.fullpath", "tanium.threat_response.match_details.match.properties.parent.parent.file.full_path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("tanium.threat_response.match_details.finding.system_info.os")
                    {
                        if let Some(input) = event.get_string(
                            "tanium.threat_response.match_details.finding.system_info.os",
                        ) {
                            // Grok pattern: ^%{DATA:tanium.threat_response.match_details.finding.system_info.os_platform} %{GREEDYDATA:tanium.threat_response.match_details.finding.system_info.os_version}$
                            // Grok pattern: ^%{GREEDYDATA:tanium.threat_response.match_details.system_info.os}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{DATA:tanium.threat_response.match_details.finding.system_info.os_platform} %{GREEDYDATA:tanium.threat_response.match_details.finding.system_info.os_version}$"
                                    ),
                                    cached_grok!(
                                        "^%{GREEDYDATA:tanium.threat_response.match_details.system_info.os}$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_tanium_threat_response_match_details_finding_system_info_os",
                    )?;
                    event.append_unique(
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
                if event.has_value("tanium.threat_response.match_details.finding.system_info.os") {
                    event.rename(
                        "tanium.threat_response.match_details.finding.system_info.os",
                        "tanium.threat_response.match_details.finding.system_info.os.value",
                    )?;
                }
                if event.has_value(
                    "tanium.threat_response.match_details.finding.system_info.os_platform",
                ) {
                    event.rename(
                        "tanium.threat_response.match_details.finding.system_info.os_platform",
                        "tanium.threat_response.match_details.finding.system_info.os.platform",
                    )?;
                }
                if let Some(v) = event
                    .get("tanium.threat_response.match_details.finding.system_info.os.platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("os.platform", v)?;
                }
                if event.has_value(
                    "tanium.threat_response.match_details.finding.system_info.os_version",
                ) {
                    event.rename(
                        "tanium.threat_response.match_details.finding.system_info.os_version",
                        "tanium.threat_response.match_details.finding.system_info.os.version",
                    )?;
                }
                if let Some(v) = event
                    .get("tanium.threat_response.match_details.finding.system_info.os.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("os.version", v)?;
                }
                if event
                    .has_value("tanium.threat_response.match_details.finding.system_info.platform")
                {
                    map_strings(
                        event,
                        "tanium.threat_response.match_details.finding.system_info.platform",
                        "tanium.threat_response.match_details.finding.system_info.platform",
                        str::to_lowercase,
                    )?;
                }
                if let Some(v) = event
                    .get("tanium.threat_response.match_details.finding.system_info.platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("os.type", v)?;
                }
                let _cond = {
                    event.get_str("tanium.threat_response.match_details.finding.system_info.bits")
                        != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "tanium.threat_response.match_details.finding.system_info.bits",
                        ) {
                            if let Some(val) = event.get(
                                "tanium.threat_response.match_details.finding.system_info.bits",
                            ) {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "tanium.threat_response.match_details.finding.system_info.bits".into(),
                message,
                }
                                })?;
                                event.set(
                                    "tanium.threat_response.match_details.finding.system_info.bits",
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
                            "convert_tanium_threat_response_match_details_finding_system_info_bits",
                        )?;
                        event.append_unique(
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
                let _cond = {
                    event
                        .get("tanium.threat_response.match_details.match.contexts")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "tanium.threat_response.match_details.match.contexts",
                        |event| {
                            if event.has_value("_ingest._value.file.uniqueEventId") {
                                event.rename(
                                    "_ingest._value.file.uniqueEventId",
                                    "_ingest._value.file.unique_event_id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event
                        .get("tanium.threat_response.match_details.match.contexts")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "tanium.threat_response.match_details.match.contexts",
                        |event| {
                            if event.has_value("_ingest._value.event.fileCreate.path") {
                                event.rename(
                                    "_ingest._value.event.fileCreate.path",
                                    "_ingest._value.event.file_create.path",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond =
                    { event.has_value("tanium.threat_response.match_details.finding.first_seen") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "tanium.threat_response.match_details.finding.first_seen",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "tanium.threat_response.match_details.finding.first_seen",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                path: "tanium.threat_response.match_details.finding.first_seen".into(),
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
                            "date_tanium_threat_response_match_details_finding_first_seen",
                        )?;
                        event.append_unique(
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
                let _cond =
                    { event.has_value("tanium.threat_response.match_details.finding.last_seen") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) =
                        (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string(
                                "tanium.threat_response.match_details.finding.last_seen",
                            ) {
                                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                    Some(parsed) => event.set(
                                        "tanium.threat_response.match_details.finding.last_seen",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                path: "tanium.threat_response.match_details.finding.last_seen".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                                    }
                                }
                            }
                            Ok(())
                        })()
                    {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_tanium_threat_response_match_details_finding_last_seen",
                        )?;
                        event.append_unique(
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
                let _cond = {
                    event
                        .get("tanium.threat_response.match_details.match.contexts")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("tanium.threat_response.match_details.match.contexts")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.event.timestampMs")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.event.timestampMs", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.event.timestampMs".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_ingest_value_event_timestampms",
                                    )?;
                                    event.append_unique(
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
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "tanium.threat_response.match_details.match.contexts",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("tanium.threat_response.match_details.match.properties.parent.parent.start_time")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.match.properties.parent.parent.start_time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("tanium.threat_response.match_details.match.properties.parent.parent.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "tanium.threat_response.match_details.match.properties.parent.parent.start_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_match_properties_parent_parent_start_time")?;
                        event.append_unique(
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
                let _cond = {
                    event.has_value(
                        "tanium.threat_response.match_details.match.properties.parent.start_time",
                    )
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.match.properties.parent.start_time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("tanium.threat_response.match_details.match.properties.parent.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "tanium.threat_response.match_details.match.properties.parent.start_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_match_properties_parent_start_time")?;
                        event.append_unique(
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
                let _cond = {
                    event.has_value(
                        "tanium.threat_response.match_details.match.properties.start_time",
                    )
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "tanium.threat_response.match_details.match.properties.start_time",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("tanium.threat_response.match_details.match.properties.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "tanium.threat_response.match_details.match.properties.start_time".into(),
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
                            "date_tanium_threat_response_match_details_match_properties_start_time",
                        )?;
                        event.append_unique(
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
                if event.has_value("tanium.threat_response.match_details.finding.whats") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("tanium.threat_response.match_details.finding.whats")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "match_details_whats"
                                event.rename("_ingest._value", "_what")?;
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.domain") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.domain", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.domain")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.user.user.domain") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.user.user.domain", "_what.artifact_activity.acting_artifact.process.parent.process.user.domain")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.domain") {
                event.rename("_what.artifact_activity.acting_artifact.process.user.user.domain", "_what.artifact_activity.acting_artifact.process.user.domain")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.name") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.name", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.name")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.user.user.name") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.user.user.name", "_what.artifact_activity.acting_artifact.process.parent.process.user.name")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.name") {
                event.rename("_what.artifact_activity.acting_artifact.process.user.user.name", "_what.artifact_activity.acting_artifact.process.user.name")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.user_id") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.user_id", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.id")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.user.user.user_id") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.user.user.user_id", "_what.artifact_activity.acting_artifact.process.parent.process.user.id")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.user_id") {
                event.rename("_what.artifact_activity.acting_artifact.process.user.user.user_id", "_what.artifact_activity.acting_artifact.process.user.id")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.group_id") {
                event.rename("_what.artifact_activity.acting_artifact.process.user.user.group_id", "_what.artifact_activity.acting_artifact.process.user.group_id")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.path") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.path", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.path")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.file.file.path") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.file.file.path", "_what.artifact_activity.acting_artifact.process.parent.process.file.path")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.file.file.path") {
                event.rename("_what.artifact_activity.acting_artifact.process.file.file.path", "_what.artifact_activity.acting_artifact.process.file.path")?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.hash.md5") {
                event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.hash.md5", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.hash.md5")?;
                }
                                let _cond = {
                                    event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time")
                                };
                                if _cond {
                                    // on_failure: 1 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) = event.get_as_string("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.set("_ingest.on_failure_processor_tag", "date_what_artifact_activity_acting_artifact_process_parent_process_parent_process_start_time")?;
                                        event.append_unique(
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                }
                                let _cond = {
                                    event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.start_time")
                                };
                                if _cond {
                                    // on_failure: 1 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) = event.get_as_string("_what.artifact_activity.acting_artifact.process.parent.process.start_time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("_what.artifact_activity.acting_artifact.process.parent.process.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_what.artifact_activity.acting_artifact.process.parent.process.start_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.set("_ingest.on_failure_processor_tag", "date_what_artifact_activity_acting_artifact_process_parent_process_start_time")?;
                                        event.append_unique(
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                }
                                let _cond = {
                                    event.has_value("_what.artifact_activity.acting_artifact.process.start_time")
                                };
                                if _cond {
                                    // on_failure: 1 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) = event.get_as_string("_what.artifact_activity.acting_artifact.process.start_time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("_what.artifact_activity.acting_artifact.process.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_what.artifact_activity.acting_artifact.process.start_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.set("_ingest.on_failure_processor_tag", "date_what_artifact_activity_acting_artifact_process_start_time")?;
                                        event.append_unique(
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.handles") {
                foreach_array(event, "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.handles", |event| {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })?;
                }
                                if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.handles") {
                foreach_array(event, "_what.artifact_activity.acting_artifact.process.parent.process.handles", |event| {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })?;
                }
                                if event.has_value(
                                    "_what.artifact_activity.acting_artifact.process.handles",
                                ) {
                                    foreach_array(
                                        event,
                                        "_what.artifact_activity.acting_artifact.process.handles",
                                        |event| {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "string")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value".into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event.set("_ingest._value", converted)?;
                                                }
                                            }
                                            Ok(())
                                        },
                                    )?;
                                }
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value(
                                        "_what.artifact_activity.acting_artifact.is_intel_target",
                                    ) {
                                        if let Some(val) = event.get("_what.artifact_activity.acting_artifact.is_intel_target") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_what.artifact_activity.acting_artifact.is_intel_target".into(),
                message,
                })?;
                event.set("_what.artifact_activity.acting_artifact.is_intel_target", converted)?;
                }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_what_artifact_activity_acting_artifact_is_intel_target")?;
                                    event.append_unique(
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
                                if event.has_value("_what.artifact_activity.relevant_actions") {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject = event
                                            .get("_what.artifact_activity.relevant_actions")
                                            .cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
                                            _ => Vec::new(),
                                        };
                                        if !entries.is_empty() {
                                            // A NESTED loop borrows the same slots, so the enclosing
                                            // entry is saved and put back afterwards.
                                            let enclosing = event.get("_ingest._value").cloned();
                                            let enclosing_key = event.get("_ingest._key").cloned();
                                            let mut list = Vec::with_capacity(entries.len());
                                            let mut fields = Map::new();
                                            for (key, item) in entries {
                                                if let Some(key) = key.as_deref() {
                                                    event.set(
                                                        "_ingest._key",
                                                        Value::String(key.to_string()),
                                                    )?;
                                                }
                                                event.set("_ingest._value", item)?;
                                                // Begin nested pipeline: "match_details_whats_actions"
                                                event.rename("_ingest._value", "_action")?;
                                                // on_failure: 1 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if event.has_value("_action.verb") {
                                                        if let Some(val) = event.get("_action.verb")
                                                        {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                                                                            path: "_action.verb"
                                                                                .into(),
                                                                            message,
                                                                        }
                                                                    })?;
                                                            event.set("_action.verb", converted)?;
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "convert",
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_tag",
                                                        "convert_action_verb",
                                                    )?;
                                                    event.append_unique(
                                                        "error.message",
                                                        json!(
                                                            event
                                                                .get("_ingest.on_failure_message")
                                                                .map_or_else(
                                                                    String::new,
                                                                    template_to_string
                                                                )
                                                        ),
                                                    )?;
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                let _cond = {
                                                    event.has_value("_action.tanium_recorder_context.event.timestamp_ms")
                                                };
                                                if _cond {
                                                    // on_failure: 1 handler(s)
                                                    if let Err(err) = (|| -> Result<()> {
                                                        if let Some(date_str) = event.get_as_string("_action.tanium_recorder_context.event.timestamp_ms") {
                match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                Some(parsed) => event.set("_action.tanium_recorder_context.event.timestamp_ms", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_action.tanium_recorder_context.event.timestamp_ms".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                                                        Ok(())
                                                    })(
                                                    ) {
                                                        event.set(
                                                            "_ingest.on_failure_message",
                                                            err.to_string(),
                                                        )?;
                                                        event.set(
                                                            "_ingest.on_failure_processor_type",
                                                            "date",
                                                        )?;
                                                        event.set("_ingest.on_failure_processor_tag", "date_action_tanium_recorder_context_event_timestamp_ms")?;
                                                        event.append_unique(
                                                            "error.message",
                                                            json!(
                                                                event
                                                                    .get(
                                                                        "_ingest.on_failure_message"
                                                                    )
                                                                    .map_or_else(
                                                                        String::new,
                                                                        template_to_string
                                                                    )
                                                            ),
                                                        )?;
                                                        event.remove("_ingest.on_failure_message");
                                                        event.remove(
                                                            "_ingest.on_failure_processor_type",
                                                        );
                                                        event.remove(
                                                            "_ingest.on_failure_processor_tag",
                                                        );
                                                        if event
                                                            .get_object("_ingest")
                                                            .is_some_and(|m| m.is_empty())
                                                        {
                                                            event.remove("_ingest");
                                                        }
                                                    }
                                                }
                                                let _cond = {
                                                    event.has_value(
                                                        "_action.target.file.modification_time",
                                                    )
                                                };
                                                if _cond {
                                                    // on_failure: 1 handler(s)
                                                    if let Err(err) = (|| -> Result<()> {
                                                        if let Some(date_str) = event.get_as_string(
                                                            "_action.target.file.modification_time",
                                                        ) {
                                                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("_action.target.file.modification_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_action.target.file.modification_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                                                        }
                                                        Ok(())
                                                    })(
                                                    ) {
                                                        event.set(
                                                            "_ingest.on_failure_message",
                                                            err.to_string(),
                                                        )?;
                                                        event.set(
                                                            "_ingest.on_failure_processor_type",
                                                            "date",
                                                        )?;
                                                        event.set("_ingest.on_failure_processor_tag", "date_action_target_file_modification_time")?;
                                                        event.append_unique(
                                                            "error.message",
                                                            json!(
                                                                event
                                                                    .get(
                                                                        "_ingest.on_failure_message"
                                                                    )
                                                                    .map_or_else(
                                                                        String::new,
                                                                        template_to_string
                                                                    )
                                                            ),
                                                        )?;
                                                        event.remove("_ingest.on_failure_message");
                                                        event.remove(
                                                            "_ingest.on_failure_processor_type",
                                                        );
                                                        event.remove(
                                                            "_ingest.on_failure_processor_tag",
                                                        );
                                                        if event
                                                            .get_object("_ingest")
                                                            .is_some_and(|m| m.is_empty())
                                                        {
                                                            event.remove("_ingest");
                                                        }
                                                    }
                                                }
                                                let _cond =
                                                    { event.has_value("_action.timestamp") };
                                                if _cond {
                                                    // on_failure: 1 handler(s)
                                                    if let Err(err) = (|| -> Result<()> {
                                                        if let Some(date_str) =
                                                            event.get_as_string("_action.timestamp")
                                                        {
                                                            match parse_date_out(
                                                                &date_str,
                                                                &["ISO8601"],
                                                                None,
                                                                None,
                                                            ) {
                                                                Some(parsed) => event.set(
                                                                    "_action.timestamp",
                                                                    parsed,
                                                                )?,
                                                                None => {
                                                                    return Err(TransformError::ParseError {
                path: "_action.timestamp".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                                                                }
                                                            }
                                                        }
                                                        Ok(())
                                                    })(
                                                    ) {
                                                        event.set(
                                                            "_ingest.on_failure_message",
                                                            err.to_string(),
                                                        )?;
                                                        event.set(
                                                            "_ingest.on_failure_processor_type",
                                                            "date",
                                                        )?;
                                                        event.set(
                                                            "_ingest.on_failure_processor_tag",
                                                            "date_action_timestamp",
                                                        )?;
                                                        event.append_unique(
                                                            "error.message",
                                                            json!(
                                                                event
                                                                    .get(
                                                                        "_ingest.on_failure_message"
                                                                    )
                                                                    .map_or_else(
                                                                        String::new,
                                                                        template_to_string
                                                                    )
                                                            ),
                                                        )?;
                                                        event.remove("_ingest.on_failure_message");
                                                        event.remove(
                                                            "_ingest.on_failure_processor_type",
                                                        );
                                                        event.remove(
                                                            "_ingest.on_failure_processor_tag",
                                                        );
                                                        if event
                                                            .get_object("_ingest")
                                                            .is_some_and(|m| m.is_empty())
                                                        {
                                                            event.remove("_ingest");
                                                        }
                                                    }
                                                }
                                                // ignore_failure: true
                                                let _ = (|| -> Result<()> {
                                                    event.append_unique(
                                                        "related.hash",
                                                        json!(
                                                            event
                                                                .get("_action.target.file.hash.md5")
                                                                .map_or_else(
                                                                    String::new,
                                                                    template_to_string
                                                                )
                                                        ),
                                                    )?;
                                                    Ok(())
                                                })(
                                                );
                                                // ignore_failure: true
                                                let _ = (|| -> Result<()> {
                                                    event.append_unique(
                                                        "related.hash",
                                                        json!(
                                                            event
                                                                .get(
                                                                    "_action.target.file.hash.sha1"
                                                                )
                                                                .map_or_else(
                                                                    String::new,
                                                                    template_to_string
                                                                )
                                                        ),
                                                    )?;
                                                    Ok(())
                                                })(
                                                );
                                                // ignore_failure: true
                                                let _ = (|| -> Result<()> {
                                                    event.append_unique("related.hash", json!(event.get("_action.target.file.hash.sha256").map_or_else(String::new, template_to_string)))?;
                                                    Ok(())
                                                })(
                                                );
                                                // on_failure: 1 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if event
                                                        .has_value("_action.target.file.size_bytes")
                                                    {
                                                        if let Some(val) = event
                                                            .get("_action.target.file.size_bytes")
                                                        {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                path: "_action.target.file.size_bytes".into(),
                message,
                }
                                                                    })?;
                                                            event.set(
                                                                "_action.target.file.size_bytes",
                                                                converted,
                                                            )?;
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "convert",
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_tag",
                                                        "convert_action_target_file_size_bytes",
                                                    )?;
                                                    // ignore_failure: true
                                                    let _ = (|| -> Result<()> {
                                                        event.append_unique(
                                                            "error.message",
                                                            json!(
                                                                event
                                                                    .get(
                                                                        "_ingest.on_failure_message"
                                                                    )
                                                                    .map_or_else(
                                                                        String::new,
                                                                        template_to_string
                                                                    )
                                                            ),
                                                        )?;
                                                        Ok(())
                                                    })(
                                                    );
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                if let Some(v) = event.get("_action").cloned() {
                                                    event.set("_ingest._value", v)?;
                                                }
                                                if event.remove("_action").is_none() {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_action".into(),
                                                    });
                                                }
                                                // End nested pipeline: "match_details_whats_actions"
                                                let left = event.remove("_ingest._value");
                                                match key {
                                                    // An entry the body renamed AWAY is gone from the
                                                    // object, which is how a foreach lifts fields up.
                                                    Some(key) => {
                                                        if let Some(value) = left {
                                                            fields.insert(key, value);
                                                        }
                                                    }
                                                    None => list.push(left.unwrap_or(Value::Null)),
                                                }
                                            }
                                            match enclosing {
                                                Some(previous) => {
                                                    event.set("_ingest._value", previous)?;
                                                }
                                                None => {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            if let Some(previous) = enclosing_key {
                                                event.set("_ingest._key", previous)?;
                                            }
                                            event.set(
                                                "_what.artifact_activity.relevant_actions",
                                                if keyed {
                                                    Value::Object(fields)
                                                } else {
                                                    Value::Array(list)
                                                },
                                            )?;
                                        }
                                    }
                                }
                                if let Some(v) = event.get("_what").cloned() {
                                    event.set("_ingest._value", v)?;
                                }
                                if event.remove("_what").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_what".into(),
                                    });
                                }
                                // End nested pipeline: "match_details_whats"
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "tanium.threat_response.match_details.finding.whats",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
                if let Some(v) = event.get("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.size_bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
                }
                if let Some(v) = event.get("tanium.threat_response.match_details.finding.whats.artifact_activity.acting_artifact.process.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pid", v)?;
                }
                let _cond = {
                    event.has_value("error.message") && event.get_str("error.message") != Some("")
                };
                if _cond {
                    event.append_unique("event.kind", json!("pipeline_error"))?;
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
                    event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.hash.md5");
                    event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.hash.sha1");
                    event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.hash.sha256");
                    event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.acting_artifact.process.pid");
                    event.remove(
                        "tanium.threat_response.match_details.finding.system_info.platform",
                    );
                    event.remove(
                        "tanium.threat_response.match_details.finding.system_info.os.platform",
                    );
                    event.remove(
                        "tanium.threat_response.match_details.finding.system_info.os.version",
                    );
                }
                // End nested pipeline: "match_details"
            }

            let _cond =
                { event.has_value("error.message") && event.get_str("error.message") != Some("") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            event.remove("json");

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
                event.remove("tanium.threat_response.id");
                event.remove("tanium.threat_response.action");
                event.remove("tanium.threat_response.state.target.hostname");
                event.remove("tanium.threat_response.user.id");
                event.remove("tanium.threat_response.user.name");
                event.remove("tanium.threat_response.computer.ip");
                event.remove("tanium.threat_response.computer.name");
                event.remove("tanium.threat_response.event.id");
                event.remove("tanium.threat_response.timestamp");
                event.remove("tanium.threat_response.user.domain");
            }

            // Painless script
            // Source: Map truncateMap(Map node, int depth, int maxDepth, List path, List truncations) {\n    if (depth > maxDepth) {\n        truncations.add(String.join(\".\", path));\n        return null; // Return null if max depth is exceeded\n    }\n    for (entry in node.entrySet()) {\n        List newPath = new ArrayList(path);\n        newPath.add(entry.getKey());\n        Object value = entry.getValue();\n        if (value instanceof Map) {\n            entry.setValue(truncateMap((Map) value, depth + 1, maxDepth, newPath, truncations));\n        } else if (value instanceof List) {\n            // A list in a map doesn't add depth - multiple values are treated the same as a single value\n            entry.setValue(truncateList((List) value, depth, maxDepth, newPath, truncations));\n        }\n        // Otherwise it's a plain value, so leave it\n    }\n    return node;\n\n}\nList truncateList(List node, int depth, int maxDepth, List path, List truncations) {\n    List input = new ArrayList(node);\n    node.clear();\n    if (depth > maxDepth) {\n        truncations.add(String.join(\".\", path));\n        return null;\n    }\n    for (item in input) {\n        if (item instanceof Map) {\n            node.add(truncateMap((Map) item, depth + 1, maxDepth, path, truncations));\n        } else if (item instanceof List) {\n            // A list in a list adds depth - but isn't common in our data\n            node.add(truncateList((List) item, depth + 1, maxDepth, path, truncations));\n        } else {\n            node.add(item); // It's a plain value\n        }\n    }\n    return node;\n}\nint maxDepth = 20;\nList truncations = [];\ntruncateMap(ctx, 1, maxDepth, [], truncations);\nctx.tanium.truncations = truncations;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map truncateMap(Map node, int depth, int maxDepth, List path, List truncations) {\n    if (depth > maxDepth) {\n        truncations.add(String.join(\".\", path));\n        return null; // Return null if max depth is exceeded\n    }\n    for (entry in node.entrySet()) {\n        List newPath = new ArrayList(path);\n        newPath.add(entry.getKey());\n        Object value = entry.getValue();\n        if (value instanceof Map) {\n            entry.setValue(truncateMap((Map) value, depth + 1, maxDepth, newPath, truncations));\n        } else if (value instanceof List) {\n            // A list in a map doesn't add depth - multiple values are treated the same as a single value\n            entry.setValue(truncateList((List) value, depth, maxDepth, newPath, truncations));\n        }\n        // Otherwise it's a plain value, so leave it\n    }\n    return node;\n\n}\nList truncateList(List node, int depth, int maxDepth, List path, List truncations) {\n    List input = new ArrayList(node);\n    node.clear();\n    if (depth > maxDepth) {\n        truncations.add(String.join(\".\", path));\n        return null;\n    }\n    for (item in input) {\n        if (item instanceof Map) {\n            node.add(truncateMap((Map) item, depth + 1, maxDepth, path, truncations));\n        } else if (item instanceof List) {\n            // A list in a list adds depth - but isn't common in our data\n            node.add(truncateList((List) item, depth + 1, maxDepth, path, truncations));\n        } else {\n            node.add(item); // It's a plain value\n        }\n    }\n    return node;\n}\nint maxDepth = 20;\nList truncations = [];\ntruncateMap(ctx, 1, maxDepth, [], truncations);\nctx.tanium.truncations = truncations;\n"#
                ),
            )?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
