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
            if event.has_value("json.id") {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        })?;
                    event.set("event.id", converted)?;
                }
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
            if let Some(v) = event.get("json.event").cloned() {
                event.set("event.action", v)?;
            }
            }

                if event.has_value("json.data.device_name") {
                    event.rename("json.data.device_name", "host.name")?;
                }

            if event.has_value("json.data.device_id") {
                if let Some(val) = event.get("json.data.device_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.device_id".into(),
                            message,
                        })?;
                    event.set("host.id", converted)?;
                }
            }

                if event.has_value("json.data.registered_owner.email") {
                    event.rename("json.data.registered_owner.email", "user.email")?;
                }

                if event.has_value("json.data.registered_owner.name") {
                    event.rename("json.data.registered_owner.name", "user.name")?;
                }

            if event.has_value("json.data.registered_owner.id") {
                if let Some(val) = event.get("json.data.registered_owner.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.registered_owner.id".into(),
                            message,
                        })?;
                    event.set("user.id", converted)?;
                }
            }

                if event.has_value("json.data.device_status") {
                    event.rename("json.data.device_status", "kolide.device.device_status")?;
                }

                if event.has_value("json.data.device_url") {
                    event.rename("json.data.device_url", "kolide.device.device_url")?;
                }

            let _cond = { !event.has_value("event.original") && event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound { path: "event.id".into() });
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| TransformError::ParseError { path: "_id".into(), message })?))?;
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
