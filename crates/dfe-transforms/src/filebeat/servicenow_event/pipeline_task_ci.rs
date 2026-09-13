// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_task_ci` pipeline.
pub struct PipelineTaskCi;

impl Transform for PipelineTaskCi {
    fn name(&self) -> &str {
        "pipeline_task_ci"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                event.append("event.type", json!("info"))?;

                event.append("event.category", json!("configuration"))?;

            let _cond = { event.get_str("servicenow.event.applied.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.applied.display_value") {
                if let Some(val) = event.get("servicenow.event.applied.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.applied.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.applied.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_applied_display_value_to_boolean")?;
                        if event.remove("servicenow.event.applied.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.applied.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.applied.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.applied.value") {
                if let Some(val) = event.get("servicenow.event.applied.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.applied.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.applied.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_applied_value_to_boolean")?;
                        if event.remove("servicenow.event.applied.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.applied.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.applied_date.display_value") && event.get_str("servicenow.event.applied_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.applied_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.applied_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.applied_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_applied_date_display_value")?;
                        if event.remove("servicenow.event.applied_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.applied_date.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.applied_date.display_value") && event.get_str("servicenow.event.applied_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.applied_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.applied_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.applied_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_applied_date_display_value")?;
                        if event.remove("servicenow.event.applied_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.applied_date.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.applied_date.value") && event.get_str("servicenow.event.applied_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.applied_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.applied_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.applied_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_applied_date_value")?;
                        if event.remove("servicenow.event.applied_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.applied_date.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.applied_date.value") && event.get_str("servicenow.event.applied_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.applied_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.applied_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.applied_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_applied_date_value")?;
                        if event.remove("servicenow.event.applied_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.applied_date.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.manual_proposed_change.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.manual_proposed_change.display_value") {
                if let Some(val) = event.get("servicenow.event.manual_proposed_change.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.manual_proposed_change.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.manual_proposed_change.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_manual_proposed_change_display_value_to_boolean")?;
                        if event.remove("servicenow.event.manual_proposed_change.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.manual_proposed_change.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.manual_proposed_change.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.manual_proposed_change.value") {
                if let Some(val) = event.get("servicenow.event.manual_proposed_change.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.manual_proposed_change.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.manual_proposed_change.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_manual_proposed_change_value_to_boolean")?;
                        if event.remove("servicenow.event.manual_proposed_change.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.manual_proposed_change.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
