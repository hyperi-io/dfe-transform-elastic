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

            let _cond = { event.get_str("gcp.labels.cloudsql.name") != Some("postgres") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has("gcp.metrics.database.auto_failover_request_count.value") {
                event.rename(
                    "gcp.metrics.database.auto_failover_request_count.value",
                    "gcp.cloudsql_postgresql.database.auto_failover_request.count",
                )?;
            }

            if event.has("gcp.metrics.database.available_for_failover.value") {
                event.rename(
                    "gcp.metrics.database.available_for_failover.value",
                    "gcp.cloudsql_postgresql.database.available_for_failover",
                )?;
            }

            if event.has("gcp.metrics.database.cpu.reserved_cores.value") {
                event.rename(
                    "gcp.metrics.database.cpu.reserved_cores.value",
                    "gcp.cloudsql_postgresql.database.cpu.reserved_cores.count",
                )?;
            }

            if event.has("gcp.metrics.database.cpu.usage_time.value") {
                event.rename(
                    "gcp.metrics.database.cpu.usage_time.value",
                    "gcp.cloudsql_postgresql.database.cpu.usage_time.sec",
                )?;
            }

            if event.has("gcp.metrics.database.cpu.utilization.value") {
                event.rename(
                    "gcp.metrics.database.cpu.utilization.value",
                    "gcp.cloudsql_postgresql.database.cpu.utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.disk.bytes_used.value") {
                event.rename(
                    "gcp.metrics.database.disk.bytes_used.value",
                    "gcp.cloudsql_postgresql.database.disk.bytes_used.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.disk.quota.value") {
                event.rename(
                    "gcp.metrics.database.disk.quota.value",
                    "gcp.cloudsql_postgresql.database.disk.quota.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.disk.read_ops_count.value") {
                event.rename(
                    "gcp.metrics.database.disk.read_ops_count.value",
                    "gcp.cloudsql_postgresql.database.disk.read_ops.count",
                )?;
            }

            if event.has("gcp.metrics.database.disk.utilization.value") {
                event.rename(
                    "gcp.metrics.database.disk.utilization.value",
                    "gcp.cloudsql_postgresql.database.disk.utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.disk.write_ops_count.value") {
                event.rename(
                    "gcp.metrics.database.disk.write_ops_count.value",
                    "gcp.cloudsql_postgresql.database.disk.write_ops.count",
                )?;
            }

            if event.has("gcp.metrics.database.instance_state.value") {
                event.rename(
                    "gcp.metrics.database.instance_state.value",
                    "gcp.cloudsql_postgresql.database.instance_state",
                )?;
            }

            if event.has("gcp.metrics.database.memory.quota.value") {
                event.rename(
                    "gcp.metrics.database.memory.quota.value",
                    "gcp.cloudsql_postgresql.database.memory.quota.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.memory.total_usage.value") {
                event.rename(
                    "gcp.metrics.database.memory.total_usage.value",
                    "gcp.cloudsql_postgresql.database.memory.total_usage.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.memory.usage.value") {
                event.rename(
                    "gcp.metrics.database.memory.usage.value",
                    "gcp.cloudsql_postgresql.database.memory.usage.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.memory.utilization.value") {
                event.rename(
                    "gcp.metrics.database.memory.utilization.value",
                    "gcp.cloudsql_postgresql.database.memory.utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.network.connections.value") {
                event.rename(
                    "gcp.metrics.database.network.connections.value",
                    "gcp.cloudsql_postgresql.database.network.connections.count",
                )?;
            }

            if event.has("gcp.metrics.database.network.received_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.network.received_bytes_count.value",
                    "gcp.cloudsql_postgresql.database.network.received_bytes.count",
                )?;
            }

            if event.has("gcp.metrics.database.network.sent_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.network.sent_bytes_count.value",
                    "gcp.cloudsql_postgresql.database.network.sent_bytes.count",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.aggregate.execution_time.value")
            {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.aggregate.execution_time.value",
                    "gcp.cloudsql_postgresql.database.insights.aggregate.execution_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.aggregate.io_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.aggregate.io_time.value",
                    "gcp.cloudsql_postgresql.database.insights.aggregate.io_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.aggregate.latencies.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.aggregate.latencies.value",
                    "gcp.cloudsql_postgresql.database.insights.aggregate.latencies",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.aggregate.lock_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.aggregate.lock_time.value",
                    "gcp.cloudsql_postgresql.database.insights.aggregate.lock_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.aggregate.row_count.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.aggregate.row_count.value",
                    "gcp.cloudsql_postgresql.database.insights.aggregate.row.count",
                )?;
            }

            if event.has(
                "gcp.metrics.database.postgresql.insights.aggregate.shared_blk_access_count.value",
            ) {
                event.rename("gcp.metrics.database.postgresql.insights.aggregate.shared_blk_access_count.value", "gcp.cloudsql_postgresql.database.insights.aggregate.shared_blk_access.count")?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.perquery.execution_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.perquery.execution_time.value",
                    "gcp.cloudsql_postgresql.database.insights.perquery.execution_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.perquery.io_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.perquery.io_time.value",
                    "gcp.cloudsql_postgresql.database.insights.perquery.io_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.perquery.latencies.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.perquery.latencies.value",
                    "gcp.cloudsql_postgresql.database.insights.perquery.latencies",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.perquery.lock_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.perquery.lock_time.value",
                    "gcp.cloudsql_postgresql.database.insights.perquery.lock_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.perquery.row_count.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.perquery.row_count.value",
                    "gcp.cloudsql_postgresql.database.insights.perquery.row.count",
                )?;
            }

            if event.has(
                "gcp.metrics.database.postgresql.insights.perquery.shared_blk_access_count.value",
            ) {
                event.rename("gcp.metrics.database.postgresql.insights.perquery.shared_blk_access_count.value", "gcp.cloudsql_postgresql.database.insights.perquery.shared_blk_access.count")?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.pertag.execution_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.pertag.execution_time.value",
                    "gcp.cloudsql_postgresql.database.insights.pertag.execution_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.pertag.io_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.pertag.io_time.value",
                    "gcp.cloudsql_postgresql.database.insights.pertag.io_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.pertag.latencies.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.pertag.latencies.value",
                    "gcp.cloudsql_postgresql.database.insights.pertag.latencies",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.pertag.lock_time.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.pertag.lock_time.value",
                    "gcp.cloudsql_postgresql.database.insights.pertag.lock_time",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.insights.pertag.row_count.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.pertag.row_count.value",
                    "gcp.cloudsql_postgresql.database.insights.pertag.row.count",
                )?;
            }

            if event.has(
                "gcp.metrics.database.postgresql.insights.pertag.shared_blk_access_count.value",
            ) {
                event.rename(
                    "gcp.metrics.database.postgresql.insights.pertag.shared_blk_access_count.value",
                    "gcp.cloudsql_postgresql.database.insights.pertag.shared_blk_access.count",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.num_backends.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.num_backends.value",
                    "gcp.cloudsql_postgresql.database.num_backends.count",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.replication.replica_byte_lag.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.replication.replica_byte_lag.value",
                    "gcp.cloudsql_postgresql.database.replication.replica_byte_lag.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.transaction_count.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.transaction_count.value",
                    "gcp.cloudsql_postgresql.database.transaction.count",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.transaction_id_count.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.transaction_id_count.value",
                    "gcp.cloudsql_postgresql.database.transaction_id.count",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.transaction_id_utilization.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.transaction_id_utilization.value",
                    "gcp.cloudsql_postgresql.database.transaction_id_utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.postgresql.vacuum.oldest_transaction_age.value") {
                event.rename(
                    "gcp.metrics.database.postgresql.vacuum.oldest_transaction_age.value",
                    "gcp.cloudsql_postgresql.database.vacuum.oldest_transaction_age",
                )?;
            }

            if event.has("gcp.metrics.database.replication.network_lag.value") {
                event.rename(
                    "gcp.metrics.database.replication.network_lag.value",
                    "gcp.cloudsql_postgresql.database.replication.network_lag.sec",
                )?;
            }

            if event.has("gcp.metrics.database.replication.replica_lag.value") {
                event.rename(
                    "gcp.metrics.database.replication.replica_lag.value",
                    "gcp.cloudsql_postgresql.database.replication.replica_lag.sec",
                )?;
            }

            if event.has("gcp.metrics.database.up.value") {
                event.rename(
                    "gcp.metrics.database.up.value",
                    "gcp.cloudsql_postgresql.database.up",
                )?;
            }

            if event.has("gcp.metrics.database.uptime.value") {
                event.rename(
                    "gcp.metrics.database.uptime.value",
                    "gcp.cloudsql_postgresql.database.uptime.sec",
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
