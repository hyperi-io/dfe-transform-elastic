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
            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "413a9c642dcf3d57906708575b8d45242b47d150189e9875be5bf440ef8007dd",
                    r#"{"params":null,"known":[{"RescaleFields":{"targets":["panw.system.cpu.user","panw.system.cpu.system","panw.system.cpu.nice","panw.system.cpu.idle","panw.system.cpu.wait","panw.system.cpu.hi","panw.system.cpu.system_int","panw.system.cpu.steal","panw.system.filesystem.use_percent"],"divisor":100,"round_unit":null}}]}"#
                ),
            )?;

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
