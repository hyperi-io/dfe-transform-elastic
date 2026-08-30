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
            let _cond = { event.get_str("message") == Some("empty_events_placeholder") };
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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            event.set("event.kind", json!("event"))?;

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.ironscales = ctx.ironscales ?: [:];\nif (ctx.json != null) {\n  ctx.ironscales.incident = convertToSnakeCase(ctx.json);\n}\n// Remove json field\nctx.remove('json');
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.ironscales = ctx.ironscales ?: [:];\nif (ctx.json != null) {\n  ctx.ironscales.incident = convertToSnakeCase(ctx.json);\n}\n// Remove json field\nctx.remove('json');"#
                ),
            )?;

            let _cond = {
                event.has_value("ironscales.incident.created")
                    && event.get_str("ironscales.incident.created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("ironscales.incident.created") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'",
                                "yyyy-MM-dd'T'HH:mm:ss'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("ironscales.incident.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ironscales.incident.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ironscales_incident_created_into_ironscales_incident_created_e6476b05")?;
                    if event.remove("ironscales.incident.created").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ironscales.incident.created".into(),
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
                event.has_value("ironscales.incident.first_challenged_date")
                    && event.get_str("ironscales.incident.first_challenged_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ironscales.incident.first_challenged_date")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'",
                                "yyyy-MM-dd'T'HH:mm:ss'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ironscales.incident.first_challenged_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ironscales.incident.first_challenged_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ironscales_incident_first_challenged_date_into_ironscales_incident_first_challenged_date_65692e2b")?;
                    if event
                        .remove("ironscales.incident.first_challenged_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ironscales.incident.first_challenged_date".into(),
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
                event.has_value("ironscales.incident.latest_email_date")
                    && event.get_str("ironscales.incident.latest_email_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ironscales.incident.latest_email_date")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'",
                                "yyyy-MM-dd'T'HH:mm:ss'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ironscales.incident.latest_email_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ironscales.incident.latest_email_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ironscales_incident_latest_email_date_into_ironscales_incident_latest_email_date_45686623")?;
                    if event
                        .remove("ironscales.incident.latest_email_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ironscales.incident.latest_email_date".into(),
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
                event.has_value("ironscales.incident.first_reported_date")
                    && event.get_str("ironscales.incident.first_reported_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ironscales.incident.first_reported_date")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'",
                                "yyyy-MM-dd'T'HH:mm:ss'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ironscales.incident.first_reported_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ironscales.incident.first_reported_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ironscales_incident_first_reported_date_into_ironscales_incident_first_reported_date_eb393323")?;
                    if event
                        .remove("ironscales.incident.first_reported_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ironscales.incident.first_reported_date".into(),
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
                if event.has_value("ironscales.incident.links_count") {
                    if let Some(val) = event.get("ironscales.incident.links_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.links_count".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.links_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_links_count_to_long_b7d34611",
                )?;
                if event.remove("ironscales.incident.links_count").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.links_count".into(),
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
                if event.has_value("ironscales.incident.attachments_count") {
                    if let Some(val) = event.get("ironscales.incident.attachments_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.attachments_count".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.attachments_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_attachments_count_to_long_d9b6a438",
                )?;
                if event
                    .remove("ironscales.incident.attachments_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.attachments_count".into(),
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
                if event.has_value("ironscales.incident.affected_mailboxes_count") {
                    if let Some(val) = event.get("ironscales.incident.affected_mailboxes_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.affected_mailboxes_count".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.affected_mailboxes_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_affected_mailboxes_count_to_long_e2479f5d",
                )?;
                if event
                    .remove("ironscales.incident.affected_mailboxes_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.affected_mailboxes_count".into(),
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
                if event.has_value("ironscales.incident.comments_count") {
                    if let Some(val) = event.get("ironscales.incident.comments_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.comments_count".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.comments_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_comments_count_to_long_b88dc286",
                )?;
                if event.remove("ironscales.incident.comments_count").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.comments_count".into(),
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
                if event.has_value("ironscales.incident.release_request_count") {
                    if let Some(val) = event.get("ironscales.incident.release_request_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.release_request_count".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.release_request_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_release_request_count_to_long_38052ea7",
                )?;
                if event
                    .remove("ironscales.incident.release_request_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.release_request_count".into(),
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
                if event.has_value("ironscales.incident.federation.companies_affected") {
                    if let Some(val) =
                        event.get("ironscales.incident.federation.companies_affected")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.federation.companies_affected".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "ironscales.incident.federation.companies_affected",
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
                    "convert_ironscales_incident_federation_companies_affected_to_long_beba1771",
                )?;
                if event
                    .remove("ironscales.incident.federation.companies_affected")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.federation.companies_affected".into(),
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
                if event.has_value("ironscales.incident.federation.companies_marked_phishing") {
                    if let Some(val) =
                        event.get("ironscales.incident.federation.companies_marked_phishing")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.federation.companies_marked_phishing"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "ironscales.incident.federation.companies_marked_phishing",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ironscales_incident_federation_companies_marked_phishing_to_long_6c2a444e")?;
                if event
                    .remove("ironscales.incident.federation.companies_marked_phishing")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.federation.companies_marked_phishing".into(),
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
                if event.has_value("ironscales.incident.federation.companies_marked_spam") {
                    if let Some(val) =
                        event.get("ironscales.incident.federation.companies_marked_spam")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.federation.companies_marked_spam".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "ironscales.incident.federation.companies_marked_spam",
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
                    "convert_ironscales_incident_federation_companies_marked_spam_to_long_052de98d",
                )?;
                if event
                    .remove("ironscales.incident.federation.companies_marked_spam")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.federation.companies_marked_spam".into(),
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
                if event.has_value("ironscales.incident.federation.companies_marked_fp") {
                    if let Some(val) =
                        event.get("ironscales.incident.federation.companies_marked_fp")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.federation.companies_marked_fp".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "ironscales.incident.federation.companies_marked_fp",
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
                    "convert_ironscales_incident_federation_companies_marked_fp_to_long_bb8b0a28",
                )?;
                if event
                    .remove("ironscales.incident.federation.companies_marked_fp")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.federation.companies_marked_fp".into(),
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
                if event.has_value("ironscales.incident.federation.companies_unclassified") {
                    if let Some(val) =
                        event.get("ironscales.incident.federation.companies_unclassified")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.federation.companies_unclassified"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "ironscales.incident.federation.companies_unclassified",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ironscales_incident_federation_companies_unclassified_to_long_01254d01")?;
                if event
                    .remove("ironscales.incident.federation.companies_unclassified")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.federation.companies_unclassified".into(),
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
                event
                    .get("ironscales.incident.attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ironscales.incident.attachments").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.file_size") {
                                    if let Some(val) = event.get("_ingest._value.file_size") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.file_size".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.file_size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert__ingest__value_file_size_to_long_01e49221",
                                )?;
                                if event.remove("_ingest._value.file_size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.file_size".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "ironscales.incident.attachments",
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
                    .get("ironscales.incident.related_incidents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ironscales.incident.related_incidents").cloned();
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
                                    "convert__ingest__value_to_long_a54d4db3",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "ironscales.incident.related_incidents",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("ironscales.incident.themis_proba") {
                    if let Some(val) = event.get("ironscales.incident.themis_proba") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.themis_proba".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.themis_proba", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_themis_proba_to_double_fbb76e50",
                )?;
                if event.remove("ironscales.incident.themis_proba").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.themis_proba".into(),
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
                if event.has_value("ironscales.incident.federation.phishing_ratio") {
                    if let Some(val) = event.get("ironscales.incident.federation.phishing_ratio") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.federation.phishing_ratio".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.federation.phishing_ratio", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_federation_phishing_ratio_to_double_aee0c0d8",
                )?;
                if event
                    .remove("ironscales.incident.federation.phishing_ratio")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.federation.phishing_ratio".into(),
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
                if event.has_value("ironscales.incident.sender_is_internal") {
                    if let Some(val) = event.get("ironscales.incident.sender_is_internal") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.sender_is_internal".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.sender_is_internal", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_sender_is_internal_to_boolean_67ecc208",
                )?;
                if event
                    .remove("ironscales.incident.sender_is_internal")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.sender_is_internal".into(),
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
                if event.has_value("ironscales.incident.reported_by_end_user") {
                    if let Some(val) = event.get("ironscales.incident.reported_by_end_user") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "ironscales.incident.reported_by_end_user".into(),
                                message,
                            }
                        })?;
                        event.set("ironscales.incident.reported_by_end_user", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ironscales_incident_reported_by_end_user_to_boolean_3c785e39",
                )?;
                if event
                    .remove("ironscales.incident.reported_by_end_user")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ironscales.incident.reported_by_end_user".into(),
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

            if event.has_value("ironscales.incident.incident_id") {
                if let Some(val) = event.get("ironscales.incident.incident_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "ironscales.incident.incident_id".into(),
                            message,
                        }
                    })?;
                    event.set("ironscales.incident.incident_id", converted)?;
                }
            }

            if event.has_value("ironscales.incident.company_id") {
                if let Some(val) = event.get("ironscales.incident.company_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "ironscales.incident.company_id".into(),
                            message,
                        }
                    })?;
                    event.set("ironscales.incident.company_id", converted)?;
                }
            }

            let _cond = { event.get_str("ironscales.incident.mail_server.ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("ironscales.incident.mail_server.ip") {
                        if let Some(val) = event.get("ironscales.incident.mail_server.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "ironscales.incident.mail_server.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("ironscales.incident.mail_server.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ironscales_incident_mail_server_ip_to_ip_e9ff4c79",
                    )?;
                    if event.remove("ironscales.incident.mail_server.ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ironscales.incident.mail_server.ip".into(),
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
                    .get("ironscales.incident.reports")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ironscales.incident.reports").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.mail_server.ip") {
                                    if let Some(val) = event.get("_ingest._value.mail_server.ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.mail_server.ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.mail_server.ip", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert__ingest__value_mail_server_ip_to_ip_4a044e6a",
                                )?;
                                if event.remove("_ingest._value.mail_server.ip").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.mail_server.ip".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "ironscales.incident.reports",
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
                .get("ironscales.incident.incident_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("ironscales.incident.email_subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("ironscales.incident.assignee")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("ironscales.incident.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("ironscales.incident.company_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event
                .get("ironscales.incident.company_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if let Some(v) = event
                .get("ironscales.incident.mail_server.host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            let _cond = { event.has_value("ironscales.incident.recipient_email") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("ironscales.incident.recipient_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.sender_email") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("ironscales.incident.sender_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.reply_to") };
            if _cond {
                event.append_unique(
                    "email.reply_to.address",
                    json!(
                        event
                            .get("ironscales.incident.reply_to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.mail_server.ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("ironscales.incident.mail_server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("ironscales.incident.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ironscales.incident.links", |event| {
                    event.append_unique(
                        "url.full",
                        json!(
                            event
                                .get("_ingest._value.url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ironscales.incident.attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: // Initialize ECS email.attachments structure\nctx.email = ctx.email ?: [:];\nctx.email.attachments = ctx.email.attachments ?: [:];\nctx.email.attachments.file = ctx.email.attachments.file ?: [:];\nctx.email.attachments.file.name = ctx.email.attachments.file.name ?: [];\nctx.email.attachments.file.hash = ctx.email.attachments.file.hash ?: [:];\nctx.email.attachments.file.hash.md5 = ctx.email.attachments.file.hash.md5 ?: [];\n\n// Single iteration to extract file_name and md5 from attachments array\nfor (attachment in ctx.ironscales.incident.attachments) {\n  if (attachment.file_name != null && !ctx.email.attachments.file.name.contains(attachment.file_name)) {\n    ctx.email.attachments.file.name.add(attachment.file_name);\n  }\n  if (attachment.md5 != null && !ctx.email.attachments.file.hash.md5.contains(attachment.md5)) {\n    ctx.email.attachments.file.hash.md5.add(attachment.md5);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Initialize ECS email.attachments structure\nctx.email = ctx.email ?: [:];\nctx.email.attachments = ctx.email.attachments ?: [:];\nctx.email.attachments.file = ctx.email.attachments.file ?: [:];\nctx.email.attachments.file.name = ctx.email.attachments.file.name ?: [];\nctx.email.attachments.file.hash = ctx.email.attachments.file.hash ?: [:];\nctx.email.attachments.file.hash.md5 = ctx.email.attachments.file.hash.md5 ?: [];\n\n// Single iteration to extract file_name and md5 from attachments array\nfor (attachment in ctx.ironscales.incident.attachments) {\n  if (attachment.file_name != null && !ctx.email.attachments.file.name.contains(attachment.file_name)) {\n    ctx.email.attachments.file.name.add(attachment.file_name);\n  }\n  if (attachment.md5 != null && !ctx.email.attachments.file.hash.md5.contains(attachment.md5)) {\n    ctx.email.attachments.file.hash.md5.add(attachment.md5);\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.recipient_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.recipient_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.recipient_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.recipient_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.assignee") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.assignee")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.sender_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.sender_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.sender_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.sender_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.resolved_by") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.resolved_by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.reply_to") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.reply_to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.reporter_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ironscales.incident.reporter_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.mail_server.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("ironscales.incident.mail_server.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ironscales.incident.mail_server.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("ironscales.incident.mail_server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("ironscales.incident.reports")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: // Initialize ECS related fields structure\nctx.related = ctx.related ?: [:];\nctx.related.user = ctx.related.user ?: [];\nctx.related.hosts = ctx.related.hosts ?: [];\nctx.related.ip = ctx.related.ip ?: [];\n\n// Single iteration to extract name, email, sender_email, mail_server.host, and mail_server.ip from reports array\nfor (report in ctx.ironscales.incident.reports) {\n  if (report?.name != null && !ctx.related.user.contains(report.name)) {\n    ctx.related.user.add(report.name);\n  }\n  if (report?.email != null && !ctx.related.user.contains(report.email)) {\n    ctx.related.user.add(report.email);\n  }\n  if (report?.sender_email != null && !ctx.related.user.contains(report.sender_email)) {\n    ctx.related.user.add(report.sender_email);\n  }\n  if (report?.mail_server?.host != null && !ctx.related.hosts.contains(report.mail_server.host)) {\n    ctx.related.hosts.add(report.mail_server.host);\n  }\n  if (report?.mail_server?.ip != null && !ctx.related.ip.contains(report.mail_server.ip)) {\n    ctx.related.ip.add(report.mail_server.ip);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Initialize ECS related fields structure\nctx.related = ctx.related ?: [:];\nctx.related.user = ctx.related.user ?: [];\nctx.related.hosts = ctx.related.hosts ?: [];\nctx.related.ip = ctx.related.ip ?: [];\n\n// Single iteration to extract name, email, sender_email, mail_server.host, and mail_server.ip from reports array\nfor (report in ctx.ironscales.incident.reports) {\n  if (report?.name != null && !ctx.related.user.contains(report.name)) {\n    ctx.related.user.add(report.name);\n  }\n  if (report?.email != null && !ctx.related.user.contains(report.email)) {\n    ctx.related.user.add(report.email);\n  }\n  if (report?.sender_email != null && !ctx.related.user.contains(report.sender_email)) {\n    ctx.related.user.add(report.sender_email);\n  }\n  if (report?.mail_server?.host != null && !ctx.related.hosts.contains(report.mail_server.host)) {\n    ctx.related.hosts.add(report.mail_server.host);\n  }\n  if (report?.mail_server?.ip != null && !ctx.related.ip.contains(report.mail_server.ip)) {\n    ctx.related.ip.add(report.mail_server.ip);\n  }\n}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("ironscales.incident.links")
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
                foreach_array(event, "ironscales.incident.links", |event| {
                    event.remove("_ingest._value.url");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ironscales.incident.attachments")
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
                foreach_array(event, "ironscales.incident.attachments", |event| {
                    event.remove("_ingest._value.file_name");
                    event.remove("_ingest._value.md5");
                    Ok(())
                })?;
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
                event.remove("ironscales.incident.incident_id");
                event.remove("ironscales.incident.email_subject");
                event.remove("ironscales.incident.recipient_email");
                event.remove("ironscales.incident.assignee");
                event.remove("ironscales.incident.sender_email");
                event.remove("ironscales.incident.created");
                event.remove("ironscales.incident.company_id");
                event.remove("ironscales.incident.company_name");
                event.remove("ironscales.incident.reply_to");
                event.remove("ironscales.incident.mail_server.host");
                event.remove("ironscales.incident.mail_server.ip");
            }

            event.remove("ironscales.incident.affected_mailbox_count");

            // Painless script
            // Source: void handleMap(Map map) {\nmap.values().removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\nmap.values().removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);"#
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
