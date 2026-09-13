// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `application_control_event` pipeline.
pub struct ApplicationControlEvent;

impl Transform for ApplicationControlEvent {
    fn name(&self) -> &str {
        "application_control_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("trendmicro.deep_security.event_category", json!("application-control-event"))?;

                event.append_unique("event.category", json!("intrusion_detection"))?;

                event.append_unique("event.type", json!("info"))?;

            let _cond = { event.has_value("trendmicro.deep_security.signature_id") && event.get_i64("trendmicro.deep_security.signature_id") != Some(6002100) && event.get_i64("trendmicro.deep_security.signature_id") != Some(6002200) };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.has_value("trendmicro.deep_security.extensions.device.custom_string2.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("trendmicro.deep_security.extensions.device.custom_string2.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("trendmicro.deep_security.extensions.device.custom_string3.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("trendmicro.deep_security.extensions.device.custom_string3.value").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
