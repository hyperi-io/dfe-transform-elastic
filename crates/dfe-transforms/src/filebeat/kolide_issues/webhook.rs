// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `webhook` pipeline.
pub struct Webhook;

impl Transform for Webhook {
    fn name(&self) -> &str {
        "webhook"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.event") {
                    event.rename("json.event", "event.action")?;
                }

                if event.has_value("json.data.device.name") {
                    event.rename("json.data.device.name", "host.name")?;
                }

            if event.has_value("json.data.device.id") {
                if let Some(val) = event.get("json.data.device.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.device.id".into(),
                            message,
                        })?;
                    event.set("host.id", converted)?;
                }
            }

                if event.has_value("json.data.check.name") {
                    event.rename("json.data.check.name", "rule.name")?;
                }

            if event.has_value("json.data.check.id") {
                if let Some(val) = event.get("json.data.check.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.check.id".into(),
                            message,
                        })?;
                    event.set("rule.id", converted)?;
                }
            }

            let _cond = { event.get("json.data.title").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("json.data.title") {
                    event.rename("json.data.title", "kolide.issues.title")?;
                }
            }

            if event.has_value("json.data.issue_id") {
                if let Some(val) = event.get("json.data.issue_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.issue_id".into(),
                            message,
                        })?;
                    event.set("kolide.issues.id", converted)?;
                }
            }

            if event.has_value("json.data.check_id") {
                if let Some(val) = event.get("json.data.check_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.check_id".into(),
                            message,
                        })?;
                    event.set("kolide.issues.check.id", converted)?;
                }
            }

                if event.has_value("json.data.check.tags") {
                    event.rename("json.data.check.tags", "kolide.issues.check.tags")?;
                }

            let _cond = { event.has_value("json.timestamp") && event.get_str("event.action") == Some("issues.resolved") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.resolved_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.timestamp") && event.get_str("event.action") == Some("issues.new") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.detected_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
