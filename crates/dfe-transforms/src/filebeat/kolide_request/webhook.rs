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
            let _cond = { !event.has_value("kolide.request.type") && (event.get_str("json.event") == Some("requests.issue_exemption") || event.get_str("json.event") == Some("request.issue_exemption")) };
            if _cond {
            event.set("kolide.request.type", json!("exemption"))?;
            }

            let _cond = { !event.has_value("kolide.request.type") && (event.get_str("json.event") == Some("requests.registration") || event.get_str("json.event") == Some("request.registration")) };
            if _cond {
            event.set("kolide.request.type", json!("registration"))?;
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if event.has_value("json.event") {
                    event.rename("json.event", "event.action")?;
                }
            }

            let _cond = { !event.has_value("message") && event.get("json.data.message").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.message").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.state") && event.has_value("event.action") };
            if _cond {
            event.set("kolide.request.state", json!("pending"))?;
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
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
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
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
            }

            let _cond = { !event.has_value("host.name") && event.get("json.data.device_name").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.device_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }
            }

            let _cond = { !event.has_value("host.name") && event.get("json.data.device.name").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.device.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.device.url") && event.get("json.data.device.url").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.device.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.device.url", v)?;
            }
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
            if event.has_value("json.data.requester.id") {
                if let Some(val) = event.get("json.data.requester.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.requester.id".into(),
                            message,
                        })?;
                    event.set("user.id", converted)?;
                }
            }
            }

            let _cond = { !event.has_value("user.email") && event.get("json.data.person_email").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.person_email").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }
            }

            let _cond = { !event.has_value("user.email") && event.get("json.data.requester.email").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.requester.email").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.requester.url") && event.get("json.data.requester.url").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.data.requester.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.requester.url", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.issues") };
            if _cond {
                if event.has_value("json.data.issues") {
                    event.rename("json.data.issues", "kolide.request.issues")?;
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
