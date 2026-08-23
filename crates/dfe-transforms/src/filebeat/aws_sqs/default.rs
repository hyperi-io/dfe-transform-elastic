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

            if event.has("aws.sqs.metrics.ApproximateAgeOfOldestMessage.max") {
                event.rename(
                    "aws.sqs.metrics.ApproximateAgeOfOldestMessage.max",
                    "aws.sqs.oldest_message_age.sec",
                )?;
            }

            if event.has("aws.sqs.metrics.ApproximateNumberOfMessagesDelayed.avg") {
                event.rename(
                    "aws.sqs.metrics.ApproximateNumberOfMessagesDelayed.avg",
                    "aws.sqs.messages.delayed",
                )?;
            }

            if event.has("aws.sqs.metrics.ApproximateNumberOfMessagesNotVisible.avg") {
                event.rename(
                    "aws.sqs.metrics.ApproximateNumberOfMessagesNotVisible.avg",
                    "aws.sqs.messages.not_visible",
                )?;
            }

            if event.has("aws.sqs.metrics.ApproximateNumberOfMessagesVisible.avg") {
                event.rename(
                    "aws.sqs.metrics.ApproximateNumberOfMessagesVisible.avg",
                    "aws.sqs.messages.visible",
                )?;
            }

            if event.has("aws.sqs.metrics.NumberOfMessagesDeleted.sum") {
                event.rename(
                    "aws.sqs.metrics.NumberOfMessagesDeleted.sum",
                    "aws.sqs.messages.deleted",
                )?;
            }

            if event.has("aws.sqs.metrics.NumberOfMessagesReceived.sum") {
                event.rename(
                    "aws.sqs.metrics.NumberOfMessagesReceived.sum",
                    "aws.sqs.messages.received",
                )?;
            }

            if event.has("aws.sqs.metrics.NumberOfMessagesSent.sum") {
                event.rename(
                    "aws.sqs.metrics.NumberOfMessagesSent.sum",
                    "aws.sqs.messages.sent",
                )?;
            }

            if event.has("aws.sqs.metrics.NumberOfEmptyReceives.sum") {
                event.rename(
                    "aws.sqs.metrics.NumberOfEmptyReceives.sum",
                    "aws.sqs.empty_receives",
                )?;
            }

            if event.has("aws.sqs.metrics.SentMessageSize.avg") {
                event.rename(
                    "aws.sqs.metrics.SentMessageSize.avg",
                    "aws.sqs.sent_message_size.bytes",
                )?;
            }

            let _cond = { event.get_str("agent.type") != Some("firehose") };
            if _cond {
                event.remove("aws.sqs.metrics");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
