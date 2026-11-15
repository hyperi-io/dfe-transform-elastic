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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan_params(
                        event,
                        compiled_painless!(
                            "12799b674d656272a9a622aa46cad3a12de0734dfff86defdd01df76eb14e5dc",
                            r#"{"params":{"ScalarExpression":{"target":"azure.ai_foundry.model_availability_rate.avg","expr":{"Binary":[{"Field":"azure.ai_foundry.model_availability_rate.avg"},"Div",{"Param":"param_percent"}]}}},"known":[]}"#
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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan_params(
                        event,
                        compiled_painless!(
                            "e86fc5a00b0ca6d4ed7fc765e3e2e6317641e309d39232d3777d8138a707bd0f",
                            r#"{"params":{"ScalarExpression":{"target":"azure.ai_foundry.provisioned_utilization.avg","expr":{"Binary":[{"Field":"azure.ai_foundry.provisioned_utilization.avg"},"Div",{"Param":"param_percent"}]}}},"known":[]}"#
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
