// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `exemption` pipeline.
pub struct Exemption;

impl Transform for Exemption {
    fn name(&self) -> &str {
        "exemption"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { !event.has_value("message") && event.get("json.requester_message").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.requester_message").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }
            }

            let _cond = { !event.has_value("message") && event.get("json.message").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.message").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }
            }

            let _cond = { event.get("json.status").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.status").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.state", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.state") && event.get("json.state").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.state").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.state", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.internal_message") && event.get("json.internal_explanation").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.internal_explanation").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.internal_message", v)?;
            }
            }

            let _cond = { !event.has_value("event.reason") && event.get("json.denial_explanation").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.denial_explanation").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
            if event.has_value("json.device_information.identifier") {
                if let Some(val) = event.get("json.device_information.identifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.device_information.identifier".into(),
                            message,
                        })?;
                    event.set("host.id", converted)?;
                }
            }
            }

            let _cond = { !event.has_value("kolide.request.device.url") && event.get("json.device_information.link").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.device_information.link").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.device.url", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.device.url") && event.get("json.device_information.location").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.device_information.location").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.device.url", v)?;
            }
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
            if event.has_value("json.requester_information.identifier") {
                if let Some(val) = event.get("json.requester_information.identifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.requester_information.identifier".into(),
                            message,
                        })?;
                    event.set("user.id", converted)?;
                }
            }
            }

            let _cond = { !event.has_value("kolide.request.requester.url") && event.get("json.requester_information.link").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.requester_information.link").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.requester.url", v)?;
            }
            }

            let _cond = { !event.has_value("kolide.request.requester.url") && event.get("json.requester_information.location").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.requester_information.location").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.request.requester.url", v)?;
            }
            }

                if event.has_value("json.issues") {
                    event.rename("json.issues", "kolide.request.issues")?;
                }

            let _cond = { event.has_value("json.requested_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.requested_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.request.created_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.requested_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("kolide.request.created_at") && event.has_value("json.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.request.created_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.created_at".into(),
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
