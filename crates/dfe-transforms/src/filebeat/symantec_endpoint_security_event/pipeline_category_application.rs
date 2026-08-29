// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_category_application` pipeline.
pub struct PipelineCategoryApplication;

impl Transform for PipelineCategoryApplication {
    fn name(&self) -> &str {
        "pipeline_category_application"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ses.category_name", json!("Application Activity"))?;

            let _cond = { event.get_str("ses.type_id") == Some("5") };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = { event.has_value("ses.type_id") && ["3", "4"].contains(&event.get_str("ses.type_id").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = { event.get_str("ses.type_id") == Some("4") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.has_value("ses.type_id") && ["3", "5"].contains(&event.get_str("ses.type_id").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("ses.change_type_id") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.change_type_id") {
                if let Some(val) = event.get("ses.change_type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.change_type_id".into(),
                            message,
                        })?;
                    event.set("ses.change_type_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_type_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.change_type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("ses.channel_id") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.channel_id") {
                if let Some(val) = event.get("ses.channel_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.channel_id".into(),
                            message,
                        })?;
                    event.set("ses.channel_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_channel_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.channel_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("ses.content_type_id") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.content_type_id") {
                if let Some(val) = event.get("ses.content_type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.content_type_id".into(),
                            message,
                        })?;
                    event.set("ses.content_type_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_content_type_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.content_type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ses.curr_location.coordinates").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.curr_location.coordinates", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_curr_location_coordinates_to_float")?;
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

            let _cond = { event.get_str("ses.curr_location.on_premises") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.curr_location.on_premises") {
                if let Some(val) = event.get("ses.curr_location.on_premises") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.curr_location.on_premises".into(),
                            message,
                        })?;
                    event.set("ses.curr_location.on_premises", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_curr_location_on_premises_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.curr_location.on_premises");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("ses.http_status") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.http_status") {
                if let Some(val) = event.get("ses.http_status") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.http_status".into(),
                            message,
                        })?;
                    event.set("ses.http_status", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_http_status_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.http_status");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ses.http_status") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.append_unique("http.response.status_code", json!(event.get("ses.http_status").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "append")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("http.response.status_code") != Some("") };
            if _cond {
            if event.has_value("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        })?;
                    event.set("http.response.status_code", converted)?;
                }
            }
            }

            let _cond = { event.get("ses.prev_location.coordinates").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.prev_location.coordinates", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_prev_location_coordinates_to_float")?;
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

