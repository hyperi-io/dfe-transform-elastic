// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_alm_hardware` pipeline.
pub struct PipelineAlmHardware;

impl Transform for PipelineAlmHardware {
    fn name(&self) -> &str {
        "pipeline_alm_hardware"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("asset"))?;

                event.append("event.type", json!("info"))?;

                event.append("event.category", json!("host"))?;

            let _cond = { event.has_value("servicenow.event.depreciated_amount.value") && event.get_str("servicenow.event.depreciated_amount.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.depreciated_amount.value") {
                if let Some(val) = event.get("servicenow.event.depreciated_amount.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.depreciated_amount.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.depreciated_amount.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_depreciated_amount_value_to_double")?;
                        if event.remove("servicenow.event.depreciated_amount.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.depreciated_amount.value".into() });
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

            let _cond = { event.has_value("servicenow.event.depreciation_date.display_value") && event.get_str("servicenow.event.depreciation_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.depreciation_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.depreciation_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.depreciation_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_depreciation_date_display_value")?;
                        if event.remove("servicenow.event.depreciation_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.depreciation_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.depreciation_date.display_value") && event.get_str("servicenow.event.depreciation_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.depreciation_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.depreciation_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.depreciation_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_depreciation_date_display_value")?;
                        if event.remove("servicenow.event.depreciation_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.depreciation_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.depreciation_date.value") && event.get_str("servicenow.event.depreciation_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.depreciation_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.depreciation_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.depreciation_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_depreciation_date_value")?;
                        if event.remove("servicenow.event.depreciation_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.depreciation_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.depreciation_date.value") && event.get_str("servicenow.event.depreciation_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.depreciation_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.depreciation_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.depreciation_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_depreciation_date_value")?;
                        if event.remove("servicenow.event.depreciation_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.depreciation_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.retirement_date.display_value") && event.get_str("servicenow.event.retirement_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.retirement_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.retirement_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.retirement_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_retirement_date_display_value")?;
                        if event.remove("servicenow.event.retirement_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.retirement_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.retirement_date.display_value") && event.get_str("servicenow.event.retirement_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.retirement_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.retirement_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.retirement_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_retirement_date_display_value")?;
                        if event.remove("servicenow.event.retirement_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.retirement_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.retirement_date.value") && event.get_str("servicenow.event.retirement_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.retirement_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.retirement_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.retirement_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_retirement_date_value")?;
                        if event.remove("servicenow.event.retirement_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.retirement_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.retirement_date.value") && event.get_str("servicenow.event.retirement_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.retirement_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.retirement_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.retirement_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_retirement_date_value")?;
                        if event.remove("servicenow.event.retirement_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.retirement_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.eligible_for_refresh.display_value") && event.get_str("servicenow.event.eligible_for_refresh.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.eligible_for_refresh.display_value") {
                if let Some(val) = event.get("servicenow.event.eligible_for_refresh.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.eligible_for_refresh.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.eligible_for_refresh.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_eligible_for_refresh_display_value_to_boolean")?;
                        if event.remove("servicenow.event.eligible_for_refresh.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.eligible_for_refresh.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.eligible_for_refresh.value") && event.get_str("servicenow.event.eligible_for_refresh.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.eligible_for_refresh.value") {
                if let Some(val) = event.get("servicenow.event.eligible_for_refresh.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.eligible_for_refresh.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.eligible_for_refresh.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_eligible_for_refresh_value_to_boolean")?;
                        if event.remove("servicenow.event.eligible_for_refresh.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.eligible_for_refresh.value".into() });
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

                event.append_unique("device.model.name", json!(event.get("servicenow.event.model.display_value").map_or_else(String::new, template_to_string)))?;

                event.append_unique("device.model.name", json!(event.get("servicenow.event.ci.display_value").map_or_else(String::new, template_to_string)))?;

            let _cond = { event.has_value("servicenow.event.pre_allocated.display_value") && event.get_str("servicenow.event.pre_allocated.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.pre_allocated.display_value") {
                if let Some(val) = event.get("servicenow.event.pre_allocated.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.pre_allocated.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.pre_allocated.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_pre_allocated_display_value_to_boolean")?;
                        if event.remove("servicenow.event.pre_allocated.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.pre_allocated.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.pre_allocated.value") && event.get_str("servicenow.event.pre_allocated.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.pre_allocated.value") {
                if let Some(val) = event.get("servicenow.event.pre_allocated.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.pre_allocated.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.pre_allocated.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_pre_allocated_value_to_boolean")?;
                        if event.remove("servicenow.event.pre_allocated.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.pre_allocated.value".into() });
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

            let _cond = { event.has_value("servicenow.event.resale_price.value") && event.get_str("servicenow.event.resale_price.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.resale_price.value") {
                if let Some(val) = event.get("servicenow.event.resale_price.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.resale_price.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.resale_price.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_resale_price_value_to_double")?;
                        if event.remove("servicenow.event.resale_price.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.resale_price.value".into() });
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

            let _cond = { event.has_value("servicenow.event.residual.value") && event.get_str("servicenow.event.residual.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.residual.value") {
                if let Some(val) = event.get("servicenow.event.residual.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.residual.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.residual.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_residual_value_to_double")?;
                        if event.remove("servicenow.event.residual.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.residual.value".into() });
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

            let _cond = { event.has_value("servicenow.event.residual_date.display_value") && event.get_str("servicenow.event.residual_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.residual_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.residual_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.residual_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_residual_date_display_value")?;
                        if event.remove("servicenow.event.residual_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.residual_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.residual_date.display_value") && event.get_str("servicenow.event.residual_date.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.residual_date.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.residual_date.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.residual_date.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_residual_date_display_value")?;
                        if event.remove("servicenow.event.residual_date.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.residual_date.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.residual_date.value") && event.get_str("servicenow.event.residual_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.residual_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.residual_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.residual_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_residual_date_value")?;
                        if event.remove("servicenow.event.residual_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.residual_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.residual_date.value") && event.get_str("servicenow.event.residual_date.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.residual_date.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.residual_date.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.residual_date.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_residual_date_value")?;
                        if event.remove("servicenow.event.residual_date.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.residual_date.value".into() });
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

            let _cond = { event.has_value("servicenow.event.resold_value.value") && event.get_str("servicenow.event.resold_value.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.resold_value.value") {
                if let Some(val) = event.get("servicenow.event.resold_value.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.resold_value.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.resold_value.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_resold_value_value_to_double")?;
                        if event.remove("servicenow.event.resold_value.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.resold_value.value".into() });
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

            let _cond = { event.has_value("servicenow.event.salvage_value.value") && event.get_str("servicenow.event.salvage_value.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.salvage_value.value") {
                if let Some(val) = event.get("servicenow.event.salvage_value.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.salvage_value.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.salvage_value.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_salvage_value_value_to_double")?;
                        if event.remove("servicenow.event.salvage_value.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.salvage_value.value".into() });
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
