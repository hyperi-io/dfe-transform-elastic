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
            event.set("ecs.version", json!("8.11.0"))?;

            if event.has_value("prometheus") {
                event.rename("prometheus", "cockroachdb.status")?;
            }

            let _cond = {
                event.has_value("cockroachdb.status.up")
                    && event.has_value("cockroachdb.status.up.value")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.cockroachdb.status.up.value == 1){\n  ctx.cockroachdb.status.up.value_description = \"up\"\n  } else if(ctx.cockroachdb.status.up.value == 0){\n  ctx.cockroachdb.status.up.value_description = \"down\"\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.cockroachdb.status.up.value == 1){\n  ctx.cockroachdb.status.up.value_description = \"up\"\n  } else if(ctx.cockroachdb.status.up.value == 0){\n  ctx.cockroachdb.status.up.value_description = \"down\"\n}\n"#
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
