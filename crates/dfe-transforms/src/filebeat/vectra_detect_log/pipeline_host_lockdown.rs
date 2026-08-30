// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_host_lockdown` pipeline.
pub struct PipelineHostLockdown;

impl Transform for PipelineHostLockdown {
    fn name(&self) -> &str {
        "pipeline_host_lockdown"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

                if event.has_value("json.action") {
                    event.rename("json.action", "vectra_detect.log.action")?;
                }

            if let Some(v) = event.get("vectra_detect.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            let _cond = { event.get_str("json.success") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.success") {
                if let Some(val) = event.get("json.success") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.success".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.success", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_success_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_bool("vectra_detect.log.success") == Some(true) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_bool("vectra_detect.log.success") == Some(false) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

                if event.has_value("json.host_name") {
                    event.rename("json.host_name", "vectra_detect.log.host.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.host.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.hostname", v)?;
            }

                if event.has_value("json.user") {
                    event.rename("json.user", "vectra_detect.log.user.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

                if event.has_value("json.category") {
                    event.rename("json.category", "vectra_detect.log.category")?;
                }

            let _cond = { event.get_str("json.will_retry") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.will_retry") {
                if let Some(val) = event.get("json.will_retry") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.will_retry".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.will_retry", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_will_retry_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.href") {
                    event.rename("json.href", "vectra_detect.log.href")?;
                }

            if let Some(v) = event.get("vectra_detect.log.href").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            let _cond = { event.has_value("event.reference") };
            if _cond {
                uri_parts(event, "event.reference", "url", true, false)?;
            }

            let _cond = { event.get_str("json.host_id") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.host_id") {
                if let Some(val) = event.get("json.host_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.host_id".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.host.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_host_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.host.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.id", v)?;
            }

                if event.has_value("json.edr_type") {
                    event.rename("json.edr_type", "vectra_detect.log.edr_type")?;
                }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
