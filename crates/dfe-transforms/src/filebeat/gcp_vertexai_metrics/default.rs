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
            if event.has_value("gcp.metrics.publisher.online_serving.token_count.value") {
                event.rename(
                    "gcp.metrics.publisher.online_serving.token_count.value",
                    "gcp.vertexai.publisher.online_serving.token_count",
                )?;
            }

            if event.has_value("gcp.metrics.publisher.online_serving.model_invocation_count.value")
            {
                event.rename(
                    "gcp.metrics.publisher.online_serving.model_invocation_count.value",
                    "gcp.vertexai.publisher.online_serving.model_invocation_count",
                )?;
            }

            if event.has_value("gcp.metrics.publisher.online_serving.character_count.value") {
                event.rename(
                    "gcp.metrics.publisher.online_serving.character_count.value",
                    "gcp.vertexai.publisher.online_serving.character_count",
                )?;
            }

            if event.has_value(
                "gcp.metrics.publisher.online_serving.model_invocation_latencies.value.histogram",
            ) {
                event.rename("gcp.metrics.publisher.online_serving.model_invocation_latencies.value.histogram", "gcp.vertexai.publisher.online_serving.model_invocation_latencies")?;
            }

            if event.has_value("gcp.metrics.publisher.online_serving.consumed_throughput.value") {
                event.rename(
                    "gcp.metrics.publisher.online_serving.consumed_throughput.value",
                    "gcp.vertexai.publisher.online_serving.consumed_throughput",
                )?;
            }

            if event.has_value(
                "gcp.metrics.publisher.online_serving.first_token_latencies.value.histogram",
            ) {
                event.rename(
                    "gcp.metrics.publisher.online_serving.first_token_latencies.value.histogram",
                    "gcp.vertexai.publisher.online_serving.first_token_latencies",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.error_count.value") {
                event.rename(
                    "gcp.metrics.prediction.online.error_count.value",
                    "gcp.vertexai.prediction.online.error_count",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.response_count.value") {
                event.rename(
                    "gcp.metrics.prediction.online.response_count.value",
                    "gcp.vertexai.prediction.online.response_count",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.prediction_count.value") {
                event.rename(
                    "gcp.metrics.prediction.online.prediction_count.value",
                    "gcp.vertexai.prediction.online.prediction_count",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.cpu.utilization.value") {
                event.rename(
                    "gcp.metrics.prediction.online.cpu.utilization.value",
                    "gcp.vertexai.prediction.online.cpu.utilization",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.memory.bytes_used.value") {
                event.rename(
                    "gcp.metrics.prediction.online.memory.bytes_used.value",
                    "gcp.vertexai.prediction.online.memory.bytes_used",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.network.received_bytes_count.value") {
                event.rename(
                    "gcp.metrics.prediction.online.network.received_bytes_count.value",
                    "gcp.vertexai.prediction.online.network.received_bytes_count",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.network.sent_bytes_count.value") {
                event.rename(
                    "gcp.metrics.prediction.online.network.sent_bytes_count.value",
                    "gcp.vertexai.prediction.online.network.sent_bytes_count",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.replicas.value") {
                event.rename(
                    "gcp.metrics.prediction.online.replicas.value",
                    "gcp.vertexai.prediction.online.replicas",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.target_replicas.value") {
                event.rename(
                    "gcp.metrics.prediction.online.target_replicas.value",
                    "gcp.vertexai.prediction.online.target_replicas",
                )?;
            }

            if event.has_value("gcp.metrics.prediction.online.prediction_latencies.value.histogram")
            {
                event.rename(
                    "gcp.metrics.prediction.online.prediction_latencies.value.histogram",
                    "gcp.vertexai.prediction.online.prediction_latencies",
                )?;
            }

            event.remove("gcp.metrics");

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
