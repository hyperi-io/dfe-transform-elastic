// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_expenses` pipeline.
pub struct PipelineExpenses;

impl Transform for PipelineExpenses {
    fn name(&self) -> &str {
        "pipeline_expenses"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("axonius.application.amount") {
                if let Some(val) = event.get("axonius.application.amount") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "axonius.application.amount".into(),
                            message,
                        })?;
                    event.set("axonius.application.amount", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_amount_to_double")?;
                        event.remove("axonius.application.amount");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("axonius.application.related_user.email") };
            if _cond {
                event.append_unique("related.user", json!(event.get("axonius.application.related_user.email").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("axonius.application.related_user.remote_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("axonius.application.related_user.remote_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("axonius.application.related_user.remote_id").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("axonius.application.related_user.username").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("axonius.application.related_user.username") };
            if _cond {
                event.append_unique("related.user", json!(event.get("axonius.application.related_user.username").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.application.transaction_time") && event.get_str("axonius.application.transaction_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("axonius.application.transaction_time") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss 'GMT'", "yyyy-MM-dd", "EEE,dd MMM yyyy HH:mm:ss 'GMT'"], None, None) {
                        Some(parsed) => event.set("axonius.application.transaction_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "axonius.application.transaction_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_transaction_time")?;
                        event.remove("axonius.application.transaction_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("axonius.application.transaction_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.start", v)?;
            }

            if let Some(v) = event.get("axonius.application.user_email").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("user.email") && event.get("user.email").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("user.email") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("user.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "user.email".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("axonius.application.user_email") };
            if _cond {
                event.append_unique("related.user", json!(event.get("axonius.application.user_email").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}'\n{}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}'\n", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
