// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_json` pipeline.
pub struct PipelineJson;

impl Transform for PipelineJson {
    fn name(&self) -> &str {
        "pipeline_json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if event.has_value("message") {
            event.rename("message", "_ecs_json_message")?;
        }

        let _cond = { event.has("_ecs_json_message") };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field_to_root(event, "_ecs_json_message", true)?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                if event.has_value("_ecs_json_message") {
                    event.rename("_ecs_json_message", "message")?;
                }
                if !event.has("error.message") {
                    event.set("error.message", json!("Error while parsing JSON"))?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
        }

        event.remove("_ecs_json_message");

        dot_expand(event, "", "*")?;

        let _cond = { event.get("error.stack_trace").is_some_and(|v| v.is_array()) };
        if _cond {
            let joined = event
                .get("error.stack_trace")
                .and_then(|v| join_values(v, "\n"));
            if let Some(joined) = joined {
                event.set("error.stack_trace", json!(joined))?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
