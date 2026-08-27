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

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.jupiter_one = ctx.jupiter_one ?: [:];\nif (ctx.json != null) {\n  ctx.jupiter_one.asset = convertToSnakeCase(ctx.json);\n}\n// Remove json field\nctx.remove('json');
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n    def result = \"\";\n    for (int i = 0; i < str.length(); i++) {\n        char c = str.charAt(i);\n        if (Character.isUpperCase(c)) {\n            if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n                result += \"_\";\n            }\n            result += Character.toLowerCase(c);\n        } else {\n            result += c;\n        }\n    }\n    return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.jupiter_one = ctx.jupiter_one ?: [:];\nif (ctx.json != null) {\n  ctx.jupiter_one.asset = convertToSnakeCase(ctx.json);\n}\n// Remove json field\nctx.remove('json');"#
                ),
            )?;

            let _cond = {
                event.has_value("jupiter_one.asset.entity._created_on")
                    && event.get_str("jupiter_one.asset.entity._created_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("jupiter_one.asset.entity._created_on")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("jupiter_one.asset.entity._created_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "jupiter_one.asset.entity._created_on".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_entity__created_on_into_jupiter_one_asset_entity__created_on_53197dce")?;
                    if event
                        .remove("jupiter_one.asset.entity._created_on")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.entity._created_on".into(),
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
                event.has_value("jupiter_one.asset.entity._end_on")
                    && event.get_str("jupiter_one.asset.entity._end_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("jupiter_one.asset.entity._end_on")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("jupiter_one.asset.entity._end_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "jupiter_one.asset.entity._end_on".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_entity__end_on_into_jupiter_one_asset_entity__end_on_51f5d600")?;
                    if event.remove("jupiter_one.asset.entity._end_on").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.entity._end_on".into(),
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
                event.has_value("jupiter_one.asset.entity._begin_on")
                    && event.get_str("jupiter_one.asset.entity._begin_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("jupiter_one.asset.entity._begin_on")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("jupiter_one.asset.entity._begin_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "jupiter_one.asset.entity._begin_on".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_entity__begin_on_into_jupiter_one_asset_entity__begin_on_c4ed2adc")?;
                    if event.remove("jupiter_one.asset.entity._begin_on").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.entity._begin_on".into(),
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
                event.has_value("jupiter_one.asset.properties.created_on")
                    && event.get_str("jupiter_one.asset.properties.created_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("jupiter_one.asset.properties.created_on")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("jupiter_one.asset.properties.created_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.created_on".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_created_on_into_jupiter_one_asset_properties_created_on_1c9e1b12")?;
                    if event
                        .remove("jupiter_one.asset.properties.created_on")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.created_on".into(),
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
                event.has_value("jupiter_one.asset.properties.updated_on")
                    && event.get_str("jupiter_one.asset.properties.updated_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("jupiter_one.asset.properties.updated_on")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("jupiter_one.asset.properties.updated_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.updated_on".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_updated_on_into_jupiter_one_asset_properties_updated_on_b6181b48")?;
                    if event
                        .remove("jupiter_one.asset.properties.updated_on")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.updated_on".into(),
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
                if event.has_value("jupiter_one.asset.entity._deleted") {
                    if let Some(val) = event.get("jupiter_one.asset.entity._deleted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "jupiter_one.asset.entity._deleted".into(),
                                message,
                            }
                        })?;
                        event.set("jupiter_one.asset.entity._deleted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_jupiter_one_asset_entity__deleted_to_boolean_25c54b2e",
                )?;
                if event.remove("jupiter_one.asset.entity._deleted").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "jupiter_one.asset.entity._deleted".into(),
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
                if event.has_value("jupiter_one.asset.properties.active") {
                    if let Some(val) = event.get("jupiter_one.asset.properties.active") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "jupiter_one.asset.properties.active".into(),
                                message,
                            }
                        })?;
                        event.set("jupiter_one.asset.properties.active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_jupiter_one_asset_properties_active_to_boolean_5564b08c",
                )?;
                if event
                    .remove("jupiter_one.asset.properties.active")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "jupiter_one.asset.properties.active".into(),
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
                if event.has_value("jupiter_one.asset.properties.public") {
                    if let Some(val) = event.get("jupiter_one.asset.properties.public") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "jupiter_one.asset.properties.public".into(),
                                message,
                            }
                        })?;
                        event.set("jupiter_one.asset.properties.public", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_jupiter_one_asset_properties_public_to_boolean_18047c63",
                )?;
                if event
                    .remove("jupiter_one.asset.properties.public")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "jupiter_one.asset.properties.public".into(),
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
                if event.has_value("jupiter_one.asset.properties.validated") {
                    if let Some(val) = event.get("jupiter_one.asset.properties.validated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "jupiter_one.asset.properties.validated".into(),
                                message,
                            }
                        })?;
                        event.set("jupiter_one.asset.properties.validated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_jupiter_one_asset_properties_validated_to_boolean_b2fb2a7a",
                )?;
                if event
                    .remove("jupiter_one.asset.properties.validated")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "jupiter_one.asset.properties.validated".into(),
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

            if event.has_value("jupiter_one.asset.entity._version") {
                if let Some(val) = event.get("jupiter_one.asset.entity._version") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "jupiter_one.asset.entity._version".into(),
                            message,
                        }
                    })?;
                    event.set("jupiter_one.asset.entity._version", converted)?;
                }
            }

            if let Some(v) = event
                .get("jupiter_one.asset.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("jupiter_one.asset.entity._created_on")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("jupiter_one.asset.entity._end_on")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if let Some(v) = event
                .get("jupiter_one.asset.entity._begin_on")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("jupiter_one.asset.properties.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("jupiter_one.asset.properties.web_link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            let _cond = {
                event
                    .get("jupiter_one.asset.entity._class")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                        }
                        serde_json::Value::String(s) => s.contains("Vulnerability"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.description", v)?;
                }
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // Painless script
                // Source: Instant eventstart = ZonedDateTime.parse(ctx.event?.start).toInstant();\nInstant eventend = ZonedDateTime.parse(ctx.event?.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Instant eventstart = ZonedDateTime.parse(ctx.event?.start).toInstant();\nInstant eventend = ZonedDateTime.parse(ctx.event?.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);"#
                    ),
                )?;
            }

            if event.has_value("url.original") {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            let _cond = {
                event
                    .get("jupiter_one.asset.entity._class")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                        }
                        serde_json::Value::String(s) => s.contains("Vulnerability"),
                        _ => false,
                    })
                    || event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Alert"))
                            }
                            serde_json::Value::String(s) => s.contains("Alert"),
                            _ => false,
                        })
                    || event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Finding"))
                            }
                            serde_json::Value::String(s) => s.contains("Finding"),
                            _ => false,
                        })
            };
            if _cond {
                // Begin nested pipeline: "pipeline_risks_and_alerts"
                event.set("event.kind", json!("alert"))?;
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                            }
                            serde_json::Value::String(s) => s.contains("Vulnerability"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append("event.category", json!("vulnerability"))?;
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                            }
                            serde_json::Value::String(s) => s.contains("Vulnerability"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                dot_expand(event, "jupiter_one.asset.properties", "*")?;
                let _cond = {
                    event.has_value("jupiter_one.asset.properties.approved_on")
                        && event.get_str("jupiter_one.asset.properties.approved_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("jupiter_one.asset.properties.approved_on")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("jupiter_one.asset.properties.approved_on", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "jupiter_one.asset.properties.approved_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_approved_on_into_jupiter_one_asset_properties_approved_on_6c305ad2")?;
                        if event
                            .remove("jupiter_one.asset.properties.approved_on")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "jupiter_one.asset.properties.approved_on".into(),
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
                    event.has_value("jupiter_one.asset.properties.reported_on")
                        && event.get_str("jupiter_one.asset.properties.reported_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("jupiter_one.asset.properties.reported_on")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("jupiter_one.asset.properties.reported_on", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "jupiter_one.asset.properties.reported_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_reported_on_into_jupiter_one_asset_properties_reported_on_cc7dfab6")?;
                        if event
                            .remove("jupiter_one.asset.properties.reported_on")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "jupiter_one.asset.properties.reported_on".into(),
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
                    event.has_value("jupiter_one.asset.properties.detected_on")
                        && event.get_str("jupiter_one.asset.properties.detected_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("jupiter_one.asset.properties.detected_on")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("jupiter_one.asset.properties.detected_on", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "jupiter_one.asset.properties.detected_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_detected_on_into_jupiter_one_asset_properties_detected_on_2b57c1b8")?;
                        if event
                            .remove("jupiter_one.asset.properties.detected_on")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "jupiter_one.asset.properties.detected_on".into(),
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
                    event.has_value("jupiter_one.asset.properties.published_on")
                        && event.get_str("jupiter_one.asset.properties.published_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("jupiter_one.asset.properties.published_on")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("jupiter_one.asset.properties.published_on", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "jupiter_one.asset.properties.published_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_published_on_into_jupiter_one_asset_properties_published_on_1d9721a2")?;
                        if event
                            .remove("jupiter_one.asset.properties.published_on")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "jupiter_one.asset.properties.published_on".into(),
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
                    if event
                        .has_value("jupiter_one.asset.properties.total_number_of_affected_entities")
                    {
                        if let Some(val) = event
                            .get("jupiter_one.asset.properties.total_number_of_affected_entities")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                path: "jupiter_one.asset.properties.total_number_of_affected_entities".into(),
                message,
                }
                            })?;
                            event.set(
                                "jupiter_one.asset.properties.total_number_of_affected_entities",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_total_number_of_affected_entities_to_long_cf15bf27")?;
                    if event
                        .remove("jupiter_one.asset.properties.total_number_of_affected_entities")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.total_number_of_affected_entities"
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
                    if event.has_value("jupiter_one.asset.properties.numeric_severity") {
                        if let Some(val) =
                            event.get("jupiter_one.asset.properties.numeric_severity")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.numeric_severity".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("jupiter_one.asset.properties.numeric_severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_numeric_severity_to_long_5fa6bfd5",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.numeric_severity")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.numeric_severity".into(),
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
                    if event.has_value("jupiter_one.asset.properties.remediation_sla") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.remediation_sla")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.remediation_sla".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.remediation_sla", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_remediation_sla_to_long_231e4476",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.remediation_sla")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.remediation_sla".into(),
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
                    if event.has_value("jupiter_one.asset.properties.exploit_status") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.exploit_status")
                        {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.exploit_status".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.exploit_status", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_exploit_status_to_long_155fdf22",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.exploit_status")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.exploit_status".into(),
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
                    if event.has_value("jupiter_one.asset.properties.exploitability") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.exploitability")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.exploitability".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.exploitability", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_exploitability_to_double_f4e14120",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.exploitability")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.exploitability".into(),
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
                    if event.has_value("jupiter_one.asset.properties.impact") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.impact") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.impact".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.impact", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_impact_to_double_2d857527",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.impact")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.impact".into(),
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
                    if event.has_value("jupiter_one.asset.properties.score") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.score".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_score_to_double_4eba43d9",
                    )?;
                    if event.remove("jupiter_one.asset.properties.score").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.score".into(),
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
                    if event.has_value("jupiter_one.asset.properties.open") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.open") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.open".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.open", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_open_to_boolean_1597bca6",
                    )?;
                    if event.remove("jupiter_one.asset.properties.open").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.open".into(),
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
                    if event.has_value("jupiter_one.asset.properties.approved") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.approved") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.approved".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.approved", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_approved_to_boolean_154e80e1",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.approved")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.approved".into(),
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
                    if event.has_value("jupiter_one.asset.properties.exception") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.exception") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.exception".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.exception", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_exception_to_boolean_7c919b89",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.exception")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.exception".into(),
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
                    if event.has_value("jupiter_one.asset.properties.production") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.production") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.production".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.production", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_production_to_boolean_65f41341",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.production")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.production".into(),
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
                    if event.has_value("jupiter_one.asset.properties.blocks_production") {
                        if let Some(val) =
                            event.get("jupiter_one.asset.properties.blocks_production")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.blocks_production".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("jupiter_one.asset.properties.blocks_production", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_blocks_production_to_boolean_b3238d5c")?;
                    if event
                        .remove("jupiter_one.asset.properties.blocks_production")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.blocks_production".into(),
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
                    if event.has_value("jupiter_one.asset.properties.tag.production") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.tag.production")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.tag.production".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.tag.production", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_tag_production_to_boolean_b22c5127",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.tag.production")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.tag.production".into(),
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
                    if event.has_value("jupiter_one.asset.properties.blocking") {
                        if let Some(val) = event.get("jupiter_one.asset.properties.blocking") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jupiter_one.asset.properties.blocking".into(),
                                    message,
                                }
                            })?;
                            event.set("jupiter_one.asset.properties.blocking", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_jupiter_one_asset_properties_blocking_to_boolean_ac1cebd9",
                    )?;
                    if event
                        .remove("jupiter_one.asset.properties.blocking")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "jupiter_one.asset.properties.blocking".into(),
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
                let _cond =
                    { event.get_str("jupiter_one.asset.properties.device_local_ip") != Some("") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("jupiter_one.asset.properties.device_local_ip") {
                            if let Some(val) =
                                event.get("jupiter_one.asset.properties.device_local_ip")
                            {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "jupiter_one.asset.properties.device_local_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jupiter_one.asset.properties.device_local_ip",
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
                            "convert_jupiter_one_asset_properties_device_local_ip_to_ip_3375672f",
                        )?;
                        if event
                            .remove("jupiter_one.asset.properties.device_local_ip")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "jupiter_one.asset.properties.device_local_ip".into(),
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
                    event.get_str("jupiter_one.asset.properties.device_external_ip") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("jupiter_one.asset.properties.device_external_ip") {
                            if let Some(val) =
                                event.get("jupiter_one.asset.properties.device_external_ip")
                            {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "jupiter_one.asset.properties.device_external_ip"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jupiter_one.asset.properties.device_external_ip",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_device_external_ip_to_ip_b86c1d0b")?;
                        if event
                            .remove("jupiter_one.asset.properties.device_external_ip")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "jupiter_one.asset.properties.device_external_ip".into(),
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
                let _cond = { event.has_value("jupiter_one.asset.properties.reporter") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.reporter")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.device_local_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.device_local_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.user_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.device_external_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.device_external_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.device_hostname") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.device_hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.user_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.user_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.properties.approvers")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "jupiter_one.asset.properties.approvers", |event| {
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
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.level")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("log.level", v)?;
                }
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.device_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.user_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.device_os_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.device_hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.device_platform_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.platform", v)?;
                }
                if let Some(v) = event
                    .get("jupiter_one.asset.properties.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if event.has_value("log.level") {
                    map_strings(event, "log.level", "log.level", str::to_lowercase)?;
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                            }
                            serde_json::Value::String(s) => s.contains("Vulnerability"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("jupiter_one.asset.properties.cve_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("vulnerability.id", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                            }
                            serde_json::Value::String(s) => s.contains("Vulnerability"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("jupiter_one.asset.properties.score")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("vulnerability.score.base", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                            }
                            serde_json::Value::String(s) => s.contains("Vulnerability"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("jupiter_one.asset.properties.severity")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("vulnerability.severity", v)?;
                    }
                }
                if event.has_value("vulnerability.severity") {
                    map_strings(
                        event,
                        "vulnerability.severity",
                        "vulnerability.severity",
                        str::to_lowercase,
                    )?;
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Finding"))
                            }
                            serde_json::Value::String(s) => s.contains("Finding"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("jupiter_one.asset.properties.filename")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.name", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Finding"))
                            }
                            serde_json::Value::String(s) => s.contains("Finding"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("jupiter_one.asset.properties.filepath")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.path", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("jupiter_one.asset.entity._class")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Finding"))
                            }
                            serde_json::Value::String(s) => s.contains("Finding"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("jupiter_one.asset.properties.device_external_ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.ip", v)?;
                    }
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.device_mac_address") };
                if _cond {
                    event.append_unique(
                        "host.mac",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.device_mac_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("jupiter_one.asset.properties.category")
                        && event
                            .get("jupiter_one.asset.entity._class")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Vulnerability"))
                                }
                                serde_json::Value::String(s) => s.contains("Vulnerability"),
                                _ => false,
                            })
                };
                if _cond {
                    event.append_unique(
                        "vulnerability.category",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.category")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("jupiter_one.asset.properties.technique_id")
                        && event
                            .get("jupiter_one.asset.entity._class")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Finding"))
                                }
                                serde_json::Value::String(s) => s.contains("Finding"),
                                _ => false,
                            })
                };
                if _cond {
                    event.append_unique(
                        "threat.technique.id",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.technique_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("jupiter_one.asset.properties.tactic_id")
                        && event
                            .get("jupiter_one.asset.entity._class")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Finding"))
                                }
                                serde_json::Value::String(s) => s.contains("Finding"),
                                _ => false,
                            })
                };
                if _cond {
                    event.append_unique(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.tactic_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("jupiter_one.asset.properties.tactic")
                        && event
                            .get("jupiter_one.asset.entity._class")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Finding"))
                                }
                                serde_json::Value::String(s) => s.contains("Finding"),
                                _ => false,
                            })
                };
                if _cond {
                    event.append_unique(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.tactic")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("jupiter_one.asset.properties.technique")
                        && event
                            .get("jupiter_one.asset.entity._class")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Finding"))
                                }
                                serde_json::Value::String(s) => s.contains("Finding"),
                                _ => false,
                            })
                };
                if _cond {
                    event.append_unique(
                        "threat.technique.name",
                        json!(
                            event
                                .get("jupiter_one.asset.properties.technique")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("jupiter_one.asset.properties.cve_id") };
                if _cond {
                    event.set("vulnerability.enumeration", json!("CVE"))?;
                }
                // End nested pipeline: "pipeline_risks_and_alerts"
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
                event.remove("jupiter_one.asset.id");
                event.remove("jupiter_one.asset.entity._created_on");
                event.remove("jupiter_one.asset.entity._end_on");
                event.remove("jupiter_one.asset.entity._begin_on");
                event.remove("jupiter_one.asset.properties.web_link");
                event.remove("jupiter_one.asset.properties.level");
                event.remove("jupiter_one.asset.properties.device_id");
                event.remove("jupiter_one.asset.properties.user_id");
                event.remove("jupiter_one.asset.properties.device_mac_address");
                event.remove("jupiter_one.asset.properties.device_os_version");
                event.remove("jupiter_one.asset.properties.device_hostname");
                event.remove("jupiter_one.asset.properties.device_platform_name");
                event.remove("jupiter_one.asset.properties.user_name");
            }

            // Painless script
            // Source: void handleMap(Map map) {\nmap.values().removeIf(v -> {\n    if (v instanceof Map) {\n    handleMap(v);\n    } else if (v instanceof List) {\n    handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n    if (v instanceof Map) {\n    handleMap(v);\n    } else if (v instanceof List) {\n    handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\nmap.values().removeIf(v -> {\n    if (v instanceof Map) {\n    handleMap(v);\n    } else if (v instanceof List) {\n    handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n    if (v instanceof Map) {\n    handleMap(v);\n    } else if (v instanceof List) {\n    handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);"#
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
