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
            event.set("ecs.version", json!("9.2.0"))?;

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
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nctx.proofpoint_essentials = ctx.proofpoint_essentials ?: [:];\nif (ctx.json != null) {\n  ctx.proofpoint_essentials.threat = convertToSnakeCase(ctx.json);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nctx.proofpoint_essentials = ctx.proofpoint_essentials ?: [:];\nif (ctx.json != null) {\n  ctx.proofpoint_essentials.threat = convertToSnakeCase(ctx.json);\n}\n"#
                ),
            )?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.get_str("proofpoint_essentials.threat.click_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("proofpoint_essentials.threat.click_ip") {
                        if let Some(val) = event.get("proofpoint_essentials.threat.click_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "proofpoint_essentials.threat.click_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("proofpoint_essentials.threat.click_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_click_ip_to_ip")?;
                    if event
                        .remove("proofpoint_essentials.threat.click_ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "proofpoint_essentials.threat.click_ip".into(),
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
                event.has_value("proofpoint_essentials.threat.click_time")
                    && event.get_str("proofpoint_essentials.threat.click_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("proofpoint_essentials.threat.click_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("proofpoint_essentials.threat.click_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "proofpoint_essentials.threat.click_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_click_time")?;
                    if event
                        .remove("proofpoint_essentials.threat.click_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "proofpoint_essentials.threat.click_time".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("proofpoint_essentials.threat.impostor_score") {
                    if let Some(val) = event.get("proofpoint_essentials.threat.impostor_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint_essentials.threat.impostor_score".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_essentials.threat.impostor_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_impostor_score_to_long",
                )?;
                if event
                    .remove("proofpoint_essentials.threat.impostor_score")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint_essentials.threat.impostor_score".into(),
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
                if event.has_value("proofpoint_essentials.threat.malware_score") {
                    if let Some(val) = event.get("proofpoint_essentials.threat.malware_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint_essentials.threat.malware_score".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_essentials.threat.malware_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_malware_score_to_long",
                )?;
                if event
                    .remove("proofpoint_essentials.threat.malware_score")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint_essentials.threat.malware_score".into(),
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
                if event.has_value("proofpoint_essentials.threat.message_size") {
                    if let Some(val) = event.get("proofpoint_essentials.threat.message_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint_essentials.threat.message_size".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_essentials.threat.message_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_message_size_to_long",
                )?;
                if event
                    .remove("proofpoint_essentials.threat.message_size")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint_essentials.threat.message_size".into(),
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
                event.has_value("proofpoint_essentials.threat.message_time")
                    && event.get_str("proofpoint_essentials.threat.message_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("proofpoint_essentials.threat.message_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("proofpoint_essentials.threat.message_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "proofpoint_essentials.threat.message_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_message_time")?;
                    if event
                        .remove("proofpoint_essentials.threat.message_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "proofpoint_essentials.threat.message_time".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("proofpoint_essentials.threat.phish_score") {
                    if let Some(val) = event.get("proofpoint_essentials.threat.phish_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint_essentials.threat.phish_score".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_essentials.threat.phish_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_phish_score_to_long",
                )?;
                if event
                    .remove("proofpoint_essentials.threat.phish_score")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint_essentials.threat.phish_score".into(),
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

            let _cond = { event.get_str("proofpoint_essentials.threat.sender_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("proofpoint_essentials.threat.sender_ip") {
                        if let Some(val) = event.get("proofpoint_essentials.threat.sender_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "proofpoint_essentials.threat.sender_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("proofpoint_essentials.threat.sender_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sender_ip_to_ip",
                    )?;
                    if event
                        .remove("proofpoint_essentials.threat.sender_ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "proofpoint_essentials.threat.sender_ip".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("proofpoint_essentials.threat.spam_score") {
                    if let Some(val) = event.get("proofpoint_essentials.threat.spam_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint_essentials.threat.spam_score".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_essentials.threat.spam_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_spam_score_to_long",
                )?;
                if event
                    .remove("proofpoint_essentials.threat.spam_score")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint_essentials.threat.spam_score".into(),
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
                event.has_value("proofpoint_essentials.threat.threat_time")
                    && event.get_str("proofpoint_essentials.threat.threat_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("proofpoint_essentials.threat.threat_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("proofpoint_essentials.threat.threat_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "proofpoint_essentials.threat.threat_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_threat_time")?;
                    if event
                        .remove("proofpoint_essentials.threat.threat_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "proofpoint_essentials.threat.threat_time".into(),
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
                    .get("proofpoint_essentials.threat.threats_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("proofpoint_essentials.threat.threats_info_map")
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
                                    event.get_as_string("_ingest._value.threat_time")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.threat_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.threat_time".into(),
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
                                    "date_threats_info_map_threat_time",
                                )?;
                                if event.remove("_ingest._value.threat_time").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.threat_time".into(),
                                    });
                                }
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
                            "proofpoint_essentials.threat.threats_info_map",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("proofpoint_essentials.threat.message_id") {
                gsub_field(
                    event,
                    "proofpoint_essentials.threat.message_id",
                    "proofpoint_essentials.threat.message_id",
                    cached_regex!("<|>"),
                    "",
                )?;
            }

            let _cond = {
                event.get_str("proofpoint_essentials.threat.event_type") == Some("clicks_blocked")
                    || event.get_str("proofpoint_essentials.threat.event_type")
                        == Some("clicks_permitted")
            };
            if _cond {
                if let Some(v) = event
                    .get("proofpoint_essentials.threat.click_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = {
                event.get_str("proofpoint_essentials.threat.event_type") == Some("messages_blocked")
                    || event.get_str("proofpoint_essentials.threat.event_type")
                        == Some("messages_delivered")
            };
            if _cond {
                if let Some(v) = event
                    .get("proofpoint_essentials.threat.message_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def ts = Instant.parse(ctx['@timestamp']);\nif (ctx.proofpoint_essentials?.threat?.event_type == 'messages_blocked' || ctx.proofpoint_essentials?.threat?.event_type == 'messages_delivered') {\n  if (ctx.proofpoint_essentials.threat.threats_info_map instanceof List) {\n    for (item in ctx.proofpoint_essentials.threat.threats_info_map) {\n      if (item?.threat_time instanceof String && Instant.parse(item.threat_time).isAfter(ts)) {\n        ts = item.threat_time;\n      }\n    }\n  }\n}\nif (ctx.proofpoint_essentials?.threat?.event_type == 'clicks_blocked' || ctx.proofpoint_essentials?.threat?.event_type == 'clicks_permitted') {\n  if (ctx.proofpoint_essentials.threat.threat_time instanceof String && Instant.parse(ctx.proofpoint_essentials.threat.threat_time).isAfter(ts)) {\n    ts = ctx.proofpoint_essentials.threat.threat_time;\n  }\n}\nctx['@timestamp'] = ts.toString();\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def ts = Instant.parse(ctx['@timestamp']);\nif (ctx.proofpoint_essentials?.threat?.event_type == 'messages_blocked' || ctx.proofpoint_essentials?.threat?.event_type == 'messages_delivered') {\n  if (ctx.proofpoint_essentials.threat.threats_info_map instanceof List) {\n    for (item in ctx.proofpoint_essentials.threat.threats_info_map) {\n      if (item?.threat_time instanceof String && Instant.parse(item.threat_time).isAfter(ts)) {\n        ts = item.threat_time;\n      }\n    }\n  }\n}\nif (ctx.proofpoint_essentials?.threat?.event_type == 'clicks_blocked' || ctx.proofpoint_essentials?.threat?.event_type == 'clicks_permitted') {\n  if (ctx.proofpoint_essentials.threat.threat_time instanceof String && Instant.parse(ctx.proofpoint_essentials.threat.threat_time).isAfter(ts)) {\n    ts = ctx.proofpoint_essentials.threat.threat_time;\n  }\n}\nctx['@timestamp'] = ts.toString();\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "timestamp-is-maximum")?;
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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("info"))?;

            let _cond = {
                event.get_str("proofpoint_essentials.threat.event_type") == Some("clicks_blocked")
                    || event.get_str("proofpoint_essentials.threat.event_type")
                        == Some("messages_blocked")
            };
            if _cond {
                event.set("event.action", json!("denied"))?;
            }

            let _cond = {
                event.get_str("proofpoint_essentials.threat.event_type") == Some("clicks_permitted")
                    || event.get_str("proofpoint_essentials.threat.event_type")
                        == Some("messages_delivered")
            };
            if _cond {
                event.set("event.action", json!("allowed"))?;
            }

            let _cond = {
                event.get_str("proofpoint_essentials.threat.event_type") == Some("clicks_blocked")
                    || event.get_str("proofpoint_essentials.threat.event_type")
                        == Some("clicks_permitted")
            };
            if _cond {
                if let Some(v) = event.get("proofpoint_essentials.threat.id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = {
                event.get_str("proofpoint_essentials.threat.event_type") == Some("messages_blocked")
                    || event.get_str("proofpoint_essentials.threat.event_type")
                        == Some("messages_delivered")
            };
            if _cond {
                if let Some(v) = event.get("proofpoint_essentials.threat.guid").cloned() {
                    event.set("event.id", v)?;
                }
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.message_details_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            event.set("observer.vendor", json!("Proofpoint"))?;

            event.set("observer.product", json!("Proofpoint Essentials"))?;

            if let Some(v) = event
                .get("proofpoint_essentials.threat.click_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
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

            if let Some(v) = event
                .get("proofpoint_essentials.threat.cc_addresses")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.cc.address", v)?;
            }

            let _cond = { event.has_value("proofpoint_essentials.threat.from_address") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.from_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("proofpoint_essentials.threat.sender")
                    && !event.has_value("email.from.address")
            };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.recipient")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "proofpoint_essentials.threat.recipient", |event| {
                    event.append_unique(
                        "email.to.address",
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
                    .get("proofpoint_essentials.threat.recipient")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.recipient")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.reply_to_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.reply_to.address", v)?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.sender")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.sender.address", v)?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.xmailer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.x_mailer", v)?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.to_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "proofpoint_essentials.threat.to_addresses",
                    |event| {
                        event.append_unique(
                            "email.to.address",
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

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.message_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.email = ctx.email ?: [:];\nctx.email.attachments = ctx.email.attachments ?: [];\nfor (attachment in ctx.proofpoint_essentials.threat.message_parts) {\n  if (attachment.disposition == 'attached') {\n    def o = [:];\n    o.file = [:];\n    o.file.hash = [:];\n    o.file.hash.md5 = attachment.md5;\n    o.file.hash.sha256 = attachment.sha256;\n    o.file.name = attachment.filename;\n    o.file.mime_type = attachment.content_type;\n    ctx.email.attachments.add(o);\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.email = ctx.email ?: [:];\nctx.email.attachments = ctx.email.attachments ?: [];\nfor (attachment in ctx.proofpoint_essentials.threat.message_parts) {\n  if (attachment.disposition == 'attached') {\n    def o = [:];\n    o.file = [:];\n    o.file.hash = [:];\n    o.file.hash.md5 = attachment.md5;\n    o.file.hash.sha256 = attachment.sha256;\n    o.file.name = attachment.filename;\n    o.file.mime_type = attachment.content_type;\n    ctx.email.attachments.add(o);\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_email_attachments",
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
                .get("proofpoint_essentials.threat.quarantine_rule")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.sender_ip")
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

            if let Some(v) = event
                .get("proofpoint_essentials.threat.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.original", v)?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            let _cond = { event.has_value("threat.indicator.url.original") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            if let Some(v) = event
                .get("proofpoint_essentials.threat.threat_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.last_seen", v)?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.threats_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.name = ctx.threat.indicator.name ?: [];\nctx.threat.indicator.type = ctx.threat.indicator.type ?: [];\nctx.threat.indicator.email = ctx.threat.indicator.email ?: [:];\nctx.threat.indicator.email.address = ctx.threat.indicator.email.address ?: [];\nctx.threat.indicator.url = ctx.threat.indicator.url ?: [:];\nctx.threat.indicator.url.original = ctx.threat.indicator.url.original ?: [];\nctx.related = ctx.related ?: [:];\nctx.related.hash = ctx.related.hash ?: [];\nctx.related.user = ctx.related.user ?: [];\nfor (artifact in ctx.proofpoint_essentials.threat.threats_info_map) {\n  if (artifact.threat != null) {\n\n    // if artifact is hash of the attachment threat\n    if (artifact.threat.length() == 64) {\n      def str = artifact.threat.toLowerCase();\n      def hash_pattern = /^[0-9a-f]{64}$/;\n      if (hash_pattern.matcher(str).matches() && !ctx.related.hash.contains(str)) {\n        ctx.related.hash.add(str);\n      }\n    }\n\n    // if artifact is email address of the impostor sender\n    def email_pattern = /^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$/;\n    if (email_pattern.matcher(artifact.threat).matches() && !ctx.related.user.contains(artifact.threat)) {\n      ctx.threat.indicator.email.address.add(artifact.threat);\n      ctx.threat.indicator.type.add('email-addr');\n      ctx.related.user.add(artifact.threat);\n    }\n\n    // else artifact is malicious url\n    if (artifact.threat_type != null && artifact.threat_type == 'URL') {\n      ctx.threat.indicator.url.original.add(artifact.threat);\n      ctx.threat.indicator.type.add('url'); \n    }\n\n    ctx.threat.indicator.name.add(artifact.threat);\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.name = ctx.threat.indicator.name ?: [];\nctx.threat.indicator.type = ctx.threat.indicator.type ?: [];\nctx.threat.indicator.email = ctx.threat.indicator.email ?: [:];\nctx.threat.indicator.email.address = ctx.threat.indicator.email.address ?: [];\nctx.threat.indicator.url = ctx.threat.indicator.url ?: [:];\nctx.threat.indicator.url.original = ctx.threat.indicator.url.original ?: [];\nctx.related = ctx.related ?: [:];\nctx.related.hash = ctx.related.hash ?: [];\nctx.related.user = ctx.related.user ?: [];\nfor (artifact in ctx.proofpoint_essentials.threat.threats_info_map) {\n  if (artifact.threat != null) {\n\n    // if artifact is hash of the attachment threat\n    if (artifact.threat.length() == 64) {\n      def str = artifact.threat.toLowerCase();\n      def hash_pattern = /^[0-9a-f]{64}$/;\n      if (hash_pattern.matcher(str).matches() && !ctx.related.hash.contains(str)) {\n        ctx.related.hash.add(str);\n      }\n    }\n\n    // if artifact is email address of the impostor sender\n    def email_pattern = /^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$/;\n    if (email_pattern.matcher(artifact.threat).matches() && !ctx.related.user.contains(artifact.threat)) {\n      ctx.threat.indicator.email.address.add(artifact.threat);\n      ctx.threat.indicator.type.add('email-addr');\n      ctx.related.user.add(artifact.threat);\n    }\n\n    // else artifact is malicious url\n    if (artifact.threat_type != null && artifact.threat_type == 'URL') {\n      ctx.threat.indicator.url.original.add(artifact.threat);\n      ctx.threat.indicator.type.add('url'); \n    }\n\n    ctx.threat.indicator.name.add(artifact.threat);\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_threat_indicator",
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

            if event.has_value("proofpoint_essentials.threat.user_agent") {
                if let Some(ua_str) = event.get_string("proofpoint_essentials.threat.user_agent") {
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

            let _cond = { event.has_value("proofpoint_essentials.threat.from_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.from_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint_essentials.threat.sender") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.message_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "proofpoint_essentials.threat.message_parts",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.message_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "proofpoint_essentials.threat.message_parts",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.recipient")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "proofpoint_essentials.threat.recipient", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("proofpoint_essentials.threat.reply_to_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.reply_to_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint_essentials.threat.sender_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.sender_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_essentials.threat.to_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "proofpoint_essentials.threat.to_addresses",
                    |event| {
                        event.append_unique(
                            "related.user",
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

            let _cond = { event.has_value("proofpoint_essentials.threat.click_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_essentials.threat.click_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.remove("proofpoint_essentials.threat.message_details_url");
                event.remove("proofpoint_essentials.threat.click_ip");
                event.remove("proofpoint_essentials.threat.cc_addresses");
                event.remove("proofpoint_essentials.threat.from_address");
                event.remove("proofpoint_essentials.threat.message_id");
                event.remove("proofpoint_essentials.threat.recipient");
                event.remove("proofpoint_essentials.threat.reply_to_address");
                event.remove("proofpoint_essentials.threat.sender");
                event.remove("proofpoint_essentials.threat.sender_ip");
                event.remove("proofpoint_essentials.threat.subject");
                event.remove("proofpoint_essentials.threat.xmailer");
                event.remove("proofpoint_essentials.threat.to_addresses");
                event.remove("proofpoint_essentials.threat.quarantine_rule");
                event.remove("proofpoint_essentials.threat.url");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into()],
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
