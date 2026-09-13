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
            let _cond = { !event.has_value("event.action") };
            if _cond {
                if event.has_value("json.event") {
                    event.rename("json.event", "event.action")?;
                }
            }

            let _cond = { event.get_str("event.action") == Some("auth_logs.success") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("event.action") == Some("auth_logs.failure") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("json.data.person_id") {
                if let Some(val) = event.get("json.data.person_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.data.person_id".into(),
                            message,
                        })?;
                    event.set("user.id", converted)?;
                }
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if event.has_value("json.data.device_name") {
                    event.rename("json.data.device_name", "host.name")?;
                }
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

            let _cond = { !event.has_value("user_agent.name") };
            if _cond {
                if event.has_value("json.data.browser_name") {
                    event.rename("json.data.browser_name", "user_agent.name")?;
                }
            }

            let _cond = { !event.has_value("user_agent.version") };
            if _cond {
                if event.has_value("json.data.browser_version") {
                    event.rename("json.data.browser_version", "user_agent.version")?;
                }
            }

            let _cond = { !event.has_value("kolide.auth.agent_version") };
            if _cond {
                if event.has_value("json.data.launcher_version") {
                    event.rename("json.data.launcher_version", "kolide.auth.agent_version")?;
                }
            }

            let _cond = { event.has_value("json.data.auth_log_url") && event.get_str("json.data.auth_log_url") != Some("") };
            if _cond {
                if let Some(input) = event.get_string("json.data.auth_log_url") {
                    // Grok pattern: ^%{NOTSPACE}/auth_logs/%{DATA:kolide.auth.session_id}$
                    if !cached_grok!("^%{NOTSPACE}/auth_logs/%{DATA:kolide.auth.session_id}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("kolide.auth.url") };
            if _cond {
                if event.has_value("json.data.auth_log_url") {
                    event.rename("json.data.auth_log_url", "kolide.auth.url")?;
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
