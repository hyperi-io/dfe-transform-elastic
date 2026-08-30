// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `application` pipeline.
pub struct Application;

impl Transform for Application {
    fn name(&self) -> &str {
        "application"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.httpVersion") {
                    event.rename("json.httpVersion", "http.version")?;
                }

                if event.has_value("json.status") {
                    event.rename("json.status", "http.response.status_code")?;
                }

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

                if event.has_value("json.contentLength") {
                    event.rename("json.contentLength", "http.response.body.bytes")?;
                }

            if event.has_value("http.response.body.bytes") {
                if let Some(val) = event.get("http.response.body.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.response.body.bytes".into(),
                            message,
                        })?;
                    event.set("http.response.body.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("http.request.method") };
            if _cond {
                event.append_unique("event.category", json!("web"))?;
            }

            let _cond = { event.has_value("http.request.method") };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

            let _cond = { event.has_value("http.response.status_code") && event.get_i64("http.response.status_code").is_some_and(|n| n < 400) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("http.response.status_code") && event.get_i64("http.response.status_code").is_some_and(|n| n >= 400) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.has_value("json") && event.get("json").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                if event.has_value("json") {
                    event.rename("json", "backstage.log")?;
                }
            }

            let _cond = { event.has_value("json") && event.get("json").is_some_and(|v| match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                event.remove("json");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Pipeline '{}' failed at processor '{}' {}failed with message '{}'", event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
