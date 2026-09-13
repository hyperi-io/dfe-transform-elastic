// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_system_performance` pipeline.
pub struct PipelineEventSystemPerformance;

impl Transform for PipelineEventSystemPerformance {
    fn name(&self) -> &str {
        "pipeline_event_system_performance"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.reason", json!("Collection of system system_performance data"))?;

                event.append("event.category", json!("host"))?;

                if event.has_value("jamf_protect.telemetry.event.system_performance.metrics.tasks") {
                    event.rename("jamf_protect.telemetry.event.system_performance.metrics.tasks", "jamf_protect.telemetry.system_performance")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
