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
                if let Some(v) = event.get("json.simpleValues.malopLastUpdateTime.values") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_hasMalops_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_hasSuspicions_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                ) {
                                    if let Some(val) = event.get("_ingest._value.simpleValues.elementDisplayName.totalValues") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.simpleValues.elementDisplayName.totalValues".into(),
                    message,
                    })?;
                    event.set("_ingest._value.simple_values.element_display_name.total_values", converted)?;
                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_elementDisplayName_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            if event
                                .has_value("_ingest._value.simpleValues.elementDisplayName.values")
                            {
                                event.rename(
                                    "_ingest._value.simpleValues.elementDisplayName.values",
                                    "_ingest._value.simple_values.element_display_name.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.group.totalValues")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.group.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.group.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.group.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_group_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.group.totalValues");
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.group.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.group.values",
                                    "_ingest._value.simple_values.group.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.guid.totalValues") {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.guid.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.guid.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.guid.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_guid_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.guid.totalValues");
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.guid.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.guid.values",
                                    "_ingest._value.simple_values.guid.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event
                                    .has_value("_ingest._value.simpleValues.hasMalops.totalValues")
                                {
                                    if let Some(val) = event
                                        .get("_ingest._value.simpleValues.hasMalops.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasMalops.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.has_malops.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_hasMalops_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_hasMalops_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasMalops.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    "_ingest._value.simple_values.has_malops.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                ) {
                                    if let Some(val) = event.get(
                                        "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                    ) {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasSuspicions.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set("_ingest._value.simple_values.has_suspicions.total_values", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_hasSuspicions_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedMachines_elementValues_simpleValues_hasSuspicions_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasSuspicions.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    "_ingest._value.simple_values.has_suspicions.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            event.remove("_ingest._value.simpleValues.hasSuspicions.totalValues");
                            event.remove("_ingest._value.simpleValues.guid.totalValues");
                            event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
                            event.remove(
                                "_ingest._value.simpleValues.elementDisplayName.totalValues",
                            );
                            event.remove("_ingest._value.simpleValues.group.totalValues");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedMachines.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedMachines.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.affectedMachines.elementValues") {
                event.rename(
                    "json.elementValues.affectedMachines.elementValues",
                    "cybereason.malop_process.element_values.affected_machines.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.affectedMachines.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.affectedMachines.guessedTotal")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedMachines.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.affected_machines.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_affectedMachines_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.affectedMachines.totalMalicious") {
                    if let Some(val) =
                        event.get("json.elementValues.affectedMachines.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedMachines.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.affected_machines.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_affectedMachines_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.affectedMachines.totalSuspicious") {
                    if let Some(val) =
                        event.get("json.elementValues.affectedMachines.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedMachines.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.affected_machines.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_affectedMachines_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.affectedMachines.totalValues") {
                    if let Some(val) = event.get("json.elementValues.affectedMachines.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedMachines.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.affected_machines.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_affectedMachines_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_hasMalops_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_hasSuspicions_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                ) {
                                    if let Some(val) = event.get("_ingest._value.simpleValues.elementDisplayName.totalValues") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.simpleValues.elementDisplayName.totalValues".into(),
                    message,
                    })?;
                    event.set("_ingest._value.simple_values.element_display_name.total_values", converted)?;
                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_elementDisplayName_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            if event
                                .has_value("_ingest._value.simpleValues.elementDisplayName.values")
                            {
                                event.rename(
                                    "_ingest._value.simpleValues.elementDisplayName.values",
                                    "_ingest._value.simple_values.element_display_name.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.group.totalValues")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.group.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.group.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.group.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_group_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.group.totalValues");
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.group.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.group.values",
                                    "_ingest._value.simple_values.group.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.guid.totalValues") {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.guid.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.guid.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.guid.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_guid_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.guid.totalValues");
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.guid.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.guid.values",
                                    "_ingest._value.simple_values.guid.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event
                                    .has_value("_ingest._value.simpleValues.hasMalops.totalValues")
                                {
                                    if let Some(val) = event
                                        .get("_ingest._value.simpleValues.hasMalops.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasMalops.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.has_malops.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_hasMalops_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_hasMalops_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasMalops.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    "_ingest._value.simple_values.has_malops.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                ) {
                                    if let Some(val) = event.get(
                                        "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                    ) {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasSuspicions.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set("_ingest._value.simple_values.has_suspicions.total_values", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_hasSuspicions_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_affectedUsers_elementValues_simpleValues_hasSuspicions_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasSuspicions.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    "_ingest._value.simple_values.has_suspicions.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            event.remove("_ingest._value.simpleValues.hasSuspicions.totalValues");
                            event.remove("_ingest._value.simpleValues.guid.totalValues");
                            event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
                            event.remove(
                                "_ingest._value.simpleValues.elementDisplayName.totalValues",
                            );
                            event.remove("_ingest._value.simpleValues.group.totalValues");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.affectedUsers.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.affectedUsers.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.affectedUsers.elementValues") {
                event.rename(
                    "json.elementValues.affectedUsers.elementValues",
                    "cybereason.malop_process.element_values.affected_users.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.affectedUsers.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.affectedUsers.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedUsers.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.element_values.affected_users.guessed_total",
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
                    "convert_elementValues_affectedUsers_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.affectedUsers.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.affectedUsers.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedUsers.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.affected_users.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_affectedUsers_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.affectedUsers.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.affectedUsers.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedUsers.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.affected_users.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_affectedUsers_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.affectedUsers.totalValues") {
                    if let Some(val) = event.get("json.elementValues.affectedUsers.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.affectedUsers.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.element_values.affected_users.total_values",
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
                    "convert_elementValues_affectedUsers_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_hasMalops_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_hasSuspicions_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                ) {
                                    if let Some(val) = event.get("_ingest._value.simpleValues.elementDisplayName.totalValues") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.simpleValues.elementDisplayName.totalValues".into(),
                    message,
                    })?;
                    event.set("_ingest._value.simple_values.element_display_name.total_values", converted)?;
                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_elementDisplayName_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            if event
                                .has_value("_ingest._value.simpleValues.elementDisplayName.values")
                            {
                                event.rename(
                                    "_ingest._value.simpleValues.elementDisplayName.values",
                                    "_ingest._value.simple_values.element_display_name.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.group.totalValues")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.group.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.group.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.group.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_group_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.group.totalValues");
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.group.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.group.values",
                                    "_ingest._value.simple_values.group.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.guid.totalValues") {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.guid.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.guid.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.guid.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_guid_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.guid.totalValues");
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.guid.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.guid.values",
                                    "_ingest._value.simple_values.guid.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event
                                    .has_value("_ingest._value.simpleValues.hasMalops.totalValues")
                                {
                                    if let Some(val) = event
                                        .get("_ingest._value.simpleValues.hasMalops.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasMalops.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.has_malops.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_hasMalops_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_hasMalops_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasMalops.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    "_ingest._value.simple_values.has_malops.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                ) {
                                    if let Some(val) = event.get(
                                        "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                    ) {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasSuspicions.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set("_ingest._value.simple_values.has_suspicions.total_values", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_hasSuspicions_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_filesToRemediate_elementValues_simpleValues_hasSuspicions_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            event.remove("_ingest._value.simpleValues.hasSuspicions.totalValues");
                            event.remove("_ingest._value.simpleValues.guid.totalValues");
                            event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
                            event.remove(
                                "_ingest._value.simpleValues.elementDisplayName.totalValues",
                            );
                            event.remove("_ingest._value.simpleValues.group.totalValues");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasSuspicions.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    "_ingest._value.simple_values.has_suspicions.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.filesToRemediate.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.filesToRemediate.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.filesToRemediate.elementValues") {
                event.rename(
                    "json.elementValues.filesToRemediate.elementValues",
                    "cybereason.malop_process.element_values.files_to_remediate.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.filesToRemediate.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.filesToRemediate.guessedTotal")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.filesToRemediate.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.files_to_remediate.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_filesToRemediate_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.filesToRemediate.totalMalicious") {
                    if let Some(val) =
                        event.get("json.elementValues.filesToRemediate.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.filesToRemediate.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.files_to_remediate.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_filesToRemediate_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.filesToRemediate.totalSuspicious") {
                    if let Some(val) =
                        event.get("json.elementValues.filesToRemediate.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.filesToRemediate.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.files_to_remediate.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_filesToRemediate_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.filesToRemediate.totalValues") {
                    if let Some(val) = event.get("json.elementValues.filesToRemediate.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.filesToRemediate.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.files_to_remediate.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_filesToRemediate_totalValues_to_long",
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
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_primaryRootCauseElements_elementValues_hasMalops_to_boolean")?;
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_primaryRootCauseElements_elementValues_hasSuspicions_to_boolean")?;
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                ) {
                                    if let Some(val) = event.get("_ingest._value.simpleValues.elementDisplayName.totalValues") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.simpleValues.elementDisplayName.totalValues".into(),
                    message,
                    })?;
                    event.set("_ingest._value.simple_values.element_display_name.total_values", converted)?;
                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_primaryRootCauseElements_elementValues_simpleValues_elementDisplayName_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                );
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            if event
                                .has_value("_ingest._value.simpleValues.elementDisplayName.values")
                            {
                                event.rename(
                                    "_ingest._value.simpleValues.elementDisplayName.values",
                                    "_ingest._value.simple_values.element_display_name.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.group.totalValues")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.group.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.group.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.group.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_primaryRootCauseElements_elementValues_simpleValues_group_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.group.totalValues");
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.group.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.group.values",
                                    "_ingest._value.simple_values.group.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.guid.totalValues") {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.guid.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.guid.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.guid.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_primaryRootCauseElements_elementValues_simpleValues_guid_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.guid.totalValues");
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

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.guid.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.guid.values",
                                    "_ingest._value.simple_values.guid.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("json.elementValues.primaryRootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.primaryRootCauseElements.elementValues",
                        |event| {
                            event.remove("_ingest._value.simpleValues.guid.totalValues");
                            event.remove(
                                "_ingest._value.simpleValues.elementDisplayName.totalValues",
                            );
                            event.remove("_ingest._value.simpleValues.group.totalValues");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.primaryRootCauseElements.elementValues") {
                event.rename("json.elementValues.primaryRootCauseElements.elementValues", "cybereason.malop_process.element_values.primary_root_cause_elements.element_values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.primaryRootCauseElements.guessedTotal") {
                    if let Some(val) =
                        event.get("json.elementValues.primaryRootCauseElements.guessedTotal")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.primaryRootCauseElements.guessedTotal"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.primary_root_cause_elements.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_primaryRootCauseElements_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.primaryRootCauseElements.totalMalicious") {
                    if let Some(val) =
                        event.get("json.elementValues.primaryRootCauseElements.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.primaryRootCauseElements.totalMalicious"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.primary_root_cause_elements.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_primaryRootCauseElements_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.primaryRootCauseElements.totalSuspicious") {
                    if let Some(val) =
                        event.get("json.elementValues.primaryRootCauseElements.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.primaryRootCauseElements.totalSuspicious"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.primary_root_cause_elements.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_primaryRootCauseElements_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.primaryRootCauseElements.totalValues") {
                    if let Some(val) =
                        event.get("json.elementValues.primaryRootCauseElements.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.primaryRootCauseElements.totalValues"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.primary_root_cause_elements.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_primaryRootCauseElements_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_rootCauseElements_elementValues_hasMalops_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_rootCauseElements_elementValues_hasSuspicions_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                ) {
                                    if let Some(val) = event.get("_ingest._value.simpleValues.elementDisplayName.totalValues") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.simpleValues.elementDisplayName.totalValues".into(),
                    message,
                    })?;
                    event.set("_ingest._value.simple_values.element_display_name.total_values", converted)?;
                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_rootCauseElements_elementValues_simpleValues_elementDisplayName_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            if event
                                .has_value("_ingest._value.simpleValues.elementDisplayName.values")
                            {
                                event.rename(
                                    "_ingest._value.simpleValues.elementDisplayName.values",
                                    "_ingest._value.simple_values.element_display_name.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.group.totalValues")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.group.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.group.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.group.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_rootCauseElements_elementValues_simpleValues_group_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.group.totalValues");
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.group.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.group.values",
                                    "_ingest._value.simple_values.group.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.guid.totalValues") {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.guid.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.guid.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.guid.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_rootCauseElements_elementValues_simpleValues_guid_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.guid.totalValues");
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

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            event.remove("_ingest._value.simpleValues.guid.totalValues");
                            event.remove(
                                "_ingest._value.simpleValues.elementDisplayName.totalValues",
                            );
                            event.remove("_ingest._value.simpleValues.group.totalValues");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.guid.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.guid.values",
                                    "_ingest._value.simple_values.guid.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.rootCauseElements.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.rootCauseElements.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.rootCauseElements.elementValues") {
                event.rename(
                    "json.elementValues.rootCauseElements.elementValues",
                    "cybereason.malop_process.element_values.root_cause_elements.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.rootCauseElements.guessedTotal") {
                    if let Some(val) =
                        event.get("json.elementValues.rootCauseElements.guessedTotal")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.rootCauseElements.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.root_cause_elements.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_rootCauseElements_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.rootCauseElements.totalMalicious") {
                    if let Some(val) =
                        event.get("json.elementValues.rootCauseElements.totalMalicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.rootCauseElements.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.root_cause_elements.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_rootCauseElements_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.rootCauseElements.totalSuspicious") {
                    if let Some(val) =
                        event.get("json.elementValues.rootCauseElements.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.rootCauseElements.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.root_cause_elements.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_rootCauseElements_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.rootCauseElements.totalValues") {
                    if let Some(val) = event.get("json.elementValues.rootCauseElements.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.rootCauseElements.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.element_values.root_cause_elements.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_rootCauseElements_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_hasMalops_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_hasSuspicions_to_boolean")?;
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                ) {
                                    if let Some(val) = event.get("_ingest._value.simpleValues.elementDisplayName.totalValues") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.simpleValues.elementDisplayName.totalValues".into(),
                    message,
                    })?;
                    event.set("_ingest._value.simple_values.element_display_name.total_values", converted)?;
                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_elementDisplayName_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.elementDisplayName.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            if event
                                .has_value("_ingest._value.simpleValues.elementDisplayName.values")
                            {
                                event.rename(
                                    "_ingest._value.simpleValues.elementDisplayName.values",
                                    "_ingest._value.simple_values.element_display_name.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.group.totalValues")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.group.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.group.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.group.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_group_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.group.totalValues");
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.group.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.group.values",
                                    "_ingest._value.simple_values.group.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.simpleValues.guid.totalValues") {
                                    if let Some(val) =
                                        event.get("_ingest._value.simpleValues.guid.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.guid.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.guid.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_guid_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.guid.totalValues");
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.guid.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.guid.values",
                                    "_ingest._value.simple_values.guid.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event
                                    .has_value("_ingest._value.simpleValues.hasMalops.totalValues")
                                {
                                    if let Some(val) = event
                                        .get("_ingest._value.simpleValues.hasMalops.totalValues")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasMalops.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.simple_values.has_malops.total_values",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_hasMalops_totalValues_to_long")?;
                                event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_hasMalops_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasMalops.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasMalops.values",
                                    "_ingest._value.simple_values.has_malops.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                ) {
                                    if let Some(val) = event.get(
                                        "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                    ) {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.simpleValues.hasSuspicions.totalValues".into(),
                    message,
                    }
                                            })?;
                                        event.set("_ingest._value.simple_values.has_suspicions.total_values", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_hasSuspicions_totalValues_to_long")?;
                                event.remove(
                                    "_ingest._value.simpleValues.hasSuspicions.totalValues",
                                );
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

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "boolean")
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_elementValues_suspects_elementValues_simpleValues_hasSuspicions_values_to_boolean")?;
                                            event.remove("_ingest._value");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            event.remove("_ingest._value.simpleValues.hasSuspicions.totalValues");
                            event.remove("_ingest._value.simpleValues.guid.totalValues");
                            event.remove("_ingest._value.simpleValues.hasMalops.totalValues");
                            event.remove(
                                "_ingest._value.simpleValues.elementDisplayName.totalValues",
                            );
                            event.remove("_ingest._value.simpleValues.group.totalValues");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues.hasSuspicions.values") {
                                event.rename(
                                    "_ingest._value.simpleValues.hasSuspicions.values",
                                    "_ingest._value.simple_values.has_suspicions.values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.suspects.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.suspects.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.suspects.elementValues") {
                event.rename(
                    "json.elementValues.suspects.elementValues",
                    "cybereason.malop_process.element_values.suspects.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.suspects.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.suspects.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.suspects.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.element_values.suspects.guessedTotal",
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
                    "convert_elementValues_suspects_guessedTotal_to_long",
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
                if event.has_value("json.elementValues.suspects.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.suspects.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.suspects.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.element_values.suspects.total_malicious",
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
                    "convert_elementValues_suspects_totalMalicious_to_long",
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
                if event.has_value("json.elementValues.suspects.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.suspects.totalSuspicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.suspects.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.element_values.suspects.total_suspicious",
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
                    "convert_elementValues_suspects_totalSuspicious_to_long",
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
                if event.has_value("json.elementValues.suspects.totalValues") {
                    if let Some(val) = event.get("json.elementValues.suspects.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.suspects.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.element_values.suspects.total_values",
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
                    "convert_elementValues_suspects_totalValues_to_long",
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
                event.rename("evidenceMap", "cybereason.malop_process.evidence_map")?;
            }

            if event.has_value("json.filterData.groupByValue") {
                event.rename(
                    "json.filterData.groupByValue",
                    "cybereason.malop_process.filter_data.group_by_value",
                )?;
            }

            if event.has_value("json.filterData.sortInGroupValue") {
                event.rename(
                    "json.filterData.sortInGroupValue",
                    "cybereason.malop_process.filter_data.sort_in_group_value",
                )?;
            }

            if event.has_value("json.guidString") {
                event.rename("json.guidString", "cybereason.malop_process.guid_string")?;
            }

            if let Some(v) = event
                .get("cybereason.malop_process.guid_string")
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
                        event.set("cybereason.malop_process.is_malicious", converted)?;
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
                event.rename("json.labelsIds", "cybereason.malop_process.labels_ids")?;
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
                        event.set("cybereason.malop_process.malicious", converted)?;
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
                    "cybereason.malop_process.malop_priority",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.allRansomwareProcessesSuspended.totalValues")
                {
                    if let Some(val) =
                        event.get("json.simpleValues.allRansomwareProcessesSuspended.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.simpleValues.allRansomwareProcessesSuspended.totalValues"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.all_ransomware_processes_suspended.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_allRansomwareProcessesSuspended_totalValues_to_long",
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
                { event.has_value("json.simpleValues.allRansomwareProcessesSuspended.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.allRansomwareProcessesSuspended.values",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_allRansomwareProcessesSuspended_values_to_boolean")?;
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

            if event.has_value("json.simpleValues.allRansomwareProcessesSuspended.values") {
                event.rename("json.simpleValues.allRansomwareProcessesSuspended.values", "cybereason.malop_process.simple_values.all_ransomware_processes_suspended.values")?;
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
                        event.set(
                            "cybereason.malop_process.simple_values.creation_time.total_values",
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
                        if let Some(v) = event
                            .get("_ingest._value")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("event.created", v)?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.creationTime.values") {
                event.rename(
                    "json.simpleValues.creationTime.values",
                    "cybereason.malop_process.simple_values.creation_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.decisionFeature.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.decisionFeature.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.decisionFeature.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.simple_values.decision_feature.total_values",
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
                    "convert_simpleValues_decisionFeature_totalValues_to_long",
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

            if event.has_value("json.simpleValues.decisionFeature.values") {
                event.rename(
                    "json.simpleValues.decisionFeature.values",
                    "cybereason.malop_process.simple_values.decision_feature.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.decisionFeatureSet.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.decisionFeatureSet.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.decisionFeatureSet.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.decision_feature_set.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_decisionFeatureSet_totalValues_to_long",
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

            if event.has_value("json.simpleValues.decisionFeatureSet.values") {
                event.rename(
                    "json.simpleValues.decisionFeatureSet.values",
                    "cybereason.malop_process.simple_values.decision_feature_set.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.detectionType.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.detectionType.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.detectionType.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.simple_values.detection_type.total_values",
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
                    "convert_simpleValues_detectionType_totalValues_to_long",
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

            if event.has_value("json.simpleValues.detectionType.values") {
                event.rename(
                    "json.simpleValues.detectionType.values",
                    "cybereason.malop_process.simple_values.detection_type.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.hasRansomwareSuspendedProcesses.totalValues")
                {
                    if let Some(val) =
                        event.get("json.simpleValues.hasRansomwareSuspendedProcesses.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.simpleValues.hasRansomwareSuspendedProcesses.totalValues"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.has_ransomware_suspended_processes.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_hasRansomwareSuspendedProcesses_totalValues_to_long",
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
                { event.has_value("json.simpleValues.hasRansomwareSuspendedProcesses.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.hasRansomwareSuspendedProcesses.values",
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_hasRansomwareSuspendedProcesses_values_to_boolean")?;
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

            if event.has_value("json.simpleValues.hasRansomwareSuspendedProcesses.values") {
                event.rename("json.simpleValues.hasRansomwareSuspendedProcesses.values", "cybereason.malop_process.simple_values.has_ransomware_suspended_processes.values")?;
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
                            "cybereason.malop_process.simple_values.icon_base64.total_values",
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
                    "cybereason.malop_process.simple_values.icon_base64.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.isBlocked.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.isBlocked.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.isBlocked.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.simple_values.is_blocked.total_values",
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
                    "convert_simpleValues_isBlocked_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.isBlocked.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.isBlocked.values", |event| {
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
                                "convert_simpleValues_isBlocked_values_to_boolean",
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
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.isBlocked.values") {
                event.rename(
                    "json.simpleValues.isBlocked.values",
                    "cybereason.malop_process.simple_values.is_blocked.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.malopActivityTypes.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.malopActivityTypes.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.malopActivityTypes.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.malop.activity_types.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_malopActivityTypes_totalValues_to_long",
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

            if event.has_value("json.simpleValues.malopActivityTypes.values") {
                event.rename(
                    "json.simpleValues.malopActivityTypes.values",
                    "cybereason.malop_process.simple_values.malop.activity_types.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.malopLastUpdateTime.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.malopLastUpdateTime.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.malopLastUpdateTime.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.malop.last_update_time.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_malopLastUpdateTime_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.malopLastUpdateTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("json.simpleValues.malopLastUpdateTime.values")
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
                                        "date_simpleValues_malopLastUpdateTime_values",
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
                                "json.simpleValues.malopLastUpdateTime.values",
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

            let _cond = { event.has_value("json.simpleValues.malopLastUpdateTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.malopLastUpdateTime.values",
                        |event| {
                            if let Some(v) = event
                                .get("_ingest._value")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("@timestamp", v)?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.malopLastUpdateTime.values") {
                event.rename(
                    "json.simpleValues.malopLastUpdateTime.values",
                    "cybereason.malop_process.simple_values.malop.last_update_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.malopStartTime.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.malopStartTime.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.malopStartTime.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_process.simple_values.malop.start_time.total_values",
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
                    "convert_simpleValues_malopStartTime_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.malopStartTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("json.simpleValues.malopStartTime.values")
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
                                        "date_simpleValues_malopStartTime_values",
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
                                "json.simpleValues.malopStartTime.values",
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

            if event.has_value("json.simpleValues.malopStartTime.values") {
                event.rename(
                    "json.simpleValues.malopStartTime.values",
                    "cybereason.malop_process.simple_values.malop.start_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.rootCauseElementCompanyProduct.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.rootCauseElementCompanyProduct.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.simpleValues.rootCauseElementCompanyProduct.totalValues"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.root_cause_element.company_product.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_rootCauseElementCompanyProduct_totalValues_to_long",
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

            if event.has_value("json.simpleValues.rootCauseElementCompanyProduct.values") {
                event.rename("json.simpleValues.rootCauseElementCompanyProduct.values", "cybereason.malop_process.simple_values.root_cause_element.company_product.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.rootCauseElementHashes.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.rootCauseElementHashes.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.rootCauseElementHashes.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.root_cause_element.hashes.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_rootCauseElementHashes_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.rootCauseElementHashes.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.rootCauseElementHashes.values",
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

            if event.has_value("json.simpleValues.rootCauseElementHashes.values") {
                event.rename(
                    "json.simpleValues.rootCauseElementHashes.values",
                    "cybereason.malop_process.simple_values.root_cause_element.hashes.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.rootCauseElementNames.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.rootCauseElementNames.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.rootCauseElementNames.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.root_cause_element.names.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_rootCauseElementNames_totalValues_to_long",
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

            if event.has_value("json.simpleValues.rootCauseElementNames.values") {
                event.rename(
                    "json.simpleValues.rootCauseElementNames.values",
                    "cybereason.malop_process.simple_values.root_cause_element.names.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.rootCauseElementTypes.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.rootCauseElementTypes.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.rootCauseElementTypes.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.root_cause_element.types.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_rootCauseElementTypes_totalValues_to_long",
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

            if event.has_value("json.simpleValues.rootCauseElementTypes.values") {
                event.rename(
                    "json.simpleValues.rootCauseElementTypes.values",
                    "cybereason.malop_process.simple_values.root_cause_element.types.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.totalNumberOfIncomingConnections.totalValues")
                {
                    if let Some(val) =
                        event.get("json.simpleValues.totalNumberOfIncomingConnections.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.simpleValues.totalNumberOfIncomingConnections.totalValues"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.total.number_of.incoming_connections.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_totalNumberOfIncomingConnections_totalValues_to_long",
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
                { event.has_value("json.simpleValues.totalNumberOfIncomingConnections.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.totalNumberOfIncomingConnections.values",
                        |event| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_totalNumberOfIncomingConnections_values_to_long")?;
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

            if event.has_value("json.simpleValues.totalNumberOfIncomingConnections.values") {
                event.rename("json.simpleValues.totalNumberOfIncomingConnections.values", "cybereason.malop_process.simple_values.total.number_of.incoming_connections.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.totalNumberOfOutgoingConnections.totalValues")
                {
                    if let Some(val) =
                        event.get("json.simpleValues.totalNumberOfOutgoingConnections.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.simpleValues.totalNumberOfOutgoingConnections.totalValues"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.total.number_of.outgoing_connections.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_totalNumberOfOutgoingConnections_totalValues_to_long",
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
                { event.has_value("json.simpleValues.totalNumberOfOutgoingConnections.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.totalNumberOfOutgoingConnections.values",
                        |event| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_totalNumberOfOutgoingConnections_values_to_long")?;
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

            if event.has_value("json.simpleValues.totalNumberOfOutgoingConnections.values") {
                event.rename("json.simpleValues.totalNumberOfOutgoingConnections.values", "cybereason.malop_process.simple_values.total.number_of.outgoing_connections.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.totalReceivedBytes.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.totalReceivedBytes.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.totalReceivedBytes.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.total.received_bytes.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_totalReceivedBytes_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.totalReceivedBytes.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.totalReceivedBytes.values",
                        |event| {
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
                                    "convert_simpleValues_totalReceivedBytes_values_to_long",
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

            let _cond = { event.has_value("json.simpleValues.totalReceivedBytes.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.totalReceivedBytes.values",
                        |event| {
                            if let Some(v) = event
                                .get("_ingest._value")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("destination.bytes", v)?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.totalReceivedBytes.values") {
                event.rename(
                    "json.simpleValues.totalReceivedBytes.values",
                    "cybereason.malop_process.simple_values.total.received_bytes.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.totalTransmittedBytes.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.totalTransmittedBytes.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.totalTransmittedBytes.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_process.simple_values.total.transmitted_bytes.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_totalTransmittedBytes_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.totalTransmittedBytes.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.totalTransmittedBytes.values",
                        |event| {
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
                                    "convert_simpleValues_totalTransmittedBytes_values_to_long",
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

            let _cond = { event.has_value("json.simpleValues.totalTransmittedBytes.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.totalTransmittedBytes.values",
                        |event| {
                            if let Some(v) = event
                                .get("_ingest._value")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("source.bytes", v)?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.totalTransmittedBytes.values") {
                event.rename(
                    "json.simpleValues.totalTransmittedBytes.values",
                    "cybereason.malop_process.simple_values.total.transmitted_bytes.values",
                )?;
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
                        event.set("cybereason.malop_process.suspect", converted)?;
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
                        event.set("cybereason.malop_process.suspicion_count", converted)?;
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
                event.rename("json.suspicions", "cybereason.malop_process.suspicions")?;
            }

            if event.has_value("suspicionsMap") {
                event.rename("suspicionsMap", "cybereason.malop_process.suspicions_map")?;
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
                event.remove("cybereason.malop_process.simple_values.creation_time.values");
                event.remove(
                    "cybereason.malop_process.simple_values.total.transmitted_bytes.values",
                );
                event.remove("cybereason.malop_process.simple_values.total.received_bytes.values");
                event
                    .remove("cybereason.malop_process.simple_values.malop.last_update_time.values");
                event.remove("cybereason.malop_process.guid_string");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
