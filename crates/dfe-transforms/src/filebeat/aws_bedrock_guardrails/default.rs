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

            if event.has_value("aws.guardrails.metrics") {
                event.rename("aws.guardrails.metrics", "aws_bedrock.guardrails")?;
            }

            if event.has_value("aws_bedrock.guardrails.Invocations.sum") {
                event.rename(
                    "aws_bedrock.guardrails.Invocations.sum",
                    "aws_bedrock.guardrails.invocations",
                )?;
            }

            if event.has_value("aws_bedrock.guardrails.InvocationClientErrors.sum") {
                event.rename(
                    "aws_bedrock.guardrails.InvocationClientErrors.sum",
                    "aws_bedrock.guardrails.invocation_client_errors",
                )?;
            }

            if event.has_value("aws_bedrock.guardrails.InvocationServerErrors.sum") {
                event.rename(
                    "aws_bedrock.guardrails.InvocationServerErrors.sum",
                    "aws_bedrock.guardrails.invocation_server_errors",
                )?;
            }

            if event.has_value("aws_bedrock.guardrails.InvocationThrottles.sum") {
                event.rename(
                    "aws_bedrock.guardrails.InvocationThrottles.sum",
                    "aws_bedrock.guardrails.invocation_throttles",
                )?;
            }

            if event.has_value("aws_bedrock.guardrails.TextUnitCount.sum") {
                event.rename(
                    "aws_bedrock.guardrails.TextUnitCount.sum",
                    "aws_bedrock.guardrails.text_unit_count",
                )?;
            }

            if event.has_value("aws_bedrock.guardrails.InvocationsIntervened.sum") {
                event.rename(
                    "aws_bedrock.guardrails.InvocationsIntervened.sum",
                    "aws_bedrock.guardrails.invocations_intervened",
                )?;
            }

            if event.has_value("aws_bedrock.guardrails.InvocationLatency.avg") {
                event.rename(
                    "aws_bedrock.guardrails.InvocationLatency.avg",
                    "aws_bedrock.guardrails.invocation_latency",
                )?;
            }

            if event.has_value("aws.dimensions.Operation") {
                event.rename(
                    "aws.dimensions.Operation",
                    "aws_bedrock.guardrails.operation",
                )?;
            }

            if event.has_value("aws.dimensions.GuardrailContentSource") {
                event.rename(
                    "aws.dimensions.GuardrailContentSource",
                    "aws_bedrock.guardrails.guardrail_content_source",
                )?;
            }

            if event.has_value("aws.dimensions.GuardrailPolicyType") {
                event.rename(
                    "aws.dimensions.GuardrailPolicyType",
                    "aws_bedrock.guardrails.guardrail_policy_type",
                )?;
            }

            if event.has_value("aws.dimensions.GuardrailArn") {
                event.rename(
                    "aws.dimensions.GuardrailArn",
                    "aws_bedrock.guardrails.guardrail_arn",
                )?;
            }

            if event.has_value("aws.dimensions.GuardrailVersion") {
                event.rename(
                    "aws.dimensions.GuardrailVersion",
                    "aws_bedrock.guardrails.guardrail_version",
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
