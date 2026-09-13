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
                event.rename("azure.metrics", "azure.open_ai")?;
            }

            if event.has_value("azure.open_ai.azure_openairequests.total") {
                event.rename(
                    "azure.open_ai.azure_openairequests.total",
                    "azure.open_ai.requests.total",
                )?;
            }

            if event.has_value("azure.open_ai.azure_openaiprovisioned_managed_utilization_v2.avg") {
                event.rename(
                    "azure.open_ai.azure_openaiprovisioned_managed_utilization_v2.avg",
                    "azure.open_ai.provisioned_managed_utilization_v2.avg",
                )?;
            }

            if event.has_value("azure.open_ai.azure_openaitime_toresponse.avg") {
                event.rename(
                    "azure.open_ai.azure_openaitime_toresponse.avg",
                    "azure.open_ai.time_to_response.avg",
                )?;
            }

            if event.has_value("azure.open_ai.azure_openaicontext_tokens_cache_match_rate.avg") {
                event.rename(
                    "azure.open_ai.azure_openaicontext_tokens_cache_match_rate.avg",
                    "azure.open_ai.context_tokens_cache_match_rate.avg",
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
