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
            event.set("ecs.version", json!("9.4.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("message").is_some_and(|v| v.is_string())
                    && event.get_str("message") == Some("retry")
            };
            if _cond {
                return Ok(TransformResult::Drop);
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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "xm_cyber.entity_inventory")?;
            }

            event.set("event.kind", json!("asset"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("xm_cyber.entity_inventory.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nif (ctx.xm_cyber?.entity_inventory != null) {\n  ctx.xm_cyber.entity_inventory = convertToSnakeCase(ctx.xm_cyber.entity_inventory);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nif (ctx.xm_cyber?.entity_inventory != null) {\n  ctx.xm_cyber.entity_inventory = convertToSnakeCase(ctx.xm_cyber.entity_inventory);\n}"#
                ),
            )?;

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.first_seen")
                    && event.get_str("xm_cyber.entity_inventory.first_seen") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.first_seen") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.first_seen")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.first_seen", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.first_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_first_seen")?;
                    event.remove("xm_cyber.entity_inventory.first_seen");
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
                event.has_value("xm_cyber.entity_inventory.last_connection_time")
                    && event.get_str("xm_cyber.entity_inventory.last_connection_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_connection_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_connection_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("xm_cyber.entity_inventory.last_connection_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_connection_time".into(),
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
                        "date_last_connection_time",
                    )?;
                    event.remove("xm_cyber.entity_inventory.last_connection_time");
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
                event.has_value("xm_cyber.entity_inventory.last_reboot_time")
                    && event.get_str("xm_cyber.entity_inventory.last_reboot_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_reboot_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_reboot_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.last_reboot_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_reboot_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_reboot_time")?;
                    event.remove("xm_cyber.entity_inventory.last_reboot_time");
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
                event.has_value("xm_cyber.entity_inventory.last_status_change")
                    && event.get_str("xm_cyber.entity_inventory.last_status_change") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_status_change")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_status_change")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.last_status_change", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_status_change".into(),
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
                        "date_last_status_change",
                    )?;
                    event.remove("xm_cyber.entity_inventory.last_status_change");
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
                event.has_value("xm_cyber.entity_inventory.last_updated_at")
                    && event.get_str("xm_cyber.entity_inventory.last_updated_at") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_updated_at") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_updated_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.last_updated_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_updated_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_updated_at")?;
                    event.remove("xm_cyber.entity_inventory.last_updated_at");
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
                event.has_value("xm_cyber.entity_inventory.disabled_changed_at")
                    && event.get_str("xm_cyber.entity_inventory.disabled_changed_at") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.disabled_changed_at")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.disabled_changed_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("xm_cyber.entity_inventory.disabled_changed_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.disabled_changed_at".into(),
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
                        "date_disabled_changed_at",
                    )?;
                    event.remove("xm_cyber.entity_inventory.disabled_changed_at");
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
                event.has_value("xm_cyber.entity_inventory.kms_key_creation_date")
                    && event.get_str("xm_cyber.entity_inventory.kms_key_creation_date") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.kms_key_creation_date")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.kms_key_creation_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("xm_cyber.entity_inventory.kms_key_creation_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.kms_key_creation_date".into(),
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
                        "date_kms_key_creation_date",
                    )?;
                    event.remove("xm_cyber.entity_inventory.kms_key_creation_date");
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
                event.has_value("xm_cyber.entity_inventory.ssm_parameter_last_modified_date")
                    && event.get_str("xm_cyber.entity_inventory.ssm_parameter_last_modified_date")
                        != Some("")
                    && event.get_str("xm_cyber.entity_inventory.ssm_parameter_last_modified_date")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("xm_cyber.entity_inventory.ssm_parameter_last_modified_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.ssm_parameter_last_modified_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "xm_cyber.entity_inventory.ssm_parameter_last_modified_date"
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
                        "date_ssm_parameter_last_modified_date",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ssm_parameter_last_modified_date");
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
                event.has_value("xm_cyber.entity_inventory.time_to_revive_at")
                    && event.get_str("xm_cyber.entity_inventory.time_to_revive_at") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.time_to_revive_at")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.time_to_revive_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.time_to_revive_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.time_to_revive_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time_to_revive_at")?;
                    event.remove("xm_cyber.entity_inventory.time_to_revive_at");
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
                event.has_value("xm_cyber.entity_inventory.xm_update_time")
                    && event.get_str("xm_cyber.entity_inventory.xm_update_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.xm_update_time") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.xm_update_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.xm_update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.xm_update_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_xm_update_time")?;
                    event.remove("xm_cyber.entity_inventory.xm_update_time");
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
                event.has_value("xm_cyber.entity_inventory.created_date")
                    && event.get_str("xm_cyber.entity_inventory.created_date") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.created_date") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.created_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.created_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.created_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_date")?;
                    event.remove("xm_cyber.entity_inventory.created_date");
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
                event.has_value("xm_cyber.entity_inventory.last_activity_date")
                    && event.get_str("xm_cyber.entity_inventory.last_activity_date") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_activity_date")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_activity_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.last_activity_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_activity_date".into(),
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
                        "date_last_activity_date",
                    )?;
                    event.remove("xm_cyber.entity_inventory.last_activity_date");
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
                event.has_value("xm_cyber.entity_inventory.ebs_volume_create_time")
                    && event.get_str("xm_cyber.entity_inventory.ebs_volume_create_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.ebs_volume_create_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.ebs_volume_create_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("xm_cyber.entity_inventory.ebs_volume_create_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ebs_volume_create_time".into(),
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
                        "date_ebs_volume_create_time",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ebs_volume_create_time");
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
                event.has_value("xm_cyber.entity_inventory.creation_timestamp")
                    && event.get_str("xm_cyber.entity_inventory.creation_timestamp") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.creation_timestamp")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.creation_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.creation_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.creation_timestamp".into(),
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
                        "date_creation_timestamp",
                    )?;
                    event.remove("xm_cyber.entity_inventory.creation_timestamp");
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
                event.has_value("xm_cyber.entity_inventory.access_key_creation_date")
                    && event.get_str("xm_cyber.entity_inventory.access_key_creation_date")
                        != Some("")
                    && event.get_str("xm_cyber.entity_inventory.access_key_creation_date")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.access_key_creation_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.access_key_creation_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.access_key_creation_date"
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
                        "date_access_key_creation_date",
                    )?;
                    event.remove("xm_cyber.entity_inventory.access_key_creation_date");
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
                event.has_value("xm_cyber.entity_inventory.last_running_time")
                    && event.get_str("xm_cyber.entity_inventory.last_running_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_running_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_running_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.last_running_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_running_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_running_time")?;
                    event.remove("xm_cyber.entity_inventory.last_running_time");
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
                event.has_value("xm_cyber.entity_inventory.ecr_repository_creation_date")
                    && event.get_str("xm_cyber.entity_inventory.ecr_repository_creation_date")
                        != Some("")
                    && event.get_str("xm_cyber.entity_inventory.ecr_repository_creation_date")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("xm_cyber.entity_inventory.ecr_repository_creation_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.ecr_repository_creation_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ecr_repository_creation_date"
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
                        "date_ecr_repository_creation_date",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ecr_repository_creation_date");
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
                event.has_value("xm_cyber.entity_inventory.dynamo_db_table_creation_date_time")
                    && event.get_str("xm_cyber.entity_inventory.dynamo_db_table_creation_date_time")
                        != Some("")
                    && event.get_str("xm_cyber.entity_inventory.dynamo_db_table_creation_date_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "xm_cyber.entity_inventory.dynamo_db_table_creation_date_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.dynamo_db_table_creation_date_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "xm_cyber.entity_inventory.dynamo_db_table_creation_date_time".into(),
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
                        "date_dynamo_db_table_creation_date_time",
                    )?;
                    event.remove("xm_cyber.entity_inventory.dynamo_db_table_creation_date_time");
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
                event.has_value("xm_cyber.entity_inventory.elasticache_cache_cluster_create_time")
                    && event
                        .get_str("xm_cyber.entity_inventory.elasticache_cache_cluster_create_time")
                        != Some("")
                    && event
                        .get_str("xm_cyber.entity_inventory.elasticache_cache_cluster_create_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "xm_cyber.entity_inventory.elasticache_cache_cluster_create_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.elasticache_cache_cluster_create_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "xm_cyber.entity_inventory.elasticache_cache_cluster_create_time".into(),
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
                        "date_elasticache_cache_cluster_create_time",
                    )?;
                    event.remove("xm_cyber.entity_inventory.elasticache_cache_cluster_create_time");
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
                event.has_value("xm_cyber.entity_inventory.sqs_queue_last_modified_date")
                    && event.get_str("xm_cyber.entity_inventory.sqs_queue_last_modified_date")
                        != Some("")
                    && event.get_str("xm_cyber.entity_inventory.sqs_queue_last_modified_date")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("xm_cyber.entity_inventory.sqs_queue_last_modified_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.sqs_queue_last_modified_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.sqs_queue_last_modified_date"
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
                        "date_sqs_queue_last_modified_date",
                    )?;
                    event.remove("xm_cyber.entity_inventory.sqs_queue_last_modified_date");
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
                event.has_value("xm_cyber.entity_inventory.create_time")
                    && event.get_str("xm_cyber.entity_inventory.create_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.create_time") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.create_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.create_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.create_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_create_time")?;
                    event.remove("xm_cyber.entity_inventory.create_time");
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
                event.has_value("xm_cyber.entity_inventory.created")
                    && event.get_str("xm_cyber.entity_inventory.created") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.created") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("xm_cyber.entity_inventory.created")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created")?;
                    event.remove("xm_cyber.entity_inventory.created");
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
                event.has_value("xm_cyber.entity_inventory.last_modified")
                    && event.get_str("xm_cyber.entity_inventory.last_modified") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.last_modified") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.last_modified")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.last_modified", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.last_modified".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_modified")?;
                    event.remove("xm_cyber.entity_inventory.last_modified");
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
                event.has_value("xm_cyber.entity_inventory.expire_at")
                    && event.get_str("xm_cyber.entity_inventory.expire_at") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.expire_at") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.expire_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.expire_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.expire_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_expire_at")?;
                    event.remove("xm_cyber.entity_inventory.expire_at");
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
                event.has_value("xm_cyber.entity_inventory.when_created")
                    && event.get_str("xm_cyber.entity_inventory.when_created") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.when_created") != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.when_created")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("xm_cyber.entity_inventory.when_created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.when_created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_when_created")?;
                    event.remove("xm_cyber.entity_inventory.when_created");
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
                event.has_value("xm_cyber.entity_inventory.xm_mongo_update_time")
                    && event.get_str("xm_cyber.entity_inventory.xm_mongo_update_time") != Some("")
                    && event.get_str("xm_cyber.entity_inventory.xm_mongo_update_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("xm_cyber.entity_inventory.xm_mongo_update_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("xm_cyber.entity_inventory.xm_mongo_update_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.xm_mongo_update_time".into(),
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
                        "date_xm_mongo_update_time",
                    )?;
                    event.remove("xm_cyber.entity_inventory.xm_mongo_update_time");
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
                event.has_value("xm_cyber.entity_inventory.redshift_cluster_create_time")
                    && event.get_str("xm_cyber.entity_inventory.redshift_cluster_create_time")
                        != Some("")
                    && event.get_str("xm_cyber.entity_inventory.redshift_cluster_create_time")
                        != Some("Unknown")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("xm_cyber.entity_inventory.redshift_cluster_create_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "xm_cyber.entity_inventory.redshift_cluster_create_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.redshift_cluster_create_time"
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
                        "date_redshift_cluster_create_time",
                    )?;
                    event.remove("xm_cyber.entity_inventory.redshift_cluster_create_time");
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
                    .get("xm_cyber.entity_inventory.ebs_volume_attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ebs_volume_attachments") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("xm_cyber.entity_inventory.ebs_volume_attachments")
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
                                        event.get_as_string("_ingest._value.attach_time")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.attach_time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.attach_time".into(),
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
                                        "date_ebs_volume_attachments_attach_time_element",
                                    )?;
                                    event.remove("_ingest._value.attach_time");
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
                                "xm_cyber.entity_inventory.ebs_volume_attachments",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond =
                { event.get_str("xm_cyber.entity_inventory.connection_counter") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.connection_counter") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.connection_counter")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.connection_counter".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.connection_counter", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_connection_counter_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.connection_counter");
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
                { event.get_str("xm_cyber.entity_inventory.agent_version.major") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.agent_version.major") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.agent_version.major")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.agent_version.major".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("xm_cyber.entity_inventory.agent_version.major", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_agent_version_major_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.agent_version.major");
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
                { event.get_str("xm_cyber.entity_inventory.agent_version.minor") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.agent_version.minor") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.agent_version.minor")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.agent_version.minor".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("xm_cyber.entity_inventory.agent_version.minor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_agent_version_minor_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.agent_version.minor");
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
                { event.get_str("xm_cyber.entity_inventory.agent_version.patch") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.agent_version.patch") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.agent_version.patch")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.agent_version.patch".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("xm_cyber.entity_inventory.agent_version.patch", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_agent_version_patch_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.agent_version.patch");
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
                event.get_str("xm_cyber.entity_inventory.latest_possible_agent_version.major")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.latest_possible_agent_version.major")
                    {
                        if let Some(val) = event
                            .get("xm_cyber.entity_inventory.latest_possible_agent_version.major")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.latest_possible_agent_version.major".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.latest_possible_agent_version.major",
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
                        "convert_latest_possible_agent_version_major_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.latest_possible_agent_version.major");
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
                event.get_str("xm_cyber.entity_inventory.latest_possible_agent_version.minor")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.latest_possible_agent_version.minor")
                    {
                        if let Some(val) = event
                            .get("xm_cyber.entity_inventory.latest_possible_agent_version.minor")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.latest_possible_agent_version.minor".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.latest_possible_agent_version.minor",
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
                        "convert_latest_possible_agent_version_minor_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.latest_possible_agent_version.minor");
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
                event.get_str("xm_cyber.entity_inventory.latest_possible_agent_version.patch")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.latest_possible_agent_version.patch")
                    {
                        if let Some(val) = event
                            .get("xm_cyber.entity_inventory.latest_possible_agent_version.patch")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.latest_possible_agent_version.patch".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.latest_possible_agent_version.patch",
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
                        "convert_latest_possible_agent_version_patch_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.latest_possible_agent_version.patch");
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
                { event.get_str("xm_cyber.entity_inventory.os.service_pack.build") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.service_pack.build") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.os.service_pack.build")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.service_pack.build".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.os.service_pack.build",
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
                        "convert_os_service_pack_build_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.service_pack.build");
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
                { event.get_str("xm_cyber.entity_inventory.os.service_pack.major") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.service_pack.major") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.os.service_pack.major")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.service_pack.major".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.os.service_pack.major",
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
                        "convert_os_service_pack_major_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.service_pack.major");
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
                { event.get_str("xm_cyber.entity_inventory.os.service_pack.minor") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.service_pack.minor") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.os.service_pack.minor")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.service_pack.minor".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.os.service_pack.minor",
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
                        "convert_os_service_pack_minor_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.service_pack.minor");
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
                { event.get_str("xm_cyber.entity_inventory.os.service_pack.patch") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.service_pack.patch") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.os.service_pack.patch")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.service_pack.patch".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.os.service_pack.patch",
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
                        "convert_os_service_pack_patch_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.service_pack.patch");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.os.version.build") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.version.build") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.os.version.build") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.version.build".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.os.version.build", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_os_version_build_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.version.build");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.os.version.major") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.version.major") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.os.version.major") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.version.major".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.os.version.major", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_os_version_major_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.version.major");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.os.version.minor") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.version.minor") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.os.version.minor") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.version.minor".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.os.version.minor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_os_version_minor_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.version.minor");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.os.version.patch") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.os.version.patch") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.os.version.patch") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.os.version.patch".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.os.version.patch", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_os_version_patch_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.os.version.patch");
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
                { event.get_str("xm_cyber.entity_inventory.ssm_parameter_version") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.ssm_parameter_version") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.ssm_parameter_version")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ssm_parameter_version".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.ssm_parameter_version",
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
                        "convert_ssm_parameter_version_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ssm_parameter_version");
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
                    .get("xm_cyber.entity_inventory.ipv4")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def src = ctx.xm_cyber.entity_inventory.ipv4;\ndef strings = [];\ndef objects = [];\nfor (def v : src) {\n  if (v == null) { continue; }\n  if (v instanceof String) {\n    strings.add((String) v);\n  } else if (v instanceof Map) {\n    objects.add(v);\n  }\n}\nif (strings.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.remove('ipv4');\n} else {\n  ctx.xm_cyber.entity_inventory.ipv4 = strings;\n}\nif (!objects.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.ipv4_buffer = objects;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def src = ctx.xm_cyber.entity_inventory.ipv4;\ndef strings = [];\ndef objects = [];\nfor (def v : src) {\n  if (v == null) { continue; }\n  if (v instanceof String) {\n    strings.add((String) v);\n  } else if (v instanceof Map) {\n    objects.add(v);\n  }\n}\nif (strings.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.remove('ipv4');\n} else {\n  ctx.xm_cyber.entity_inventory.ipv4 = strings;\n}\nif (!objects.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.ipv4_buffer = objects;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_split_xm_cyber_entity_inventory_ipv4_by_shape",
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
                event
                    .get("xm_cyber.entity_inventory.ipv6")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def src = ctx.xm_cyber.entity_inventory.ipv6;\ndef strings = [];\ndef objects = [];\nfor (def v : src) {\n  if (v == null) { continue; }\n  if (v instanceof String) {\n    strings.add((String) v);\n  } else if (v instanceof Map) {\n    objects.add(v);\n  }\n}\nif (strings.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.remove('ipv6');\n} else {\n  ctx.xm_cyber.entity_inventory.ipv6 = strings;\n}\nif (!objects.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.ipv6_buffer = objects;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def src = ctx.xm_cyber.entity_inventory.ipv6;\ndef strings = [];\ndef objects = [];\nfor (def v : src) {\n  if (v == null) { continue; }\n  if (v instanceof String) {\n    strings.add((String) v);\n  } else if (v instanceof Map) {\n    objects.add(v);\n  }\n}\nif (strings.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.remove('ipv6');\n} else {\n  ctx.xm_cyber.entity_inventory.ipv6 = strings;\n}\nif (!objects.isEmpty()) {\n  ctx.xm_cyber.entity_inventory.ipv6_buffer = objects;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_split_xm_cyber_entity_inventory_ipv6_by_shape",
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

            let _cond = { event.has_value("xm_cyber.entity_inventory.ipv4num") };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ipv4num") {
                    foreach_array(event, "xm_cyber.entity_inventory.ipv4num", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_ipv4_num_long_element",
                            )?;
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
                }
            }

            let _cond = {
                event.get_str(
                    "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_core_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_core_count",
                    ) {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_core_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_core_count".into(),
                            message,
                        })?;
                    event.set("xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_core_count", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_custom_properties_hardware_info_cpu_core_count_long",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_core_count",
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
                event.get_str("xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_count")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_count",
                    ) {
                        if let Some(val) = event.get(
                            "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_count",
                        ) {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_count".into(),
                            message,
                        })?;
                            event.set("xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_custom_properties_hardware_info_cpu_count_long",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_count",
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
                event.get_str(
                    "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_speed_mhz",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_speed_mhz",
                    ) {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_speed_mhz") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_speed_mhz".into(),
                            message,
                        })?;
                    event.set("xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_speed_mhz", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_custom_properties_hardware_info_cpu_speed_mhz_long",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.custom_properties.hardware_info.cpu_speed_mhz",
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
                event.get_str("xm_cyber.entity_inventory.role_max_session_duration") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.role_max_session_duration") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.role_max_session_duration")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.role_max_session_duration"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.role_max_session_duration",
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
                        "convert_role_max_session_duration_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.role_max_session_duration");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.ebs_volume_iops") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.ebs_volume_iops") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.ebs_volume_iops") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ebs_volume_iops".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.ebs_volume_iops", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ebs_volume_iops_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ebs_volume_iops");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.ebs_volume_size") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.ebs_volume_size") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.ebs_volume_size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ebs_volume_size".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.ebs_volume_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ebs_volume_size_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ebs_volume_size");
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
                { event.get_str("xm_cyber.entity_inventory.user_access_keys_count") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.user_access_keys_count") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.user_access_keys_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.user_access_keys_count".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.user_access_keys_count",
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
                        "convert_user_access_keys_count_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.user_access_keys_count");
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
                event.get_str("xm_cyber.entity_inventory.dynamo_db_table_item_count") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.dynamo_db_table_item_count") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.dynamo_db_table_item_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.dynamo_db_table_item_count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.dynamo_db_table_item_count",
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
                        "convert_dynamo_db_table_item_count_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.dynamo_db_table_item_count");
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
                event.get_str("xm_cyber.entity_inventory.dynamo_db_table_size_bytes") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.dynamo_db_table_size_bytes") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.dynamo_db_table_size_bytes")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.dynamo_db_table_size_bytes"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.dynamo_db_table_size_bytes",
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
                        "convert_dynamo_db_table_size_bytes_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.dynamo_db_table_size_bytes");
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
                event.get_str("xm_cyber.entity_inventory.elasticache_cache_cache_security_groups")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.elasticache_cache_cache_security_groups",
                    ) {
                        if let Some(val) = event.get(
                            "xm_cyber.entity_inventory.elasticache_cache_cache_security_groups",
                        ) {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.elasticache_cache_cache_security_groups".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.elasticache_cache_cache_security_groups",
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
                        "convert_elasticache_cache_cache_security_groups_long",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.elasticache_cache_cache_security_groups",
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
                event.get_str("xm_cyber.entity_inventory.elasticache_cache_cluster_num_cache_nodes")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.elasticache_cache_cluster_num_cache_nodes",
                    ) {
                        if let Some(val) = event.get(
                            "xm_cyber.entity_inventory.elasticache_cache_cluster_num_cache_nodes",
                        ) {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.elasticache_cache_cluster_num_cache_nodes".into(),
                            message,
                        })?;
                            event.set("xm_cyber.entity_inventory.elasticache_cache_cluster_num_cache_nodes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_elasticache_cache_cluster_num_cache_nodes_long",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.elasticache_cache_cluster_num_cache_nodes",
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
                event.get_str("xm_cyber.entity_inventory.elasticache_cache_security_groups")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.elasticache_cache_security_groups")
                    {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.elasticache_cache_security_groups")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.elasticache_cache_security_groups".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.elasticache_cache_security_groups",
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
                        "convert_elasticache_cache_security_groups_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.elasticache_cache_security_groups");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.version_number") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.version_number") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.version_number") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.version_number".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.version_number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_version_number_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.version_number");
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
                event.get_str("xm_cyber.entity_inventory.nodes_in_node_group_count") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.nodes_in_node_group_count") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.nodes_in_node_group_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.nodes_in_node_group_count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.nodes_in_node_group_count",
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
                        "convert_nodes_in_node_group_count_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.nodes_in_node_group_count");
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
                { event.get_str("xm_cyber.entity_inventory.machine_account_quota") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.machine_account_quota") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.machine_account_quota")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.machine_account_quota".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.machine_account_quota",
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
                        "convert_machine_account_quota_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.machine_account_quota");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.endpoint_port") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.endpoint_port") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.endpoint_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.endpoint_port".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.endpoint_port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_endpoint_port_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.endpoint_port");
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
                event.get_str("xm_cyber.entity_inventory.redshift_cluster_number_of_nodes")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.redshift_cluster_number_of_nodes")
                    {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.redshift_cluster_number_of_nodes")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "xm_cyber.entity_inventory.redshift_cluster_number_of_nodes"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.redshift_cluster_number_of_nodes",
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
                        "convert_redshift_cluster_number_of_nodes_long",
                    )?;
                    event.remove("xm_cyber.entity_inventory.redshift_cluster_number_of_nodes");
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
                    .get("xm_cyber.entity_inventory.ebs_volume_attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ebs_volume_attachments") {
                    foreach_array(
                        event,
                        "xm_cyber.entity_inventory.ebs_volume_attachments",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.ebs_card_index") {
                                    if let Some(val) = event.get("_ingest._value.ebs_card_index") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.ebs_card_index".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.ebs_card_index", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_ebs_volume_attachments_ebs_card_index_element_long",
                                )?;
                                event.remove("_ingest._value.ebs_card_index");
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
            }

            let _cond = { event.get_str("xm_cyber.entity_inventory.disabled") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.disabled") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.disabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.disabled".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.disabled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_disabled_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.disabled");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.has_matching_sid") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.has_matching_sid") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.has_matching_sid") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.has_matching_sid".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.has_matching_sid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_has_matching_sid_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.has_matching_sid");
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
                { event.get_str("xm_cyber.entity_inventory.has_update_available") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.has_update_available") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.has_update_available")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.has_update_available".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("xm_cyber.entity_inventory.has_update_available", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_has_update_available_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.has_update_available");
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
                { event.get_str("xm_cyber.entity_inventory.not_included_in_attacks") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.not_included_in_attacks") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.not_included_in_attacks")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.not_included_in_attacks"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.not_included_in_attacks",
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
                        "convert_not_included_in_attacks_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.not_included_in_attacks");
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
                { event.get_str("xm_cyber.entity_inventory.entity_details.is_asset") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.entity_details.is_asset") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.entity_details.is_asset")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.entity_details.is_asset"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.entity_details.is_asset",
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
                        "convert_entity_details_is_asset_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.entity_details.is_asset");
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
                event.get_str(
                    "xm_cyber.entity_inventory.custom_properties.sniffer_status_changeable",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.custom_properties.sniffer_status_changeable",
                    ) {
                        if let Some(val) = event.get(
                            "xm_cyber.entity_inventory.custom_properties.sniffer_status_changeable",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.custom_properties.sniffer_status_changeable".into(),
                            message,
                        })?;
                            event.set("xm_cyber.entity_inventory.custom_properties.sniffer_status_changeable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_custom_properties_sniffer_status_changeable_boolean",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.custom_properties.sniffer_status_changeable",
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

            let _cond =
                { event.get_str("xm_cyber.entity_inventory.is_highly_privileged") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.is_highly_privileged") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.is_highly_privileged")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.is_highly_privileged".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("xm_cyber.entity_inventory.is_highly_privileged", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_highly_privileged_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.is_highly_privileged");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.encryption") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.encryption") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.encryption") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.encryption".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.encryption", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_encryption_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.encryption");
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
                event.get_str("xm_cyber.entity_inventory.ebs_volume_multi_attach_enabled")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.ebs_volume_multi_attach_enabled")
                    {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.ebs_volume_multi_attach_enabled")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "xm_cyber.entity_inventory.ebs_volume_multi_attach_enabled"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.ebs_volume_multi_attach_enabled",
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
                        "convert_ebs_volume_multi_attach_enabled_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ebs_volume_multi_attach_enabled");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.is_mfaenabled") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.is_mfaenabled") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.is_mfaenabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.is_mfaenabled".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.is_mfaenabled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_mfaenabled_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.is_mfaenabled");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.public") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.public") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.public") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.public".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.public", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_public_boolean")?;
                    event.remove("xm_cyber.entity_inventory.public");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.is_running") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.is_running") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.is_running") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.is_running".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.is_running", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_running_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.is_running");
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
                event.get_str("xm_cyber.entity_inventory.ecr_repository_image_scanning_on_push")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.ecr_repository_image_scanning_on_push",
                    ) {
                        if let Some(val) = event
                            .get("xm_cyber.entity_inventory.ecr_repository_image_scanning_on_push")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.ecr_repository_image_scanning_on_push".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.ecr_repository_image_scanning_on_push",
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
                        "convert_ecr_repository_image_scanning_on_push_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ecr_repository_image_scanning_on_push");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.is_valid") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.is_valid") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.is_valid") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.is_valid".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.is_valid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_valid_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.is_valid");
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
                event.get_str("xm_cyber.entity_inventory.elasticache_cache_cluster_auth_token")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.elasticache_cache_cluster_auth_token")
                    {
                        if let Some(val) = event
                            .get("xm_cyber.entity_inventory.elasticache_cache_cluster_auth_token")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.elasticache_cache_cluster_auth_token".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.elasticache_cache_cluster_auth_token",
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
                        "convert_elasticache_cache_cluster_auth_token_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.elasticache_cache_cluster_auth_token");
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
                event.get_str(
                    "xm_cyber.entity_inventory.elasticache_cache_cluster_transit_encryption",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "xm_cyber.entity_inventory.elasticache_cache_cluster_transit_encryption",
                    ) {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.elasticache_cache_cluster_transit_encryption") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.elasticache_cache_cluster_transit_encryption".into(),
                            message,
                        })?;
                    event.set("xm_cyber.entity_inventory.elasticache_cache_cluster_transit_encryption", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_elasticache_cache_cluster_transit_encryption_boolean",
                    )?;
                    event.remove(
                        "xm_cyber.entity_inventory.elasticache_cache_cluster_transit_encryption",
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.default_version") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.default_version") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.default_version") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.default_version".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.default_version", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_default_version_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.default_version");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.is_public") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.is_public") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.is_public") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.is_public".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.is_public", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_public_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.is_public");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.is_watched") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.is_watched") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.is_watched") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.is_watched".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.is_watched", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_watched_boolean",
                    )?;
                    event.remove("xm_cyber.entity_inventory.is_watched");
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
                    .get("xm_cyber.entity_inventory.ebs_volume_attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ebs_volume_attachments") {
                    foreach_array(
                        event,
                        "xm_cyber.entity_inventory.ebs_volume_attachments",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.delete_on_termination") {
                                    if let Some(val) =
                                        event.get("_ingest._value.delete_on_termination")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.delete_on_termination"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.delete_on_termination",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_ebs_volume_attachments_delete_on_termination_element_boolean")?;
                                event.remove("_ingest._value.delete_on_termination");
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
            }

            let _cond = { event.has_value("xm_cyber.entity_inventory.ipv4str") };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ipv4str") {
                    foreach_array(event, "xm_cyber.entity_inventory.ipv4str", |event| {
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_ipv4str_ip_element",
                            )?;
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
                }
            }

            let _cond = { event.has_value("xm_cyber.entity_inventory.ipv6str") };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ipv6str") {
                    foreach_array(event, "xm_cyber.entity_inventory.ipv6str", |event| {
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_ipv6str_ip_element",
                            )?;
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
                }
            }

            let _cond =
                { event.get_str("xm_cyber.entity_inventory.ec2private_ip_address") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.ec2private_ip_address") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.ec2private_ip_address")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ec2private_ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "xm_cyber.entity_inventory.ec2private_ip_address",
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
                        "convert_ec2private_ip_address_ip",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ec2private_ip_address");
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
                { event.get_str("xm_cyber.entity_inventory.ec2public_ip_address") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.ec2public_ip_address") {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.ec2public_ip_address")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.ec2public_ip_address".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("xm_cyber.entity_inventory.ec2public_ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ec2public_ip_address_ip",
                    )?;
                    event.remove("xm_cyber.entity_inventory.ec2public_ip_address");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.host_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.host_ip") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.host_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.host_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.host_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_ip_ip")?;
                    event.remove("xm_cyber.entity_inventory.host_ip");
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

            let _cond = { event.get_str("xm_cyber.entity_inventory.pod_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("xm_cyber.entity_inventory.pod_ip") {
                        if let Some(val) = event.get("xm_cyber.entity_inventory.pod_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "xm_cyber.entity_inventory.pod_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("xm_cyber.entity_inventory.pod_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_pod_ip_ip")?;
                    event.remove("xm_cyber.entity_inventory.pod_ip");
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
                event.get_str("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress")
                    {
                        if let Some(val) = event
                            .get("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress")
                        {
                            let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.redshift_cluster_private_ipaddress".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.redshift_cluster_private_ipaddress",
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
                        "convert_redshift_cluster_private_ipaddress_ip",
                    )?;
                    event.remove("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress");
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
                event.get_str("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress")
                    {
                        if let Some(val) =
                            event.get("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress")
                        {
                            let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "xm_cyber.entity_inventory.redshift_cluster_public_ipaddress".into(),
                            message,
                        })?;
                            event.set(
                                "xm_cyber.entity_inventory.redshift_cluster_public_ipaddress",
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
                        "convert_redshift_cluster_public_ipaddress_ip",
                    )?;
                    event.remove("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress");
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
                .get("xm_cyber.entity_inventory.xm_update_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.has_value("xm_cyber.entity_inventory.ipv4str") };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.ipv4str") {
                    foreach_array(event, "xm_cyber.entity_inventory.ipv4str", |event| {
                        event.append_unique(
                            "host.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.os_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.os.distribution_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.os.distribution_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.os.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.machine_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.arch")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.architecture", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.domain_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.kernel_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.kernel", v)?;
            }

            let _cond = { !event.has_value("host.os.full") };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.os_image")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.full", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.dns_host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.fqdn")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.host_ip")
                    && event.get_str("xm_cyber.entity_inventory.host_ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.host_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.pod_ip")
                    && event.get_str("xm_cyber.entity_inventory.pod_ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.pod_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.ec2private_ip_address")
                    && event.get_str("xm_cyber.entity_inventory.ec2private_ip_address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.ec2private_ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.ec2public_ip_address")
                    && event.get_str("xm_cyber.entity_inventory.ec2public_ip_address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.ec2public_ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress")
                    && event.get_str("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress")
                    && event.get_str("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.organization_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.entity_type")
                    && event
                        .get_str("xm_cyber.entity_inventory.entity_type")
                        .is_some_and(|s| s.to_lowercase().contains("aws"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.account_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.entity_type")
                    && event
                        .get_str("xm_cyber.entity_inventory.entity_type")
                        .is_some_and(|s| s.to_lowercase().contains("aws"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.name", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.ec2instance_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            let _cond = { !event.has_value("cloud.instance.id") };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.instance_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.entity_type")
                    && event
                        .get_str("xm_cyber.entity_inventory.entity_type")
                        .is_some_and(|s| s.to_lowercase().contains("aws"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.availability_zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.availability_zone", v)?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.cloud_provider")
                    && event.get_str("xm_cyber.entity_inventory.cloud_provider") != Some("")
                    && !(event
                        .get_str("xm_cyber.entity_inventory.cloud_provider")
                        .is_some_and(|s| s.to_uppercase() == "UNSUPPORTED_CLOUD_PROVIDER"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.cloud_provider")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.provider", v)?;
                }
            }

            if event.has_value("cloud.provider") {
                map_strings(event, "cloud.provider", "cloud.provider", str::to_lowercase)?;
            }

            let _cond =
                { event.has_value("xm_cyber.entity_inventory.entity_details.mac_addresses") };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.entity_details.mac_addresses") {
                    foreach_array(
                        event,
                        "xm_cyber.entity_inventory.entity_details.mac_addresses",
                        |event| {
                            event.append_unique(
                                "host.mac",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = { event.get("host.mac").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("host.mac") {
                    foreach_array(event, "host.mac", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                gsub_field(
                                    event,
                                    "_ingest._value",
                                    "_ingest._value",
                                    cached_regex!("[:.]"),
                                    "-",
                                )?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "gsub")?;
                            event
                                .set("_ingest.on_failure_processor_tag", "gsub_host_mac_element")?;
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
                }
            }

            let _cond = { event.get("host.mac").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("host.mac") {
                    foreach_array(event, "host.mac", |event| {
                        if event.has_value("_ingest._value") {
                            map_strings(
                                event,
                                "_ingest._value",
                                "_ingest._value",
                                str::to_uppercase,
                            )?;
                        }
                        Ok(())
                    })?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.cluster_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.cluster.name", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.cluster_unique_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.cluster.id", v)?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.namespace")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.namespace", v)?;
            }

            let _cond = {
                event.has_value("orchestrator.cluster.name")
                    || event.has_value("xm_cyber.entity_inventory.kubelet_version")
                    || event.has_value("xm_cyber.entity_inventory.pod_ip")
                    || event.has_value("xm_cyber.entity_inventory.kube_proxy_version")
            };
            if _cond {
                event.set("orchestrator.type", json!("kubernetes"))?;
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.aws_user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.iam_unique_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.distinguished_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.last_connection_time")
                    && event.get_str("xm_cyber.entity_inventory.last_connection_time")
                        != Some("Unknown")
                    && event.has_value("xm_cyber.entity_inventory.entity_type")
                    && event
                        .get_str("xm_cyber.entity_inventory.entity_type")
                        .is_some_and(|s| s.to_lowercase().contains("agent"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.last_connection_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.entity.lifecycle.last_activity", v)?;
                }
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.last_activity_date")
                    && event.get_str("xm_cyber.entity_inventory.last_activity_date")
                        != Some("Unknown")
                    && event.has_value("xm_cyber.entity_inventory.entity_type")
                    && event
                        .get_str("xm_cyber.entity_inventory.entity_type")
                        .is_some_and(|s| s.to_lowercase().contains("user"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.last_activity_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.entity.lifecycle.last_activity", v)?;
                }
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.entity_type")
                    && event
                        .get_str("xm_cyber.entity_inventory.entity_type")
                        .is_some_and(|s| s.to_lowercase().contains("user"))
            };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.is_mfaenabled")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.entity.attributes.mfa_enabled", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.engine")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.type", v)?;
            }

            let _cond = { !event.has_value("service.type") };
            if _cond {
                if let Some(v) = event
                    .get("xm_cyber.entity_inventory.lambda_runtime")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("service.type", v)?;
                }
            }

            if let Some(v) = event
                .get("xm_cyber.entity_inventory.engine_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.version", v)?;
            }

            let _cond = { event.has_value("xm_cyber.entity_inventory.tags_str") };
            if _cond {
                if event.has_value("xm_cyber.entity_inventory.tags_str") {
                    foreach_array(event, "xm_cyber.entity_inventory.tags_str", |event| {
                        event.append_unique(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
                if event.has_value("host.ip") {
                    foreach_array(event, "host.ip", |event| {
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
                }
            }

            let _cond = { event.has_value("host.mac") };
            if _cond {
                if event.has_value("host.mac") {
                    foreach_array(event, "host.mac", |event| {
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
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.arn")
                    && event.get_str("xm_cyber.entity_inventory.arn") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.arn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.ecr_repository_arn")
                    && event.get_str("xm_cyber.entity_inventory.ecr_repository_arn") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.ecr_repository_arn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.ebs_volume_kms_key_id")
                    && event.get_str("xm_cyber.entity_inventory.ebs_volume_kms_key_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.ebs_volume_kms_key_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.secret_rotation_lambda_arn")
                    && event.get_str("xm_cyber.entity_inventory.secret_rotation_lambda_arn")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.secret_rotation_lambda_arn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.dns_host_name")
                    && event.get_str("xm_cyber.entity_inventory.dns_host_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.dns_host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.fqdn")
                    && event.get_str("xm_cyber.entity_inventory.fqdn") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.password_hash")
                    && event.get_str("xm_cyber.entity_inventory.password_hash") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.password_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("xm_cyber.entity_inventory.name")
                    && event.get_str("xm_cyber.entity_inventory.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("xm_cyber.entity_inventory.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("xm_cyber.entity_inventory.arch");
            event.remove("xm_cyber.entity_inventory.domain_name");
            event.remove("xm_cyber.entity_inventory.machine_id");
            event.remove("xm_cyber.entity_inventory.os_type");
            event.remove("xm_cyber.entity_inventory.os.distribution_name");
            event.remove("xm_cyber.entity_inventory.os.distribution_version");
            event.remove("xm_cyber.entity_inventory.os.name");
            event.remove("xm_cyber.entity_inventory.kernel_version");
            event.remove("xm_cyber.entity_inventory.os_image");
            event.remove("xm_cyber.entity_inventory.dns_host_name");
            event.remove("xm_cyber.entity_inventory.fqdn");
            event.remove("xm_cyber.entity_inventory.host_ip");
            event.remove("xm_cyber.entity_inventory.pod_ip");
            event.remove("xm_cyber.entity_inventory.ec2private_ip_address");
            event.remove("xm_cyber.entity_inventory.ec2public_ip_address");
            event.remove("xm_cyber.entity_inventory.redshift_cluster_private_ipaddress");
            event.remove("xm_cyber.entity_inventory.redshift_cluster_public_ipaddress");
            event.remove("xm_cyber.entity_inventory.ipv4str");
            event.remove("xm_cyber.entity_inventory.entity_details.mac_addresses");
            event.remove("xm_cyber.entity_inventory.account_id");
            event.remove("xm_cyber.entity_inventory.account_name");
            event.remove("xm_cyber.entity_inventory.region");
            event.remove("xm_cyber.entity_inventory.availability_zone");
            event.remove("xm_cyber.entity_inventory.cloud_provider");
            event.remove("xm_cyber.entity_inventory.ec2instance_id");
            event.remove("xm_cyber.entity_inventory.instance_id");
            event.remove("xm_cyber.entity_inventory.cluster_name");
            event.remove("xm_cyber.entity_inventory.cluster_unique_id");
            event.remove("xm_cyber.entity_inventory.namespace");
            event.remove("xm_cyber.entity_inventory.aws_user_name");
            event.remove("xm_cyber.entity_inventory.user_name");
            event.remove("xm_cyber.entity_inventory.iam_unique_id");
            event.remove("xm_cyber.entity_inventory.sid");
            event.remove("xm_cyber.entity_inventory.distinguished_name");
            event.remove("xm_cyber.entity_inventory.engine");
            event.remove("xm_cyber.entity_inventory.lambda_runtime");
            event.remove("xm_cyber.entity_inventory.engine_version");
            event.remove("xm_cyber.entity_inventory.id");
            event.remove("xm_cyber.entity_inventory.organization_id");
            event.remove("xm_cyber.entity_inventory.xm_update_time");
            event.remove("xm_cyber.entity_inventory.password_hash");
            event.remove("xm_cyber.entity_inventory.tags_str");

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
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
