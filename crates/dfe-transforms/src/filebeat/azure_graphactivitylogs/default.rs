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

            if event.has_value("azure") {
                event.rename("azure", "azure-eventhub")?;
            }

            event.remove("routing.category");

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

            parse_json_field(event, "event.original", "azure.graphactivitylogs")?;

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\n\nctx.azure['graphactivitylogs'] = keysToSnakeCase(ctx.azure.graphactivitylogs);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\n\nctx.azure['graphactivitylogs'] = keysToSnakeCase(ctx.azure.graphactivitylogs);\n"#
                ),
            )?;

            let _cond = { event.has_value("azure.graphactivitylogs.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("azure.graphactivitylogs.time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.graphactivitylogs.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("azure.graphactivitylogs.time");

            let _cond = { event.has_value("azure.graphactivitylogs.properties.token_issued_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("azure.graphactivitylogs.properties.token_issued_at")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "M/d/yyyy h:mm:ss a XXX",
                                "M/d/yyyy h:mm:ss a",
                                "M/d/yyyy H:mm:ss",
                                "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "azure.graphactivitylogs.properties.token_issued_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "azure.graphactivitylogs.properties.token_issued_at"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date-token_issued_at")?;
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

            if event.has_value("azure.graphactivitylogs.resource_id") {
                event.rename("azure.graphactivitylogs.resource_id", "azure.resource_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.graphactivitylogs.caller_ip_address") {
                    if let Some(val) = event.get("azure.graphactivitylogs.caller_ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.graphactivitylogs.caller_ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert-caller_ip_address",
                )?;
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("azure.graphactivitylogs.caller_ip_address") {
                        event.rename(
                            "azure.graphactivitylogs.caller_ip_address",
                            "source.address",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "rename")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "rename-caller_ip_address",
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
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("source.ip") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("azure.graphactivitylogs.properties.ip_address") {
                        if let Some(val) =
                            event.get("azure.graphactivitylogs.properties.ip_address")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "azure.graphactivitylogs.properties.ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert-properties_ip_address",
                    )?;
                    let _cond = { !event.has_value("source.address") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("azure.graphactivitylogs.properties.ip_address") {
                                event.rename(
                                    "azure.graphactivitylogs.properties.ip_address",
                                    "source.address",
                                )?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "rename")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "rename-properties_ip_address",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.remove("azure.graphactivitylogs.caller_ip_address");
                event.remove("azure.graphactivitylogs.properties.ip_address");
            }

            let v = json!(
                event
                    .get("source.ip")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.ip", v)?;
            }

            if let Some(v) = event
                .get("azure.graphactivitylogs.properties.service_principal_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.id", v)?;
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if let Some(v) = event
                    .get("azure.graphactivitylogs.properties.service_principal_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            if let Some(v) = event
                .get("azure.graphactivitylogs.properties.c_device_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if let Some(v) = event
                .get("azure.graphactivitylogs.properties.c_sid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("session.id", v)?;
            }

            if let Some(v) = event
                .get("azure.graphactivitylogs.properties.sign_in_activity_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("token.id", v)?;
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

            if event.has_value("azure.graphactivitylogs.level") {
                if let Some(val) = event.get("azure.graphactivitylogs.level") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.graphactivitylogs.level".into(),
                            message,
                        }
                    })?;
                    event.set("log.level", converted)?;
                }
            }

            event.remove("azure.graphactivitylogs.level");

            let _cond = { event.has_value("azure.graphactivitylogs.duration_ms") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("azure.graphactivitylogs.duration_ms") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.graphactivitylogs.duration_ms".into(),
                                message,
                            }
                        })?;
                        event.set("event.duration", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("azure.graphactivitylogs.properties.duration_ms")
                    && !event.has_value("event.duration")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("azure.graphactivitylogs.properties.duration_ms") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.graphactivitylogs.properties.duration_ms".into(),
                                message,
                            }
                        })?;
                        event.set("event.duration", converted)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.event.duration!= null) {\n  ctx.event.duration = ctx.event.duration * 1000000;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.event.duration!= null) {\n  ctx.event.duration = ctx.event.duration * 1000000;\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("event.duration") };
            if _cond {
                event.remove("azure.graphactivitylogs.duration_ms");
                event.remove("azure.graphactivitylogs.properties.duration_ms");
            }

            if event.has_value("azure.graphactivitylogs.location") {
                event.rename(
                    "azure.graphactivitylogs.location",
                    "destination.geo.region_name",
                )?;
            }

            let _cond = {
                !event.has_value("destination.geo.region_name")
                    && event.has_value("azure.graphactivitylogs.properties.location")
            };
            if _cond {
                event.rename(
                    "azure.graphactivitylogs.properties.location",
                    "destination.geo.region_name",
                )?;
            }

            let _cond = { event.has_value("destination.geo.region_name") };
            if _cond {
                event.remove("azure.graphactivitylogs.location");
                event.remove("azure.graphactivitylogs.properties.location");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.graphactivitylogs.operation_name") {
                    if let Some(val) = event.get("azure.graphactivitylogs.operation_name") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.graphactivitylogs.operation_name".into(),
                                message,
                            }
                        })?;
                        event.set("event.action", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert-operation_name")?;
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

            if event.has_value("azure.graphactivitylogs.tenant_id") {
                event.rename("azure.graphactivitylogs.tenant_id", "azure.tenant_id")?;
            }

            let _cond = { !event.has_value("azure.tenant_id") };
            if _cond {
                if event.has_value("azure.graphactivitylogs.properties.tenant_id") {
                    event.rename(
                        "azure.graphactivitylogs.properties.tenant_id",
                        "azure.tenant_id",
                    )?;
                }
            }

            let _cond = { event.has_value("azure.tenant_id") };
            if _cond {
                event.remove("azure.graphactivitylogs.tenant_id");
                event.remove("azure.graphactivitylogs.properties.tenant_id");
            }

            if event.has_value("azure.graphactivitylogs.correlation_id") {
                event.rename(
                    "azure.graphactivitylogs.correlation_id",
                    "azure.correlation_id",
                )?;
            }

            if event.has_value("azure.graphactivitylogs.properties.request_id") {
                event.rename(
                    "azure.graphactivitylogs.properties.request_id",
                    "http.request.id",
                )?;
            }

            if event.has_value("azure.graphactivitylogs.properties.request_method") {
                event.rename(
                    "azure.graphactivitylogs.properties.request_method",
                    "http.request.method",
                )?;
            }

            let _cond = { event.has_value("azure.graphactivitylogs.properties.request_uri") };
            if _cond {
                uri_parts(
                    event,
                    "azure.graphactivitylogs.properties.request_uri",
                    "url",
                    true,
                    false,
                )?;
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                event.remove("azure.graphactivitylogs.properties.request_uri");
            }

            if event.has_value("azure.graphactivitylogs.properties.response_size_bytes") {
                event.rename(
                    "azure.graphactivitylogs.properties.response_size_bytes",
                    "http.response.bytes",
                )?;
            }

            if event.has_value("azure.graphactivitylogs.properties.response_status_code") {
                event.rename(
                    "azure.graphactivitylogs.properties.response_status_code",
                    "http.response.status_code",
                )?;
            }

            let _cond = {
                event.has_value("azure.graphactivitylogs.properties.roles")
                    && event.get_str("azure.graphactivitylogs.properties.roles") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("azure.graphactivitylogs.properties.roles") {
                        let mut parts: Vec<Value> = s.split(" ").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "azure.graphactivitylogs.properties.roles",
                            Value::Array(parts),
                        )?;
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
                event.has_value("azure.graphactivitylogs.properties.scopes")
                    && event.get_str("azure.graphactivitylogs.properties.scopes") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("azure.graphactivitylogs.properties.scopes") {
                        let mut parts: Vec<Value> = s.split(" ").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "azure.graphactivitylogs.properties.scopes",
                            Value::Array(parts),
                        )?;
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
                event.has_value("azure.graphactivitylogs.properties.wids")
                    && event.get_str("azure.graphactivitylogs.properties.wids") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("azure.graphactivitylogs.properties.wids") {
                        let mut parts: Vec<Value> = s.split(" ").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "azure.graphactivitylogs.properties.wids",
                            Value::Array(parts),
                        )?;
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
                event.has_value("azure.graphactivitylogs.properties.user_agent")
                    && event.get_str("azure.graphactivitylogs.properties.user_agent") != Some("")
            };
            if _cond {
                if let Some(ua_str) =
                    event.get_string("azure.graphactivitylogs.properties.user_agent")
                {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
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

            let _cond = { event.has_value("user_agent") };
            if _cond {
                event.remove("azure.graphactivitylogs.properties.user_agent");
            }

            let _cond = {
                event.has_value("azure.graphactivitylogs.properties.user_id")
                    && event.get_str("azure.graphactivitylogs.properties.user_id") != Some("")
            };
            if _cond {
                event.rename("azure.graphactivitylogs.properties.user_id", "user.id")?;
            }

            let v = json!(
                event
                    .get("user.id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.user.id", v)?;
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

            let _cond = { event.has_value("azure.graphactivitylogs.properties._billed_size") };
            if _cond {
                event.rename(
                    "azure.graphactivitylogs.properties._billed_size",
                    "azure.graphactivitylogs.properties.billed_size",
                )?;
            }

            let _cond = { event.has_value("azure.graphactivitylogs.properties._is_billable") };
            if _cond {
                event.rename(
                    "azure.graphactivitylogs.properties._is_billable",
                    "azure.graphactivitylogs.properties.is_billable",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.graphactivitylogs.properties.is_billable") {
                    if let Some(val) = event.get("azure.graphactivitylogs.properties.is_billable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.graphactivitylogs.properties.is_billable".into(),
                                message,
                            }
                        })?;
                        event.set("azure.graphactivitylogs.properties.is_billable", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert-properties_is_billable",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.graphactivitylogs.properties.client_auth_method") {
                    if let Some(val) =
                        event.get("azure.graphactivitylogs.properties.client_auth_method")
                    {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.graphactivitylogs.properties.client_auth_method"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "azure.graphactivitylogs.properties.client_auth_method",
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
                    "convert-properties_client_auth_method",
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

            let _cond = { event.has_value("azure.graphactivitylogs.properties.time_generated") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("azure.graphactivitylogs.properties.time_generated")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "M/d/yyyy h:mm:ss a XXX",
                                "M/d/yyyy h:mm:ss a",
                                "M/d/yyyy H:mm:ss",
                                "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("azure.graphactivitylogs.properties.time_generated", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "azure.graphactivitylogs.properties.time_generated"
                                        .into(),
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
                        "date-properties_time_generated",
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

            event.append_unique("event.category", json!("api"))?;

            let _cond = {
                !event.has_value("http.request.method")
                    || event.get_str("http.request.method") == Some("GET")
                    || event.get_str("http.request.method") == Some("HEAD")
            };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("http.request.method") == Some("POST") };
            if _cond {
                event.append_unique("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get_str("http.request.method") == Some("PATCH")
                    || event.get_str("http.request.method") == Some("PUT")
            };
            if _cond {
                event.append_unique("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("http.request.method") == Some("DELETE") };
            if _cond {
                event.append_unique("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.get_i64("http.response.status_code") == Some(401)
                    || event.get_i64("http.response.status_code") == Some(403)
            };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = {
                event.has_value("http.response.status_code")
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n >= 200)
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n < 300)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("http.response.status_code")
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n >= 400)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))
                    // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))",
                                [
                                    ("azure_subscription_id", "azure.subscription_id"),
                                    ("azure_resource_group", "azure.resource.group"),
                                    ("azure_resource_provider", "azure.resource.provider"),
                                    ("azure_resource_namespace", "azure.resource.namespace"),
                                    (
                                        "azure_resource_authorization_rule",
                                        "azure.resource.authorization_rule"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))",
                                [
                                    ("azure_subscription_id", "azure.subscription_id"),
                                    ("azure_resource_group", "azure.resource.group"),
                                    ("azure_resource_provider", "azure.resource.provider"),
                                    ("azure_resource_namespace", "azure.resource.namespace"),
                                    (
                                        "azure_resource_authorization_rule",
                                        "azure.resource.authorization_rule"
                                    )
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                        // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                        // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /providers/(?P<azure_resource_provider>(?:.+))
                        // Grok pattern: /PROVIDERS/(?P<azure_resource_provider>(?:.+))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/providers/(?P<azure_resource_provider>(?:.+))",
                                    [("azure_resource_provider", "azure.resource.provider")]
                                ),
                                cached_grok_mapped!(
                                    "/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                    [("azure_resource_provider", "azure.resource.provider")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                        // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_provider", "azure.resource.provider")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_provider", "azure.resource.provider")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))
                        // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                        // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))",
                                    [("azure_subscription_id", "azure.subscription_id")]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))",
                                    [("azure_subscription_id", "azure.subscription_id")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            if event.has_value("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }
            // End nested pipeline: "azure-shared-pipeline"

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("azure.graphactivitylogs.properties.request_uri") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("azure.graphactivitylogs.properties.time_generated") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("azure.tenant_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("http.request.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let v = json!(
                event
                    .get("azure.tenant_id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("cloud.account.id", v)?;
            }

            let v = json!(
                event
                    .get("destination.geo.region_name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("cloud.region", v)?;
            }

            let v = json!("Microsoft Graph");
            if !painless_is_empty_value(&v) {
                event.set("cloud.service.name", v)?;
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
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

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
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

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
