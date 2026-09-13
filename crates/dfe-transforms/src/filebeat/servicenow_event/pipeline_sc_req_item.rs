// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_sc_req_item` pipeline.
pub struct PipelineScReqItem;

impl Transform for PipelineScReqItem {
    fn name(&self) -> &str {
        "pipeline_sc_req_item"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                event.append("event.type", json!("info"))?;

                event.append("event.category", json!("configuration"))?;

            let _cond = { event.get_str("servicenow.event.price.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.price.value") {
                if let Some(val) = event.get("servicenow.event.price.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.price.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.price.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_price_value_to_double")?;
                        if event.remove("servicenow.event.price.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.price.value".into() });
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

            let _cond = { event.get_str("servicenow.event.backordered.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.backordered.display_value") {
                if let Some(val) = event.get("servicenow.event.backordered.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.backordered.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.backordered.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_backordered_display_value_to_boolean")?;
                        if event.remove("servicenow.event.backordered.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.backordered.display_value".into() });
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

            let _cond = { event.get_str("servicenow.event.backordered.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.backordered.value") {
                if let Some(val) = event.get("servicenow.event.backordered.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.backordered.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.backordered.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_backordered_value_to_boolean")?;
                        if event.remove("servicenow.event.backordered.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.backordered.value".into() });
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

            let _cond = { event.get_str("servicenow.event.recurring_price.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.recurring_price.value") {
                if let Some(val) = event.get("servicenow.event.recurring_price.value") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.recurring_price.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.recurring_price.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_recurring_price_value_to_double")?;
                        if event.remove("servicenow.event.recurring_price.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.recurring_price.value".into() });
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

            let _cond = { event.get_str("servicenow.event.billable.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.billable.display_value") {
                if let Some(val) = event.get("servicenow.event.billable.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.billable.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.billable.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_billable_display_value_to_boolean")?;
                        if event.remove("servicenow.event.billable.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.billable.display_value".into() });
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

            let _cond = { event.get_str("servicenow.event.billable.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.billable.value") {
                if let Some(val) = event.get("servicenow.event.billable.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.billable.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.billable.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_billable_value_to_boolean")?;
                        if event.remove("servicenow.event.billable.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.billable.value".into() });
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

            let _cond = { event.has_value("servicenow.event.estimated_delivery.display_value") && event.get_str("servicenow.event.estimated_delivery.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.estimated_delivery.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.estimated_delivery.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.estimated_delivery.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_estimated_delivery_display_value")?;
                        if event.remove("servicenow.event.estimated_delivery.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.estimated_delivery.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.estimated_delivery.display_value") && event.get_str("servicenow.event.estimated_delivery.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.estimated_delivery.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("servicenow.event.estimated_delivery.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.estimated_delivery.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_estimated_delivery_display_value")?;
                        if event.remove("servicenow.event.estimated_delivery.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.estimated_delivery.display_value".into() });
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

            let _cond = { event.has_value("servicenow.event.estimated_delivery.value") && event.get_str("servicenow.event.estimated_delivery.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.estimated_delivery.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.estimated_delivery.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.estimated_delivery.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_estimated_delivery_value")?;
                        if event.remove("servicenow.event.estimated_delivery.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.estimated_delivery.value".into() });
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

            let _cond = { event.has_value("servicenow.event.estimated_delivery.value") && event.get_str("servicenow.event.estimated_delivery.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.estimated_delivery.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.estimated_delivery.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.estimated_delivery.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_estimated_delivery_value")?;
                        if event.remove("servicenow.event.estimated_delivery.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.estimated_delivery.value".into() });
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
