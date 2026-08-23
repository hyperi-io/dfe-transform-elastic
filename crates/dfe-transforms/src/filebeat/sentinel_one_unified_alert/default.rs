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
            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("9.3.0"))?;

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

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.updatedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.detectedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.externalId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.sentinel_one = ctx.sentinel_one ?: [:];\nctx.sentinel_one.unified_alert = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.sentinel_one = ctx.sentinel_one ?: [:];\nctx.sentinel_one.unified_alert = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.real_time.scope.site.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sentinel_one.site.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.real_time.scope.site.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sentinel_one.site.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.real_time.scope.account.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sentinel_one.account.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.real_time.scope.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sentinel_one.account.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.classification")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sentinel_one.threat_classification.name", v)?;
            }

            let _cond = {
                event.has_value("sentinel_one.unified_alert.created_at")
                    && event.get_str("sentinel_one.unified_alert.created_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sentinel_one.unified_alert.created_at")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.created_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_at")?;
                    event.remove("sentinel_one.unified_alert.created_at");
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
                    .get("sentinel_one.unified_alert.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "sentinel_one.unified_alert.assets", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.accessible") {
                            if let Some(val) = event.get("_ingest._value.accessible") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.accessible".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.accessible", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_assets_accessible_to_boolean",
                        )?;
                        event.remove("_ingest._value.accessible");
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
                    .get("sentinel_one.unified_alert.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "sentinel_one.unified_alert.assets", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.decommissioned") {
                            if let Some(val) = event.get("_ingest._value.decommissioned") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.decommissioned".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.decommissioned", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_assets_decommissioned_to_boolean",
                        )?;
                        event.remove("_ingest._value.decommissioned");
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
                    .get("sentinel_one.unified_alert.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "sentinel_one.unified_alert.assets", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.pending_reboot") {
                            if let Some(val) = event.get("_ingest._value.pending_reboot") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.pending_reboot".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.pending_reboot", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_assets_pending_reboot_to_boolean",
                        )?;
                        event.remove("_ingest._value.pending_reboot");
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
                event.has_value("sentinel_one.unified_alert.detected_at")
                    && event.get_str("sentinel_one.unified_alert.detected_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sentinel_one.unified_alert.detected_at")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.detected_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_detected_at")?;
                    event.remove("sentinel_one.unified_alert.detected_at");
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
                    .get("sentinel_one.unified_alert.detection_time.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sentinel_one.unified_alert.detection_time.assets",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.accessible") {
                                if let Some(val) = event.get("_ingest._value.accessible") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.accessible".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.accessible", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_detection_time_assets_accessible_to_boolean",
                            )?;
                            event.remove("_ingest._value.accessible");
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
                    .get("sentinel_one.unified_alert.detection_time.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sentinel_one.unified_alert.detection_time.assets",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.asset.console_ip_address") {
                                if let Some(val) =
                                    event.get("_ingest._value.asset.console_ip_address")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.asset.console_ip_address"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.asset.console_ip_address",
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
                                "convert_detection_time_assets_asset_console_ip_address_to_ip",
                            )?;
                            event.remove("_ingest._value.asset.console_ip_address");
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
                    .get("sentinel_one.unified_alert.detection_time.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sentinel_one.unified_alert.detection_time.assets",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.asset.ip_v4") {
                                if let Some(val) = event.get("_ingest._value.asset.ip_v4") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.asset.ip_v4".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.asset.ip_v4", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_detection_time_assets_asset_ip_v4_to_ip",
                            )?;
                            event.remove("_ingest._value.asset.ip_v4");
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
                    .get("sentinel_one.unified_alert.detection_time.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sentinel_one.unified_alert.detection_time.assets",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.asset.ip_v6") {
                                if let Some(val) = event.get("_ingest._value.asset.ip_v6") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.asset.ip_v6".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.asset.ip_v6", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_detection_time_assets_asset_ip_v6_to_ip",
                            )?;
                            event.remove("_ingest._value.asset.ip_v6");
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
                    .get("sentinel_one.unified_alert.detection_time.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sentinel_one.unified_alert.detection_time.assets",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("_ingest._value.asset.subscription_time")
                            {
                                if let Some(parsed) =
                                    parse_date_out(&date_str, &["ISO8601"], None, None)
                                {
                                    event.set("_ingest._value.asset.subscription_time", parsed)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_detection_time_assets_asset_subscription_time",
                            )?;
                            event.remove("_ingest._value.asset.subscription_time");
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
                event.has_value("sentinel_one.unified_alert.detection_time.attacker.ip")
                    && event.get_str("sentinel_one.unified_alert.detection_time.attacker.ip")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sentinel_one.unified_alert.detection_time.attacker.ip") {
                        if let Some(val) =
                            event.get("sentinel_one.unified_alert.detection_time.attacker.ip")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sentinel_one.unified_alert.detection_time.attacker.ip"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one.unified_alert.detection_time.attacker.ip",
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
                        "convert_detection_time_attacker_ip_to_ip",
                    )?;
                    if event
                        .remove("sentinel_one.unified_alert.detection_time.attacker.ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "sentinel_one.unified_alert.detection_time.attacker.ip".into(),
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
                event.has_value("sentinel_one.unified_alert.first_seen_at")
                    && event.get_str("sentinel_one.unified_alert.first_seen_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sentinel_one.unified_alert.first_seen_at")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.first_seen_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_first_seen_at")?;
                    event.remove("sentinel_one.unified_alert.first_seen_at");
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
                event.has_value("sentinel_one.unified_alert.last_seen_at")
                    && event.get_str("sentinel_one.unified_alert.last_seen_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sentinel_one.unified_alert.last_seen_at")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.last_seen_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_seen_at")?;
                    event.remove("sentinel_one.unified_alert.last_seen_at");
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
                if event.has_value("sentinel_one.unified_alert.note_exists") {
                    if let Some(val) = event.get("sentinel_one.unified_alert.note_exists") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "sentinel_one.unified_alert.note_exists".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.unified_alert.note_exists", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_note_exists_to_boolean",
                )?;
                if event
                    .remove("sentinel_one.unified_alert.note_exists")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "sentinel_one.unified_alert.note_exists".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_complete",
                ) {
                    if let Some(val) = event.get("sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_complete") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_complete".into(),
                            message,
                        })?;
                    event.set("sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_complete", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_slo_details_time_to_resolve_data_action_complete_to_boolean",
                )?;
                if event.remove("sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_complete").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_complete".into() });
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_due",
                ) {
                    if let Some(val) = event.get(
                        "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_due",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_due".into(),
                            message,
                        })?;
                        event.set("sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_due", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_slo_details_time_to_resolve_data_action_due_to_long",
                )?;
                if event
                    .remove(
                        "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_due",
                    )
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path:
                            "sentinel_one.unified_alert.slo_details.time_to_resolve_data.action_due"
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion",
                ) {
                    if let Some(val) = event.get(
                        "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion".into(),
                            message,
                        })?;
                        event.set("sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_slo_details_time_to_resolve_data_completion_to_long",
                )?;
                if event
                    .remove(
                        "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion",
                    )
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path:
                            "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion"
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

            let _cond = {
                event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion_time",
                ) && event.get_str(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion_time",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion_time") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion_time", parsed)?;
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_slo_details_time_to_resolve_data_completion_time",
                    )?;
                    event.remove("sentinel_one.unified_alert.slo_details.time_to_resolve_data.completion_time");
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
                if event
                    .has_value("sentinel_one.unified_alert.slo_details.time_to_resolve_data.target")
                {
                    if let Some(val) = event
                        .get("sentinel_one.unified_alert.slo_details.time_to_resolve_data.target")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target".into(),
                            message,
                        })?;
                        event.set(
                            "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target",
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
                    "convert_slo_details_time_to_resolve_data_target_to_long",
                )?;
                if event
                    .remove("sentinel_one.unified_alert.slo_details.time_to_resolve_data.target")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target"
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

            let _cond = {
                event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target_time",
                ) && event.get_str(
                    "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target_time",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target_time",
                    ) {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.slo_details.time_to_resolve_data.target_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_slo_details_time_to_resolve_data_target_time",
                    )?;
                    event.remove(
                        "sentinel_one.unified_alert.slo_details.time_to_resolve_data.target_time",
                    );
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
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.action_complete",
                ) {
                    if let Some(val) = event.get("sentinel_one.unified_alert.slo_details.time_to_response_data.action_complete") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_response_data.action_complete".into(),
                            message,
                        })?;
                    event.set("sentinel_one.unified_alert.slo_details.time_to_response_data.action_complete", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_slo_details_time_to_response_data_action_complete_to_boolean",
                )?;
                if event.remove("sentinel_one.unified_alert.slo_details.time_to_response_data.action_complete").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sentinel_one.unified_alert.slo_details.time_to_response_data.action_complete".into() });
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.action_due",
                ) {
                    if let Some(val) = event.get(
                        "sentinel_one.unified_alert.slo_details.time_to_response_data.action_due",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_response_data.action_due".into(),
                            message,
                        })?;
                        event.set("sentinel_one.unified_alert.slo_details.time_to_response_data.action_due", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_slo_details_time_to_response_data_action_due_to_long",
                )?;
                if event
                    .remove(
                        "sentinel_one.unified_alert.slo_details.time_to_response_data.action_due",
                    )
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound { path: "sentinel_one.unified_alert.slo_details.time_to_response_data.action_due".into() });
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.completion",
                ) {
                    if let Some(val) = event.get(
                        "sentinel_one.unified_alert.slo_details.time_to_response_data.completion",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_response_data.completion".into(),
                            message,
                        })?;
                        event.set("sentinel_one.unified_alert.slo_details.time_to_response_data.completion", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_slo_details_time_to_response_data_completion_to_long",
                )?;
                if event
                    .remove(
                        "sentinel_one.unified_alert.slo_details.time_to_response_data.completion",
                    )
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound { path: "sentinel_one.unified_alert.slo_details.time_to_response_data.completion".into() });
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

            let _cond = {
                event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.completion_time",
                ) && event.get_str(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.completion_time",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("sentinel_one.unified_alert.slo_details.time_to_response_data.completion_time") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("sentinel_one.unified_alert.slo_details.time_to_response_data.completion_time", parsed)?;
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_slo_details_time_to_response_data_completion_time",
                    )?;
                    event.remove("sentinel_one.unified_alert.slo_details.time_to_response_data.completion_time");
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
                if event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.target",
                ) {
                    if let Some(val) = event
                        .get("sentinel_one.unified_alert.slo_details.time_to_response_data.target")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sentinel_one.unified_alert.slo_details.time_to_response_data.target".into(),
                            message,
                        })?;
                        event.set(
                            "sentinel_one.unified_alert.slo_details.time_to_response_data.target",
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
                    "convert_slo_details_time_to_response_data_target_to_long",
                )?;
                if event
                    .remove("sentinel_one.unified_alert.slo_details.time_to_response_data.target")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "sentinel_one.unified_alert.slo_details.time_to_response_data.target"
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

            let _cond = {
                event.has_value(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.target_time",
                ) && event.get_str(
                    "sentinel_one.unified_alert.slo_details.time_to_response_data.target_time",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "sentinel_one.unified_alert.slo_details.time_to_response_data.target_time",
                    ) {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.slo_details.time_to_response_data.target_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_slo_details_time_to_response_data_target_time",
                    )?;
                    event.remove(
                        "sentinel_one.unified_alert.slo_details.time_to_response_data.target_time",
                    );
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
                event.has_value("sentinel_one.unified_alert.updated_at")
                    && event.get_str("sentinel_one.unified_alert.updated_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sentinel_one.unified_alert.updated_at")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.unified_alert.updated_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_updated_at")?;
                    event.remove("sentinel_one.unified_alert.updated_at");
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
                .get("sentinel_one.unified_alert.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            event.set("event.kind", json!("alert"))?;

            let _cond = {
                event.has_value("sentinel_one.unified_alert.classification")
                    && [
                        "MALWARE",
                        "RANSOMWARE",
                        "TROJAN",
                        "DOWNLOADER",
                        "WORM",
                        "VIRUS",
                        "SPYWARE",
                        "KEYLOGGER",
                        "INFO_STEALER",
                        "ROOTKIT",
                        "BACKDOOR",
                        "DROPPER",
                        "COINMINER",
                        "CRYPTOMINER",
                        "MINER",
                        "PACKED",
                        "LINUX_MALWARE",
                        "MALICIOUS_OFFICE_DOC",
                        "MALICIOUS_PDF",
                        "GENERIC_HEURISTIC",
                        "PUA",
                        "ROGUE",
                        "TOOLBAR",
                    ]
                    .contains(
                        &event
                            .get_str("sentinel_one.unified_alert.classification")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("sentinel_one.unified_alert.classification")
                    && [
                        "COMMAND_AND_CONTROL",
                        "EXPLOIT",
                        "HACKTOOL",
                        "ENUMERATION",
                        "INTERACTIVE_SHELL",
                        "REMOTE_SHELL",
                        "LATERAL_MOVEMENT",
                        "MANUAL",
                    ]
                    .contains(
                        &event
                            .get_str("sentinel_one.unified_alert.classification")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("sentinel_one.unified_alert.classification")
                    && event.get_str("sentinel_one.unified_alert.classification") == Some("NETWORK")
            };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = {
                event.has_value("sentinel_one.unified_alert.classification")
                    && event.get_str("sentinel_one.unified_alert.classification")
                        == Some("PHISHING")
            };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("sentinel_one.unified_alert.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.first_seen_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.last_seen_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("sentinel_one.unified_alert.result")
                    && event
                        .get_str("sentinel_one.unified_alert.result")
                        .is_some_and(|s| s.eq_ignore_ascii_case("mitigated"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("sentinel_one.unified_alert.result")
                    && event
                        .get_str("sentinel_one.unified_alert.result")
                        .is_some_and(|s| s.eq_ignore_ascii_case("unmitigated"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event
                    .get("sentinel_one.unified_alert.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString severity = ctx.sentinel_one.unified_alert.severity;\nif (severity.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString severity = ctx.sentinel_one.unified_alert.severity;\nif (severity.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
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
                .get("sentinel_one.unified_alert.updated_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.real_time.scope.group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.real_time.scope.group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.detection_time.attacker.host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.detection_time.attacker.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.detection_time.target_user.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.detection_time.target_user.email_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.detection_time.target_user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: // In SentinelOne, it appears that each alert is typically associated with a single primary asset.\n// Therefore, we will extract ECS fields from the first asset in the assets array\n\nctx.related = ctx.related ?: [:];\nctx.related.hosts = ctx.related.hosts ?: [];\nctx.related.user = ctx.related.user ?: [];\nctx.related.ip = ctx.related.ip ?: [];\nctx.host = ctx.host ?: [:];\nctx.host.ip = ctx.host.ip ?: [];\nctx.host.os = ctx.host.os ?: [:];\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\nctx.cloud.account = ctx.cloud.account ?: [:];\nctx.cloud.machine.type = ctx.cloud.machine.type ?: [];\nctx.cloud.region = ctx.cloud.region ?: [];\nctx.cloud.instance.id = ctx.cloud.instance.id ?: [];\nctx.cloud.account.id = ctx.cloud.account.id ?: [];\nctx.cloud.account.name = ctx.cloud.account.name ?: [];\nctx.cloud.project = ctx.cloud.project ?: [:];\nctx.cloud.project.id = ctx.cloud.project.id ?: [];\nctx.container = ctx.container ?: [:];\nctx.container.id = ctx.container.id ?: [];\nctx.container.name = ctx.container.name ?: [];\nctx.container.image = ctx.container.image ?: [:];\nctx.container.image.name = ctx.container.image.name ?: [];\nctx.container.labels = ctx.container.labels ?: [];\nctx.orchestrator = ctx.orchestrator ?: [:];\nctx.orchestrator.cluster = ctx.orchestrator.cluster ?: [:];\nctx.orchestrator.namespace = ctx.orchestrator.namespace ?: [];\nctx.orchestrator.resource = ctx.orchestrator.resource ?: [:];\nctx.orchestrator.resource.parent = ctx.orchestrator.resource.parent ?: [:];\nctx.orchestrator.resource.parent.type = ctx.orchestrator.resource.parent.type ?: [];\nctx.orchestrator.resource.label = ctx.orchestrator.resource.label ?: [];\nctx.orchestrator.resource.name = ctx.orchestrator.resource.name ?: [];\nctx.observer = ctx.observer ?: [:];\n\ndef assets = ctx.sentinel_one?.unified_alert?.assets;\nif (assets instanceof List && assets.size() > 0) {\n  def asset = assets[0];\n  if (asset.agent_uuid != null) { ctx.observer.serial_number = asset.agent_uuid; }\n  if (asset.agent_version != null) { ctx.observer.version = asset.agent_version; }\n  if (asset.id != null) { ctx.related.hosts.add(asset.id); }\n  if (asset.name != null) { ctx.related.hosts.add(asset.name); ctx.host.name = asset.name; }\n  if (asset.subcategory != null) { ctx.host.type = asset.subcategory; }\n  if (asset.last_logged_in_user != null) { ctx.related.user.add(asset.last_logged_in_user); }\n}\n\ndef dtAssets = ctx.sentinel_one?.unified_alert?.detection_time?.assets;\nif (dtAssets instanceof List && dtAssets.size() > 0) {\n  def a = dtAssets[0].asset;\n  def c = dtAssets[0].cloud;\n  def pd = c?.provider_details;\n  def k = dtAssets[0].kubernetes;\n  if (a != null) {\n    if (a.console_ip_address != null) { ctx.host.ip.add(a.console_ip_address); ctx.related.ip.add(a.console_ip_address); }\n    if (a.ip_v4 != null) { ctx.host.ip.add(a.ip_v4); ctx.related.ip.add(a.ip_v4); }\n    if (a.ip_v6 != null) { ctx.host.ip.add(a.ip_v6); ctx.related.ip.add(a.ip_v6); }\n    if (a.last_logged_in_user != null) { ctx.related.user.add(a.last_logged_in_user); }\n    if (a.os_name != null) { ctx.host.os.name = a.os_name; }\n    if (a.os_revision != null) { ctx.host.os.version = a.os_revision; }\n    if (a.os_type != null) {\n      String os_type = a.os_type.toLowerCase();\n      for (String os: params.os_type) {\n        if (os_type.contains(os)) {\n          ctx.host.os.type = os;\n          return;\n        }\n      }\n    }\n  }\n  if (c != null) {\n    if (c.account_id != null) { ctx.cloud.account.name = c.account_id; }\n    if (c.cloud_provider != null) { ctx.cloud.provider = c.cloud_provider; }\n    if (c.instance_id != null) { ctx.cloud.instance.id = c.instance_id; }\n    if (c.instance_size != null) { ctx.cloud.machine.type = c.instance_size; }\n    if (c.location != null) { ctx.cloud.region = c.location; }\n    if (pd != null) {\n      if (pd.account_id != null) { ctx.cloud.account.id = pd.account_id; }\n      if (pd.instance_id != null && ctx.cloud?.instance?.id == null) { ctx.cloud.instance.id = pd.instance_id; }\n      if (pd.instance_type != null && ctx.cloud?.machine?.type == null) { ctx.cloud.machine.type = pd.instance_type; }\n      if (pd.project_id != null) { ctx.cloud.project.id = pd.project_id; }\n      if (pd.region != null && ctx.cloud?.region == null) { ctx.cloud.region = pd.region; }\n      if (pd.service_account != null) { ctx.cloud.account.id = pd.service_account; }\n      if (pd.subscription_id != null) { ctx.cloud.account.id = pd.subscription_id; }\n    }\n  }\n  if (k != null) {\n    if (k.cluster_name != null) { ctx.orchestrator.cluster.name = k.cluster_name; }\n    if (k.container_id != null) { ctx.container.id = k.container_id; }\n    if (k.container_image_name != null) { ctx.container.image.name = k.container_image_name; }\n    if (k.container_name != null) { ctx.container.name = k.container_name; }\n    if (k.controller_type != null) { ctx.orchestrator.resource.parent.type = k.controller_type; }\n    if (k.namespace_name != null) { ctx.orchestrator.namespace = k.namespace_name; }\n    if (k.pod_labels != null) { ctx.orchestrator.resource.label = k.pod_labels; }\n    if (k.pod_name != null) { ctx.orchestrator.resource.name = k.pod_name; }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"// In SentinelOne, it appears that each alert is typically associated with a single primary asset.\n// Therefore, we will extract ECS fields from the first asset in the assets array\n\nctx.related = ctx.related ?: [:];\nctx.related.hosts = ctx.related.hosts ?: [];\nctx.related.user = ctx.related.user ?: [];\nctx.related.ip = ctx.related.ip ?: [];\nctx.host = ctx.host ?: [:];\nctx.host.ip = ctx.host.ip ?: [];\nctx.host.os = ctx.host.os ?: [:];\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\nctx.cloud.account = ctx.cloud.account ?: [:];\nctx.cloud.machine.type = ctx.cloud.machine.type ?: [];\nctx.cloud.region = ctx.cloud.region ?: [];\nctx.cloud.instance.id = ctx.cloud.instance.id ?: [];\nctx.cloud.account.id = ctx.cloud.account.id ?: [];\nctx.cloud.account.name = ctx.cloud.account.name ?: [];\nctx.cloud.project = ctx.cloud.project ?: [:];\nctx.cloud.project.id = ctx.cloud.project.id ?: [];\nctx.container = ctx.container ?: [:];\nctx.container.id = ctx.container.id ?: [];\nctx.container.name = ctx.container.name ?: [];\nctx.container.image = ctx.container.image ?: [:];\nctx.container.image.name = ctx.container.image.name ?: [];\nctx.container.labels = ctx.container.labels ?: [];\nctx.orchestrator = ctx.orchestrator ?: [:];\nctx.orchestrator.cluster = ctx.orchestrator.cluster ?: [:];\nctx.orchestrator.namespace = ctx.orchestrator.namespace ?: [];\nctx.orchestrator.resource = ctx.orchestrator.resource ?: [:];\nctx.orchestrator.resource.parent = ctx.orchestrator.resource.parent ?: [:];\nctx.orchestrator.resource.parent.type = ctx.orchestrator.resource.parent.type ?: [];\nctx.orchestrator.resource.label = ctx.orchestrator.resource.label ?: [];\nctx.orchestrator.resource.name = ctx.orchestrator.resource.name ?: [];\nctx.observer = ctx.observer ?: [:];\n\ndef assets = ctx.sentinel_one?.unified_alert?.assets;\nif (assets instanceof List && assets.size() > 0) {\n  def asset = assets[0];\n  if (asset.agent_uuid != null) { ctx.observer.serial_number = asset.agent_uuid; }\n  if (asset.agent_version != null) { ctx.observer.version = asset.agent_version; }\n  if (asset.id != null) { ctx.related.hosts.add(asset.id); }\n  if (asset.name != null) { ctx.related.hosts.add(asset.name); ctx.host.name = asset.name; }\n  if (asset.subcategory != null) { ctx.host.type = asset.subcategory; }\n  if (asset.last_logged_in_user != null) { ctx.related.user.add(asset.last_logged_in_user); }\n}\n\ndef dtAssets = ctx.sentinel_one?.unified_alert?.detection_time?.assets;\nif (dtAssets instanceof List && dtAssets.size() > 0) {\n  def a = dtAssets[0].asset;\n  def c = dtAssets[0].cloud;\n  def pd = c?.provider_details;\n  def k = dtAssets[0].kubernetes;\n  if (a != null) {\n    if (a.console_ip_address != null) { ctx.host.ip.add(a.console_ip_address); ctx.related.ip.add(a.console_ip_address); }\n    if (a.ip_v4 != null) { ctx.host.ip.add(a.ip_v4); ctx.related.ip.add(a.ip_v4); }\n    if (a.ip_v6 != null) { ctx.host.ip.add(a.ip_v6); ctx.related.ip.add(a.ip_v6); }\n    if (a.last_logged_in_user != null) { ctx.related.user.add(a.last_logged_in_user); }\n    if (a.os_name != null) { ctx.host.os.name = a.os_name; }\n    if (a.os_revision != null) { ctx.host.os.version = a.os_revision; }\n    if (a.os_type != null) {\n      String os_type = a.os_type.toLowerCase();\n      for (String os: params.os_type) {\n        if (os_type.contains(os)) {\n          ctx.host.os.type = os;\n          return;\n        }\n      }\n    }\n  }\n  if (c != null) {\n    if (c.account_id != null) { ctx.cloud.account.name = c.account_id; }\n    if (c.cloud_provider != null) { ctx.cloud.provider = c.cloud_provider; }\n    if (c.instance_id != null) { ctx.cloud.instance.id = c.instance_id; }\n    if (c.instance_size != null) { ctx.cloud.machine.type = c.instance_size; }\n    if (c.location != null) { ctx.cloud.region = c.location; }\n    if (pd != null) {\n      if (pd.account_id != null) { ctx.cloud.account.id = pd.account_id; }\n      if (pd.instance_id != null && ctx.cloud?.instance?.id == null) { ctx.cloud.instance.id = pd.instance_id; }\n      if (pd.instance_type != null && ctx.cloud?.machine?.type == null) { ctx.cloud.machine.type = pd.instance_type; }\n      if (pd.project_id != null) { ctx.cloud.project.id = pd.project_id; }\n      if (pd.region != null && ctx.cloud?.region == null) { ctx.cloud.region = pd.region; }\n      if (pd.service_account != null) { ctx.cloud.account.id = pd.service_account; }\n      if (pd.subscription_id != null) { ctx.cloud.account.id = pd.subscription_id; }\n    }\n  }\n  if (k != null) {\n    if (k.cluster_name != null) { ctx.orchestrator.cluster.name = k.cluster_name; }\n    if (k.container_id != null) { ctx.container.id = k.container_id; }\n    if (k.container_image_name != null) { ctx.container.image.name = k.container_image_name; }\n    if (k.container_name != null) { ctx.container.name = k.container_name; }\n    if (k.controller_type != null) { ctx.orchestrator.resource.parent.type = k.controller_type; }\n    if (k.namespace_name != null) { ctx.orchestrator.namespace = k.namespace_name; }\n    if (k.pod_labels != null) { ctx.orchestrator.resource.label = k.pod_labels; }\n    if (k.pod_name != null) { ctx.orchestrator.resource.name = k.pod_name; }\n  }\n}"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_extract_ecs_from_single_asset",
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
                .get("sentinel_one.unified_alert.process.cmd_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.process.parent_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.process.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.process.file.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.process.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.process.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.unified_alert.process.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond =
                { event.has_value("sentinel_one.unified_alert.detection_time.attacker.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.detection_time.attacker.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one.unified_alert.detection_time.attacker.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.detection_time.attacker.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value(
                    "sentinel_one.unified_alert.detection_time.target_user.email_address",
                )
            };
            if _cond {
                event.append_unique("related.user", json!(event.get("sentinel_one.unified_alert.detection_time.target_user.email_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond =
                { event.has_value("sentinel_one.unified_alert.detection_time.target_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.detection_time.target_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.assignee.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.assignee.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.assignee.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.assignee.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.assignee.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.assignee.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.process.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.process.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.process.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.process.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.process.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.process.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sentinel_one.unified_alert.exclusion_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one.unified_alert.exclusion_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("sentinel_one.unified_alert.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "sentinel_one.unified_alert.assets", |event| {
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
                        event.remove("_ingest._value.agent_uuid");
                        event.remove("_ingest._value.agent_version");
                        event.remove("_ingest._value.name");
                        event.remove("_ingest._value.subcategory");
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("sentinel_one.unified_alert.detection_time.assets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sentinel_one.unified_alert.detection_time.assets",
                    |event| {
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
                            event.remove("_ingest._value.asset.os_name");
                            event.remove("_ingest._value.asset.os_revision");
                            event.remove("_ingest._value.cloud.account_id");
                            event.remove("_ingest._value.cloud.cloud_provider");
                            event.remove("_ingest._value.cloud.instance_id");
                            event.remove("_ingest._value.cloud.instance_size");
                            event.remove("_ingest._value.cloud.location");
                            event.remove("_ingest._value.cloud.provider_details.account_id");
                            event.remove("_ingest._value.cloud.provider_details.subscription_id");
                            event.remove("_ingest._value.cloud.provider_details.project_id");
                            event.remove("_ingest._value.cloud.provider_details.service_account");
                            event.remove("_ingest._value.kubernetes.cluster_name");
                            event.remove("_ingest._value.kubernetes.container_id");
                            event.remove("_ingest._value.kubernetes.container_name");
                            event.remove("_ingest._value.kubernetes.controller_type");
                            event.remove("_ingest._value.kubernetes.namespace_name");
                            event.remove("_ingest._value.kubernetes.pod_labels");
                            event.remove("_ingest._value.kubernetes.pod_name");
                        }
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
                event.remove("sentinel_one.unified_alert.created_at");
                event.remove("sentinel_one.unified_alert.detection_time.attacker.host");
                event.remove("sentinel_one.unified_alert.detection_time.attacker.ip");
                event.remove("sentinel_one.unified_alert.detection_time.target_user.domain");
                event.remove("sentinel_one.unified_alert.detection_time.target_user.email_address");
                event.remove("sentinel_one.unified_alert.detection_time.target_user.name");
                event.remove("sentinel_one.unified_alert.first_seen_at");
                event.remove("sentinel_one.unified_alert.id");
                event.remove("sentinel_one.unified_alert.last_seen_at");
                event.remove("sentinel_one.unified_alert.process.cmd_line");
                event.remove("sentinel_one.unified_alert.process.file.md5");
                event.remove("sentinel_one.unified_alert.process.file.name");
                event.remove("sentinel_one.unified_alert.process.file.path");
                event.remove("sentinel_one.unified_alert.process.file.sha1");
                event.remove("sentinel_one.unified_alert.process.file.sha256");
                event.remove("sentinel_one.unified_alert.process.parent_name");
                event.remove("sentinel_one.unified_alert.real_time.scope.group.name");
                event.remove("sentinel_one.unified_alert.real_time.scope.group.id");
                event.remove("sentinel_one.unified_alert.updated_at");
            }

            event.remove("json");

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
                        "Processor '{}' {}with tag '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
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
