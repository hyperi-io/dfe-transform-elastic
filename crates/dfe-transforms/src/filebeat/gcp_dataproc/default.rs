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

            if event.has("gcp.metrics.batch.spark.executors.count") {
                event.rename(
                    "gcp.metrics.batch.spark.executors.count",
                    "gcp.dataproc.batch.spark.executors.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.hdfs.datanodes.count") {
                event.rename(
                    "gcp.metrics.cluster.hdfs.datanodes.count",
                    "gcp.dataproc.cluster.hdfs.datanodes.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.hdfs.storage_capacity.value") {
                event.rename(
                    "gcp.metrics.cluster.hdfs.storage_capacity.value",
                    "gcp.dataproc.cluster.hdfs.storage_capacity.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.hdfs.storage_utilization.value") {
                event.rename(
                    "gcp.metrics.cluster.hdfs.storage_utilization.value",
                    "gcp.dataproc.cluster.hdfs.storage_utilization.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.hdfs.unhealthy_blocks.count") {
                event.rename(
                    "gcp.metrics.cluster.hdfs.unhealthy_blocks.count",
                    "gcp.dataproc.cluster.hdfs.unhealthy_blocks.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.job.failed.count") {
                event.rename(
                    "gcp.metrics.cluster.job.failed.count",
                    "gcp.dataproc.cluster.job.failed.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.job.running.count") {
                event.rename(
                    "gcp.metrics.cluster.job.running.count",
                    "gcp.dataproc.cluster.job.running.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.job.submitted.count") {
                event.rename(
                    "gcp.metrics.cluster.job.submitted.count",
                    "gcp.dataproc.cluster.job.submitted.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.operation.failed.count") {
                event.rename(
                    "gcp.metrics.cluster.operation.failed.count",
                    "gcp.dataproc.cluster.operation.failed.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.operation.running.count") {
                event.rename(
                    "gcp.metrics.cluster.operation.running.count",
                    "gcp.dataproc.cluster.operation.running.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.operation.submitted.count") {
                event.rename(
                    "gcp.metrics.cluster.operation.submitted.count",
                    "gcp.dataproc.cluster.operation.submitted.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.allocated_memory_percentage.value") {
                event.rename(
                    "gcp.metrics.cluster.yarn.allocated_memory_percentage.value",
                    "gcp.dataproc.cluster.yarn.allocated_memory_percentage.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.apps.count") {
                event.rename(
                    "gcp.metrics.cluster.yarn.apps.count",
                    "gcp.dataproc.cluster.yarn.apps.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.containers.count") {
                event.rename(
                    "gcp.metrics.cluster.yarn.containers.count",
                    "gcp.dataproc.cluster.yarn.containers.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.memory_size.value") {
                event.rename(
                    "gcp.metrics.cluster.yarn.memory_size.value",
                    "gcp.dataproc.cluster.yarn.memory_size.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.nodemanagers.count") {
                event.rename(
                    "gcp.metrics.cluster.yarn.nodemanagers.count",
                    "gcp.dataproc.cluster.yarn.nodemanagers.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.pending_memory_size.value") {
                event.rename(
                    "gcp.metrics.cluster.yarn.pending_memory_size.value",
                    "gcp.dataproc.cluster.yarn.pending_memory_size.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.yarn.virtual_cores.count") {
                event.rename(
                    "gcp.metrics.cluster.yarn.virtual_cores.count",
                    "gcp.dataproc.cluster.yarn.virtual_cores.count",
                )?;
            }

            if event.has("gcp.metrics.cluster.job.completion_time.value") {
                event.rename(
                    "gcp.metrics.cluster.job.completion_time.value",
                    "gcp.dataproc.cluster.job.completion_time.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.job.duration.value") {
                event.rename(
                    "gcp.metrics.cluster.job.duration.value",
                    "gcp.dataproc.cluster.job.duration.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.operation.completion_time.value") {
                event.rename(
                    "gcp.metrics.cluster.operation.completion_time.value",
                    "gcp.dataproc.cluster.operation.completion_time.value",
                )?;
            }

            if event.has("gcp.metrics.cluster.operation.duration.value") {
                event.rename(
                    "gcp.metrics.cluster.operation.duration.value",
                    "gcp.dataproc.cluster.operation.duration.value",
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
