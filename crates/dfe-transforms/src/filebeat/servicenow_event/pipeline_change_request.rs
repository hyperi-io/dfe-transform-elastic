// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_change_request` pipeline.
pub struct PipelineChangeRequest;

impl Transform for PipelineChangeRequest {
    fn name(&self) -> &str {
        "pipeline_change_request"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                event.append("event.type", json!("change"))?;

                event.append("event.category", json!("configuration"))?;

            let _cond = { event.has_value("servicenow.event.cab_required.display_value") && event.get_str("servicenow.event.cab_required.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.cab_required.display_value") {
                if let Some(val) = event.get("servicenow.event.cab_required.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.cab_required.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.cab_required.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cab_required_display_value_to_boolean")?;
                        if event.remove("servicenow.event.cab_required.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.cab_required.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.cab_required.value") && event.get_str("servicenow.event.cab_required.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.cab_required.value") {
                if let Some(val) = event.get("servicenow.event.cab_required.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.cab_required.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.cab_required.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cab_required_value_to_boolean")?;
                        if event.remove("servicenow.event.cab_required.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.cab_required.value".into() });
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

            let _cond = { event.has_value("servicenow.event.outside_maintenance_schedule.display_value") && event.get_str("servicenow.event.outside_maintenance_schedule.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.outside_maintenance_schedule.display_value") {
                if let Some(val) = event.get("servicenow.event.outside_maintenance_schedule.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.outside_maintenance_schedule.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.outside_maintenance_schedule.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_outside_maintenance_schedule_display_value_to_boolean")?;
                        if event.remove("servicenow.event.outside_maintenance_schedule.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.outside_maintenance_schedule.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.outside_maintenance_schedule.value") && event.get_str("servicenow.event.outside_maintenance_schedule.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.outside_maintenance_schedule.value") {
                if let Some(val) = event.get("servicenow.event.outside_maintenance_schedule.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.outside_maintenance_schedule.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.outside_maintenance_schedule.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_outside_maintenance_schedule_value_to_boolean")?;
                        if event.remove("servicenow.event.outside_maintenance_schedule.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.outside_maintenance_schedule.value".into() });
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

            let _cond = { event.has_value("servicenow.event.production_system.display_value") && event.get_str("servicenow.event.production_system.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.production_system.display_value") {
                if let Some(val) = event.get("servicenow.event.production_system.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.production_system.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.production_system.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_production_system_display_value_to_boolean")?;
                        if event.remove("servicenow.event.production_system.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.production_system.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.production_system.value") && event.get_str("servicenow.event.production_system.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.production_system.value") {
                if let Some(val) = event.get("servicenow.event.production_system.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.production_system.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.production_system.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_production_system_value_to_boolean")?;
                        if event.remove("servicenow.event.production_system.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.production_system.value".into() });
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

            if let Some(v) = event.get("servicenow.event.reason.display_value").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            let _cond = { event.has_value("servicenow.event.requested_by_date.display_value") && event.get_str("servicenow.event.requested_by_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.requested_by_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.requested_by_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.requested_by_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_requested_by_date_display_value")?;
                        if event.remove("servicenow.event.requested_by_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.requested_by_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.requested_by_date.display_value") && event.get_str("servicenow.event.requested_by_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.requested_by_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.requested_by_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.requested_by_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_requested_by_date_display_value")?;
                        if event.remove("servicenow.event.requested_by_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.requested_by_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.requested_by_date.value") && event.get_str("servicenow.event.requested_by_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.requested_by_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.requested_by_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.requested_by_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_requested_by_date_value")?;
                        if event.remove("servicenow.event.requested_by_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.requested_by_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.requested_by_date.value") && event.get_str("servicenow.event.requested_by_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.requested_by_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.requested_by_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.requested_by_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_requested_by_date_value")?;
                        if event.remove("servicenow.event.requested_by_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.requested_by_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.review_date.display_value") && event.get_str("servicenow.event.review_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.review_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.review_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.review_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_review_date_display_value")?;
                        if event.remove("servicenow.event.review_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.review_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.review_date.display_value") && event.get_str("servicenow.event.review_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.review_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.review_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.review_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_review_date_display_value")?;
                        if event.remove("servicenow.event.review_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.review_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.review_date.value") && event.get_str("servicenow.event.review_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.review_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.review_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.review_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_review_date_value")?;
                        if event.remove("servicenow.event.review_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.review_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.review_date.value") && event.get_str("servicenow.event.review_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.review_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.review_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.review_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_review_date_value")?;
                        if event.remove("servicenow.event.review_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.review_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.risk.value") && event.get_str("servicenow.event.risk.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.risk.value") {
                if let Some(val) = event.get("servicenow.event.risk.value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.risk.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.risk.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_risk_value_to_long")?;
                        if event.remove("servicenow.event.risk.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.risk.value".into() });
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

            let _cond = { event.has_value("servicenow.event.scope.value") && event.get_str("servicenow.event.scope.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.scope.value") {
                if let Some(val) = event.get("servicenow.event.scope.value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.scope.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.scope.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_scope_value_to_long")?;
                        if event.remove("servicenow.event.scope.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.scope.value".into() });
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

            let _cond = { event.has_value("servicenow.event.unauthorized.display_value") && event.get_str("servicenow.event.unauthorized.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.unauthorized.display_value") {
                if let Some(val) = event.get("servicenow.event.unauthorized.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.unauthorized.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.unauthorized.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_unauthorized_display_value_to_boolean")?;
                        if event.remove("servicenow.event.unauthorized.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.unauthorized.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.unauthorized.value") && event.get_str("servicenow.event.unauthorized.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.unauthorized.value") {
                if let Some(val) = event.get("servicenow.event.unauthorized.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.unauthorized.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.unauthorized.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_unauthorized_value_to_boolean")?;
                        if event.remove("servicenow.event.unauthorized.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.unauthorized.value".into() });
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

            let _cond = { event.get_str("servicenow.event.review_status.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.review_status.value") {
                if let Some(val) = event.get("servicenow.event.review_status.value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.review_status.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.review_status.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_review_status_to_long")?;
                        if event.remove("servicenow.event.review_status.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.review_status.value".into() });
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

            let _cond = { event.has_value("servicenow.event.conflict_last_run.display_value") && event.get_str("servicenow.event.conflict_last_run.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.conflict_last_run.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.conflict_last_run.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.conflict_last_run.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_conflict_last_run_display_value")?;
                        if event.remove("servicenow.event.conflict_last_run.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.conflict_last_run.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.conflict_last_run.display_value") && event.get_str("servicenow.event.conflict_last_run.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.conflict_last_run.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.conflict_last_run.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.conflict_last_run.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_conflict_last_run_display_value")?;
                        if event.remove("servicenow.event.conflict_last_run.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.conflict_last_run.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.conflict_last_run.value") && event.get_str("servicenow.event.conflict_last_run.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.conflict_last_run.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.conflict_last_run.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.conflict_last_run.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_conflict_last_run_value")?;
                        if event.remove("servicenow.event.conflict_last_run.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.conflict_last_run.value".into() });
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

            let _cond = { event.has_value("servicenow.event.conflict_last_run.value") && event.get_str("servicenow.event.conflict_last_run.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.conflict_last_run.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.conflict_last_run.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.conflict_last_run.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_conflict_last_run_value")?;
                        if event.remove("servicenow.event.conflict_last_run.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.conflict_last_run.value".into() });
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

            let _cond = { event.has_value("servicenow.event.cab_date_time.display_value") && event.get_str("servicenow.event.cab_date_time.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.cab_date_time.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.cab_date_time.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.cab_date_time.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cab_date_time_display_value")?;
                        if event.remove("servicenow.event.cab_date_time.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.cab_date_time.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.cab_date_time.display_value") && event.get_str("servicenow.event.cab_date_time.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.cab_date_time.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.cab_date_time.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.cab_date_time.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cab_date_time_display_value")?;
                        if event.remove("servicenow.event.cab_date_time.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.cab_date_time.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.cab_date_time.value") && event.get_str("servicenow.event.cab_date_time.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.cab_date_time.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.cab_date_time.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.cab_date_time.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cab_date_time_value")?;
                        if event.remove("servicenow.event.cab_date_time.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.cab_date_time.value".into() });
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

            let _cond = { event.has_value("servicenow.event.cab_date_time.value") && event.get_str("servicenow.event.cab_date_time.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.cab_date_time.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.cab_date_time.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.cab_date_time.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cab_date_time_value")?;
                        if event.remove("servicenow.event.cab_date_time.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.cab_date_time.value".into() });
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
