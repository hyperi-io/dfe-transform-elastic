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

            if event.has("gcp.metrics.container.cpu.core_usage_time.sec") {
                event.rename(
                    "gcp.metrics.container.cpu.core_usage_time.sec",
                    "gcp.gke.container.cpu.core_usage_time.sec",
                )?;
            }

            if event.has("gcp.metrics.container.cpu.limit_cores.value") {
                event.rename(
                    "gcp.metrics.container.cpu.limit_cores.value",
                    "gcp.gke.container.cpu.limit_cores.value",
                )?;
            }

            if event.has("gcp.metrics.container.cpu.limit_utilization.pct") {
                event.rename(
                    "gcp.metrics.container.cpu.limit_utilization.pct",
                    "gcp.gke.container.cpu.limit_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.container.cpu.request_cores.value") {
                event.rename(
                    "gcp.metrics.container.cpu.request_cores.value",
                    "gcp.gke.container.cpu.request_cores.value",
                )?;
            }

            if event.has("gcp.metrics.container.cpu.request_utilization.pct") {
                event.rename(
                    "gcp.metrics.container.cpu.request_utilization.pct",
                    "gcp.gke.container.cpu.request_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.container.ephemeral_storage.limit.bytes") {
                event.rename(
                    "gcp.metrics.container.ephemeral_storage.limit.bytes",
                    "gcp.gke.container.ephemeral_storage.limit.bytes",
                )?;
            }

            if event.has("gcp.metrics.container.ephemeral_storage.request.bytes") {
                event.rename(
                    "gcp.metrics.container.ephemeral_storage.request.bytes",
                    "gcp.gke.container.ephemeral_storage.request.bytes",
                )?;
            }

            if event.has("gcp.metrics.container.ephemeral_storage.used.bytes") {
                event.rename(
                    "gcp.metrics.container.ephemeral_storage.used.bytes",
                    "gcp.gke.container.ephemeral_storage.used.bytes",
                )?;
            }

            if event.has("gcp.metrics.container.memory.limit.bytes") {
                event.rename(
                    "gcp.metrics.container.memory.limit.bytes",
                    "gcp.gke.container.memory.limit.bytes",
                )?;
            }

            if event.has("gcp.metrics.container.memory.limit_utilization.pct") {
                event.rename(
                    "gcp.metrics.container.memory.limit_utilization.pct",
                    "gcp.gke.container.memory.limit_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.container.memory.page_fault.count") {
                event.rename(
                    "gcp.metrics.container.memory.page_fault.count",
                    "gcp.gke.container.memory.page_fault.count",
                )?;
            }

            if event.has("gcp.metrics.container.memory.request.bytes") {
                event.rename(
                    "gcp.metrics.container.memory.request.bytes",
                    "gcp.gke.container.memory.request.bytes",
                )?;
            }

            if event.has("gcp.metrics.container.memory.request_utilization.pct") {
                event.rename(
                    "gcp.metrics.container.memory.request_utilization.pct",
                    "gcp.gke.container.memory.request_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.container.memory.used.bytes") {
                event.rename(
                    "gcp.metrics.container.memory.used.bytes",
                    "gcp.gke.container.memory.used.bytes",
                )?;
            }

            if event.has("gcp.metrics.container.restart.count") {
                event.rename(
                    "gcp.metrics.container.restart.count",
                    "gcp.gke.container.restart.count",
                )?;
            }

            if event.has("gcp.metrics.container.uptime.sec") {
                event.rename(
                    "gcp.metrics.container.uptime.sec",
                    "gcp.gke.container.uptime.sec",
                )?;
            }

            if event.has("gcp.metrics.node.cpu.allocatable_cores.value") {
                event.rename(
                    "gcp.metrics.node.cpu.allocatable_cores.value",
                    "gcp.gke.node.cpu.allocatable_cores.value",
                )?;
            }

            if event.has("gcp.metrics.node.cpu.allocatable_utilization.pct") {
                event.rename(
                    "gcp.metrics.node.cpu.allocatable_utilization.pct",
                    "gcp.gke.node.cpu.allocatable_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.node.cpu.core_usage_time.sec") {
                event.rename(
                    "gcp.metrics.node.cpu.core_usage_time.sec",
                    "gcp.gke.node.cpu.core_usage_time.sec",
                )?;
            }

            if event.has("gcp.metrics.node.cpu.total_cores.value") {
                event.rename(
                    "gcp.metrics.node.cpu.total_cores.value",
                    "gcp.gke.node.cpu.total_cores.value",
                )?;
            }

            if event.has("gcp.metrics.node.ephemeral_storage.allocatable.bytes") {
                event.rename(
                    "gcp.metrics.node.ephemeral_storage.allocatable.bytes",
                    "gcp.gke.node.ephemeral_storage.allocatable.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.ephemeral_storage.inodes_free.value") {
                event.rename(
                    "gcp.metrics.node.ephemeral_storage.inodes_free.value",
                    "gcp.gke.node.ephemeral_storage.inodes_free.value",
                )?;
            }

            if event.has("gcp.metrics.node.ephemeral_storage.inodes_total.value") {
                event.rename(
                    "gcp.metrics.node.ephemeral_storage.inodes_total.value",
                    "gcp.gke.node.ephemeral_storage.inodes_total.value",
                )?;
            }

            if event.has("gcp.metrics.node.ephemeral_storage.total.bytes") {
                event.rename(
                    "gcp.metrics.node.ephemeral_storage.total.bytes",
                    "gcp.gke.node.ephemeral_storage.total.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.ephemeral_storage.used.bytes") {
                event.rename(
                    "gcp.metrics.node.ephemeral_storage.used.bytes",
                    "gcp.gke.node.ephemeral_storage.used.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.memory.allocatable.bytes") {
                event.rename(
                    "gcp.metrics.node.memory.allocatable.bytes",
                    "gcp.gke.node.memory.allocatable.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.memory.allocatable_utilization.pct") {
                event.rename(
                    "gcp.metrics.node.memory.allocatable_utilization.pct",
                    "gcp.gke.node.memory.allocatable_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.node.memory.total.bytes") {
                event.rename(
                    "gcp.metrics.node.memory.total.bytes",
                    "gcp.gke.node.memory.total.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.memory.used.bytes") {
                event.rename(
                    "gcp.metrics.node.memory.used.bytes",
                    "gcp.gke.node.memory.used.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.network.received.bytes") {
                event.rename(
                    "gcp.metrics.node.network.received.bytes",
                    "gcp.gke.node.network.received.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.network.sent.bytes") {
                event.rename(
                    "gcp.metrics.node.network.sent.bytes",
                    "gcp.gke.node.network.sent.bytes",
                )?;
            }

            if event.has("gcp.metrics.node.pid_limit.value") {
                event.rename(
                    "gcp.metrics.node.pid_limit.value",
                    "gcp.gke.node.pid_limit.value",
                )?;
            }

            if event.has("gcp.metrics.node.pid_used.value") {
                event.rename(
                    "gcp.metrics.node.pid_used.value",
                    "gcp.gke.node.pid_used.value",
                )?;
            }

            if event.has("gcp.metrics.node_daemon.cpu.core_usage_time.sec") {
                event.rename(
                    "gcp.metrics.node_daemon.cpu.core_usage_time.sec",
                    "gcp.gke.node_daemon.cpu.core_usage_time.sec",
                )?;
            }

            if event.has("gcp.metrics.node_daemon.memory.used.bytes") {
                event.rename(
                    "gcp.metrics.node_daemon.memory.used.bytes",
                    "gcp.gke.node_daemon.memory.used.bytes",
                )?;
            }

            if event.has("gcp.metrics.pod.network.received.bytes") {
                event.rename(
                    "gcp.metrics.pod.network.received.bytes",
                    "gcp.gke.pod.network.received.bytes",
                )?;
            }

            if event.has("gcp.metrics.pod.network.sent.bytes") {
                event.rename(
                    "gcp.metrics.pod.network.sent.bytes",
                    "gcp.gke.pod.network.sent.bytes",
                )?;
            }

            if event.has("gcp.metrics.pod.volume.total.bytes") {
                event.rename(
                    "gcp.metrics.pod.volume.total.bytes",
                    "gcp.gke.pod.volume.total.bytes",
                )?;
            }

            if event.has("gcp.metrics.pod.volume.used.bytes") {
                event.rename(
                    "gcp.metrics.pod.volume.used.bytes",
                    "gcp.gke.pod.volume.used.bytes",
                )?;
            }

            if event.has("gcp.metrics.pod.volume.utilization.pct") {
                event.rename(
                    "gcp.metrics.pod.volume.utilization.pct",
                    "gcp.gke.pod.volume.utilization.pct",
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