            let _cond = { event.get_str("ses.prev_location.on_premises") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.prev_location.on_premises") {
                if let Some(val) = event.get("ses.prev_location.on_premises") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.prev_location.on_premises".into(),
                            message,
                        })?;
                    event.set("ses.prev_location.on_premises", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_prev_location_on_premises_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.prev_location.on_premises");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("ses.sender_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.sender_ip") {
                if let Some(val) = event.get("ses.sender_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.sender_ip".into(),
                            message,
                        })?;
                    event.set("ses.sender_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sender_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.sender_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ses.sender_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("ses.sender_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ses.sender_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.get("ses.url.category_ids").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.url.category_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_url_category_ids_to_string")?;
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

            let _cond = { event.has_value("ses.url.host") };
            if _cond {
                event.append_unique("url.domain", json!(event.get("ses.url.host").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ses.url.method") };
            if _cond {
                event.append_unique("http.request.method", json!(event.get("ses.url.method").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ses.url.path") };
            if _cond {
                event.append_unique("url.path", json!(event.get("ses.url.path").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("ses.url.port") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.url.port") {
                if let Some(val) = event.get("ses.url.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.url.port".into(),
                            message,
                        })?;
                    event.set("ses.url.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_url_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.url.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ses.url.port") };
            if _cond {
                event.append_unique("url.port", json!(event.get("ses.url.port").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ses.url.query") };
            if _cond {
                event.append_unique("url.query", json!(event.get("ses.url.query").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ses.url.referrer_category_ids").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.url.referrer_category_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_url_referrer_category_ids_to_string")?;
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

            let _cond = { event.get_str("ses.url.rep_score_id") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ses.url.rep_score_id") {
                if let Some(val) = event.get("ses.url.rep_score_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ses.url.rep_score_id".into(),
                            message,
                        })?;
                    event.set("ses.url.rep_score_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_url_rep_score_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("ses.url.rep_score_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ses.url.scheme") };
            if _cond {
                event.append_unique("url.scheme", json!(event.get("ses.url.scheme").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ses.url.text") };
            if _cond {
                event.append_unique("url.full", json!(event.get("ses.url.text").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ses.type_id") && ["2", "3", "4", "5", "11", "12", "13", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && event.has_value("ses.container") };
            if _cond {
                // Begin nested pipeline: "pipeline_object_container"
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.container.networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.gateway_ip") {
                if let Some(val) = event.get("_ingest._value.gateway_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.gateway_ip".into(),
                message,
                })?;
                event.set("_ingest._value.gateway_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_container_networks_gateway_ip_to_ip")?;
                event.remove("_ingest._value.gateway_ip");
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
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.container.networks").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.container.networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.gateway_ip").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.container.networks", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.container.networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.ipv4") {
                if let Some(val) = event.get("_ingest._value.ipv4") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.ipv4".into(),
                message,
                })?;
                event.set("_ingest._value.ipv4", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_container_networks_ipv4_to_ip")?;
                event.remove("_ingest._value.ipv4");
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
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.container.networks").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.container.networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.ipv4").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.container.networks", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.container.networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.ipv6") {
                if let Some(val) = event.get("_ingest._value.ipv6") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.ipv6".into(),
                message,
                })?;
                event.set("_ingest._value.ipv6", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_container_networks_ipv6_to_ip")?;
                event.remove("_ingest._value.ipv6");
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
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.container.networks").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.container.networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.ipv6").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.container.networks", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.container.networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.is_public") {
                if let Some(val) = event.get("_ingest._value.is_public") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.is_public".into(),
                message,
                })?;
                event.set("_ingest._value.is_public", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_container_networks_is_public_to_boolean")?;
                event.remove("_ingest._value.is_public");
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
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.container.networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_score_id") {
                if let Some(val) = event.get("_ingest._value.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_score_id".into(),
                message,
                })?;
                event.set("_ingest._value.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_container_networks_rep_score_id_to_string")?;
                event.remove("_ingest._value.rep_score_id");
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
                let _cond = { event.get("ses.container.networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.container.networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.type_id") {
                if let Some(val) = event.get("_ingest._value.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.type_id".into(),
                message,
                })?;
                event.set("_ingest._value.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_container_networks_type_id_to_string")?;
                event.remove("_ingest._value.type_id");
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
                // End nested pipeline: "pipeline_object_container"
            }

            let _cond = { event.has_value("ses.type_id") && ["2", "3", "4", "5", "11", "12", "13", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && event.has_value("ses.cybox") };
            if _cond {
                // Begin nested pipeline: "pipeline_object_cybox"
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.direction_id") {
                if let Some(val) = event.get("_ingest._value.direction_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.direction_id".into(),
                message,
                })?;
                event.set("_ingest._value.direction_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_emails_direction_id_to_string")?;
                event.remove("_ingest._value.direction_id");
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
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet();\n  if (ctx.email != null && ctx.email.direction != null) {\n      var = ctx.email.direction;\n  } else {\n    if (ctx.email == null)\n    {\n      ctx.email = new HashMap();\n    }\n  }\n\nfor (def email : ctx.ses.cybox.emails) {\n  def direction = email.direction_id;\n  if (params.containsKey(direction.toString())) {\n    def type = params.get(direction.toString());\n    var.add(type);\n  }\n}\nctx.email.put('direction', var)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet();\n  if (ctx.email != null && ctx.email.direction != null) {\n      var = ctx.email.direction;\n  } else {\n    if (ctx.email == null)\n    {\n      ctx.email = new HashMap();\n    }\n  }\n\nfor (def email : ctx.ses.cybox.emails) {\n  def direction = email.direction_id;\n  if (params.containsKey(direction.toString())) {\n    def type = params.get(direction.toString());\n    var.add(type);\n  }\n}\nctx.email.put('direction', var)"#), cached_params!("{\"0\":\"unknown\",\"1\":\"inbound\",\"2\":\"outbound\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_email_direction")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                event.append_unique("email.from.address", json!(event.get("_ingest._value.header_from").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                event.append_unique("email.subject", json!(event.get("_ingest._value.header_subject").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                foreach_array(event, "_ingest._value.header_to", |event| {
                event.append_unique("email.to.address", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.sender_ip") {
                if let Some(val) = event.get("_ingest._value.sender_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.sender_ip".into(),
                message,
                })?;
                event.set("_ingest._value.sender_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_emails_sender_ip_to_ip")?;
                event.remove("_ingest._value.sender_ip");
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
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                let _cond = { event.has_value("ses.container.networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.sender_ip").map_or_else(String::new, template_to_string)))?;
                }
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.size") {
                if let Some(val) = event.get("_ingest._value.size") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.size".into(),
                message,
                })?;
                event.set("_ingest._value.size", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_emails_size_to_long")?;
                event.remove("_ingest._value.size");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.files").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.accessed") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("_ingest._value.accessed", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.accessed".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cybox_files_accessed")?;
                event.remove("_ingest._value.accessed");
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.files", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.accessed", json!(event.get("_ingest._value.accessed").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.attribute_ids == null || !file.containsKey('attribute_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.attribute_ids.length; j++) {\n    if (file.attribute_ids[j] != null) {\n      new_ids.add(file.attribute_ids[j].toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.attribute_ids = new_ids;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.attribute_ids == null || !file.containsKey('attribute_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.attribute_ids.length; j++) {\n    if (file.attribute_ids[j] != null) {\n      new_ids.add(file.attribute_ids[j].toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.attribute_ids = new_ids;\n  }\n}"#))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_convert_attribute_ids")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def file : ctx.ses.cybox.files) {\n  if (file.attribute_ids == null) {\n    continue;\n  }\n  for (def id : file.attribute_ids) {\n    def type = params[id.toString()];\n    if (type != null) {\n      var.add(type);\n    }\n  }\n} ctx.file.put('attributes', var)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def file : ctx.ses.cybox.files) {\n  if (file.attribute_ids == null) {\n    continue;\n  }\n  for (def id : file.attribute_ids) {\n    def type = params[id.toString()];\n    if (type != null) {\n      var.add(type);\n    }\n  }\n} ctx.file.put('attributes', var)"#), cached_params!("{\"1\":\"archive\",\"2\":\"compressed\",\"3\":\"directory\",\"4\":\"encrypted\",\"5\":\"hidden\",\"8\":\"readonly\",\"11\":\"system\",\"16\":\"execute\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_attributes")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.attributes") {
                if let Some(val) = event.get("_ingest._value.attributes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.attributes".into(),
                message,
                })?;
                event.set("_ingest._value.attributes", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_attributes_to_long")?;
                event.remove("_ingest._value.attributes");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.confidentiality_id") {
                if let Some(val) = event.get("_ingest._value.confidentiality_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.confidentiality_id".into(),
                message,
                })?;
                event.set("_ingest._value.confidentiality_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_confidentiality_id_to_string")?;
                event.remove("_ingest._value.confidentiality_id");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.content_type.family_id") {
                if let Some(val) = event.get("_ingest._value.content_type.family_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.content_type.family_id".into(),
                message,
                })?;
                event.set("_ingest._value.content_type.family_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_content_type_family_id_to_string")?;
                event.remove("_ingest._value.content_type.family_id");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.content_type.type_id") {
                if let Some(val) = event.get("_ingest._value.content_type.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.content_type.type_id".into(),
                message,
                })?;
                event.set("_ingest._value.content_type.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_content_type_type_id_to_string")?;
                event.remove("_ingest._value.content_type.type_id");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.files").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.created") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("_ingest._value.created", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.created".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cybox_files_created")?;
                event.remove("_ingest._value.created");
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.files", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.created", json!(event.get("_ingest._value.created").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.is_system") {
                if let Some(val) = event.get("_ingest._value.is_system") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.is_system".into(),
                message,
                })?;
                event.set("_ingest._value.is_system", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_is_system_to_boolean")?;
                event.remove("_ingest._value.is_system");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.hash.md5", json!(event.get("_ingest._value.md5").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.mime_type", json!(event.get("_ingest._value.mime_type").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.files").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.modified") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("_ingest._value.modified", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.modified".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cybox_files_modified")?;
                event.remove("_ingest._value.modified");
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.files", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.directory", json!(event.get("_ingest._value.folder").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.path", json!(event.get("_ingest._value.path").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_discovered_band") {
                if let Some(val) = event.get("_ingest._value.rep_discovered_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_discovered_band".into(),
                message,
                })?;
                event.set("_ingest._value.rep_discovered_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_discovered_band_to_long")?;
                event.remove("_ingest._value.rep_discovered_band");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.files").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.rep_discovered_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("_ingest._value.rep_discovered_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.rep_discovered_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cybox_files_rep_discovered_date")?;
                event.remove("_ingest._value.rep_discovered_date");
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.files", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_prevalence") {
                if let Some(val) = event.get("_ingest._value.rep_prevalence") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_prevalence".into(),
                message,
                })?;
                event.set("_ingest._value.rep_prevalence", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_prevalence_to_long")?;
                event.remove("_ingest._value.rep_prevalence");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_prevalence_band") {
                if let Some(val) = event.get("_ingest._value.rep_prevalence_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_prevalence_band".into(),
                message,
                })?;
                event.set("_ingest._value.rep_prevalence_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_prevalence_band_to_long")?;
                event.remove("_ingest._value.rep_prevalence_band");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_score") {
                if let Some(val) = event.get("_ingest._value.rep_score") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_score".into(),
                message,
                })?;
                event.set("_ingest._value.rep_score", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_score_to_long")?;
                event.remove("_ingest._value.rep_score");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_score_band") {
                if let Some(val) = event.get("_ingest._value.rep_score_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_score_band".into(),
                message,
                })?;
                event.set("_ingest._value.rep_score_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_score_band_to_long")?;
                event.remove("_ingest._value.rep_score_band");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.hash.sha1", json!(event.get("_ingest._value.sha1").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.files").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.signature_created_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("_ingest._value.signature_created_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.signature_created_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cybox_files_signature_created_date")?;
                event.remove("_ingest._value.signature_created_date");
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.files", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.x509.issuer.distinguished_name", json!(event.get("_ingest._value.signature_issuer").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.signature_level_id") {
                if let Some(val) = event.get("_ingest._value.signature_level_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.signature_level_id".into(),
                message,
                })?;
                event.set("_ingest._value.signature_level_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_signature_level_id_to_string")?;
                event.remove("_ingest._value.signature_level_id");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.x509.serial_number", json!(event.get("_ingest._value.signature_serial_number").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.signature_value") {
                if let Some(val) = event.get("_ingest._value.signature_value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.signature_value".into(),
                message,
                })?;
                event.set("_ingest._value.signature_value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_signature_value_to_long")?;
                event.remove("_ingest._value.signature_value");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.signature_value_ids == null || !file.containsKey('signature_value_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.signature_value_ids.length; j++) {\n    def value_id = file.signature_value_ids[j];\n    if (value_id != null) {\n      new_ids.add(value_id.toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.signature_value_ids = new_ids;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.signature_value_ids == null || !file.containsKey('signature_value_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.signature_value_ids.length; j++) {\n    def value_id = file.signature_value_ids[j];\n    if (value_id != null) {\n      new_ids.add(value_id.toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.signature_value_ids = new_ids;\n  }\n}"#))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_convert_signature_value_ids")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.size") {
                if let Some(val) = event.get("_ingest._value.size") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.size".into(),
                message,
                })?;
                event.set("_ingest._value.size", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_size_to_long")?;
                event.remove("_ingest._value.size");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.size", json!(event.get("_ingest._value.size").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.size_compressed") {
                if let Some(val) = event.get("_ingest._value.size_compressed") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.size_compressed".into(),
                message,
                })?;
                event.set("_ingest._value.size_compressed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_size_compressed_to_long")?;
                event.remove("_ingest._value.size_compressed");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.src_ip") {
                if let Some(val) = event.get("_ingest._value.src_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.src_ip".into(),
                message,
                })?;
                event.set("_ingest._value.src_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_src_ip_to_ip")?;
                event.remove("_ingest._value.src_ip");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.files").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.cybox.files") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.src_ip").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.files", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.type_id") {
                if let Some(val) = event.get("_ingest._value.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.type_id".into(),
                message,
                })?;
                event.set("_ingest._value.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_type_id_to_string")?;
                event.remove("_ingest._value.type_id");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n}\nfor (def file : ctx.ses.cybox.files) {\n  if (file.type_id == null) {\n    continue;\n  }\n  def type = params[file.type_id.toString()];\n  if (type != null) {\n      var.add(type);\n  }  \n}\nctx.file.put('type', var);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n}\nfor (def file : ctx.ses.cybox.files) {\n  if (file.type_id == null) {\n    continue;\n  }\n  def type = params[file.type_id.toString()];\n  if (type != null) {\n      var.add(type);\n  }  \n}\nctx.file.put('type', var); "#), cached_params!("{\"1\":\"file\",\"2\":\"dir\",\"6\":\"symlink\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_type")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                if event.has_value("_ingest._value.url.category_ids") {
                foreach_array(event, "_ingest._value.url.category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_category_ids_to_string")?;
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
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.url.port") {
                if let Some(val) = event.get("_ingest._value.url.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.url.port".into(),
                message,
                })?;
                event.set("_ingest._value.url.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_port_to_long")?;
                event.remove("_ingest._value.url.port");
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
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                if event.has_value("_ingest._value.url.referrer_category_ids") {
                foreach_array(event, "_ingest._value.url.referrer_category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_referrer_category_ids_to_string")?;
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
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.url.rep_score_id") {
                if let Some(val) = event.get("_ingest._value.url.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.url.rep_score_id".into(),
                message,
                })?;
                event.set("_ingest._value.url.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_rep_score_id_to_string")?;
                event.remove("_ingest._value.url.rep_score_id");
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
                let _cond = { event.get("ses.cybox.ipv4s").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.ipv4s", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_ipv4s_to_ip")?;
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
                let _cond = { event.get("ses.cybox.ipv4s").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.ipv4s").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.cybox.ipv4s") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.ipv4s", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.ipv6s").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.ipv6s", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_ipv6s_to_ip")?;
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
                let _cond = { event.get("ses.cybox.ipv6s").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.cybox.ipv6s").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.cybox.ipv6s") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.cybox.ipv6s", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                foreach_array(event, "_ingest._value.category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_category_ids_to_string")?;
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
                })?;
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                event.append_unique("url.path", json!(event.get("_ingest._value.path").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.port") {
                if let Some(val) = event.get("_ingest._value.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.port".into(),
                message,
                })?;
                event.set("_ingest._value.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_port_to_long")?;
                event.remove("_ingest._value.port");
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
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                event.append_unique("url.port", json!(event.get("_ingest._value.port").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("url.port").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "url.port", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_url_port_to_long")?;
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
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                event.append_unique("url.query", json!(event.get("_ingest._value.query").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                foreach_array(event, "_ingest._value.referrer_category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_referrer_category_ids_to_string")?;
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
                })?;
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_score_id") {
                if let Some(val) = event.get("_ingest._value.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_score_id".into(),
                message,
                })?;
                event.set("_ingest._value.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_rep_score_id_to_string")?;
                event.remove("_ingest._value.rep_score_id");
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
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                event.append_unique("url.scheme", json!(event.get("_ingest._value.scheme").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                event.append_unique("url.full", json!(event.get("_ingest._value.text").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("related.hash", json!(event.get("_ingest._value.md5").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("related.hash", json!(event.get("_ingest._value.sha1").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("related.hash", json!(event.get("_ingest._value.sha2").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("file.hash.sha256", json!(event.get("_ingest._value.sha2").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("related.hash", json!(event.get("_ingest._value.parent_sha2").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest._value.src_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("_ingest._value.header_from");
                event.remove("_ingest._value.header_subject");
                event.remove("_ingest._value.header_to");
                }
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("_ingest._value.accessed");
                event.remove("_ingest._value.created");
                event.remove("_ingest._value.md5");
                event.remove("_ingest._value.sha1");
                event.remove("_ingest._value.sha2");
                event.remove("_ingest._value.mime_type");
                event.remove("_ingest._value.name");
                event.remove("_ingest._value.folder");
                event.remove("_ingest._value.path");
                event.remove("_ingest._value.size");
                event.remove("_ingest._value.signature_issuer");
                event.remove("_ingest._value.signature_serial_number");
                }
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("_ingest._value.text");
                event.remove("_ingest._value.path");
                event.remove("_ingest._value.port");
                event.remove("_ingest._value.query");
                event.remove("_ingest._value.scheme");
                }
                Ok(())
                })?;
                }
                // End nested pipeline: "pipeline_object_cybox"
            }

            let _cond = { event.has_value("ses.type_id") && ["2", "3", "4", "5", "11", "12", "13", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && (event.has_value("ses.device_cloud_vm") || event.has_value("ses.device_location") || event.has_value("ses.device_networks")) };
            if _cond {
                // Begin nested pipeline: "pipeline_object_device"
                if let Some(v) = event.get("ses.device_location.city").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.geo.city_name", v)?;
                }
                if let Some(v) = event.get("ses.device_location.continent").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.geo.continent_name", v)?;
                }
                let _cond = { event.get("ses.device_location.coordinates").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_location.coordinates", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "float")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_location_coordinates_to_float")?;
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
                if let Some(v) = event.get("ses.device_location.country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.geo.country_iso_code", v)?;
                }
                let _cond = { event.get_str("ses.device_location.on_premises") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.device_location.on_premises") {
                if let Some(val) = event.get("ses.device_location.on_premises") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.device_location.on_premises".into(),
                message,
                })?;
                event.set("ses.device_location.on_premises", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_location_on_premises_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.device_location.on_premises");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if let Some(v) = event.get("ses.device_location.region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.geo.region_name", v)?;
                }
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.gateway_ip") {
                if let Some(val) = event.get("_ingest._value.gateway_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.gateway_ip".into(),
                message,
                })?;
                event.set("_ingest._value.gateway_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_networks_gateway_ip_to_ip")?;
                event.remove("_ingest._value.gateway_ip");
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
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.device_networks").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.device_networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.gateway_ip").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.device_networks", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.ipv4") {
                if let Some(val) = event.get("_ingest._value.ipv4") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.ipv4".into(),
                message,
                })?;
                event.set("_ingest._value.ipv4", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_networks_ipv4_to_ip")?;
                event.remove("_ingest._value.ipv4");
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
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                event.append_unique("host.ip", json!(event.get("_ingest._value.ipv4").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.device_networks").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.device_networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.ipv4").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.device_networks", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.ipv6") {
                if let Some(val) = event.get("_ingest._value.ipv6") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.ipv6".into(),
                message,
                })?;
                event.set("_ingest._value.ipv6", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_networks_ipv6_to_ip")?;
                event.remove("_ingest._value.ipv6");
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
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.device_networks").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.device_networks") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.ipv6").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.device_networks", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.is_public") {
                if let Some(val) = event.get("_ingest._value.is_public") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.is_public".into(),
                message,
                })?;
                event.set("_ingest._value.is_public", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_networks_is_public_to_boolean")?;
                event.remove("_ingest._value.is_public");
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
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                event.append_unique("_tmp_raw_host_macs", json!(event.get("_ingest._value.mac").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("_tmp_raw_host_macs").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "_tmp_raw_host_macs", |event| {
                if event.has_value("_ingest._value") {
                gsub_field(event, "_ingest._value", "_ingest._value", cached_regex!("[-:.]"), "-")?;
                }
                Ok(())
                })?;
                }
                let _cond = { event.get("_tmp_raw_host_macs").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "_tmp_raw_host_macs", |event| {
                event.append_unique("host.mac", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                event.remove("_tmp_raw_host_macs");
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.rep_score_id") {
                if let Some(val) = event.get("_ingest._value.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.rep_score_id".into(),
                message,
                })?;
                event.set("_ingest._value.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_networks_rep_score_id_to_string")?;
                event.remove("_ingest._value.rep_score_id");
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
                let _cond = { event.get("ses.device_networks").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.device_networks", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.type_id") {
                if let Some(val) = event.get("_ingest._value.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.type_id".into(),
                message,
                })?;
                event.set("_ingest._value.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_device_networks_type_id_to_string")?;
                event.remove("_ingest._value.type_id");
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
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("ses.device_location.city");
                event.remove("ses.device_location.continent");
                event.remove("ses.device_location.country");
                event.remove("ses.device_location.region");
                }
                // End nested pipeline: "pipeline_object_device"
            }

            let _cond = { event.has_value("ses.type_id") && ["5"].contains(&event.get_str("ses.type_id").unwrap_or("")) && (event.has_value("ses.file") || event.has_value("ses.file_result")) };
            if _cond {
                // Begin nested pipeline: "pipeline_object_file"
                let _cond = { event.has_value("ses.file.accessed") && event.get_str("ses.file.accessed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file.accessed") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file.accessed", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file.accessed".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_accessed")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.accessed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.accessed") };
                if _cond {
                event.append_unique("file.accessed", json!(event.get("ses.file.accessed").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get("ses.file.attribute_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file.attribute_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_attribute_ids_to_string")?;
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
                let _cond = { event.get("ses.file.attribute_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def obj : ctx.ses.file.attribute_ids) {\n  if (params.containsKey(obj.toString())) {\n    def type = params.get(obj.toString());\n    var.add(type);\n  }\n}\nctx.file.put('attributes', var)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def obj : ctx.ses.file.attribute_ids) {\n  if (params.containsKey(obj.toString())) {\n    def type = params.get(obj.toString());\n    var.add(type);\n  }\n}\nctx.file.put('attributes', var)"#), cached_params!("{\"1\":\"archive\",\"2\":\"compressed\",\"3\":\"directory\",\"4\":\"encrypted\",\"5\":\"hidden\",\"8\":\"readonly\",\"11\":\"system\",\"16\":\"execute\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_attributes")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.attributes") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.attributes") {
                if let Some(val) = event.get("ses.file.attributes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.attributes".into(),
                message,
                })?;
                event.set("ses.file.attributes", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_attributes_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.attributes");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.confidentiality_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.confidentiality_id") {
                if let Some(val) = event.get("ses.file.confidentiality_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.confidentiality_id".into(),
                message,
                })?;
                event.set("ses.file.confidentiality_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_confidentiality_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.confidentiality_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.content_type.family_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.content_type.family_id") {
                if let Some(val) = event.get("ses.file.content_type.family_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.content_type.family_id".into(),
                message,
                })?;
                event.set("ses.file.content_type.family_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_content_type_family_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.content_type.family_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.content_type.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.content_type.type_id") {
                if let Some(val) = event.get("ses.file.content_type.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.content_type.type_id".into(),
                message,
                })?;
                event.set("ses.file.content_type.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_content_type_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.content_type.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.type_id") };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def obj = ctx.ses.file.type_id;\nif (params.containsKey(obj.toString())) {\n  def type = params.get(obj.toString());\n  ctx.ses.file.type_value = type\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def obj = ctx.ses.file.type_id;\nif (params.containsKey(obj.toString())) {\n  def type = params.get(obj.toString());\n  ctx.ses.file.type_value = type\n}"#), cached_params!("{\"1\":\"File\",\"2\":\"Directory\",\"3\":\"Hard Link\",\"4\":\"Mount\",\"5\":\"Node\",\"6\":\"Symbolic Link\",\"7\":\"Named Pipe\",\"8\":\"Socket\",\"9\":\"Device\",\"10\":\"Email\",\"11\":\"Memory File\",\"12\":\"File in container\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_type_value")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.created") && event.get_str("ses.file.created") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file.created") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file.created", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file.created".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_created")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.created");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.created") };
                if _cond {
                event.append_unique("file.created", json!(event.get("ses.file.created").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.file.is_system") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.is_system") {
                if let Some(val) = event.get("ses.file.is_system") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.is_system".into(),
                message,
                })?;
                event.set("ses.file.is_system", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_is_system_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.is_system");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.md5") };
                if _cond {
                event.append_unique("file.hash.md5", json!(event.get("ses.file.md5").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.mime_type") };
                if _cond {
                event.append_unique("file.mime_type", json!(event.get("ses.file.mime_type").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.modified") && event.get_str("ses.file.modified") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file.modified") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file.modified", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file.modified".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_modified")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.modified");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.name") };
                if _cond {
                event.append_unique("file.name", json!(event.get("ses.file.name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.folder") };
                if _cond {
                event.append_unique("file.directory", json!(event.get("ses.file.folder").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.path") };
                if _cond {
                event.append_unique("file.path", json!(event.get("ses.file.path").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.file.rep_discovered_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.rep_discovered_band") {
                if let Some(val) = event.get("ses.file.rep_discovered_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.rep_discovered_band".into(),
                message,
                })?;
                event.set("ses.file.rep_discovered_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_rep_discovered_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.rep_discovered_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.rep_discovered_date") && event.get_str("ses.file.rep_discovered_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file.rep_discovered_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file.rep_discovered_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file.rep_discovered_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_rep_discovered_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.rep_discovered_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.rep_prevalence") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.rep_prevalence") {
                if let Some(val) = event.get("ses.file.rep_prevalence") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.rep_prevalence".into(),
                message,
                })?;
                event.set("ses.file.rep_prevalence", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_rep_prevalence_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.rep_prevalence");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.rep_prevalence_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.rep_prevalence_band") {
                if let Some(val) = event.get("ses.file.rep_prevalence_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.rep_prevalence_band".into(),
                message,
                })?;
                event.set("ses.file.rep_prevalence_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_rep_prevalence_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.rep_prevalence_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.rep_score") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.rep_score") {
                if let Some(val) = event.get("ses.file.rep_score") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.rep_score".into(),
                message,
                })?;
                event.set("ses.file.rep_score", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_rep_score_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.rep_score");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.rep_score_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.rep_score_band") {
                if let Some(val) = event.get("ses.file.rep_score_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.rep_score_band".into(),
                message,
                })?;
                event.set("ses.file.rep_score_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_rep_score_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.rep_score_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.sha1") };
                if _cond {
                event.append_unique("file.hash.sha1", json!(event.get("ses.file.sha1").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.signature_created_date") && event.get_str("ses.file.signature_created_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file.signature_created_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file.signature_created_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file.signature_created_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_created_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.signature_created_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.signature_issuer") };
                if _cond {
                event.append_unique("file.x509.issuer.distinguished_name", json!(event.get("ses.file.signature_issuer").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.file.signature_level_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.signature_level_id") {
                if let Some(val) = event.get("ses.file.signature_level_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.signature_level_id".into(),
                message,
                })?;
                event.set("ses.file.signature_level_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_signature_level_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.signature_level_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.signature_serial_number") };
                if _cond {
                event.append_unique("file.x509.serial_number", json!(event.get("ses.file.signature_serial_number").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.file.signature_value") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.signature_value") {
                if let Some(val) = event.get("ses.file.signature_value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.signature_value".into(),
                message,
                })?;
                event.set("ses.file.signature_value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_signature_value_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.signature_value");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file.signature_value_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file.signature_value_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_signature_value_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file.size") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.size") {
                if let Some(val) = event.get("ses.file.size") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.size".into(),
                message,
                })?;
                event.set("ses.file.size", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.size");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.size_compressed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.size_compressed") {
                if let Some(val) = event.get("ses.file.size_compressed") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.size_compressed".into(),
                message,
                })?;
                event.set("ses.file.size_compressed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size_compressed_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.size_compressed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file.src_ip") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.src_ip") {
                if let Some(val) = event.get("ses.file.src_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.src_ip".into(),
                message,
                })?;
                event.set("ses.file.src_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_src_ip_to_ip")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.src_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.src_ip") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("ses.file.src_ip").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.file.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.type_id") {
                if let Some(val) = event.get("ses.file.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.type_id".into(),
                message,
                })?;
                event.set("ses.file.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.size") && event.get_str("ses.file.type_id") == Some("1") };
                if _cond {
                event.append_unique("file.size", json!(event.get("ses.file.size").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.type_id") };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n} def type_id = ctx.ses.file.type_id; if (params.containsKey(type_id.toString())) {\n    def type = params.get(type_id.toString());\n    var.add(type);\n} ctx.file.put('type', var);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n} def type_id = ctx.ses.file.type_id; if (params.containsKey(type_id.toString())) {\n    def type = params.get(type_id.toString());\n    var.add(type);\n} ctx.file.put('type', var);"#), cached_params!("{\"1\":\"file\",\"2\":\"dir\",\"6\":\"symlink\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_type")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file.url.category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file.url.category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_url_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file.url.port") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.url.port") {
                if let Some(val) = event.get("ses.file.url.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.url.port".into(),
                message,
                })?;
                event.set("ses.file.url.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_url_port_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.url.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file.url.referrer_category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file.url.referrer_category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_url_referrer_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file.url.rep_score_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file.url.rep_score_id") {
                if let Some(val) = event.get("ses.file.url.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file.url.rep_score_id".into(),
                message,
                })?;
                event.set("ses.file.url.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_url_rep_score_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file.url.rep_score_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file_result.accessed") && event.get_str("ses.file_result.accessed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file_result.accessed") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file_result.accessed", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file_result.accessed".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_accessed")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.accessed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file_result.attribute_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file_result.attribute_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_attribute_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file_result.attributes") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.attributes") {
                if let Some(val) = event.get("ses.file_result.attributes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.attributes".into(),
                message,
                })?;
                event.set("ses.file_result.attributes", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_attributes_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.attributes");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.confidentiality_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.confidentiality_id") {
                if let Some(val) = event.get("ses.file_result.confidentiality_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.confidentiality_id".into(),
                message,
                })?;
                event.set("ses.file_result.confidentiality_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_confidentiality_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.confidentiality_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.content_type.family_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.content_type.family_id") {
                if let Some(val) = event.get("ses.file_result.content_type.family_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.content_type.family_id".into(),
                message,
                })?;
                event.set("ses.file_result.content_type.family_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_content_type_family_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.content_type.family_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.content_type.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.content_type.type_id") {
                if let Some(val) = event.get("ses.file_result.content_type.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.content_type.type_id".into(),
                message,
                })?;
                event.set("ses.file_result.content_type.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_content_type_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.content_type.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file_result.created") && event.get_str("ses.file_result.created") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file_result.created") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file_result.created", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file_result.created".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_created")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.created");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.is_system") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.is_system") {
                if let Some(val) = event.get("ses.file_result.is_system") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.is_system".into(),
                message,
                })?;
                event.set("ses.file_result.is_system", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_is_system_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.is_system");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file_result.modified") && event.get_str("ses.file_result.modified") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file_result.modified") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file_result.modified", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file_result.modified".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_modified")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.modified");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.rep_discovered_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.rep_discovered_band") {
                if let Some(val) = event.get("ses.file_result.rep_discovered_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.rep_discovered_band".into(),
                message,
                })?;
                event.set("ses.file_result.rep_discovered_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_rep_discovered_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.rep_discovered_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file_result.rep_discovered_date") && event.get_str("ses.file_result.rep_discovered_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file_result.rep_discovered_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file_result.rep_discovered_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file_result.rep_discovered_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_rep_discovered_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.rep_discovered_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.rep_prevalence") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.rep_prevalence") {
                if let Some(val) = event.get("ses.file_result.rep_prevalence") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.rep_prevalence".into(),
                message,
                })?;
                event.set("ses.file_result.rep_prevalence", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_rep_prevalence_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.rep_prevalence");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.rep_prevalence_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.rep_prevalence_band") {
                if let Some(val) = event.get("ses.file_result.rep_prevalence_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.rep_prevalence_band".into(),
                message,
                })?;
                event.set("ses.file_result.rep_prevalence_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_rep_prevalence_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.rep_prevalence_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.rep_score") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.rep_score") {
                if let Some(val) = event.get("ses.file_result.rep_score") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.rep_score".into(),
                message,
                })?;
                event.set("ses.file_result.rep_score", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_rep_score_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.rep_score");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.rep_score_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.rep_score_band") {
                if let Some(val) = event.get("ses.file_result.rep_score_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.rep_score_band".into(),
                message,
                })?;
                event.set("ses.file_result.rep_score_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_rep_score_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.rep_score_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file_result.signature_created_date") && event.get_str("ses.file_result.signature_created_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.file_result.signature_created_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.file_result.signature_created_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.file_result.signature_created_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_created_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.signature_created_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.signature_level_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.signature_level_id") {
                if let Some(val) = event.get("ses.file_result.signature_level_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.signature_level_id".into(),
                message,
                })?;
                event.set("ses.file_result.signature_level_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_signature_level_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.signature_level_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.signature_value") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.signature_value") {
                if let Some(val) = event.get("ses.file_result.signature_value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.signature_value".into(),
                message,
                })?;
                event.set("ses.file_result.signature_value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_signature_value_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.signature_value");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file_result.signature_value_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file_result.signature_value_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_signature_value_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file_result.size") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.size") {
                if let Some(val) = event.get("ses.file_result.size") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.size".into(),
                message,
                })?;
                event.set("ses.file_result.size", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_size_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.size");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.size_compressed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.size_compressed") {
                if let Some(val) = event.get("ses.file_result.size_compressed") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.size_compressed".into(),
                message,
                })?;
                event.set("ses.file_result.size_compressed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_size_compressed_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.size_compressed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.file_result.src_ip") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.src_ip") {
                if let Some(val) = event.get("ses.file_result.src_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.src_ip".into(),
                message,
                })?;
                event.set("ses.file_result.src_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_src_ip_to_ip")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.src_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file_result.src_ip") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("ses.file_result.src_ip").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.file_result.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.type_id") {
                if let Some(val) = event.get("ses.file_result.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.type_id".into(),
                message,
                })?;
                event.set("ses.file_result.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file_result.url.category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file_result.url.category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_url_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file_result.url.port") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.url.port") {
                if let Some(val) = event.get("ses.file_result.url.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.url.port".into(),
                message,
                })?;
                event.set("ses.file_result.url.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_url_port_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.url.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.file_result.url.referrer_category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.file_result.url.referrer_category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_url_referrer_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.file_result.url.rep_score_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.file_result.url.rep_score_id") {
                if let Some(val) = event.get("ses.file_result.url.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.file_result.url.rep_score_id".into(),
                message,
                })?;
                event.set("ses.file_result.url.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_url_rep_score_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.file_result.url.rep_score_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.file.md5") && event.get_str("ses.file.md5") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file.md5").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.sha1") && event.get_str("ses.file.sha1") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file.sha1").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get("ses.file.sha2").is_some_and(|v| v.is_string()) && event.get_str("ses.file.sha2") != Some("") };
                if _cond {
                if let Some(v) = event.get("ses.file.sha2").cloned() {
                event.set("file.hash.sha256", v)?;
                }
                }
                let _cond = { event.has_value("ses.file.sha2") && event.get_str("ses.file.sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file.sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.parent_sha2") && event.get_str("ses.file.parent_sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file.parent_sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file_result.md5") && event.get_str("ses.file_result.md5") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file_result.md5").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file_result.parent_sha2") && event.get_str("ses.file_result.parent_sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file_result.parent_sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file_result.sha1") && event.get_str("ses.file_result.sha1") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file_result.sha1").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file_result.sha2") && event.get_str("ses.file_result.sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.file_result.sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file.src_name") && event.get_str("ses.file.src_name") != Some("") };
                if _cond {
                event.append_unique("related.hosts", json!(event.get("ses.file.src_name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.file_result.src_name") && event.get_str("ses.file_result.src_name") != Some("") };
                if _cond {
                event.append_unique("related.hosts", json!(event.get("ses.file_result.src_name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("ses.file.accessed");
                event.remove("ses.file.created");
                event.remove("ses.file.md5");
                event.remove("ses.file.mime_type");
                event.remove("ses.file.name");
                event.remove("ses.file.folder");
                event.remove("ses.file.path");
                event.remove("ses.file.sha1");
                event.remove("ses.file.sha2");
                event.remove("ses.file.signature_issuer");
                event.remove("ses.file.signature_serial_number");
                event.remove("ses.file.size");
                }
                // End nested pipeline: "pipeline_object_file"
            }

            let _cond = { event.has_value("ses.type_id") && ["5", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && event.has_value("ses.parent") };
            if _cond {
                // Begin nested pipeline: "pipeline_object_parent"
                let _cond = { event.has_value("ses.parent.cmd_line") };
                if _cond {
                event.append_unique("process.parent.command_line", json!(event.get("ses.parent.cmd_line").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.file.accessed") && event.get_str("ses.parent.file.accessed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.file.accessed") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.file.accessed", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.file.accessed".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_file_accessed")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.accessed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.file.attribute_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.file.attribute_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_attribute_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.file.attributes") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.attributes") {
                if let Some(val) = event.get("ses.parent.file.attributes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.attributes".into(),
                message,
                })?;
                event.set("ses.parent.file.attributes", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_attributes_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.attributes");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.confidentiality_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.confidentiality_id") {
                if let Some(val) = event.get("ses.parent.file.confidentiality_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.confidentiality_id".into(),
                message,
                })?;
                event.set("ses.parent.file.confidentiality_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_confidentiality_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.confidentiality_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.content_type.family_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.content_type.family_id") {
                if let Some(val) = event.get("ses.parent.file.content_type.family_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.content_type.family_id".into(),
                message,
                })?;
                event.set("ses.parent.file.content_type.family_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_content_type_family_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.content_type.family_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.content_type.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.content_type.type_id") {
                if let Some(val) = event.get("ses.parent.file.content_type.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.content_type.type_id".into(),
                message,
                })?;
                event.set("ses.parent.file.content_type.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_content_type_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.content_type.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.file.created") && event.get_str("ses.parent.file.created") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.file.created") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.file.created", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.file.created".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_file_created")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.created");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.is_system") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.is_system") {
                if let Some(val) = event.get("ses.parent.file.is_system") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.is_system".into(),
                message,
                })?;
                event.set("ses.parent.file.is_system", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_is_system_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.is_system");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.file.modified") && event.get_str("ses.parent.file.modified") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.file.modified") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.file.modified", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.file.modified".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_file_modified")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.modified");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.rep_discovered_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.rep_discovered_band") {
                if let Some(val) = event.get("ses.parent.file.rep_discovered_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.rep_discovered_band".into(),
                message,
                })?;
                event.set("ses.parent.file.rep_discovered_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_rep_discovered_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.rep_discovered_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.file.rep_discovered_date") && event.get_str("ses.parent.file.rep_discovered_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.file.rep_discovered_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.file.rep_discovered_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.file.rep_discovered_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_file_rep_discovered_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.rep_discovered_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.rep_prevalence") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.rep_prevalence") {
                if let Some(val) = event.get("ses.parent.file.rep_prevalence") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.rep_prevalence".into(),
                message,
                })?;
                event.set("ses.parent.file.rep_prevalence", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_rep_prevalence_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.rep_prevalence");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.rep_prevalence_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.rep_prevalence_band") {
                if let Some(val) = event.get("ses.parent.file.rep_prevalence_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.rep_prevalence_band".into(),
                message,
                })?;
                event.set("ses.parent.file.rep_prevalence_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_rep_prevalence_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.rep_prevalence_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.rep_score") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.rep_score") {
                if let Some(val) = event.get("ses.parent.file.rep_score") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.rep_score".into(),
                message,
                })?;
                event.set("ses.parent.file.rep_score", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_rep_score_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.rep_score");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.rep_score_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.rep_score_band") {
                if let Some(val) = event.get("ses.parent.file.rep_score_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.rep_score_band".into(),
                message,
                })?;
                event.set("ses.parent.file.rep_score_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_rep_score_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.rep_score_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.file.signature_created_date") && event.get_str("ses.parent.file.signature_created_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.file.signature_created_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.file.signature_created_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.file.signature_created_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_file_signature_created_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.signature_created_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.signature_level_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.signature_level_id") {
                if let Some(val) = event.get("ses.parent.file.signature_level_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.signature_level_id".into(),
                message,
                })?;
                event.set("ses.parent.file.signature_level_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_signature_level_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.signature_level_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.signature_value") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.signature_value") {
                if let Some(val) = event.get("ses.parent.file.signature_value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.signature_value".into(),
                message,
                })?;
                event.set("ses.parent.file.signature_value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_signature_value_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.signature_value");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.file.signature_value_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.file.signature_value_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_signature_value_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.file.size") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.size") {
                if let Some(val) = event.get("ses.parent.file.size") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.size".into(),
                message,
                })?;
                event.set("ses.parent.file.size", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_size_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.size");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.size_compressed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.size_compressed") {
                if let Some(val) = event.get("ses.parent.file.size_compressed") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.size_compressed".into(),
                message,
                })?;
                event.set("ses.parent.file.size_compressed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_size_compressed_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.size_compressed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.file.src_ip") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.src_ip") {
                if let Some(val) = event.get("ses.parent.file.src_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.src_ip".into(),
                message,
                })?;
                event.set("ses.parent.file.src_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_src_ip_to_ip")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.src_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.file.src_ip") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("ses.parent.file.src_ip").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.parent.file.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.type_id") {
                if let Some(val) = event.get("ses.parent.file.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.type_id".into(),
                message,
                })?;
                event.set("ses.parent.file.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.file.url.category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.file.url.category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_url_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.file.url.port") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.url.port") {
                if let Some(val) = event.get("ses.parent.file.url.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.url.port".into(),
                message,
                })?;
                event.set("ses.parent.file.url.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_url_port_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.url.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.file.url.referrer_category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.file.url.referrer_category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_url_referrer_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.file.url.rep_score_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.file.url.rep_score_id") {
                if let Some(val) = event.get("ses.parent.file.url.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.file.url.rep_score_id".into(),
                message,
                })?;
                event.set("ses.parent.file.url.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_file_url_rep_score_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.file.url.rep_score_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.integrity_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.integrity_id") {
                if let Some(val) = event.get("ses.parent.integrity_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.integrity_id".into(),
                message,
                })?;
                event.set("ses.parent.integrity_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_integrity_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.integrity_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.module.accessed") && event.get_str("ses.parent.module.accessed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.module.accessed") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.module.accessed", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.module.accessed".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_module_accessed")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.accessed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.module.attribute_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.module.attribute_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_attribute_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.module.attributes") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.attributes") {
                if let Some(val) = event.get("ses.parent.module.attributes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.attributes".into(),
                message,
                })?;
                event.set("ses.parent.module.attributes", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_attributes_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.attributes");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.confidentiality_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.confidentiality_id") {
                if let Some(val) = event.get("ses.parent.module.confidentiality_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.confidentiality_id".into(),
                message,
                })?;
                event.set("ses.parent.module.confidentiality_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_confidentiality_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.confidentiality_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.content_type.family_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.content_type.family_id") {
                if let Some(val) = event.get("ses.parent.module.content_type.family_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.content_type.family_id".into(),
                message,
                })?;
                event.set("ses.parent.module.content_type.family_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_content_type_family_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.content_type.family_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.content_type.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.content_type.type_id") {
                if let Some(val) = event.get("ses.parent.module.content_type.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.content_type.type_id".into(),
                message,
                })?;
                event.set("ses.parent.module.content_type.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_content_type_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.content_type.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.module.created") && event.get_str("ses.parent.module.created") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.module.created") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.module.created", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.module.created".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_module_created")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.created");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.is_system") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.is_system") {
                if let Some(val) = event.get("ses.parent.module.is_system") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.is_system".into(),
                message,
                })?;
                event.set("ses.parent.module.is_system", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_is_system_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.is_system");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.load_type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.load_type_id") {
                if let Some(val) = event.get("ses.parent.module.load_type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.load_type_id".into(),
                message,
                })?;
                event.set("ses.parent.module.load_type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_load_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.load_type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.module.modified") && event.get_str("ses.parent.module.modified") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.module.modified") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.module.modified", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.module.modified".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_module_modified")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.modified");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.rep_discovered_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.rep_discovered_band") {
                if let Some(val) = event.get("ses.parent.module.rep_discovered_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.rep_discovered_band".into(),
                message,
                })?;
                event.set("ses.parent.module.rep_discovered_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_rep_discovered_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.rep_discovered_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.module.rep_discovered_date") && event.get_str("ses.parent.module.rep_discovered_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.module.rep_discovered_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.module.rep_discovered_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.module.rep_discovered_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_module_rep_discovered_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.rep_discovered_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.rep_prevalence") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.rep_prevalence") {
                if let Some(val) = event.get("ses.parent.module.rep_prevalence") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.rep_prevalence".into(),
                message,
                })?;
                event.set("ses.parent.module.rep_prevalence", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_rep_prevalence_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.rep_prevalence");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.rep_prevalence_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.rep_prevalence_band") {
                if let Some(val) = event.get("ses.parent.module.rep_prevalence_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.rep_prevalence_band".into(),
                message,
                })?;
                event.set("ses.parent.module.rep_prevalence_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_rep_prevalence_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.rep_prevalence_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.rep_score") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.rep_score") {
                if let Some(val) = event.get("ses.parent.module.rep_score") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.rep_score".into(),
                message,
                })?;
                event.set("ses.parent.module.rep_score", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_rep_score_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.rep_score");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.rep_score_band") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.rep_score_band") {
                if let Some(val) = event.get("ses.parent.module.rep_score_band") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.rep_score_band".into(),
                message,
                })?;
                event.set("ses.parent.module.rep_score_band", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_rep_score_band_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.rep_score_band");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.module.signature_created_date") && event.get_str("ses.parent.module.signature_created_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.module.signature_created_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.module.signature_created_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.module.signature_created_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_module_signature_created_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.signature_created_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.signature_level_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.signature_level_id") {
                if let Some(val) = event.get("ses.parent.module.signature_level_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.signature_level_id".into(),
                message,
                })?;
                event.set("ses.parent.module.signature_level_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_signature_level_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.signature_level_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.signature_value") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.signature_value") {
                if let Some(val) = event.get("ses.parent.module.signature_value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.signature_value".into(),
                message,
                })?;
                event.set("ses.parent.module.signature_value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_signature_value_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.signature_value");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.module.signature_value_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.module.signature_value_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_signature_value_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.module.size") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.size") {
                if let Some(val) = event.get("ses.parent.module.size") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.size".into(),
                message,
                })?;
                event.set("ses.parent.module.size", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_size_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.size");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.size_compressed") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.size_compressed") {
                if let Some(val) = event.get("ses.parent.module.size_compressed") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.size_compressed".into(),
                message,
                })?;
                event.set("ses.parent.module.size_compressed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_size_compressed_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.size_compressed");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.module.src_ip") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.src_ip") {
                if let Some(val) = event.get("ses.parent.module.src_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.src_ip".into(),
                message,
                })?;
                event.set("ses.parent.module.src_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_src_ip_to_ip")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.src_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.module.src_ip") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("ses.parent.module.src_ip").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.parent.module.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.type_id") {
                if let Some(val) = event.get("ses.parent.module.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.type_id".into(),
                message,
                })?;
                event.set("ses.parent.module.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.module.url.category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.module.url.category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_url_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.module.url.port") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.url.port") {
                if let Some(val) = event.get("ses.parent.module.url.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.url.port".into(),
                message,
                })?;
                event.set("ses.parent.module.url.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_url_port_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.url.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.parent.module.url.referrer_category_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.parent.module.url.referrer_category_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_url_referrer_category_ids_to_string")?;
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
                let _cond = { event.get_str("ses.parent.module.url.rep_score_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.module.url.rep_score_id") {
                if let Some(val) = event.get("ses.parent.module.url.rep_score_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.module.url.rep_score_id".into(),
                message,
                })?;
                event.set("ses.parent.module.url.rep_score_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_module_url_rep_score_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.module.url.rep_score_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.pid") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.pid") {
                if let Some(val) = event.get("ses.parent.pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.pid".into(),
                message,
                })?;
                event.set("ses.parent.pid", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_pid_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.pid");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if let Some(v) = event.get("ses.parent.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
                }
                let _cond = { event.get_str("ses.parent.session.auth_protocol_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.auth_protocol_id") {
                if let Some(val) = event.get("ses.parent.session.auth_protocol_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.auth_protocol_id".into(),
                message,
                })?;
                event.set("ses.parent.session.auth_protocol_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_auth_protocol_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.auth_protocol_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.cleartext_credentials") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.cleartext_credentials") {
                if let Some(val) = event.get("ses.parent.session.cleartext_credentials") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.cleartext_credentials".into(),
                message,
                })?;
                event.set("ses.parent.session.cleartext_credentials", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_cleartext_credentials_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.cleartext_credentials");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.direction_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.direction_id") {
                if let Some(val) = event.get("ses.parent.session.direction_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.direction_id".into(),
                message,
                })?;
                event.set("ses.parent.session.direction_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_direction_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.direction_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.id") {
                if let Some(val) = event.get("ses.parent.session.id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.id".into(),
                message,
                })?;
                event.set("ses.parent.session.id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_id_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.is_admin") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.is_admin") {
                if let Some(val) = event.get("ses.parent.session.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.is_admin".into(),
                message,
                })?;
                event.set("ses.parent.session.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_is_admin_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.is_admin");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.logon_type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.logon_type_id") {
                if let Some(val) = event.get("ses.parent.session.logon_type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.logon_type_id".into(),
                message,
                })?;
                event.set("ses.parent.session.logon_type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_logon_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.logon_type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.port") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.port") {
                if let Some(val) = event.get("ses.parent.session.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.port".into(),
                message,
                })?;
                event.set("ses.parent.session.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_port_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.remote") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.remote") {
                if let Some(val) = event.get("ses.parent.session.remote") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.remote".into(),
                message,
                })?;
                event.set("ses.parent.session.remote", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_remote_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.remote");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.remote_ip") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.remote_ip") {
                if let Some(val) = event.get("ses.parent.session.remote_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.remote_ip".into(),
                message,
                })?;
                event.set("ses.parent.session.remote_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_remote_ip_to_ip")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.remote_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.session.remote_ip") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("ses.parent.session.remote_ip").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.parent.session.user.account_disabled") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.user.account_disabled") {
                if let Some(val) = event.get("ses.parent.session.user.account_disabled") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.user.account_disabled".into(),
                message,
                })?;
                event.set("ses.parent.session.user.account_disabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_user_account_disabled_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.user.account_disabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.user.is_admin") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.user.is_admin") {
                if let Some(val) = event.get("ses.parent.session.user.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.user.is_admin".into(),
                message,
                })?;
                event.set("ses.parent.session.user.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_user_is_admin_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.user.is_admin");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session.user.password_expires") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session.user.password_expires") {
                if let Some(val) = event.get("ses.parent.session.user.password_expires") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session.user.password_expires".into(),
                message,
                })?;
                event.set("ses.parent.session.user.password_expires", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_user_password_expires_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session.user.password_expires");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.session_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.session_id") {
                if let Some(val) = event.get("ses.parent.session_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.session_id".into(),
                message,
                })?;
                event.set("ses.parent.session_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_session_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.session_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.start_time") && event.get_str("ses.parent.start_time") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.parent.start_time") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.parent.start_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.parent.start_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_parent_start_time")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.start_time");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.start_time") };
                if _cond {
                event.append_unique("process.parent.start", json!(event.get("ses.parent.start_time").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.parent.tid") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.tid") {
                if let Some(val) = event.get("ses.parent.tid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.tid".into(),
                message,
                })?;
                event.set("ses.parent.tid", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_tid_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.tid");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.tid") };
                if _cond {
                event.append_unique("process.parent.thread.id", json!(event.get("ses.parent.tid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get("process.parent.thread.id").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "process.parent.thread.id", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_threat_id_to_long")?;
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
                let _cond = { event.has_value("ses.parent.uid") };
                if _cond {
                event.append_unique("process.parent.entity_id", json!(event.get("ses.parent.uid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.parent.user.account_disabled") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.user.account_disabled") {
                if let Some(val) = event.get("ses.parent.user.account_disabled") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.user.account_disabled".into(),
                message,
                })?;
                event.set("ses.parent.user.account_disabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_user_account_disabled_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.user.account_disabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.parent.user.is_admin") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.user.is_admin") {
                if let Some(val) = event.get("ses.parent.user.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.user.is_admin".into(),
                message,
                })?;
                event.set("ses.parent.user.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_user_is_admin_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.user.is_admin");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.user.name") };
                if _cond {
                event.append_unique("process.parent.user.name", json!(event.get("ses.parent.user.name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.parent.user.password_expires") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.parent.user.password_expires") {
                if let Some(val) = event.get("ses.parent.user.password_expires") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.parent.user.password_expires".into(),
                message,
                })?;
                event.set("ses.parent.user.password_expires", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_parent_user_password_expires_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.parent.user.password_expires");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.parent.user.uid") };
                if _cond {
                event.append_unique("process.parent.user.id", json!(event.get("ses.parent.user.uid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.file.md5") && event.get_str("ses.parent.file.md5") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.file.md5").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.file.sha1") && event.get_str("ses.parent.file.sha1") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.file.sha1").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.file.sha2") && event.get_str("ses.parent.file.sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.file.sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.file.parent_sha2") && event.get_str("ses.parent.file.parent_sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.file.parent_sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.module.md5") && event.get_str("ses.parent.module.md5") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.module.md5").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.module.sha1") && event.get_str("ses.parent.module.sha1") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.module.sha1").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.module.sha2") && event.get_str("ses.parent.module.sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.module.sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.module.parent_sha2") && event.get_str("ses.parent.module.parent_sha2") != Some("") };
                if _cond {
                event.append_unique("related.hash", json!(event.get("ses.parent.module.parent_sha2").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.session.user.uid") && event.get_str("ses.parent.session.user.uid") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.parent.session.user.uid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.session.user.name") && event.get_str("ses.parent.session.user.name") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.parent.session.user.name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.file.src_name") && event.get_str("ses.parent.file.src_name") != Some("") };
                if _cond {
                event.append_unique("related.hosts", json!(event.get("ses.parent.file.src_name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.module.src_name") && event.get_str("ses.parent.module.src_name") != Some("") };
                if _cond {
                event.append_unique("related.hosts", json!(event.get("ses.parent.module.src_name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.user.uid") && event.get_str("ses.parent.user.uid") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.parent.user.uid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.parent.user.name") && event.get_str("ses.parent.user.name") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.parent.user.name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("ses.parent.cmd_line");
                event.remove("ses.parent.pid");
                event.remove("ses.parent.start_time");
                event.remove("ses.parent.tid");
                event.remove("ses.parent.uid");
                event.remove("ses.parent.user.name");
                event.remove("ses.parent.user.uid");
                }
                // End nested pipeline: "pipeline_object_parent"
            }

            let _cond = { event.has_value("ses.type_id") && ["4"].contains(&event.get_str("ses.type_id").unwrap_or("")) && event.has_value("ses.policy") };
            if _cond {
                // Begin nested pipeline: "pipeline_object_policy"
                let _cond = { event.has_value("ses.policy.effective_date") && event.get_str("ses.policy.effective_date") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ses.policy.effective_date") {
                match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                Some(parsed) => event.set("ses.policy.effective_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "ses.policy.effective_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_policy_effective_date")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.policy.effective_date");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.policy.rule_category_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.policy.rule_category_id") {
                if let Some(val) = event.get("ses.policy.rule_category_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.policy.rule_category_id".into(),
                message,
                })?;
                event.set("ses.policy.rule_category_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_policy_rule_category_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.policy.rule_category_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.category_id") {
                if let Some(val) = event.get("_ingest._value.category_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.category_id".into(),
                message,
                })?;
                event.set("_ingest._value.category_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_policy_rules_category_id_to_string")?;
                event.remove("_ingest._value.category_id");
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
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                event.append_unique("rule.description", json!(event.get("_ingest._value.desc").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.dlp_type_id") {
                if let Some(val) = event.get("_ingest._value.dlp_type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.dlp_type_id".into(),
                message,
                })?;
                event.set("_ingest._value.dlp_type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_policy_rules_dlp_type_id_to_string")?;
                event.remove("_ingest._value.dlp_type_id");
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
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                event.append_unique("rule.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.num_violations") {
                if let Some(val) = event.get("_ingest._value.num_violations") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.num_violations".into(),
                message,
                })?;
                event.set("_ingest._value.num_violations", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_policy_rules_num_violations_to_long")?;
                event.remove("_ingest._value.num_violations");
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
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                event.append_unique("rule.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })?;
                }
                let _cond = { event.get("ses.policy.state_ids").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.state_ids", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_policy_state_ids_to_string")?;
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
                let _cond = { event.get_str("ses.policy.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.policy.type_id") {
                if let Some(val) = event.get("ses.policy.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.policy.type_id".into(),
                message,
                })?;
                event.set("ses.policy.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_policy_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.policy.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.policy.rules").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.policy.rules", |event| {
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("_ingest._value.desc");
                event.remove("_ingest._value.uid");
                event.remove("_ingest._value.name");
                }
                Ok(())
                })?;
                }
                // End nested pipeline: "pipeline_object_policy"
            }

            let _cond = { event.has_value("ses.type_id") && ["2", "3", "4", "5", "11", "12", "13", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && (event.has_value("ses.session") || event.has_value("ses.sessions")) };
            if _cond {
                // Begin nested pipeline: "pipeline_object_session"
                let _cond = { event.get_str("ses.session.auth_protocol_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.auth_protocol_id") {
                if let Some(val) = event.get("ses.session.auth_protocol_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.auth_protocol_id".into(),
                message,
                })?;
                event.set("ses.session.auth_protocol_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_auth_protocol_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.auth_protocol_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.session.auth_protocol_id") };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def obj = ctx.ses.session.auth_protocol_id;\nif (params.containsKey(obj.toString())) {\n  def type = params.get(obj.toString());\n  ctx.ses.session.auth_protocol_value = type\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def obj = ctx.ses.session.auth_protocol_id;\nif (params.containsKey(obj.toString())) {\n  def type = params.get(obj.toString());\n  ctx.ses.session.auth_protocol_value = type\n}"#), cached_params!("{\"0\":\"Unknown\",\"1\":\"NTLM\",\"2\":\"Kerberos\",\"3\":\"Digest\",\"4\":\"OpenID\",\"5\":\"SAML\",\"6\":\"OAUTH 2.0\",\"7\":\"PAP\",\"8\":\"CHAP\",\"9\":\"EAP\",\"10\":\"RADIUS\"}"))?;
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_auth_protocol_value")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.cleartext_credentials") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.cleartext_credentials") {
                if let Some(val) = event.get("ses.session.cleartext_credentials") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.cleartext_credentials".into(),
                message,
                })?;
                event.set("ses.session.cleartext_credentials", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_cleartext_credentials_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.cleartext_credentials");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.direction_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.direction_id") {
                if let Some(val) = event.get("ses.session.direction_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.direction_id".into(),
                message,
                })?;
                event.set("ses.session.direction_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_direction_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.direction_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.id") {
                if let Some(val) = event.get("ses.session.id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.id".into(),
                message,
                })?;
                event.set("ses.session.id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_id_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.is_admin") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.is_admin") {
                if let Some(val) = event.get("ses.session.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.is_admin".into(),
                message,
                })?;
                event.set("ses.session.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_is_admin_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.is_admin");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.logon_type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.logon_type_id") {
                if let Some(val) = event.get("ses.session.logon_type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.logon_type_id".into(),
                message,
                })?;
                event.set("ses.session.logon_type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_logon_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.logon_type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.port") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.port") {
                if let Some(val) = event.get("ses.session.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.port".into(),
                message,
                })?;
                event.set("ses.session.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_port_to_long")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.port");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.remote") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.remote") {
                if let Some(val) = event.get("ses.session.remote") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.remote".into(),
                message,
                })?;
                event.set("ses.session.remote", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_remote_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.remote");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.remote_ip") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.remote_ip") {
                if let Some(val) = event.get("ses.session.remote_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.remote_ip".into(),
                message,
                })?;
                event.set("ses.session.remote_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_remote_ip_to_ip")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.remote_ip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.user.account_disabled") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.user.account_disabled") {
                if let Some(val) = event.get("ses.session.user.account_disabled") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.user.account_disabled".into(),
                message,
                })?;
                event.set("ses.session.user.account_disabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_user_account_disabled_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.user.account_disabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.user.is_admin") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.user.is_admin") {
                if let Some(val) = event.get("ses.session.user.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.user.is_admin".into(),
                message,
                })?;
                event.set("ses.session.user.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_user_is_admin_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.user.is_admin");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get_str("ses.session.user.password_expires") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.session.user.password_expires") {
                if let Some(val) = event.get("ses.session.user.password_expires") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.session.user.password_expires".into(),
                message,
                })?;
                event.set("ses.session.user.password_expires", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_user_password_expires_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.session.user.password_expires");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.auth_protocol_id") {
                if let Some(val) = event.get("_ingest._value.auth_protocol_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.auth_protocol_id".into(),
                message,
                })?;
                event.set("_ingest._value.auth_protocol_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_auth_protocol_id_to_string")?;
                event.remove("_ingest._value.auth_protocol_id");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.cleartext_credentials") {
                if let Some(val) = event.get("_ingest._value.cleartext_credentials") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.cleartext_credentials".into(),
                message,
                })?;
                event.set("_ingest._value.cleartext_credentials", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_cleartext_credentials_to_boolean")?;
                event.remove("_ingest._value.cleartext_credentials");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.direction_id") {
                if let Some(val) = event.get("_ingest._value.direction_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.direction_id".into(),
                message,
                })?;
                event.set("_ingest._value.direction_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_direction_id_to_string")?;
                event.remove("_ingest._value.direction_id");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.id") {
                if let Some(val) = event.get("_ingest._value.id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.id".into(),
                message,
                })?;
                event.set("_ingest._value.id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_id_to_long")?;
                event.remove("_ingest._value.id");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.is_admin") {
                if let Some(val) = event.get("_ingest._value.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.is_admin".into(),
                message,
                })?;
                event.set("_ingest._value.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_is_admin_to_boolean")?;
                event.remove("_ingest._value.is_admin");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.logon_type_id") {
                if let Some(val) = event.get("_ingest._value.logon_type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.logon_type_id".into(),
                message,
                })?;
                event.set("_ingest._value.logon_type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_logon_type_id_to_string")?;
                event.remove("_ingest._value.logon_type_id");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.port") {
                if let Some(val) = event.get("_ingest._value.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.port".into(),
                message,
                })?;
                event.set("_ingest._value.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_port_to_long")?;
                event.remove("_ingest._value.port");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.remote") {
                if let Some(val) = event.get("_ingest._value.remote") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.remote".into(),
                message,
                })?;
                event.set("_ingest._value.remote", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_remote_to_boolean")?;
                event.remove("_ingest._value.remote");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.remote_ip") {
                if let Some(val) = event.get("_ingest._value.remote_ip") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.remote_ip".into(),
                message,
                })?;
                event.set("_ingest._value.remote_ip", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_remote_ip_to_ip")?;
                event.remove("_ingest._value.remote_ip");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.sessions").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.sessions") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("_ingest._value.remote_ip").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.sessions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.has_value("ses.session.remote_ip") };
                if _cond {
                event.append_unique("related.ip", json!(event.get("ses.session.remote_ip").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.session.user.uid") && event.get_str("ses.session.user.uid") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.session.user.uid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.session.user.name") && event.get_str("ses.session.user.name") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.session.user.name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.sessions").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.sessions") };
                if _cond {
                event.append_unique("related.user", json!(event.get("_ingest._value.user.uid").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.sessions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("ses.sessions").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                let _cond = { event.has_value("ses.sessions") };
                if _cond {
                event.append_unique("related.user", json!(event.get("_ingest._value.user.name").map_or_else(String::new, template_to_string)))?;
                }
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("ses.sessions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.user.account_disabled") {
                if let Some(val) = event.get("_ingest._value.user.account_disabled") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.user.account_disabled".into(),
                message,
                })?;
                event.set("_ingest._value.user.account_disabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_user_account_disabled_to_boolean")?;
                event.remove("_ingest._value.user.account_disabled");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.user.is_admin") {
                if let Some(val) = event.get("_ingest._value.user.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.user.is_admin".into(),
                message,
                })?;
                event.set("_ingest._value.user.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_user_is_admin_to_boolean")?;
                event.remove("_ingest._value.user.is_admin");
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
                let _cond = { event.get("ses.sessions").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "ses.sessions", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.user.password_expires") {
                if let Some(val) = event.get("_ingest._value.user.password_expires") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.user.password_expires".into(),
                message,
                })?;
                event.set("_ingest._value.user.password_expires", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sessions_user_password_expires_to_boolean")?;
                event.remove("_ingest._value.user.password_expires");
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
                // End nested pipeline: "pipeline_object_session"
            }

            let _cond = { event.has_value("ses.type_id") && ["2", "3", "4", "5", "11", "12", "13", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && event.has_value("ses.source") };
            if _cond {
                // Begin nested pipeline: "pipeline_object_source"
                let _cond = { event.get_str("ses.source.type_id") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.source.type_id") {
                if let Some(val) = event.get("ses.source.type_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "ses.source.type_id".into(),
                message,
                })?;
                event.set("ses.source.type_id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_type_id_to_string")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.source.type_id");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                // End nested pipeline: "pipeline_object_source"
            }

            let _cond = { event.has_value("ses.type_id") && ["2", "3", "4", "5", "11", "12", "13", "42"].contains(&event.get_str("ses.type_id").unwrap_or("")) && event.has_value("ses.user") };
            if _cond {
                // Begin nested pipeline: "pipeline_object_user"
                let _cond = { event.get_str("ses.user.account_disabled") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.user.account_disabled") {
                if let Some(val) = event.get("ses.user.account_disabled") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.user.account_disabled".into(),
                message,
                })?;
                event.set("ses.user.account_disabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_user_account_disabled_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.user.account_disabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("ses.user.domain") };
                if _cond {
                event.append_unique("user.domain", json!(event.get("ses.user.domain").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.get_str("ses.user.is_admin") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.user.is_admin") {
                if let Some(val) = event.get("ses.user.is_admin") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.user.is_admin".into(),
                message,
                })?;
                event.set("ses.user.is_admin", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_user_is_admin_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.user.is_admin");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if let Some(v) = event.get("ses.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
                }
                let _cond = { event.get_str("ses.user.password_expires") != Some("") };
                if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("ses.user.password_expires") {
                if let Some(val) = event.get("ses.user.password_expires") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "ses.user.password_expires".into(),
                message,
                })?;
                event.set("ses.user.password_expires", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_user_password_expires_to_boolean")?;
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("ses.user.password_expires");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if let Some(v) = event.get("ses.user.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
                }
                let _cond = { event.has_value("ses.user.uid") && event.get_str("ses.user.uid") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.user.uid").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("ses.user.name") && event.get_str("ses.user.name") != Some("") };
                if _cond {
                event.append_unique("related.user", json!(event.get("ses.user.name").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("ses.user.domain");
                event.remove("ses.user.name");
                event.remove("ses.user.uid");
                }
                // End nested pipeline: "pipeline_object_user"
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("ses.event.http_status");
                event.remove("ses.sender_ip");
                event.remove("ses.url.host");
                event.remove("ses.url.method");
                event.remove("ses.url.path");
                event.remove("ses.url.port");
                event.remove("ses.url.query");
                event.remove("ses.url.scheme");
                event.remove("ses.url.text");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
