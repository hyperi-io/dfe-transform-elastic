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
            let _cond = { event.get_str("json.event") != Some("audit_log.recorded") && !event.has_value("event.action") };
            if _cond {
                if event.has_value("json.event") {
                    event.rename("json.event", "event.action")?;
                }
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if event.has_value("json.data.actor_name") {
                    event.rename("json.data.actor_name", "user.name")?;
                }
            }

            let _cond = { !event.has_value("user.email") };
            if _cond {
                if event.has_value("json.data.actor_email") {
                    event.rename("json.data.actor_email", "user.email")?;
                }
            }

            let _cond = { !event.has_value("kolide.audit.actor_type") };
            if _cond {
                if event.has_value("json.data.actor_type") {
                    event.rename("json.data.actor_type", "kolide.audit.actor_type")?;
                }
            }

            let _cond = { !event.has_value("source.ip") };
            if _cond {
                if event.has_value("json.data.ip_address") {
                    event.rename("json.data.ip_address", "source.ip")?;
                }
            }

            let _cond = { !event.has_value("message") };
            if _cond {
                if event.has_value("json.data.description") {
                    event.rename("json.data.description", "message")?;
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
