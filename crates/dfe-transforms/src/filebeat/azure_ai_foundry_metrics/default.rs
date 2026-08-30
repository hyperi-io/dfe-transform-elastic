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
            if event.has_value("azure.metrics") {
                event.rename("azure.metrics", "azure.ai_foundry")?;
            }

            if event.has_value("azure.ai_foundry.time_toresponse.avg") {
                event.rename(
                    "azure.ai_foundry.time_toresponse.avg",
                    "azure.ai_foundry.time_to_response.avg",
                )?;
            }

            if event.has_value("azure.ai_foundry.normalized_time_tofirst_token.avg") {
                event.rename(
                    "azure.ai_foundry.normalized_time_tofirst_token.avg",
                    "azure.ai_foundry.normalized_time_to_first_token.avg",
                )?;
            }

            if event.has_value("azure.ai_foundry.time_tolast_byte.avg") {
                event.rename(
                    "azure.ai_foundry.time_tolast_byte.avg",
                    "azure.ai_foundry.time_to_last_byte.avg",
                )?;
            }

            let _cond = { event.has_value("azure.ai_foundry.model_availability_rate.avg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.azure.ai_foundry.model_availability_rate.avg = ctx.azure.ai_foundry.model_availability_rate.avg\n        / params.param_percent;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.azure.ai_foundry.model_availability_rate.avg = ctx.azure.ai_foundry.model_availability_rate.avg\n        / params.param_percent;"#
                        ),
                        cached_params!("{\"param_percent\":100}"),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("azure.ai_foundry.provisioned_utilization.avg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.azure.ai_foundry.provisioned_utilization.avg = ctx.azure.ai_foundry.provisioned_utilization.avg\n        / params.param_percent;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.azure.ai_foundry.provisioned_utilization.avg = ctx.azure.ai_foundry.provisioned_utilization.avg\n        / params.param_percent;"#
                        ),
                        cached_params!("{\"param_percent\":100}"),
                    )?;
                    Ok(())
                })();
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
