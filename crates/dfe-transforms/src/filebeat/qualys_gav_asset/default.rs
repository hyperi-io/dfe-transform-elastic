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

            if event.has_value("interval_id") {
                event.rename("interval_id", "qualys_gav.asset.interval_id")?;
            }

            let _cond = {
                event.has_value("interval_start") && event.get_str("interval_start") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("interval_start") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("qualys_gav.asset.interval_start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "interval_start".into(),
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
                        "date_interval_start_718f7fe5",
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
            }

            event.remove("interval_start");

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

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("both")),
                    serde_json::Value::String(s) => s.contains("both"),
                    _ => false,
                })
            };
            if _cond {
                event.append("tags", json!("elastic_cloud_data"))?;
                event.append("tags", json!("provider_cloud_data"))?;
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("both")),
                    serde_json::Value::String(s) => s.contains("both"),
                    _ => false,
                })
            };
            if _cond {
                // Painless script
                // Source: ctx.tags.remove(ctx.tags.indexOf('both'));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.tags.remove(ctx.tags.indexOf('both'));"#),
                )?;
            }

            let _cond = {
                !event.has_value("cloud")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("elastic_cloud_data"))
                        }
                        serde_json::Value::String(s) => s.contains("elastic_cloud_data"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: ctx.tags.remove(ctx.tags.indexOf('elastic_cloud_data'));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.tags.remove(ctx.tags.indexOf('elastic_cloud_data'));"#),
                )?;
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("provider_cloud_data"))
                    }
                    serde_json::Value::String(s) => s.contains("provider_cloud_data"),
                    _ => false,
                })
            };
            if _cond {
                event.set("_conf.want_provider_cloud", json!(true))?;
            }

            let _cond = {
                event.has_value("tags")
                    && !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("elastic_cloud_data"))
                        }
                        serde_json::Value::String(s) => s.contains("elastic_cloud_data"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("cloud");
                    Ok(())
                })();
            }

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

            parse_json_field(event, "event.original", "json")?;

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.qualys_gav = ctx.qualys_gav ?: [:];\nctx.qualys_gav.asset = ctx.qualys_gav.asset ?: [:];\nif (ctx.json != null) {\n  ctx.qualys_gav.asset.putAll(convertToSnakeCase(ctx.json));\n}\n// Remove json field\nctx.remove('json');\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.qualys_gav = ctx.qualys_gav ?: [:];\nctx.qualys_gav.asset = ctx.qualys_gav.asset ?: [:];\nif (ctx.json != null) {\n  ctx.qualys_gav.asset.putAll(convertToSnakeCase(ctx.json));\n}\n// Remove json field\nctx.remove('json');\n"#
                ),
            )?;

            event.set("observer.vendor", json!("Qualys"))?;

            event.set("observer.product", json!("Global AssetView"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("host"))?;

            event.append("event.type", json!("info"))?;

            let _cond = { event.get_str("qualys_gav.asset.address") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.address") {
                        if let Some(val) = event.get("qualys_gav.asset.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.address".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_address_to_ip_d1d43fcb",
                    )?;
                    if event.remove("qualys_gav.asset.address").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.address".into(),
                        });
                    }
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

            let _cond = { event.has_value("qualys_gav.asset.address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("qualys_gav.asset.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("qualys_gav.asset.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("qualys_gav.asset.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("qualys_gav.asset.agent.connected_from") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.agent.connected_from") {
                        if let Some(val) = event.get("qualys_gav.asset.agent.connected_from") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.agent.connected_from".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.agent.connected_from", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_agent_connected_from_to_ip_1199a23a",
                    )?;
                    if event
                        .remove("qualys_gav.asset.agent.connected_from")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.agent.connected_from".into(),
                        });
                    }
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

            let _cond = { event.has_value("qualys_gav.asset.agent.connected_from") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("qualys_gav.asset.agent.connected_from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.agent.last_activity")
                    && event.get_str("qualys_gav.asset.agent.last_activity") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.agent.last_activity")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.agent.last_activity", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.agent.last_activity".into(),
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
                        "date_agent_last_activity_ebb9b1a2",
                    )?;
                    if event
                        .remove("qualys_gav.asset.agent.last_activity")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.agent.last_activity".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.agent.last_checked_in")
                    && event.get_str("qualys_gav.asset.agent.last_checked_in") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.agent.last_checked_in")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.agent.last_checked_in", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.agent.last_checked_in".into(),
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
                        "date_agent_last_checked_in_6dd93541",
                    )?;
                    if event
                        .remove("qualys_gav.asset.agent.last_checked_in")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.agent.last_checked_in".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.agent.last_inventory")
                    && event.get_str("qualys_gav.asset.agent.last_inventory") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.agent.last_inventory")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.agent.last_inventory", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.agent.last_inventory".into(),
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
                        "date_agent_last_inventory_48fd6be7",
                    )?;
                    if event
                        .remove("qualys_gav.asset.agent.last_inventory")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.agent.last_inventory".into(),
                        });
                    }
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
                { event.get_str("qualys_gav.asset.agent.udc_manifest_assigned") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.agent.udc_manifest_assigned") {
                        if let Some(val) = event.get("qualys_gav.asset.agent.udc_manifest_assigned")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.agent.udc_manifest_assigned".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.agent.udc_manifest_assigned", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_agent_udc_manifest_assigned_to_boolean_7d41636c",
                    )?;
                    if event
                        .remove("qualys_gav.asset.agent.udc_manifest_assigned")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.agent.udc_manifest_assigned".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.agent.error_status") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.agent.error_status") {
                        if let Some(val) = event.get("qualys_gav.asset.agent.error_status") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.agent.error_status".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.agent.error_status", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_agent_error_status_to_boolean_c5eb0130",
                    )?;
                    if event
                        .remove("qualys_gav.asset.agent.error_status")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.agent.error_status".into(),
                        });
                    }
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
                event
                    .get("qualys_gav.asset.sensor")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: ctx.qualys_gav.asset.sensor.values().removeIf(v -> { return v == 0 });\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.qualys_gav.asset.sensor.values().removeIf(v -> { return v == 0 });\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.sensor.last_vmscan")
                    && event.get_str("qualys_gav.asset.sensor.last_vmscan") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_vmscan")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.sensor.last_vmscan", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_vmscan".into(),
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
                        "date_agent_last_vm_scan_fa6518e4",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_vmscan")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_vmscan".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_compliance_scan")
                    && event.get_str("qualys_gav.asset.sensor.last_compliance_scan") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_compliance_scan")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.sensor.last_compliance_scan", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_compliance_scan".into(),
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
                        "date_agent_last_compliance_scan_bce238f9",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_compliance_scan")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_compliance_scan".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_full_scan")
                    && event.get_str("qualys_gav.asset.sensor.last_full_scan") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_full_scan")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.sensor.last_full_scan", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_full_scan".into(),
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
                        "date_agent_last_full_scan_a89be436",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_full_scan")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_full_scan".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_vm_scan_date_scanner")
                    && event.get_str("qualys_gav.asset.sensor.last_vm_scan_date_scanner")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_vm_scan_date_scanner")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.sensor.last_vm_scan_date_scanner", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_vm_scan_date_scanner"
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
                        "date_agent_last_vm_scan_date_scanner_4d20be17",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_vm_scan_date_scanner")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_vm_scan_date_scanner".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.first_easm_scan_date")
                    && event.get_str("qualys_gav.asset.sensor.first_easm_scan_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.first_easm_scan_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.sensor.first_easm_scan_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.first_easm_scan_date".into(),
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
                        "date_agent_first_easm_scan_date_a348ee42",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.first_easm_scan_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.first_easm_scan_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_vm_scan_date_agent")
                    && event.get_str("qualys_gav.asset.sensor.last_vm_scan_date_agent") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_vm_scan_date_agent")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.sensor.last_vm_scan_date_agent", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_vm_scan_date_agent".into(),
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
                        "date_agent_last_vm_scan_date_agent_1cde43a5",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_vm_scan_date_agent")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_vm_scan_date_agent".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_pc_scan_date_scanner")
                    && event.get_str("qualys_gav.asset.sensor.last_pc_scan_date_scanner")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_pc_scan_date_scanner")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.sensor.last_pc_scan_date_scanner", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_pc_scan_date_scanner"
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
                        "date_agent_last_pc_scan_date_scanner_aaed3c3f",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_pc_scan_date_scanner")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_pc_scan_date_scanner".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_pc_scan_date_agent")
                    && event.get_str("qualys_gav.asset.sensor.last_pc_scan_date_agent") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_pc_scan_date_agent")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.sensor.last_pc_scan_date_agent", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_pc_scan_date_agent".into(),
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
                        "date_agent_last_pc_scan_date_agent_12201bb1",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_pc_scan_date_agent")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_pc_scan_date_agent".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.sensor.last_easm_scan_date")
                    && event.get_str("qualys_gav.asset.sensor.last_easm_scan_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor.last_easm_scan_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.sensor.last_easm_scan_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor.last_easm_scan_date".into(),
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
                        "date_agent_last_easm_scan_date_054a385f",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor.last_easm_scan_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor.last_easm_scan_date".into(),
                        });
                    }
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

            if event.has_value("qualys_gav.asset.asset_id") {
                if let Some(val) = event.get("qualys_gav.asset.asset_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.asset_id".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.asset_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("qualys_gav.asset.asset_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("qualys_gav.asset.asset_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_gav.asset.asset_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.asset_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if event.has_value("host.name") {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
            }

            let _cond = { event.has_value("qualys_gav.asset.asset_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_gav.asset.asset_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.asset_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            let _cond = { event.has_value("qualys_gav.asset.asset_uuid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_gav.asset.asset_uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("qualys_gav.asset.container.has_sensor") {
                if let Some(val) = event.get("qualys_gav.asset.container.has_sensor") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.container.has_sensor".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.container.has_sensor", converted)?;
                }
            }

            let _cond =
                { event.get_str("qualys_gav.asset.container.no_of_containers") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.container.no_of_containers") {
                        if let Some(val) = event.get("qualys_gav.asset.container.no_of_containers")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.container.no_of_containers".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.container.no_of_containers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_container_no_of_containers_to_long_48cfa6f2",
                    )?;
                    if event
                        .remove("qualys_gav.asset.container.no_of_containers")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.container.no_of_containers".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.container.no_of_images") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.container.no_of_images") {
                        if let Some(val) = event.get("qualys_gav.asset.container.no_of_images") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.container.no_of_images".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.container.no_of_images", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_container_no_of_images_to_long_c53cf40b",
                    )?;
                    if event
                        .remove("qualys_gav.asset.container.no_of_images")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.container.no_of_images".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.cpu_count") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.cpu_count") {
                        if let Some(val) = event.get("qualys_gav.asset.cpu_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.cpu_count".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.cpu_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cpu_count_to_long_3591cb22",
                    )?;
                    if event.remove("qualys_gav.asset.cpu_count").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.cpu_count".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.created_date")
                    && event.get_str("qualys_gav.asset.created_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_gav.asset.created_date") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set("qualys_gav.asset.created_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.created_date".into(),
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
                        "date_created_date_89126040",
                    )?;
                    if event.remove("qualys_gav.asset.created_date").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.created_date".into(),
                        });
                    }
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
                .get("qualys_gav.asset.created_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = { event.get_str("qualys_gav.asset.criticality.is_default") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.criticality.is_default") {
                        if let Some(val) = event.get("qualys_gav.asset.criticality.is_default") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.criticality.is_default".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.criticality.is_default", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_criticality_is_default_to_boolean_86c69371",
                    )?;
                    if event
                        .remove("qualys_gav.asset.criticality.is_default")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.criticality.is_default".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.criticality.last_updated")
                    && event.get_str("qualys_gav.asset.criticality.last_updated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.criticality.last_updated")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.criticality.last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.criticality.last_updated".into(),
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
                        "date_criticality_last_updated_dcf502fa",
                    )?;
                    if event
                        .remove("qualys_gav.asset.criticality.last_updated")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.criticality.last_updated".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.criticality.score") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.criticality.score") {
                        if let Some(val) = event.get("qualys_gav.asset.criticality.score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.criticality.score".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.criticality.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_criticality_score_to_long_8ff57787",
                    )?;
                    if event.remove("qualys_gav.asset.criticality.score").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.criticality.score".into(),
                        });
                    }
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
                .get("qualys_gav.asset.dns_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("qualys_gav.asset.dns_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_gav.asset.dns_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.domain")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.domain", |event| {
                    event.append_unique(
                        "host.domain",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.domain")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.domain", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.subdomain")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.subdomain", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.hardware.lifecycle.eos_date")
                    && event.get_str("qualys_gav.asset.hardware.lifecycle.eos_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.hardware.lifecycle.eos_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.hardware.lifecycle.eos_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.hardware.lifecycle.eos_date".into(),
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
                        "date_qualys_gav_asset_hardware_lifecycle_eos_date_3931e96a",
                    )?;
                    if event
                        .remove("qualys_gav.asset.hardware.lifecycle.eos_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.hardware.lifecycle.eos_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.hardware.lifecycle.ga_date")
                    && event.get_str("qualys_gav.asset.hardware.lifecycle.ga_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.hardware.lifecycle.ga_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.hardware.lifecycle.ga_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.hardware.lifecycle.ga_date".into(),
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
                        "date_qualys_gav_asset_hardware_lifecycle_ga_date_8eb2d0a5",
                    )?;
                    if event
                        .remove("qualys_gav.asset.hardware.lifecycle.ga_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.hardware.lifecycle.ga_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.hardware.lifecycle.intro_date")
                    && event.get_str("qualys_gav.asset.hardware.lifecycle.intro_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.hardware.lifecycle.intro_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.hardware.lifecycle.intro_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.hardware.lifecycle.intro_date".into(),
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
                        "date_qualys_gav_asset_hardware_lifecycle_intro_date_ff3f4301",
                    )?;
                    if event
                        .remove("qualys_gav.asset.hardware.lifecycle.intro_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.hardware.lifecycle.intro_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.hardware.lifecycle.obsolete_date")
                    && event.get_str("qualys_gav.asset.hardware.lifecycle.obsolete_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.hardware.lifecycle.obsolete_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.hardware.lifecycle.obsolete_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.hardware.lifecycle.obsolete_date"
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
                        "date_qualys_gav_asset_hardware_lifecycle_obsolete_date_7a629b28",
                    )?;
                    if event
                        .remove("qualys_gav.asset.hardware.lifecycle.obsolete_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.hardware.lifecycle.obsolete_date".into(),
                        });
                    }
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
                .get("qualys_gav.asset.hardware.manufacturer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.manufacturer", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.hardware.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            if event.has_value("qualys_gav.asset.hardware.taxonomy.id") {
                if let Some(val) = event.get("qualys_gav.asset.hardware.taxonomy.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.hardware.taxonomy.id".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.hardware.taxonomy.id", converted)?;
                }
            }

            if event.has_value("qualys_gav.asset.host_id") {
                if let Some(val) = event.get("qualys_gav.asset.host_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.host_id".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.host_id", converted)?;
                }
            }

            let _cond = { event.has_value("qualys_gav.asset.host_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_gav.asset.host_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.inventory.created")
                    && event.get_str("qualys_gav.asset.inventory.created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.inventory.created")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.inventory.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.inventory.created".into(),
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
                        "date_qualys_gav_asset_inventory_created_73f80a4d",
                    )?;
                    if event.remove("qualys_gav.asset.inventory.created").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.inventory.created".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.inventory.last_updated")
                    && event.get_str("qualys_gav.asset.inventory.last_updated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.inventory.last_updated")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.inventory.last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.inventory.last_updated".into(),
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
                        "date_qualys_gav_asset_inventory_last_updated_1d6688ad",
                    )?;
                    if event
                        .remove("qualys_gav.asset.inventory.last_updated")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.inventory.last_updated".into(),
                        });
                    }
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
                event
                    .get("qualys_gav.asset.inventory_list_data.inventory")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.inventory_list_data.inventory")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.created")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.created", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.created".into(),
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
                                    "date_asset_inventory_list_data_inventory_created_370aff7f",
                                )?;
                                event.remove("_ingest._value.created");
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
                            "qualys_gav.asset.inventory_list_data.inventory",
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
                event
                    .get("qualys_gav.asset.inventory_list_data.inventory")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.inventory_list_data.inventory")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.last_updated")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_updated", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_updated".into(),
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
                                event.set("_ingest.on_failure_processor_tag", "date_asset_inventory_list_data_inventory_last_updated_eca5b6c9")?;
                                event.remove("_ingest._value.last_updated");
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
                            "qualys_gav.asset.inventory_list_data.inventory",
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
                event.has_value("qualys_gav.asset.activity.last_scanned_date")
                    && event.get_str("qualys_gav.asset.activity.last_scanned_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.activity.last_scanned_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.activity.last_scanned_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.activity.last_scanned_date".into(),
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
                        "date_qualys_gav_asset_activity_last_scanned_date_6de00f4b",
                    )?;
                    if event
                        .remove("qualys_gav.asset.activity.last_scanned_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.activity.last_scanned_date".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.is_container_host") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.is_container_host") {
                        if let Some(val) = event.get("qualys_gav.asset.is_container_host") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.is_container_host".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.is_container_host", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_container_host_to_boolean_8f313971",
                    )?;
                    if event.remove("qualys_gav.asset.is_container_host").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.is_container_host".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.last_boot")
                    && event.get_str("qualys_gav.asset.last_boot") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_gav.asset.last_boot") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set("qualys_gav.asset.last_boot", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.last_boot".into(),
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
                        "date_last_boot_bfb23966",
                    )?;
                    if event.remove("qualys_gav.asset.last_boot").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.last_boot".into(),
                        });
                    }
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
                .get("qualys_gav.asset.last_location.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.last_location.continent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.continent_name", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.last_location.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.country_name", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.last_location.postal")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.postal_code", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.last_logged_on_user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("qualys_gav.asset.last_logged_on_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("qualys_gav.asset.last_logged_on_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("qualys_gav.asset.netbios_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_gav.asset.netbios_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.address_ip_v4") {
                                if let Some(s) = event.get_string("_ingest._value.address_ip_v4") {
                                    let mut parts: Vec<Value> =
                                        s.split(",").map(|p| json!(p)).collect();
                                    if parts.len() > 1 {
                                        while parts.last().and_then(Value::as_str) == Some("") {
                                            parts.pop();
                                        }
                                    }
                                    event
                                        .set("_ingest._value.address_ip_v4", Value::Array(parts))?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "split")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.address_ip_v4", |event| {
                                if event.has_value("_ingest._value") {
                                    map_strings(event, "_ingest._value", "_ingest._value", |s| {
                                        s.trim().to_string()
                                    })?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.address_ip_v4", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value") {
                                        if let Some(val) = event.get("_ingest._value") {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_asset_network_interface_list_data_network_interface_address_ip_v4_79e2329c")?;
                                    event.remove("_ingest._value");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.address_ip_v4", |event| {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.mac_address") {
                                gsub_field(
                                    event,
                                    "_ingest._value.mac_address",
                                    "_ingest._value.mac_address",
                                    cached_regex!(":"),
                                    "-",
                                )?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "gsub")?;
                            event.set("_ingest.on_failure_processor_tag", "gsub_network_interface_list_data_network_interface_mac_address_3bd7aa9e")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        if event.has_value("_ingest._value.mac_address") {
                            map_strings(
                                event,
                                "_ingest._value.mac_address",
                                "_ingest._value.mac_address",
                                str::to_uppercase,
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.address_ip_v6") {
                                if let Some(s) = event.get_string("_ingest._value.address_ip_v6") {
                                    let mut parts: Vec<Value> =
                                        s.split(", ").map(|p| json!(p)).collect();
                                    if parts.len() > 1 {
                                        while parts.last().and_then(Value::as_str) == Some("") {
                                            parts.pop();
                                        }
                                    }
                                    event
                                        .set("_ingest._value.address_ip_v6", Value::Array(parts))?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "split")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.address_ip_v6", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value") {
                                        if let Some(val) = event.get("_ingest._value") {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_asset_network_interface_list_data_network_interface_address_ip_v6_63d8abe7")?;
                                    event.remove("_ingest._value");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.network_interface_list_data.network_interface",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.address_ip_v6", |event| {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.network_interface_list_data.network_interface")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.network_interface_list_data.network_interface")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.mac_vendor_intro_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event
                                            .set("_ingest._value.mac_vendor_intro_date", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.mac_vendor_intro_date".into(),
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
                                event.set("_ingest.on_failure_processor_tag", "date_asset_network_interface_list_data_network_interface_mac_vendor_intro_date_a4ed9d95")?;
                                event.remove("_ingest._value.mac_vendor_intro_date");
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
                            "qualys_gav.asset.network_interface_list_data.network_interface",
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
                event
                    .get("qualys_gav.asset.open_port_list_data.open_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.open_port_list_data.open_port",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.port") {
                                if let Some(val) = event.get("_ingest._value.port") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.port".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.port", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_qualys_gav_asset_open_port_list_data_open_port_to_long_9948cb81")?;
                            event.remove("_ingest._value.port");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.open_port_list_data.open_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.open_port_list_data.open_port",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.detection_score") {
                                if let Some(val) = event.get("_ingest._value.detection_score") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.detection_score".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.detection_score", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_qualys_gav_asset_open_port_list_data_open_port_detection_score_to_long_d1eaf59a")?;
                            event.remove("_ingest._value.detection_score");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.open_port_list_data.open_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.open_port_list_data.open_port")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.first_found")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.first_found", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.first_found".into(),
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
                                    "date_open_port_list_data_open_port_first_found_6d14a17f",
                                )?;
                                event.remove("_ingest._value.first_found");
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
                            "qualys_gav.asset.open_port_list_data.open_port",
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
                event
                    .get("qualys_gav.asset.open_port_list_data.open_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.open_port_list_data.open_port")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.last_updated")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_updated", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_updated".into(),
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
                                    "date_open_port_list_data_open_port_last_updated_b7c289b7",
                                )?;
                                event.remove("_ingest._value.last_updated");
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
                            "qualys_gav.asset.open_port_list_data.open_port",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if let Some(v) = event
                .get("qualys_gav.asset.operating_system.architecture")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.architecture", v)?;
            }

            let _cond = { event.has_value("qualys_gav.asset.operating_system.category1") };
            if _cond {
                // Painless script
                // Source: def os_type = ctx.qualys_gav.asset.operating_system.category1.toLowerCase();\n\nctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\n\nif (os_type.contains('centos') || os_type.contains('ubuntu')) {\n  ctx.host.os.put('type', 'linux');\n} else {\n  ctx.host.os.put('type', params.get(os_type));\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def os_type = ctx.qualys_gav.asset.operating_system.category1.toLowerCase();\n\nctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\n\nif (os_type.contains('centos') || os_type.contains('ubuntu')) {\n  ctx.host.os.put('type', 'linux');\n} else {\n  ctx.host.os.put('type', params.get(os_type));\n}\n"#
                    ),
                    cached_params!(
                        "{\"macos\":\"macos\",\"linux\":\"linux\",\"unix\":\"unix\",\"windows\":\"windows\",\"ios\":\"ios\",\"android\":\"android\"}"
                    ),
                )?;
            }

            if event.has_value("qualys_gav.asset.operating_system.cpe_id") {
                if let Some(val) = event.get("qualys_gav.asset.operating_system.cpe_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.operating_system.cpe_id".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.operating_system.cpe_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("qualys_gav.asset.operating_system.full_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.operating_system.install_date")
                    && event.get_str("qualys_gav.asset.operating_system.install_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.operating_system.install_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_gav.asset.operating_system.install_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.operating_system.install_date".into(),
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
                        "date_qualys_gav_asset_operating_system_install_date_19d9d229",
                    )?;
                    if event
                        .remove("qualys_gav.asset.operating_system.install_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.operating_system.install_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.operating_system.lifecycle.eol_date")
                    && event.get_str("qualys_gav.asset.operating_system.lifecycle.eol_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.operating_system.lifecycle.eol_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_gav.asset.operating_system.lifecycle.eol_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.operating_system.lifecycle.eol_date"
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
                        "date_qualys_gav_asset_operating_system_lifecycle_eol_date_c87a81a8",
                    )?;
                    if event
                        .remove("qualys_gav.asset.operating_system.lifecycle.eol_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.operating_system.lifecycle.eol_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.operating_system.lifecycle.eos_date")
                    && event.get_str("qualys_gav.asset.operating_system.lifecycle.eos_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.operating_system.lifecycle.eos_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_gav.asset.operating_system.lifecycle.eos_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.operating_system.lifecycle.eos_date"
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
                        "date_qualys_gav_asset_operating_system_lifecycle_eos_date_cb248794",
                    )?;
                    if event
                        .remove("qualys_gav.asset.operating_system.lifecycle.eos_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.operating_system.lifecycle.eos_date".into(),
                        });
                    }
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
                event.has_value("qualys_gav.asset.operating_system.lifecycle.ga_date")
                    && event.get_str("qualys_gav.asset.operating_system.lifecycle.ga_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.operating_system.lifecycle.ga_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_gav.asset.operating_system.lifecycle.ga_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.operating_system.lifecycle.ga_date"
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
                        "date_qualys_gav_asset_operating_system_lifecycle_ga_date_a52f0df3",
                    )?;
                    if event
                        .remove("qualys_gav.asset.operating_system.lifecycle.ga_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.operating_system.lifecycle.ga_date".into(),
                        });
                    }
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
                event.get_str("qualys_gav.asset.operating_system.lifecycle.detection_score")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("qualys_gav.asset.operating_system.lifecycle.detection_score")
                    {
                        if let Some(val) =
                            event.get("qualys_gav.asset.operating_system.lifecycle.detection_score")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "qualys_gav.asset.operating_system.lifecycle.detection_score".into(),
                            message,
                        })?;
                            event.set(
                                "qualys_gav.asset.operating_system.lifecycle.detection_score",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_qualys_gav_asset_operating_system_lifecycle_detection_score_to_long_022dcda6")?;
                    if event
                        .remove("qualys_gav.asset.operating_system.lifecycle.detection_score")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.operating_system.lifecycle.detection_score"
                                .into(),
                        });
                    }
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
                .get("qualys_gav.asset.operating_system.os_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.operating_system.product_family")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }

            if let Some(v) = event
                .get("qualys_gav.asset.operating_system.product_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            if event.has_value("qualys_gav.asset.operating_system.taxonomy.id") {
                if let Some(val) = event.get("qualys_gav.asset.operating_system.taxonomy.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.operating_system.taxonomy.id".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.operating_system.taxonomy.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("qualys_gav.asset.operating_system.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("qualys_gav.asset.provider") {
                map_strings(
                    event,
                    "qualys_gav.asset.provider",
                    "cloud.provider",
                    str::to_lowercase,
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("qualys_gav.asset.cloud_provider.aws.ec2.has_agent") {
                    if let Some(val) =
                        event.get("qualys_gav.asset.cloud_provider.aws.ec2.has_agent")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "qualys_gav.asset.cloud_provider.aws.ec2.has_agent".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "qualys_gav.asset.cloud_provider.aws.ec2.has_agent",
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
                    "convert_qualys_gav_asset_cloud_provider_aws_ec2_has_agent_to_boolean_6d53d690",
                )?;
                event.remove("qualys_gav.asset.cloud_provider.aws.ec2.has_agent");
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

            let _cond = {
                event.has_value("qualys_gav.asset.cloud_provider.aws.ec2.launchdate")
                    && event.get_str("qualys_gav.asset.cloud_provider.aws.ec2.launchdate")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.cloud_provider.aws.ec2.launchdate")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_gav.asset.cloud_provider.aws.ec2.launchdate",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.cloud_provider.aws.ec2.launchdate"
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
                        "date_qualys_gav_asset_cloud_provider_aws_ec2_launchdate_448ac46a",
                    )?;
                    event.remove("qualys_gav.asset.cloud_provider.aws.ec2.launchdate");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("qualys_gav.asset.cloud_provider.aws.ec2.qualys_scanner") {
                    if let Some(val) =
                        event.get("qualys_gav.asset.cloud_provider.aws.ec2.qualys_scanner")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "qualys_gav.asset.cloud_provider.aws.ec2.qualys_scanner"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "qualys_gav.asset.cloud_provider.aws.ec2.qualys_scanner",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_qualys_gav_asset_cloud_provider_aws_ec2_qualys_scanner_to_boolean_660df783")?;
                event.remove("qualys_gav.asset.cloud_provider.aws.ec2.qualys_scanner");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("qualys_gav.asset.cloud_provider.aws.ec2.spot_instance") {
                    if let Some(val) =
                        event.get("qualys_gav.asset.cloud_provider.aws.ec2.spot_instance")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "qualys_gav.asset.cloud_provider.aws.ec2.spot_instance"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "qualys_gav.asset.cloud_provider.aws.ec2.spot_instance",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_qualys_gav_asset_cloud_provider_aws_ec2_spot_instance_to_boolean_84768335")?;
                event.remove("qualys_gav.asset.cloud_provider.aws.ec2.spot_instance");
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

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.aws.ec2.account_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.aws.ec2.availability_zone")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.availability_zone", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.aws.ec2.instance_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.aws.ec2.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.aws.ec2.instance_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.machine.type", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.aws.ec2.region.code")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.region", v)?;
                }
            }

            let _cond = {
                event.get_bool("_conf.want_provider_cloud") == Some(true)
                    && event.get_str("cloud.provider") == Some("aws")
                    && event.has_value("qualys_gav.asset.cloud_provider.aws.ec2")
            };
            if _cond {
                event.set("cloud.service.name", json!("ec2"))?;
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.azure.vm.subscription_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.azure.vm.vm_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.azure.vm.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.azure.vm.size")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.machine.type", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.azure.vm.location")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.region", v)?;
                }
            }

            let _cond = {
                event.get_bool("_conf.want_provider_cloud") == Some(true)
                    && event.get_str("cloud.provider") == Some("azure")
                    && event.has_value("qualys_gav.asset.cloud_provider.azure.vm")
            };
            if _cond {
                event.set("cloud.service.name", json!("vm"))?;
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.gcp.compute.project_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.gcp.compute.zone")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.availability_zone", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.gcp.compute.instance_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.gcp.compute.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.gcp.compute.machine_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.machine.type", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.gcp.compute.project_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.project.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.cloud_provider.gcp.compute.zone") {
                        if let Some(input) =
                            event.get_string("qualys_gav.asset.cloud_provider.gcp.compute.zone")
                        {
                            // Grok pattern: %{GREEDYDATA:cloud.region}-%{WORD}$
                            if !cached_grok!("%{GREEDYDATA:cloud.region}-%{WORD}$")
                                .extract_into(&input, event)?
                            {
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
                        "grok_cloud_region_from_asset_cloud_provider_gcp_compute_zone_f0d55a6c",
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

            let _cond = {
                event.get_bool("_conf.want_provider_cloud") == Some(true)
                    && event.get_str("cloud.provider") == Some("gcp")
                    && event.has_value("qualys_gav.asset.cloud_provider.gcp.compute")
            };
            if _cond {
                event.set("cloud.service.name", json!("compute"))?;
            }

            if event.has_value("qualys_gav.asset.cloud_provider.ibm.virtual_server.datacenter_id") {
                if let Some(val) =
                    event.get("qualys_gav.asset.cloud_provider.ibm.virtual_server.datacenter_id")
                {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "qualys_gav.asset.cloud_provider.ibm.virtual_server.datacenter_id"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "qualys_gav.asset.cloud_provider.ibm.virtual_server.datacenter_id",
                        converted,
                    )?;
                }
            }

            if event.has_value("qualys_gav.asset.cloud_provider.ibm.virtual_server.ibm_id") {
                if let Some(val) =
                    event.get("qualys_gav.asset.cloud_provider.ibm.virtual_server.ibm_id")
                {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.cloud_provider.ibm.virtual_server.ibm_id"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "qualys_gav.asset.cloud_provider.ibm.virtual_server.ibm_id",
                        converted,
                    )?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.ibm.virtual_server.datacenter_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.ibm.virtual_server.ibm_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.ibm.virtual_server.device_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_gav.asset.cloud_provider.ibm.virtual_server.location")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.availability_zone", v)?;
                }
            }

            let _cond = {
                event.get_bool("_conf.want_provider_cloud") == Some(true)
                    && event.get_str("cloud.provider") == Some("azure")
                    && event.has_value("qualys_gav.asset.cloud_provider.ibm.virtual_server")
            };
            if _cond {
                event.set("cloud.service.name", json!("virtual_server"))?;
            }

            let _cond = { event.get_str("qualys_gav.asset.risk_score") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.risk_score") {
                        if let Some(val) = event.get("qualys_gav.asset.risk_score") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.risk_score".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.risk_score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_qualys_gav_asset_risk_score_to_float_9da7aa85",
                    )?;
                    if event.remove("qualys_gav.asset.risk_score").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.risk_score".into(),
                        });
                    }
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
                .get("qualys_gav.asset.risk_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.risk_score", v)?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.business_app_list_data.business_app")
                    && event.get_str("qualys_gav.asset.business_app_list_data.business_app")
                        != Some("")
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.business_app_list_data.business_app",
                    |event| {
                        if event.has_value("_ingest._value.id") {
                            if let Some(val) = event.get("_ingest._value.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.id", converted)?;
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.sensor_last_updated_date")
                    && event.get_str("qualys_gav.asset.sensor_last_updated_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.sensor_last_updated_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.sensor_last_updated_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.sensor_last_updated_date".into(),
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
                        "date_sensor_last_updated_date_8dbb485e",
                    )?;
                    if event
                        .remove("qualys_gav.asset.sensor_last_updated_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.sensor_last_updated_date".into(),
                        });
                    }
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        if event.has_value("_ingest._value.id") {
                            if let Some(val) = event.get("_ingest._value.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.id", converted)?;
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.name",
                            json!(
                                event
                                    .get("_ingest._value.full_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.type",
                            json!(
                                event
                                    .get("_ingest._value.software_type")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_ignored") {
                                if let Some(val) = event.get("_ingest._value.is_ignored") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_ignored".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_ignored", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_sofware_list_data_software_is_ignored_d13e9160",
                            )?;
                            event.remove("_ingest._value.is_ignored");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.version",
                            json!(
                                event
                                    .get("_ingest._value.version")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.architecture",
                            json!(
                                event
                                    .get("_ingest._value.architecture")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.install_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.install_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.install_date".into(),
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
                                    "date_software_list_data_software_installDate_29df0a76",
                                )?;
                                event.remove("_ingest._value.install_date");
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.installed",
                            json!(
                                event
                                    .get("_ingest._value.install_date")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.path",
                            json!(
                                event
                                    .get("_ingest._value.install_path")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.last_updated")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_updated", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_updated".into(),
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
                                    "date_software_list_data_software_last_updated_ab5e077a",
                                )?;
                                event.remove("_ingest._value.last_updated");
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.last_use_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_use_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_use_date".into(),
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
                                    "date_software_list_data_software_last_use_date_a4128bab",
                                )?;
                                event.remove("_ingest._value.last_use_date");
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_package") {
                                if let Some(val) = event.get("_ingest._value.is_package") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_package".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_package", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_sofware_list_data_software_is_package_1c2243a1",
                            )?;
                            event.remove("_ingest._value.is_package");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_package_component") {
                                if let Some(val) = event.get("_ingest._value.is_package_component")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_package_component".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_package_component", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_sofware_list_data_software_is_package_component_bd7d3e93",
                            )?;
                            event.remove("_ingest._value.is_package_component");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.reference",
                            json!(
                                event
                                    .get("_ingest._value.product_url")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.lifecycle.ga_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.lifecycle.ga_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.lifecycle.ga_date".into(),
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
                                    "date_software_list_data_software_ga_date_bfd56579",
                                )?;
                                event.remove("_ingest._value.lifecycle.ga_date");
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.lifecycle.eol_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event
                                            .set("_ingest._value.lifecycle.eol_date", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.lifecycle.eol_date".into(),
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
                                    "date_software_list_data_software_lifecycle_eol_date_6f5bc07c",
                                )?;
                                event.remove("_ingest._value.lifecycle.eol_date");
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.lifecycle.eos_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event
                                            .set("_ingest._value.lifecycle.eos_date", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.lifecycle.eos_date".into(),
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
                                    "date_software_list_data_software_lifecycle_eos_date_d9032321",
                                )?;
                                event.remove("_ingest._value.lifecycle.eos_date");
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.lifecycle.detection_score") {
                                if let Some(val) =
                                    event.get("_ingest._value.lifecycle.detection_score")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.lifecycle.detection_score"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.lifecycle.detection_score",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_software_list_data_software_lifecycle_detection_score_717e66e4")?;
                            event.remove("_ingest._value.lifecycle.detection_score");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.description",
                            json!(
                                event
                                    .get("_ingest._value.support_stage_desc")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.append_unique(
                            "package.license",
                            json!(
                                event
                                    .get("_ingest._value.license.category")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) =
                            (|| -> Result<()> {
                                if event.has_value("_ingest._value.authorization_detection_score") {
                                    if let Some(val) =
                                        event.get("_ingest._value.authorization_detection_score")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.authorization_detection_score".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.authorization_detection_score",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })()
                        {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_software_list_data_software_authorization_detection_score_b3d41ac2")?;
                            event.remove("_ingest._value.authorization_detection_score");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.software_instances").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if let Some(date_str) =
                                                    event.get_as_string("_ingest._value.first_seen")
                                                {
                                                    match parse_date_out(
                                                        &date_str,
                                                        &["UNIX_MS", "ISO8601"],
                                                        None,
                                                        None,
                                                    ) {
                                                        Some(parsed) => event.set(
                                                            "_ingest._value.first_seen",
                                                            parsed,
                                                        )?,
                                                        None => {
                                                            return Err(
                                                                TransformError::ParseError {
                                                                    path:
                                                                        "_ingest._value.first_seen"
                                                                            .into(),
                                                                    message: format!(
                                                                        "unable to parse date [{date_str}]"
                                                                    ),
                                                                },
                                                            );
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
                                                event.set("_ingest.on_failure_processor_tag", "date_software_list_data_software_software_instances_firstSeen_7ba0d98b")?;
                                                event.remove("_ingest._value.first_seen");
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.software_instances",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.software_list_data.software")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_gav.asset.software_list_data.software")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.software_instances").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if let Some(date_str) =
                                                    event.get_as_string("_ingest._value.last_seen")
                                                {
                                                    match parse_date_out(
                                                        &date_str,
                                                        &["UNIX_MS", "ISO8601"],
                                                        None,
                                                        None,
                                                    ) {
                                                        Some(parsed) => event.set(
                                                            "_ingest._value.last_seen",
                                                            parsed,
                                                        )?,
                                                        None => {
                                                            return Err(
                                                                TransformError::ParseError {
                                                                    path:
                                                                        "_ingest._value.last_seen"
                                                                            .into(),
                                                                    message: format!(
                                                                        "unable to parse date [{date_str}]"
                                                                    ),
                                                                },
                                                            );
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
                                                event.set("_ingest.on_failure_processor_tag", "date_software_list_data_software_software_instances_last_seen_bb92ebe5")?;
                                                event.remove("_ingest._value.last_seen");
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.software_instances",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
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
                            "qualys_gav.asset.software_list_data.software",
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
                event
                    .get("qualys_gav.asset.tag_list.tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.tag_list.tag", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.criticality_score") {
                            if let Some(val) = event.get("_ingest._value.criticality_score") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.criticality_score".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.criticality_score", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_tagList_tag_criticality_score_42a8debf",
                        )?;
                        event.remove("_ingest._value.criticality_score");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.tag_list.tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.tag_list.tag", |event| {
                    if event.has_value("_ingest._value.tag_id") {
                        if let Some(val) = event.get("_ingest._value.tag_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.tag_id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.tag_id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.tag_list.tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.tag_list.tag", |event| {
                    if event.has_value("_ingest._value.background_color") {
                        if let Some(val) = event.get("_ingest._value.background_color") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.background_color".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.background_color", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.tag_list.tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.tag_list.tag", |event| {
                    if event.has_value("_ingest._value.foreground_color") {
                        if let Some(val) = event.get("_ingest._value.foreground_color") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.foreground_color".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.foreground_color", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("qualys_gav.asset.threads_per_core") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.threads_per_core") {
                        if let Some(val) = event.get("qualys_gav.asset.threads_per_core") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.threads_per_core".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.threads_per_core", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_threads_per_core_d2d319f2",
                    )?;
                    if event.remove("qualys_gav.asset.threads_per_core").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.threads_per_core".into(),
                        });
                    }
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
                .get("qualys_gav.asset.time_zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { event.get_str("qualys_gav.asset.total_memory") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.total_memory") {
                        if let Some(val) = event.get("qualys_gav.asset.total_memory") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.total_memory".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.total_memory", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_total_memory_to_long_27024366",
                    )?;
                    if event.remove("qualys_gav.asset.total_memory").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.total_memory".into(),
                        });
                    }
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
                event
                    .get("qualys_gav.asset.volume_list_data.volume")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.volume_list_data.volume", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.free") {
                            if let Some(val) = event.get("_ingest._value.free") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.free".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.free", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_volume_list_data_volume_free_to_long_63409b2a",
                        )?;
                        event.remove("_ingest._value.free");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.volume_list_data.volume")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.volume_list_data.volume", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.size") {
                            if let Some(val) = event.get("_ingest._value.size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.size".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_volume_list_data_volume_size_to_long_9c7b5bf0",
                        )?;
                        event.remove("_ingest._value.size");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.whois")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("qualys_gav.asset.whois").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.created_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.created_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.created_date".into(),
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
                                    "date_whois_created_date_a6037c5d",
                                )?;
                                event.remove("_ingest._value.created_date");
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
                            "qualys_gav.asset.whois",
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
                event
                    .get("qualys_gav.asset.whois")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("qualys_gav.asset.whois").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.expiration_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.expiration_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.expiration_date".into(),
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
                                    "date_whois_expiration_date_3e1ac050",
                                )?;
                                event.remove("_ingest._value.expiration_date");
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
                            "qualys_gav.asset.whois",
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
                event
                    .get("qualys_gav.asset.whois")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("qualys_gav.asset.whois").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.updated_date")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.updated_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.updated_date".into(),
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
                                    "date_whois_updated_date_37a941e7",
                                )?;
                                event.remove("_ingest._value.updated_date");
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
                            "qualys_gav.asset.whois",
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
                event
                    .get("qualys_gav.asset.whois")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.whois", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("qualys_gav.asset.last_modified_date")
                    && event.get_str("qualys_gav.asset.last_modified_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("qualys_gav.asset.last_modified_date")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("qualys_gav.asset.last_modified_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "qualys_gav.asset.last_modified_date".into(),
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
                        "date_last_modified_date_cd52caa3",
                    )?;
                    if event
                        .remove("qualys_gav.asset.last_modified_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.last_modified_date".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.processor.speed") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.processor.speed") {
                        if let Some(val) = event.get("qualys_gav.asset.processor.speed") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.processor.speed".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.processor.speed", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_processor_speed_to_long_977a1e8b",
                    )?;
                    if event.remove("qualys_gav.asset.processor.speed").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.processor.speed".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.processor.num_cpus") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.processor.num_cpus") {
                        if let Some(val) = event.get("qualys_gav.asset.processor.num_cpus") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.processor.num_cpus".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.processor.num_cpus", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_container_num_cpus_to_long_48e519a5",
                    )?;
                    if event
                        .remove("qualys_gav.asset.processor.num_cpus")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.processor.num_cpus".into(),
                        });
                    }
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

            let _cond = { event.get_str("qualys_gav.asset.processor.no_of_socket") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.processor.no_of_socket") {
                        if let Some(val) = event.get("qualys_gav.asset.processor.no_of_socket") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.processor.no_of_socket".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.processor.no_of_socket", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_container_no_of_socket_to_long_73968f3b",
                    )?;
                    if event
                        .remove("qualys_gav.asset.processor.no_of_socket")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.processor.no_of_socket".into(),
                        });
                    }
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
                { event.get_str("qualys_gav.asset.processor.threads_per_core") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.processor.threads_per_core") {
                        if let Some(val) = event.get("qualys_gav.asset.processor.threads_per_core")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.processor.threads_per_core".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.processor.threads_per_core", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_container_threads_per_core_to_long_d785cdee",
                    )?;
                    if event
                        .remove("qualys_gav.asset.processor.threads_per_core")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.processor.threads_per_core".into(),
                        });
                    }
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
                { event.get_str("qualys_gav.asset.processor.cores_per_socket") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_gav.asset.processor.cores_per_socket") {
                        if let Some(val) = event.get("qualys_gav.asset.processor.cores_per_socket")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_gav.asset.processor.cores_per_socket".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_gav.asset.processor.cores_per_socket", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_container_cores_per_socket_to_long_99ed9560",
                    )?;
                    if event
                        .remove("qualys_gav.asset.processor.cores_per_socket")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "qualys_gav.asset.processor.cores_per_socket".into(),
                        });
                    }
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

            if event.has_value("qualys_gav.asset.lpar_id") {
                if let Some(val) = event.get("qualys_gav.asset.lpar_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "qualys_gav.asset.lpar_id".into(),
                            message,
                        }
                    })?;
                    event.set("qualys_gav.asset.lpar_id", converted)?;
                }
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.whois")
                    .is_some_and(|v| v.is_array())
                    && event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
            };
            if _cond {
                foreach_array(event, "qualys_gav.asset.whois", |event| {
                    event.set("_ingest._value.registrant_contact", json!("REDACTED"))?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("qualys_gav.asset.software_list_data.software")
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
                foreach_array(
                    event,
                    "qualys_gav.asset.software_list_data.software",
                    |event| {
                        event.remove("_ingest._value.architecture");
                        event.remove("_ingest._value.install_date");
                        event.remove("_ingest._value.full_name");
                        event.remove("_ingest._value.license.category");
                        event.remove("_ingest._value.install_path");
                        event.remove("_ingest._value.software_type");
                        event.remove("_ingest._value.product_url");
                        event.remove("_ingest._value.version");
                        event.remove("_ingest._value.support_stage_desc");
                        Ok(())
                    },
                )?;
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
                event.remove("qualys_gav.asset.address");
                event.remove("qualys_gav.asset.asset_id");
                event.remove("qualys_gav.asset.asset_name");
                event.remove("qualys_gav.asset.operating_system.category1");
                event.remove("qualys_gav.asset.asset_type");
                event.remove("qualys_gav.asset.created_date");
                event.remove("qualys_gav.asset.dns_name");
                event.remove("qualys_gav.asset.domain");
                event.remove("qualys_gav.asset.hardware.manufacturer");
                event.remove("qualys_gav.asset.hardware.model");
                event.remove("qualys_gav.asset.last_location.city");
                event.remove("qualys_gav.asset.last_location.continent");
                event.remove("qualys_gav.asset.last_location.name");
                event.remove("qualys_gav.asset.operating_system.full_name");
                event.remove("qualys_gav.asset.last_location.postal");
                event.remove("qualys_gav.asset.last_logged_on_user");
                event.remove("qualys_gav.asset.operating_system.architecture");
                event.remove("qualys_gav.asset.operating_system.os_name");
                event.remove("qualys_gav.asset.operating_system.product_family");
                event.remove("qualys_gav.asset.operating_system.product_name");
                event.remove("qualys_gav.asset.operating_system.version");
                event.remove("qualys_gav.asset.cloud_provider");
                event.remove("qualys_gav.asset.risk_score");
                event.remove("qualys_gav.asset.time_zone");
            }

            event.remove("_conf");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || (v instanceof String && ((String) v).trim() == '')|| (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || (v instanceof String && ((String) v).trim() == '')|| (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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
