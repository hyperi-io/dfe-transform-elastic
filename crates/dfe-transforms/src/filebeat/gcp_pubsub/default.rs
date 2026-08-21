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
            {
                let mut values = Vec::new();
                if let Some(v) = event.get("gcp.labels") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "gcp.labels_fingerprint",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            if event.has("gcp.metrics.snapshot.backlog.bytes") {
                event.rename(
                    "gcp.metrics.snapshot.backlog.bytes",
                    "gcp.pubsub.snapshot.backlog.bytes",
                )?;
            }

            if event.has("gcp.metrics.snapshot.backlog_bytes_by_region.bytes") {
                event.rename(
                    "gcp.metrics.snapshot.backlog_bytes_by_region.bytes",
                    "gcp.pubsub.snapshot.backlog_bytes_by_region.bytes",
                )?;
            }

            if event.has("gcp.metrics.snapshot.config_updates.count") {
                event.rename(
                    "gcp.metrics.snapshot.config_updates.count",
                    "gcp.pubsub.snapshot.config_updates.count",
                )?;
            }

            if event.has("gcp.metrics.snapshot.num_messages.value") {
                event.rename(
                    "gcp.metrics.snapshot.num_messages.value",
                    "gcp.pubsub.snapshot.num_messages.value",
                )?;
            }

            if event.has("gcp.metrics.snapshot.num_messages_by_region.value") {
                event.rename(
                    "gcp.metrics.snapshot.num_messages_by_region.value",
                    "gcp.pubsub.snapshot.num_messages_by_region.value",
                )?;
            }

            if event.has("gcp.metrics.snapshot.oldest_message_age.sec") {
                event.rename(
                    "gcp.metrics.snapshot.oldest_message_age.sec",
                    "gcp.pubsub.snapshot.oldest_message_age.sec",
                )?;
            }

            if event.has("gcp.metrics.snapshot.oldest_message_age_by_region.sec") {
                event.rename(
                    "gcp.metrics.snapshot.oldest_message_age_by_region.sec",
                    "gcp.pubsub.snapshot.oldest_message_age_by_region.sec",
                )?;
            }

            if event.has("gcp.metrics.subscription.ack_message.count") {
                event.rename(
                    "gcp.metrics.subscription.ack_message.count",
                    "gcp.pubsub.subscription.ack_message.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.backlog.bytes") {
                event.rename(
                    "gcp.metrics.subscription.backlog.bytes",
                    "gcp.pubsub.subscription.backlog.bytes",
                )?;
            }

            if event.has("gcp.metrics.subscription.byte_cost.bytes") {
                event.rename(
                    "gcp.metrics.subscription.byte_cost.bytes",
                    "gcp.pubsub.subscription.byte_cost.bytes",
                )?;
            }

            if event.has("gcp.metrics.subscription.config_updates.count") {
                event.rename(
                    "gcp.metrics.subscription.config_updates.count",
                    "gcp.pubsub.subscription.config_updates.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.dead_letter_message.count") {
                event.rename(
                    "gcp.metrics.subscription.dead_letter_message.count",
                    "gcp.pubsub.subscription.dead_letter_message.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.mod_ack_deadline_message.count") {
                event.rename(
                    "gcp.metrics.subscription.mod_ack_deadline_message.count",
                    "gcp.pubsub.subscription.mod_ack_deadline_message.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.mod_ack_deadline_message_operation.count") {
                event.rename(
                    "gcp.metrics.subscription.mod_ack_deadline_message_operation.count",
                    "gcp.pubsub.subscription.mod_ack_deadline_message_operation.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.mod_ack_deadline_request.count") {
                event.rename(
                    "gcp.metrics.subscription.mod_ack_deadline_request.count",
                    "gcp.pubsub.subscription.mod_ack_deadline_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.num_outstanding_messages.value") {
                event.rename(
                    "gcp.metrics.subscription.num_outstanding_messages.value",
                    "gcp.pubsub.subscription.num_outstanding_messages.value",
                )?;
            }

            if event.has("gcp.metrics.subscription.num_undelivered_messages.value") {
                event.rename(
                    "gcp.metrics.subscription.num_undelivered_messages.value",
                    "gcp.pubsub.subscription.num_undelivered_messages.value",
                )?;
            }

            if event.has("gcp.metrics.subscription.oldest_retained_acked_message_age.sec") {
                event.rename(
                    "gcp.metrics.subscription.oldest_retained_acked_message_age.sec",
                    "gcp.pubsub.subscription.oldest_retained_acked_message_age.sec",
                )?;
            }

            if event
                .has("gcp.metrics.subscription.oldest_retained_acked_message_age_by_region.value")
            {
                event.rename(
                    "gcp.metrics.subscription.oldest_retained_acked_message_age_by_region.value",
                    "gcp.pubsub.subscription.oldest_retained_acked_message_age_by_region.value",
                )?;
            }

            if event.has("gcp.metrics.subscription.oldest_unacked_message_age.sec") {
                event.rename(
                    "gcp.metrics.subscription.oldest_unacked_message_age.sec",
                    "gcp.pubsub.subscription.oldest_unacked_message_age.sec",
                )?;
            }

            if event.has("gcp.metrics.subscription.oldest_unacked_message_age_by_region.value") {
                event.rename(
                    "gcp.metrics.subscription.oldest_unacked_message_age_by_region.value",
                    "gcp.pubsub.subscription.oldest_unacked_message_age_by_region.value",
                )?;
            }

            if event.has("gcp.metrics.subscription.pull_ack_message_operation.count") {
                event.rename(
                    "gcp.metrics.subscription.pull_ack_message_operation.count",
                    "gcp.pubsub.subscription.pull_ack_message_operation.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.pull_ack_request.count") {
                event.rename(
                    "gcp.metrics.subscription.pull_ack_request.count",
                    "gcp.pubsub.subscription.pull_ack_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.pull_message_operation.count") {
                event.rename(
                    "gcp.metrics.subscription.pull_message_operation.count",
                    "gcp.pubsub.subscription.pull_message_operation.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.pull_request.count") {
                event.rename(
                    "gcp.metrics.subscription.pull_request.count",
                    "gcp.pubsub.subscription.pull_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.push_request.count") {
                event.rename(
                    "gcp.metrics.subscription.push_request.count",
                    "gcp.pubsub.subscription.push_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.retained_acked.bytes") {
                event.rename(
                    "gcp.metrics.subscription.retained_acked.bytes",
                    "gcp.pubsub.subscription.retained_acked.bytes",
                )?;
            }

            if event.has("gcp.metrics.subscription.retained_acked_bytes_by_region.bytes") {
                event.rename(
                    "gcp.metrics.subscription.retained_acked_bytes_by_region.bytes",
                    "gcp.pubsub.subscription.retained_acked_bytes_by_region.bytes",
                )?;
            }

            if event.has("gcp.metrics.subscription.seek_request.count") {
                event.rename(
                    "gcp.metrics.subscription.seek_request.count",
                    "gcp.pubsub.subscription.seek_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.sent_message.count") {
                event.rename(
                    "gcp.metrics.subscription.sent_message.count",
                    "gcp.pubsub.subscription.sent_message.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.streaming_pull_ack_message_operation.count") {
                event.rename(
                    "gcp.metrics.subscription.streaming_pull_ack_message_operation.count",
                    "gcp.pubsub.subscription.streaming_pull_ack_message_operation.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.streaming_pull_ack_request.count") {
                event.rename(
                    "gcp.metrics.subscription.streaming_pull_ack_request.count",
                    "gcp.pubsub.subscription.streaming_pull_ack_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.streaming_pull_message_operation.count") {
                event.rename(
                    "gcp.metrics.subscription.streaming_pull_message_operation.count",
                    "gcp.pubsub.subscription.streaming_pull_message_operation.count",
                )?;
            }

            if event.has(
                "gcp.metrics.subscription.streaming_pull_mod_ack_deadline_message_operation.count",
            ) {
                event.rename("gcp.metrics.subscription.streaming_pull_mod_ack_deadline_message_operation.count", "gcp.pubsub.subscription.streaming_pull_mod_ack_deadline_message_operation.count")?;
            }

            if event.has("gcp.metrics.subscription.streaming_pull_mod_ack_deadline_request.count") {
                event.rename(
                    "gcp.metrics.subscription.streaming_pull_mod_ack_deadline_request.count",
                    "gcp.pubsub.subscription.streaming_pull_mod_ack_deadline_request.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.streaming_pull_response.count") {
                event.rename(
                    "gcp.metrics.subscription.streaming_pull_response.count",
                    "gcp.pubsub.subscription.streaming_pull_response.count",
                )?;
            }

            if event.has("gcp.metrics.subscription.unacked_bytes_by_region.bytes") {
                event.rename(
                    "gcp.metrics.subscription.unacked_bytes_by_region.bytes",
                    "gcp.pubsub.subscription.unacked_bytes_by_region.bytes",
                )?;
            }

            if event.has("gcp.metrics.topic.byte_cost.bytes") {
                event.rename(
                    "gcp.metrics.topic.byte_cost.bytes",
                    "gcp.pubsub.topic.byte_cost.bytes",
                )?;
            }

            if event.has("gcp.metrics.topic.config_updates.count") {
                event.rename(
                    "gcp.metrics.topic.config_updates.count",
                    "gcp.pubsub.topic.config_updates.count",
                )?;
            }

            if event.has("gcp.metrics.topic.message_sizes.bytes") {
                event.rename(
                    "gcp.metrics.topic.message_sizes.bytes",
                    "gcp.pubsub.topic.message_sizes.bytes",
                )?;
            }

            if event.has("gcp.metrics.topic.oldest_retained_acked_message_age_by_region.value") {
                event.rename(
                    "gcp.metrics.topic.oldest_retained_acked_message_age_by_region.value",
                    "gcp.pubsub.topic.oldest_retained_acked_message_age_by_region.value",
                )?;
            }

            if event.has("gcp.metrics.topic.oldest_unacked_message_age_by_region.value") {
                event.rename(
                    "gcp.metrics.topic.oldest_unacked_message_age_by_region.value",
                    "gcp.pubsub.topic.oldest_unacked_message_age_by_region.value",
                )?;
            }

            if event.has("gcp.metrics.topic.retained_acked_bytes_by_region.bytes") {
                event.rename(
                    "gcp.metrics.topic.retained_acked_bytes_by_region.bytes",
                    "gcp.pubsub.topic.retained_acked_bytes_by_region.bytes",
                )?;
            }

            if event.has("gcp.metrics.topic.send_message_operation.count") {
                event.rename(
                    "gcp.metrics.topic.send_message_operation.count",
                    "gcp.pubsub.topic.send_message_operation.count",
                )?;
            }

            if event.has("gcp.metrics.topic.send_request.count") {
                event.rename(
                    "gcp.metrics.topic.send_request.count",
                    "gcp.pubsub.topic.send_request.count",
                )?;
            }

            if event.has("gcp.metrics.topic.streaming_pull_response.count") {
                event.rename(
                    "gcp.metrics.topic.streaming_pull_response.count",
                    "gcp.pubsub.topic.streaming_pull_response.count",
                )?;
            }

            if event.has("gcp.metrics.topic.unacked_bytes_by_region.bytes") {
                event.rename(
                    "gcp.metrics.topic.unacked_bytes_by_region.bytes",
                    "gcp.pubsub.topic.unacked_bytes_by_region.bytes",
                )?;
            }

            if event.has("gcp.metrics.subscription.ack_latencies.value") {
                event.rename(
                    "gcp.metrics.subscription.ack_latencies.value",
                    "gcp.pubsub.subscription.ack_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.subscription.push_request_latencies.value") {
                event.rename(
                    "gcp.metrics.subscription.push_request_latencies.value",
                    "gcp.pubsub.subscription.push_request_latencies.value",
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
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
