// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("gcp.vertexai_logs") {
                event.rename("gcp.vertexai_logs", "gcp.vertexai.prompt_response_logs")?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("cloud.provider", json!("gcp"))?;

            event.set("cloud.service.name", json!("vertex-ai"))?;

            event.set("service.name", json!("vertex-ai"))?;

            event.set("service.type", json!("ai-platform"))?;

            let _cond = { event.has_value("gcp.vertexai.prompt_response_logs.api_method") };
            if _cond {
                if let Some(v) = event
                    .get("gcp.vertexai.prompt_response_logs.api_method")
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "52de863f006b3ea222121714f06e7b5a09659dc844a621b8bc027adebef0232c",
                    r#"{"params":null,"known":[{"KeysToSnakeCase":["gcp.vertexai.prompt_response_logs.full_response","CamelBreak"]}]}"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, emitted as the runtime's equivalent
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        empty_strings: true,
                        empty_collections: true,
                        prune_lists: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
                Ok(())
            })();

            event.remove("gcp.vertexai_logs");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
