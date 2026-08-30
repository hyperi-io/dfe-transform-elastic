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

            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("malware")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "json")?;
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("json") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.suspicionsMap = ctx.json[1]; ctx.evidenceMap = ctx.json[2]; ctx.json = ctx.json[0];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.suspicionsMap = ctx.json[1]; ctx.evidenceMap = ctx.json[2]; ctx.json = ctx.json[0];"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_parse_different_message_object",
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.guidString") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.elementValues.calculatedUser.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.calculatedUser.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.calculatedUser.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.calculatedUser.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_calculatedUser_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.calculatedUser.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.calculatedUser.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_calculatedUser_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.calculatedUser.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.calculatedUser.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.calculatedUser.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.calculatedUser.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.calculatedUser.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.calculatedUser.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.calculatedUser.elementValues") {
                event.rename(
                    "json.elementValues.calculatedUser.elementValues",
                    "cybereason.suspicions_process.element_values.calculated_user.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.calculatedUser.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.calculatedUser.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.calculatedUser.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.calculated_user.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_calculatedUser_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.calculatedUser.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.calculatedUser.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.calculatedUser.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.calculated_user.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_calculatedUser_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.calculatedUser.totalSuspicious") {
                    if let Some(val) =
                        event.get("json.elementValues.calculatedUser.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.calculatedUser.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.calculated_user.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_calculatedUser_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.calculatedUser.totalValues") {
                    if let Some(val) = event.get("json.elementValues.calculatedUser.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.calculatedUser.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.calculated_user.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_calculatedUser_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.children.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.children.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.children.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.children.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_children_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.children.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.children.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_children_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.children.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.children.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.children.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.children.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.children.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.children.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.children.elementValues") {
                event.rename(
                    "json.elementValues.children.elementValues",
                    "cybereason.suspicions_process.element_values.children.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.children.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.children.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.children.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.element_values.children.guessed_total",
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
                    "convert_elementValues_children_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.children.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.children.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.children.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.element_values.children.total_malicious",
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
                    "convert_elementValues_children_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.children.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.children.totalSuspicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.children.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.children.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_children_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.children.totalValues") {
                    if let Some(val) = event.get("json.elementValues.children.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.children.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.element_values.children.total_values",
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
                    "convert_elementValues_children_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            event.append_unique(
                                "file.uid",
                                json!(
                                    event
                                        .get("_ingest._value.guid")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_imageFile_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_imageFile_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            event.append_unique(
                                "file.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.imageFile.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.imageFile.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.imageFile.elementValues") {
                event.rename(
                    "json.elementValues.imageFile.elementValues",
                    "cybereason.suspicions_process.element_values.image_file.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.imageFile.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.imageFile.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.imageFile.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.element_values.image_file.guessed_total",
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
                    "convert_elementValues_imageFile_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.imageFile.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.imageFile.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.imageFile.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.image_file.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_imageFile_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.imageFile.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.imageFile.totalSuspicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.imageFile.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.image_file.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_imageFile_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.imageFile.totalValues") {
                    if let Some(val) = event.get("json.elementValues.imageFile.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.imageFile.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.element_values.image_file.total_values",
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
                    "convert_elementValues_imageFile_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.rename(
                                "_ingest._value.elementType",
                                "_ingest._value.element_type",
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "process.real_user.id",
                                json!(
                                    event
                                        .get("_ingest._value.guid")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.guid")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerMachine_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerMachine_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "process.real_user.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.ownerMachine.elementValues") {
                event.rename(
                    "json.elementValues.ownerMachine.elementValues",
                    "cybereason.suspicions_process.element_values.owner_machine.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerMachine.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.owner_machine.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.ownerMachine.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.owner_machine.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.ownerMachine.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.owner_machine.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.ownerMachine.totalValues") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.owner_machine.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            event.append_unique(
                                "process.parent.entity_id",
                                json!(
                                    event
                                        .get("_ingest._value.guid")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_parentProcess_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_parentProcess_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
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
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            event.append_unique(
                                "process.parent.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.parentProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.parentProcess.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.parentProcess.elementValues") {
                event.rename(
                    "json.elementValues.parentProcess.elementValues",
                    "cybereason.suspicions_process.element_values.parent_process.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.parentProcess.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.parentProcess.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.parentProcess.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.parent_process.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_parentProcess_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.parentProcess.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.parentProcess.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.parentProcess.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.parent_process.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_parentProcess_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.parentProcess.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.parentProcess.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.parentProcess.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.parent_process.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_parentProcess_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.parentProcess.totalValues") {
                    if let Some(val) = event.get("json.elementValues.parentProcess.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.parentProcess.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.element_values.parent_process.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_parentProcess_totalValues_to_long",
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

            if event.has_value("evidenceMap") {
                event.rename("evidenceMap", "cybereason.suspicions_process.evidence_map")?;
            }

            if event.has_value("json.filterData.groupByValue") {
                event.rename(
                    "json.filterData.groupByValue",
                    "cybereason.suspicions_process.filter_data.group_by_value",
                )?;
            }

            if event.has_value("json.filterData.sortInGroupValue") {
                event.rename(
                    "json.filterData.sortInGroupValue",
                    "cybereason.suspicions_process.filter_data.sort_in_group_value",
                )?;
            }

            if event.has_value("json.guidString") {
                event.rename(
                    "json.guidString",
                    "cybereason.suspicions_process.guid_string",
                )?;
            }

            if let Some(v) = event
                .get("cybereason.suspicions_process.guid_string")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.isMalicious") {
                    if let Some(val) = event.get("json.isMalicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.is_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isMalicious_to_boolean",
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

            if event.has_value("json.labelsIds") {
                event.rename("json.labelsIds", "cybereason.suspicions_process.labels_ids")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.malicious") {
                    if let Some(val) = event.get("json.malicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.malicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_malicious_to_boolean",
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

            if event.has_value("json.malopPriority") {
                event.rename(
                    "json.malopPriority",
                    "cybereason.suspicions_process.malop_priority",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.commandLine.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.commandLine.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.commandLine.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.simple_values.command_line.total_values",
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
                    "convert_simpleValues_commandLine_totalValues_to_long",
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

            if event.has_value("json.simpleValues.commandLine.values") {
                event.rename(
                    "json.simpleValues.commandLine.values",
                    "cybereason.suspicions_process.simple_values.command_line.values",
                )?;
            }

            if let Some(v) = event
                .get("cybereason.suspicions_process.simple_values.command_line.values")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.creationTime.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.creationTime.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.creationTime.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.simple_values.creation_time.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_creationTime_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.creationTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.simpleValues.creationTime.values").cloned();
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
                                    if let Some(date_str) = event.get_as_string("_ingest._value") {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event.set("_ingest._value", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value".into(),
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
                                        "date_simpleValues_creationTime_values",
                                    )?;
                                    event.remove("_ingest._value");
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
                                "json.simpleValues.creationTime.values",
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
            }

            let _cond = { event.has_value("json.simpleValues.creationTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.creationTime.values", |event| {
                        event.append_unique(
                            "event.created",
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
            }

            if event.has_value("json.simpleValues.creationTime.values") {
                event.rename(
                    "json.simpleValues.creationTime.values",
                    "cybereason.suspicions_process.simple_values.creation_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.elementDisplayName.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.elementDisplayName.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.elementDisplayName.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.simple_values.element_display_name.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_elementDisplayName_totalValues_to_long",
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

            if event.has_value("json.simpleValues.elementDisplayName.values") {
                event.rename(
                    "json.simpleValues.elementDisplayName.values",
                    "cybereason.suspicions_process.simple_values.element_display_name.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.endTime.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.endTime.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.endTime.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.simple_values.end_time.total_values",
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
                    "convert_simpleValues_endTime_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.endTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.simpleValues.endTime.values").cloned();
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
                                    if let Some(date_str) = event.get_as_string("_ingest._value") {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event.set("_ingest._value", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value".into(),
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
                                        "date_simpleValues_endTime_values",
                                    )?;
                                    event.remove("_ingest._value");
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
                                "json.simpleValues.endTime.values",
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
            }

            let _cond = { event.has_value("json.simpleValues.endTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.endTime.values", |event| {
                        if let Some(v) = event
                            .get("_ingest._value")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("@timestamp", v)?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.endTime.values") {
                event.rename(
                    "json.simpleValues.endTime.values",
                    "cybereason.suspicions_process.simple_values.end_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.executionPrevented.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.executionPrevented.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.executionPrevented.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.simple_values.execution_prevented.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_executionPrevented_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.executionPrevented.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.executionPrevented.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
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
                                    "convert_simpleValues_executionPrevented_values_to_boolean",
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
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.executionPrevented.values") {
                event.rename(
                    "json.simpleValues.executionPrevented.values",
                    "cybereason.suspicions_process.simple_values.execution_prevented.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.group.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.group.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.group.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.simple_values.group.total_values",
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
                    "convert_simpleValues_group_totalValues_to_long",
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

            if event.has_value("json.simpleValues.group.values") {
                event.rename(
                    "json.simpleValues.group.values",
                    "cybereason.suspicions_process.simple_values.group.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.iconBase64.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.iconBase64.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.iconBase64.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.simple_values.icon_base64.total_values",
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
                    "convert_simpleValues_iconBase64_totalValues_to_long",
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

            if event.has_value("json.simpleValues.iconBase64.values") {
                event.rename(
                    "json.simpleValues.iconBase64.values",
                    "cybereason.suspicions_process.simple_values.icon_base64.values",
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.json?.simpleValues["imageFile.companyName"] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: def obj = ctx.json.simpleValues.remove(\"imageFile.companyName\"); ctx.cybereason.suspicions_process.simple_values.image_file_company_name = obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.simpleValues.remove(\"imageFile.companyName\"); ctx.cybereason.suspicions_process.simple_values.image_file_company_name = obj;"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cybereason.suspicions_process.simple_values.image_file_company_name.totalValues") {
                if let Some(val) = event.get("cybereason.suspicions_process.simple_values.image_file_company_name.totalValues") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.suspicions_process.simple_values.image_file_company_name.totalValues".into(),
                            message,
                        })?;
                    event.set("cybereason.suspicions_process.simple_values.image_file_company_name.total_values", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_imageFile_companyName_totalValues_to_long",
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

            // SKIPPED: condition not transpiled: ctx.json?.simpleValues["imageFile.fileHash.iconBase64"] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: def obj = ctx.json.simpleValues.remove(\"imageFile.fileHash.iconBase64\"); ctx.cybereason.suspicions_process.simple_values.image_file_hash_icon_base64 = obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.simpleValues.remove(\"imageFile.fileHash.iconBase64\"); ctx.cybereason.suspicions_process.simple_values.image_file_hash_icon_base64 = obj;"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cybereason.suspicions_process.simple_values.image_file_hash_icon_base64.totalValues") {
                if let Some(val) = event.get("cybereason.suspicions_process.simple_values.image_file_hash_icon_base64.totalValues") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.suspicions_process.simple_values.image_file_hash_icon_base64.totalValues".into(),
                            message,
                        })?;
                    event.set("cybereason.suspicions_process.simple_values.image_file_hash_icon_base64.total_values", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_imageFile_fileHash_iconBase64_totalValues_to_long",
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

            // SKIPPED: condition not transpiled: ctx.json?.simpleValues["imageFile.maliciousClassificationType"] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: def obj = ctx.json.simpleValues.remove(\"imageFile.maliciousClassificationType\"); ctx.cybereason.suspicions_process.simple_values.image_file_malicious_classification_type = obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.simpleValues.remove(\"imageFile.maliciousClassificationType\"); ctx.cybereason.suspicions_process.simple_values.image_file_malicious_classification_type = obj;"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cybereason.suspicions_process.simple_values.image_file_malicious_classification_type.totalValues") {
                if let Some(val) = event.get("cybereason.suspicions_process.simple_values.image_file_malicious_classification_type.totalValues") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.suspicions_process.simple_values.image_file_malicious_classification_type.totalValues".into(),
                            message,
                        })?;
                    event.set("cybereason.suspicions_process.simple_values.image_file_malicious_classification_type.total_values", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_imageFile_maliciousClassificationType_totalValues_to_long")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // SKIPPED: condition not transpiled: ctx.json?.simpleValues["imageFile.md5String"] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: def obj = ctx.json.simpleValues.remove(\"imageFile.md5String\"); ctx.cybereason.suspicions_process.simple_values.image_file_md5_string = obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.simpleValues.remove(\"imageFile.md5String\"); ctx.cybereason.suspicions_process.simple_values.image_file_md5_string = obj;"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cybereason.suspicions_process.simple_values.image_file_md5_string.totalValues",
                ) {
                    if let Some(val) = event.get("cybereason.suspicions_process.simple_values.image_file_md5_string.totalValues") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.suspicions_process.simple_values.image_file_md5_string.totalValues".into(),
                            message,
                        })?;
                    event.set("cybereason.suspicions_process.simple_values.image_file_md5_string.total_values", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_imageFile_md5String_totalValues_to_long",
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
                .get("cybereason.suspicions_process.simple_values.image_file_md5_string.values")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = {
                event.has_value(
                    "cybereason.suspicions_process.simple_values.image_file_md5_string.values",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.suspicions_process.simple_values.image_file_md5_string.values",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            // SKIPPED: condition not transpiled: ctx.json?.simpleValues["imageFile.productName"] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: def obj = ctx.json.simpleValues.remove(\"imageFile.productName\"); ctx.cybereason.suspicions_process.simple_values.image_file_product_name = obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.simpleValues.remove(\"imageFile.productName\"); ctx.cybereason.suspicions_process.simple_values.image_file_product_name = obj;"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cybereason.suspicions_process.simple_values.image_file_product_name.totalValues") {
                if let Some(val) = event.get("cybereason.suspicions_process.simple_values.image_file_product_name.totalValues") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.suspicions_process.simple_values.image_file_product_name.totalValues".into(),
                            message,
                        })?;
                    event.set("cybereason.suspicions_process.simple_values.image_file_product_name.total_values", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_imageFile_productName_totalValues_to_long",
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

            // SKIPPED: condition not transpiled: ctx.json?.simpleValues["imageFile.sha1String"] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: def obj = ctx.json.simpleValues.remove(\"imageFile.sha1String\"); ctx.cybereason.suspicions_process.simple_values.image_file_sha1_string = obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.simpleValues.remove(\"imageFile.sha1String\"); ctx.cybereason.suspicions_process.simple_values.image_file_sha1_string = obj;"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cybereason.suspicions_process.simple_values.image_file_sha1_string.totalValues") {
                if let Some(val) = event.get("cybereason.suspicions_process.simple_values.image_file_sha1_string.totalValues") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.suspicions_process.simple_values.image_file_sha1_string.totalValues".into(),
                            message,
                        })?;
                    event.set("cybereason.suspicions_process.simple_values.image_file_sha1_string.total_values", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_imageFile_sha1String_totalValues_to_long",
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
                .get("cybereason.suspicions_process.simple_values.image_file_sha1_string.values")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            let _cond = {
                event.has_value(
                    "cybereason.suspicions_process.simple_values.image_file_sha1_string.values",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.suspicions_process.simple_values.image_file_sha1_string.values",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            event.remove(
                "cybereason.suspicions_process.simple_values.image_file_company_name.totalValues",
            );
            event.remove("cybereason.suspicions_process.simple_values.image_file_hash_icon_base64.totalValues");
            event.remove("cybereason.suspicions_process.simple_values.image_file_malicious_classification_type.totalValues");
            event.remove(
                "cybereason.suspicions_process.simple_values.image_file_md5_string.totalValues",
            );
            event.remove(
                "cybereason.suspicions_process.simple_values.image_file_product_name.totalValues",
            );
            event.remove(
                "cybereason.suspicions_process.simple_values.image_file_sha1_string.totalValues",
            );

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.isImageFileSignedAndVerified.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.isImageFileSignedAndVerified.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.isImageFileSignedAndVerified.totalValues"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.simple_values.is_image_file_signed_and_verified.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_isImageFileSignedAndVerified_totalValues_to_long",
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

            let _cond =
                { event.has_value("json.simpleValues.isImageFileSignedAndVerified.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.isImageFileSignedAndVerified.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_isImageFileSignedAndVerified_values_to_boolean")?;
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
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.isImageFileSignedAndVerified.values") {
                event.rename("json.simpleValues.isImageFileSignedAndVerified.values", "cybereason.suspicions_process.simple_values.is_image_file_signed_and_verified.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.isWhiteListClassification.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.isWhiteListClassification.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.isWhiteListClassification.totalValues"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.simple_values.is_white_list_classification.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_isWhiteListClassification_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.isWhiteListClassification.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.isWhiteListClassification.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_isWhiteListClassification_values_to_boolean")?;
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
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.isWhiteListClassification.values") {
                event.rename("json.simpleValues.isWhiteListClassification.values", "cybereason.suspicions_process.simple_values.is_white_list_classification.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.productType.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.productType.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.productType.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.suspicions_process.simple_values.product_type.total_values",
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
                    "convert_simpleValues_productType_totalValues_to_long",
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

            if event.has_value("json.simpleValues.productType.values") {
                event.rename(
                    "json.simpleValues.productType.values",
                    "cybereason.suspicions_process.simple_values.product_type.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.simpleValues.ransomwareAutoRemediationSuspended.totalValues")
                {
                    if let Some(val) = event
                        .get("json.simpleValues.ransomwareAutoRemediationSuspended.totalValues")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.simpleValues.ransomwareAutoRemediationSuspended.totalValues".into(),
                            message,
                        })?;
                        event.set("cybereason.suspicions_process.simple_values.ransomware_auto_remediation_suspended.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_ransomwareAutoRemediationSuspended_totalValues_to_long",
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

            let _cond =
                { event.has_value("json.simpleValues.ransomwareAutoRemediationSuspended.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.ransomwareAutoRemediationSuspended.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_ransomwareAutoRemediationSuspended_values_to_boolean")?;
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
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.ransomwareAutoRemediationSuspended.values") {
                event.rename("json.simpleValues.ransomwareAutoRemediationSuspended.values", "cybereason.suspicions_process.simple_values.ransomware_auto_remediation_suspended.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.suspect") {
                    if let Some(val) = event.get("json.suspect") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.suspect".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.suspect", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_suspect_to_boolean",
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
                if event.has_value("json.suspicionCount") {
                    if let Some(val) = event.get("json.suspicionCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.suspicionCount".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.suspicions_process.suspicion_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_suspicionCount_to_long",
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

            if event.has_value("json.suspicions") {
                event.rename(
                    "json.suspicions",
                    "cybereason.suspicions_process.suspicions",
                )?;
            }

            if event.has_value("suspicionsMap") {
                event.rename(
                    "suspicionsMap",
                    "cybereason.suspicions_process.suspicions_map",
                )?;
            }

            let _cond = {
                event.has_value(
                    "cybereason.suspicions_process.element_values.image_file.element_values",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.suspicions_process.element_values.image_file.element_values",
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
                                event.remove("_ingest._value.guid");
                                event.remove("_ingest._value.name");
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.suspicions_process.element_values.owner_machine.element_values",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.suspicions_process.element_values.owner_machine.element_values",
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
                                event.remove("_ingest._value.guid");
                                event.remove("_ingest._value.name");
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.suspicions_process.element_values.parent_process.element_values",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.suspicions_process.element_values.parent_process.element_values",
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
                                event.remove("_ingest._value.guid");
                                event.remove("_ingest._value.name");
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
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
                event.remove("cybereason.suspicions_process.simple_values.command_line.values");
                event.remove("cybereason.suspicions_process.simple_values.creation_time.values");
                event.remove("cybereason.suspicions_process.simple_values.end_time.values");
                event.remove(
                    "cybereason.suspicions_process.simple_values.image_file_sha1_string.values",
                );
                event.remove(
                    "cybereason.suspicions_process.simple_values.image_file_md5_string.values",
                );
                event.remove("cybereason.suspicions_process.guid_string");
            }

            event.remove("json");

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
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
