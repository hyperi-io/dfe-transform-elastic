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

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

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

            if event.has_value("json") {
                event.rename("json", "threatq")?;
            }

            let _cond = { event.has_value("threatq.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threatq.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("threatq.updated_at")
                    && event.get_str("threatq.updated_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.updated_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.updated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.updated_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_updated_at")?;
                    event.remove("threatq.updated_at");
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
                .get("threatq.updated_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("threatq.created_at")
                    && event.get_str("threatq.created_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.created_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_at")?;
                    event.remove("threatq.created_at");
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
                event.has_value("threatq.expired_at")
                    && event.get_str("threatq.expired_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.expired_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.expired_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.expired_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_expired_at")?;
                    event.remove("threatq.expired_at");
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
                event.has_value("threatq.expires_at")
                    && event.get_str("threatq.expires_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.expires_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.expires_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.expires_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_expires_at")?;
                    event.remove("threatq.expires_at");
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
                event.has_value("threatq.expires_calculated_at")
                    && event.get_str("threatq.expires_calculated_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.expires_calculated_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.expires_calculated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.expires_calculated_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
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
                        "date_expires_calculated_at",
                    )?;
                    event.remove("threatq.expires_calculated_at");
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
                event.has_value("threatq.published_at")
                    && event.get_str("threatq.published_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.published_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.published_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.published_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_published_at")?;
                    event.remove("threatq.published_at");
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
                event.has_value("threatq.touched_at")
                    && event.get_str("threatq.touched_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threatq.touched_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("threatq.touched_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threatq.touched_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_touched_at")?;
                    event.remove("threatq.touched_at");
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
                if event.has_value("threatq.id") {
                    if let Some(val) = event.get("threatq.id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.id".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_id_to_long")?;
                event.remove("threatq.id");
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
                if event.has_value("threatq.indicator_id") {
                    if let Some(val) = event.get("threatq.indicator_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.indicator_id".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.indicator_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_indicator_id_to_long",
                )?;
                event.remove("threatq.indicator_id");
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

            if event.has_value("threatq.status.id") {
                if let Some(val) = event.get("threatq.status.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "threatq.status.id".into(),
                            message,
                        }
                    })?;
                    event.set("threatq.status.id", converted)?;
                }
            }

            if event.has_value("threatq.status_id") {
                if let Some(val) = event.get("threatq.status_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "threatq.status_id".into(),
                            message,
                        }
                    })?;
                    event.set("threatq.status_id", converted)?;
                }
            }

            if event.has_value("threatq.type.id") {
                if let Some(val) = event.get("threatq.type.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "threatq.type.id".into(),
                            message,
                        }
                    })?;
                    event.set("threatq.type.id", converted)?;
                }
            }

            if event.has_value("threatq.type_id") {
                if let Some(val) = event.get("threatq.type_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "threatq.type_id".into(),
                            message,
                        }
                    })?;
                    event.set("threatq.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("threatq.score") {
                    if let Some(val) = event.get("threatq.score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.score".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_to_long")?;
                event.remove("threatq.score");
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
                .get("threatq.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.description", v)?;
            }

            let _cond = { event.has_value("threatq.score") };
            if _cond {
                // Painless script
                // Source: def value = ctx.threatq.score; if (ctx.containsKey('threat') == false || ctx.threat == null) {\n  ctx.threat = new HashMap();\n}\nif (ctx.threat.containsKey('indicator') == false || ctx.threat.indicator == null) {\n  ctx.threat.indicator = new HashMap();\n} if (value >= 0 && value <= 4) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} if (value >= 5 && value <=7) {\n  ctx.threat.indicator.confidence = \"Medium\";\n  return;\n} if (value >= 8 && value <= 10) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def value = ctx.threatq.score; if (ctx.containsKey('threat') == false || ctx.threat == null) {\n  ctx.threat = new HashMap();\n}\nif (ctx.threat.containsKey('indicator') == false || ctx.threat.indicator == null) {\n  ctx.threat.indicator = new HashMap();\n} if (value >= 0 && value <= 4) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} if (value >= 5 && value <=7) {\n  ctx.threat.indicator.confidence = \"Medium\";\n  return;\n} if (value >= 8 && value <= 10) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n"#
                    ),
                )?;
            }

            if event.has_value("threatq.value") {
                event.rename("threatq.value", "threatq.indicator_value")?;
            }

            let _cond = { event.has_value("threatq.expires_at") };
            if _cond {
                if let Some(v) = event.get("threatq.expires_at").cloned() {
                    event.set("threatq.ioc_expired_at", v)?;
                }
            }

            let _cond = { event.has_value("threatq.expired_at") };
            if _cond {
                if let Some(v) = event.get("threatq.expired_at").cloned() {
                    event.set("threatq.ioc_expired_at", v)?;
                }
            }

            let _cond = {
                event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.threatq.ioc_expired_at == null) {\n  ZonedDateTime ioc_expired_at;\n  if (ctx.threatq.status == 'Expired') {\n    ioc_expired_at = ZonedDateTime.parse(ctx['@timestamp']);\n  }\n  else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    ZonedDateTime updated_at = ZonedDateTime.parse(ctx.threatq.updated_at, DateTimeFormatter.ISO_ZONED_DATE_TIME);\n    if (dur instanceof String){\n      char time_unit;\n      String time_value;\n      if (dur.length() != 0){\n        time_unit = dur.charAt(dur.length() - 1);\n        time_value = dur.substring(0, dur.length() - 1);\n      }\n      if (time_unit == (char)'d') {\n        ioc_expired_at = updated_at.plusDays(Long.parseLong(time_value));\n      } else if (time_unit == (char)'h') {\n        ioc_expired_at = updated_at.plusHours(Long.parseLong(time_value));\n      } else if (time_unit == (char)'m') {\n        ioc_expired_at = updated_at.plusMinutes(Long.parseLong(time_value));\n      } else {\n        if (ctx.error == null) {\n          ctx.error = new HashMap();\n        }\n        if (ctx.error.message == null) {\n          ctx.error.message = new ArrayList();\n        }\n        ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n        ioc_expired_at = updated_at.plusDays(90L);\n      }\n    }\n    else {\n      ioc_expired_at = updated_at.plusDays(90L);\n    }\n  }\n  ctx.threatq.ioc_expired_at = ioc_expired_at;\n}\n\n// Set threatq.ioc_expiration_reason\nif (ctx.threatq.expires_at != null && ctx.threatq.expired_at == null) {\n  ctx.threatq.ioc_expiration_reason = params.expires_at;\n} else if (ctx.threatq.expired_at != null) {\n  ctx.threatq.ioc_expiration_reason = params.expired_at;\n} else if (ctx.threatq.status == 'Expired') {\n  ctx.threatq.ioc_expiration_reason = params.expired_status;\n} else {\n  ctx.threatq.ioc_expiration_reason = params.expired_default;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.threatq.ioc_expired_at == null) {\n  ZonedDateTime ioc_expired_at;\n  if (ctx.threatq.status == 'Expired') {\n    ioc_expired_at = ZonedDateTime.parse(ctx['@timestamp']);\n  }\n  else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    ZonedDateTime updated_at = ZonedDateTime.parse(ctx.threatq.updated_at, DateTimeFormatter.ISO_ZONED_DATE_TIME);\n    if (dur instanceof String){\n      char time_unit;\n      String time_value;\n      if (dur.length() != 0){\n        time_unit = dur.charAt(dur.length() - 1);\n        time_value = dur.substring(0, dur.length() - 1);\n      }\n      if (time_unit == (char)'d') {\n        ioc_expired_at = updated_at.plusDays(Long.parseLong(time_value));\n      } else if (time_unit == (char)'h') {\n        ioc_expired_at = updated_at.plusHours(Long.parseLong(time_value));\n      } else if (time_unit == (char)'m') {\n        ioc_expired_at = updated_at.plusMinutes(Long.parseLong(time_value));\n      } else {\n        if (ctx.error == null) {\n          ctx.error = new HashMap();\n        }\n        if (ctx.error.message == null) {\n          ctx.error.message = new ArrayList();\n        }\n        ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n        ioc_expired_at = updated_at.plusDays(90L);\n      }\n    }\n    else {\n      ioc_expired_at = updated_at.plusDays(90L);\n    }\n  }\n  ctx.threatq.ioc_expired_at = ioc_expired_at;\n}\n\n// Set threatq.ioc_expiration_reason\nif (ctx.threatq.expires_at != null && ctx.threatq.expired_at == null) {\n  ctx.threatq.ioc_expiration_reason = params.expires_at;\n} else if (ctx.threatq.expired_at != null) {\n  ctx.threatq.ioc_expiration_reason = params.expired_at;\n} else if (ctx.threatq.status == 'Expired') {\n  ctx.threatq.ioc_expiration_reason = params.expired_status;\n} else {\n  ctx.threatq.ioc_expiration_reason = params.expired_default;\n}\n"#
                        ),
                        cached_params!(
                            "{\"expires_at\":\"Expiration set by ThreatQ from expires_at field\",\"expired_at\":\"Expiration set by ThreatQ from expired_at field\",\"expired_status\":\"Expiration set by Elastic when threatq.status is 'Expired'\",\"expired_default\":\"Expiration set by Elastic from the integration's parameter `IOC Expiration Duration`\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script-expired_ioc")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("threatq.type.name") == Some("Email Address") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.email.address", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("Email Address") };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("FQDN") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.url.domain", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("FQDN") };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("IP Address") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.ip", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("IP Address") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("IP Address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("IPv6 Address") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.ip", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("IPv6 Address") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("IPv6 Address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("MD5") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.md5", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("MD5") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("MD5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-1") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha1", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-1") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-256") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha256", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-256") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-512") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha512", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-512") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("SHA-512") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("Filename") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.name", v)?;
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("Filename") };
            if _cond {
                event.set("threat.indicator.file.type", json!("file"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("Filename") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("URL") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(
                        event,
                        "threatq.indicator_value",
                        "threat.indicator.url",
                        true,
                        true,
                    )? && event
                        .get_str("threatq.indicator_value")
                        .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "threatq.indicator_value".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    let v = json!(
                        event
                            .get("threatq.indicator_value")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("threat.indicator.url.full", v)?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("threatq.type.name") == Some("URL") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("threatq.type.name") == Some("x509 Serial") };
            if _cond {
                if let Some(v) = event
                    .get("threatq.indicator_value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.x509.serial_number", v)?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.indicator_type_id") {
                            if let Some(val) = event.get("_ingest._value.indicator_type_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.indicator_type_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.indicator_type_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.indicator_status_id") {
                            if let Some(val) = event.get("_ingest._value.indicator_status_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.indicator_status_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.indicator_status_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.reference_id") {
                            if let Some(val) = event.get("_ingest._value.reference_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.reference_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.reference_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.creator_source_id") {
                            if let Some(val) = event.get("_ingest._value.creator_source_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.creator_source_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.creator_source_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.id") {
                            if let Some(val) = event.get("_ingest._value.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.indicator_id") {
                            if let Some(val) = event.get("_ingest._value.indicator_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.indicator_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.indicator_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.source_id") {
                            if let Some(val) = event.get("_ingest._value.source_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.source_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.source_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        if event.has_value("_ingest._value.tlp_id") {
                            if let Some(val) = event.get("_ingest._value.tlp_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.tlp_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.tlp_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(val) = event.get("_ingest._value.source_score") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.source_score".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.source_score", converted)?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_score_to_long",
                            )?;
                            event.remove("_ingest._value.source_score");
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
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("threatq.sources").cloned();
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
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.created_at")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["yyyy-MM-dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.created_at", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.created_at".into(),
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
                                        "date_sources_created_at",
                                    )?;
                                    event.remove("_ingest._value.created_at");
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
                                "threatq.sources",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("threatq.sources").cloned();
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
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.published_at")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["yyyy-MM-dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.published_at", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.published_at".into(),
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
                                        "date_sources_published_at",
                                    )?;
                                    event.remove("_ingest._value.published_at");
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
                                "threatq.sources",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("threatq.sources").cloned();
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
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.updated_at")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["yyyy-MM-dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.updated_at", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.updated_at".into(),
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
                                        "date_sources_updated_at",
                                    )?;
                                    event.remove("_ingest._value.updated_at");
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
                                "threatq.sources",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = {
                event.get("threatq.sources").is_some_and(|v| v.is_array()) && event.get("threatq.sources").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: def ecsTlps = ['WHITE', 'GREEN', 'AMBER', 'RED', 'CLEAR', 'AMBER+STRICT'];\ndef providers = new ArrayList();\ndef tlps = new ArrayList();\nfor (source in ctx.threatq.sources) {\n  if (source == null) {\n    return;\n  }\n  if (source.containsKey(\"name\") && source[\"name\"] != null) {\n    providers.add(source[\"name\"]);\n  }\n  if (source.containsKey(\"tlp_name\") && source[\"tlp_name\"] != null && ecsTlps.contains(source[\"tlp_name\"])) {\n    tlps.add(source[\"tlp_name\"]);\n  }\n}\nif (tlps.size() > 0) {\n  if (ctx.threat.indicator.marking == null) {\n    ctx.threat.indicator.marking = new HashMap();\n  }\n  ctx.threat.indicator.marking.tlp = tlps;\n}\nif (providers.size() > 0) {\n  if (ctx.threat.indicator.provider == null) {\n    ctx.threat.indicator.provider = new HashMap();\n  }\n  ctx.threat.indicator.provider = providers;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def ecsTlps = ['WHITE', 'GREEN', 'AMBER', 'RED', 'CLEAR', 'AMBER+STRICT'];\ndef providers = new ArrayList();\ndef tlps = new ArrayList();\nfor (source in ctx.threatq.sources) {\n  if (source == null) {\n    return;\n  }\n  if (source.containsKey(\"name\") && source[\"name\"] != null) {\n    providers.add(source[\"name\"]);\n  }\n  if (source.containsKey(\"tlp_name\") && source[\"tlp_name\"] != null && ecsTlps.contains(source[\"tlp_name\"])) {\n    tlps.add(source[\"tlp_name\"]);\n  }\n}\nif (tlps.size() > 0) {\n  if (ctx.threat.indicator.marking == null) {\n    ctx.threat.indicator.marking = new HashMap();\n  }\n  ctx.threat.indicator.marking.tlp = tlps;\n}\nif (providers.size() > 0) {\n  if (ctx.threat.indicator.provider == null) {\n    ctx.threat.indicator.provider = new HashMap();\n  }\n  ctx.threat.indicator.provider = providers;\n}"#
                    ),
                )?;
            }

            if event.has_value("threatq.attributes") {
                foreach_array(event, "threatq.attributes", |event| {
                    map_strings(
                        event,
                        "_ingest._value.name",
                        "_ingest._value.name",
                        str::to_lowercase,
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("threatq.attributes") {
                foreach_array(event, "threatq.attributes", |event| {
                    gsub_field(
                        event,
                        "_ingest._value.name",
                        "_ingest._value.name",
                        cached_regex!(" "),
                        "_",
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("threatq.attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("threatq.attributes") {
                    foreach_array(event, "threatq.attributes", |event| {
                        if event.has_value("_ingest._value.attribute_id") {
                            if let Some(val) = event.get("_ingest._value.attribute_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.attribute_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.attribute_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("threatq.attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("threatq.attributes") {
                    foreach_array(event, "threatq.attributes", |event| {
                        if event.has_value("_ingest._value.id") {
                            if let Some(val) = event.get("_ingest._value.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("threatq.attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("threatq.attributes") {
                    foreach_array(event, "threatq.attributes", |event| {
                        if event.has_value("_ingest._value.indicator_id") {
                            if let Some(val) = event.get("_ingest._value.indicator_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.indicator_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.indicator_id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("threatq.attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("threatq.attributes") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("threatq.attributes").cloned();
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
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.created_at")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["yyyy-MM-dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.created_at", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.created_at".into(),
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
                                        "date_attributes_created_at",
                                    )?;
                                    event.remove("_ingest._value.created_at");
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
                                "threatq.attributes",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("threatq.attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("threatq.attributes") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("threatq.attributes").cloned();
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
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.updated_at")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["yyyy-MM-dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.updated_at", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.updated_at".into(),
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
                                        "date_attributes_updated_at",
                                    )?;
                                    event.remove("_ingest._value.updated_at");
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
                                "threatq.attributes",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("threatq.attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("threatq.attributes") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("threatq.attributes").cloned();
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
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.touched_at")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["yyyy-MM-dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.touched_at", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.touched_at".into(),
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
                                        "date_attributes_touched_at",
                                    )?;
                                    event.remove("_ingest._value.touched_at");
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
                                "threatq.attributes",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("threatq.related_adversary_count") {
                    if let Some(val) = event.get("threatq.related_adversary_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_adversary_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_adversary_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_adversary_count_to_long",
                )?;
                event.remove("threatq.related_adversary_count");
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
                if event.has_value("threatq.related_asset_count") {
                    if let Some(val) = event.get("threatq.related_asset_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_asset_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_asset_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_asset_count_to_long",
                )?;
                event.remove("threatq.related_asset_count");
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
                if event.has_value("threatq.related_attachment_count") {
                    if let Some(val) = event.get("threatq.related_attachment_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_attachment_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_attachment_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_attachment_count_to_long",
                )?;
                event.remove("threatq.related_attachment_count");
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
                if event.has_value("threatq.related_attack_pattern_count") {
                    if let Some(val) = event.get("threatq.related_attack_pattern_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_attack_pattern_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_attack_pattern_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_attack_pattern_count_to_long",
                )?;
                event.remove("threatq.related_attack_pattern_count");
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
                if event.has_value("threatq.related_campaign_count") {
                    if let Some(val) = event.get("threatq.related_campaign_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_campaign_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_campaign_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_campaign_count_to_long",
                )?;
                event.remove("threatq.related_campaign_count");
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
                if event.has_value("threatq.related_course_of_action_count") {
                    if let Some(val) = event.get("threatq.related_course_of_action_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_course_of_action_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_course_of_action_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_course_of_action_count_to_long",
                )?;
                event.remove("threatq.related_course_of_action_count");
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
                if event.has_value("threatq.related_event_count") {
                    if let Some(val) = event.get("threatq.related_event_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_event_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_event_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_event_count_to_long",
                )?;
                event.remove("threatq.related_event_count");
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
                if event.has_value("threatq.related_exploit_target_count") {
                    if let Some(val) = event.get("threatq.related_exploit_target_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_exploit_target_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_exploit_target_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_exploit_target_count_to_long",
                )?;
                event.remove("threatq.related_exploit_target_count");
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
                if event.has_value("threatq.related_identity_count") {
                    if let Some(val) = event.get("threatq.related_identity_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_identity_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_identity_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_identity_count_to_long",
                )?;
                event.remove("threatq.related_identity_count");
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
                if event.has_value("threatq.related_incident_count") {
                    if let Some(val) = event.get("threatq.related_incident_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_incident_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_incident_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_incident_count_to_long",
                )?;
                event.remove("threatq.related_incident_count");
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
                if event.has_value("threatq.related_indicator_count") {
                    if let Some(val) = event.get("threatq.related_indicator_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_indicator_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_indicator_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_indicator_count_to_long",
                )?;
                event.remove("threatq.related_indicator_count");
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
                if event.has_value("threatq.related_infrastructure_count") {
                    if let Some(val) = event.get("threatq.related_infrastructure_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_infrastructure_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_infrastructure_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_infrastructure_count_to_long",
                )?;
                event.remove("threatq.related_infrastructure_count");
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
                if event.has_value("threatq.related_intrusion_set_count") {
                    if let Some(val) = event.get("threatq.related_intrusion_set_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_intrusion_set_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_intrusion_set_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_intrusion_set_count_to_long",
                )?;
                event.remove("threatq.related_intrusion_set_count");
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
                if event.has_value("threatq.related_investigation_count") {
                    if let Some(val) = event.get("threatq.related_investigation_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_investigation_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_investigation_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_investigation_count_to_long",
                )?;
                event.remove("threatq.related_investigation_count");
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
                if event.has_value("threatq.related_malware_count") {
                    if let Some(val) = event.get("threatq.related_malware_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_malware_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_malware_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_malware_count_to_long",
                )?;
                event.remove("threatq.related_malware_count");
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
                if event.has_value("threatq.related_note_count") {
                    if let Some(val) = event.get("threatq.related_note_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_note_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_note_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_note_count_to_long",
                )?;
                event.remove("threatq.related_note_count");
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
                if event.has_value("threatq.related_report_count") {
                    if let Some(val) = event.get("threatq.related_report_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_report_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_report_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_report_count_to_long",
                )?;
                event.remove("threatq.related_report_count");
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
                if event.has_value("threatq.related_signature_count") {
                    if let Some(val) = event.get("threatq.related_signature_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_signature_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_signature_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_signature_count_to_long",
                )?;
                event.remove("threatq.related_signature_count");
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
                if event.has_value("threatq.related_task_count") {
                    if let Some(val) = event.get("threatq.related_task_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_task_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_task_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_task_count_to_long",
                )?;
                event.remove("threatq.related_task_count");
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
                if event.has_value("threatq.related_tool_count") {
                    if let Some(val) = event.get("threatq.related_tool_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_tool_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_tool_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_tool_count_to_long",
                )?;
                event.remove("threatq.related_tool_count");
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
                if event.has_value("threatq.related_ttp_count") {
                    if let Some(val) = event.get("threatq.related_ttp_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_ttp_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_ttp_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_ttp_count_to_long",
                )?;
                event.remove("threatq.related_ttp_count");
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
                if event.has_value("threatq.related_vulnerability_count") {
                    if let Some(val) = event.get("threatq.related_vulnerability_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threatq.related_vulnerability_count".into(),
                                message,
                            }
                        })?;
                        event.set("threatq.related_vulnerability_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_related_vulnerability_count_to_long",
                )?;
                event.remove("threatq.related_vulnerability_count");
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

            if event.has_value("threatq.adversaries") {
                foreach_array(event, "threatq.adversaries", |event| {
                    if event.has_value("_ingest._value.id") {
                        if let Some(val) = event.get("_ingest._value.id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("threatq.adversaries") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("threatq.adversaries").cloned();
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
                            event.append(
                                "threatq.adversaries",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
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
                            "threatq.adversaries",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("threat") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
            }

            let _cond = { event.get("threatq.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("threatq.sources") {
                    foreach_array(event, "threatq.sources", |event| {
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
                            event.remove("_ingest._value.provider");
                            event.remove("_ingest._value.tlp_name");
                        }
                        Ok(())
                    })?;
                }
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
                event.remove("threatq.description");
                event.remove("threatq.score");
                event.remove("threatq.updated_at");
            }

            event.remove("_conf");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
