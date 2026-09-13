// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_service` pipeline.
pub struct PipelineService;

impl Transform for PipelineService {
    fn name(&self) -> &str {
        "pipeline_service"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("cef.extensions.InfobloxLogName") {
                    event.rename("cef.extensions.InfobloxLogName", "infoblox_threat_defense.event.infoblox.log_name")?;
                }

                if event.has_value("cef.extensions.InfobloxPoolId") {
                    event.rename("cef.extensions.InfobloxPoolId", "infoblox_threat_defense.event.infoblox.pool_id")?;
                }

                if event.has_value("cef.extensions.InfobloxServiceId") {
                    event.rename("cef.extensions.InfobloxServiceId", "infoblox_threat_defense.event.infoblox.service_id")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.service_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

                if event.has_value("cef.extensions.message") {
                    event.rename("cef.extensions.message", "infoblox_threat_defense.event.message")?;
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
