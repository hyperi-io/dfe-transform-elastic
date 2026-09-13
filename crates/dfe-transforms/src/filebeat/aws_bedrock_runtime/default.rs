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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "", "*")?;
                Ok(())
            })();

            if let Some(v) = event
                .get("cloud.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("cloud.account.name") {
                    event.set("cloud.account.name", v)?;
                }
            }

            if event.has_value("aws.bedrock.metrics") {
                event.rename("aws.bedrock.metrics", "aws_bedrock.runtime")?;
            }

            if event.has_value("aws_bedrock.runtime.Invocations.sum") {
                event.rename(
                    "aws_bedrock.runtime.Invocations.sum",
                    "aws_bedrock.runtime.invocations",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.InvocationLatency.avg") {
                event.rename(
                    "aws_bedrock.runtime.InvocationLatency.avg",
                    "aws_bedrock.runtime.invocation_latency",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.InvocationClientErrors.sum") {
                event.rename(
                    "aws_bedrock.runtime.InvocationClientErrors.sum",
                    "aws_bedrock.runtime.invocation_client_errors",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.InvocationServerErrors.sum") {
                event.rename(
                    "aws_bedrock.runtime.InvocationServerErrors.sum",
                    "aws_bedrock.runtime.invocation_server_errors",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.InvocationThrottles.sum") {
                event.rename(
                    "aws_bedrock.runtime.InvocationThrottles.sum",
                    "aws_bedrock.runtime.invocation_throttles",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.InputTokenCount.sum") {
                event.rename(
                    "aws_bedrock.runtime.InputTokenCount.sum",
                    "aws_bedrock.runtime.input_token_count",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.LegacyModelInvocations.sum") {
                event.rename(
                    "aws_bedrock.runtime.LegacyModelInvocations.sum",
                    "aws_bedrock.runtime.legacymodel_invocations",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.OutputTokenCount.sum") {
                event.rename(
                    "aws_bedrock.runtime.OutputTokenCount.sum",
                    "aws_bedrock.runtime.output_token_count",
                )?;
            }

            if event.has_value("aws_bedrock.runtime.OutputImageCount.sum") {
                event.rename(
                    "aws_bedrock.runtime.OutputImageCount.sum",
                    "aws_bedrock.runtime.output_image_count",
                )?;
            }

            if event.has_value("aws.dimensions.ModelId") {
                event.rename("aws.dimensions.ModelId", "aws_bedrock.runtime.model_id")?;
            }

            if event.has_value("aws.dimensions.ImageSize") {
                event.rename("aws.dimensions.ImageSize", "aws_bedrock.runtime.image_size")?;
            }

            if event.has_value("aws.dimensions.Quality") {
                event.rename("aws.dimensions.Quality", "aws_bedrock.runtime.quality")?;
            }

            if event.has_value("aws.dimensions.BucketedStepSize") {
                event.rename(
                    "aws.dimensions.BucketedStepSize",
                    "aws_bedrock.runtime.bucketed_step_size",
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.set(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
