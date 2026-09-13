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

            if event.has_value("gcp.metrics.container.billable_instance_time.value") {
                event.rename(
                    "gcp.metrics.container.billable_instance_time.value",
                    "gcp.cloudrun_metrics.container.billable_instance_time",
                )?;
            }

            if event.has_value("gcp.metrics.container.cpu.allocation_time.value") {
                event.rename(
                    "gcp.metrics.container.cpu.allocation_time.value",
                    "gcp.cloudrun_metrics.container.cpu.allocation_time.sec",
                )?;
            }

            if event.has_value("gcp.metrics.container.cpu.utilizations.value") {
                event.rename(
                    "gcp.metrics.container.cpu.utilizations.value",
                    "gcp.cloudrun_metrics.container.cpu.utilizations",
                )?;
            }

            if event.has_value("gcp.metrics.container.instance_count.value") {
                event.rename(
                    "gcp.metrics.container.instance_count.value",
                    "gcp.cloudrun_metrics.container.instance.count",
                )?;
            }

            if event.has_value("gcp.metrics.container.max_request_concurrencies.value") {
                event.rename(
                    "gcp.metrics.container.max_request_concurrencies.value",
                    "gcp.cloudrun_metrics.container.max_request_concurrencies",
                )?;
            }

            if event.has_value("gcp.metrics.container.memory.allocation_time.value") {
                event.rename(
                    "gcp.metrics.container.memory.allocation_time.value",
                    "gcp.cloudrun_metrics.container.memory.allocation_time",
                )?;
            }

            if event.has_value("gcp.metrics.container.memory.utilizations.value") {
                event.rename(
                    "gcp.metrics.container.memory.utilizations.value",
                    "gcp.cloudrun_metrics.container.memory.utilizations",
                )?;
            }

            if event.has_value("gcp.metrics.container.network.received_bytes_count.value") {
                event.rename(
                    "gcp.metrics.container.network.received_bytes_count.value",
                    "gcp.cloudrun_metrics.container.network.received.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.container.network.sent_bytes_count.value") {
                event.rename(
                    "gcp.metrics.container.network.sent_bytes_count.value",
                    "gcp.cloudrun_metrics.container.network.sent.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.request_count.value") {
                event.rename(
                    "gcp.metrics.request_count.value",
                    "gcp.cloudrun_metrics.request.count",
                )?;
            }

            if event.has_value("gcp.metrics.request_latencies.value") {
                event.rename(
                    "gcp.metrics.request_latencies.value",
                    "gcp.cloudrun_metrics.request_latencies",
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
