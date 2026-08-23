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

            let _cond = { event.get_str("gcp.labels.cloudsql.name") != Some("mysql") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has("gcp.metrics.database.auto_failover_request_count.value") {
                event.rename(
                    "gcp.metrics.database.auto_failover_request_count.value",
                    "gcp.cloudsql_mysql.database.auto_failover_request.count",
                )?;
            }

            if event.has("gcp.metrics.database.available_for_failover.value") {
                event.rename(
                    "gcp.metrics.database.available_for_failover.value",
                    "gcp.cloudsql_mysql.database.available_for_failover",
                )?;
            }

            if event.has("gcp.metrics.database.cpu.reserved_cores.value") {
                event.rename(
                    "gcp.metrics.database.cpu.reserved_cores.value",
                    "gcp.cloudsql_mysql.database.cpu.reserved_cores.count",
                )?;
            }

            if event.has("gcp.metrics.database.cpu.usage_time.value") {
                event.rename(
                    "gcp.metrics.database.cpu.usage_time.value",
                    "gcp.cloudsql_mysql.database.cpu.usage_time.sec",
                )?;
            }

            if event.has("gcp.metrics.database.cpu.utilization.value") {
                event.rename(
                    "gcp.metrics.database.cpu.utilization.value",
                    "gcp.cloudsql_mysql.database.cpu.utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.disk.bytes_used.value") {
                event.rename(
                    "gcp.metrics.database.disk.bytes_used.value",
                    "gcp.cloudsql_mysql.database.disk.bytes_used.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.disk.quota.value") {
                event.rename(
                    "gcp.metrics.database.disk.quota.value",
                    "gcp.cloudsql_mysql.database.disk.quota.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.disk.read_ops_count.value") {
                event.rename(
                    "gcp.metrics.database.disk.read_ops_count.value",
                    "gcp.cloudsql_mysql.database.disk.read_ops.count",
                )?;
            }

            if event.has("gcp.metrics.database.disk.utilization.value") {
                event.rename(
                    "gcp.metrics.database.disk.utilization.value",
                    "gcp.cloudsql_mysql.database.disk.utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.disk.write_ops_count.value") {
                event.rename(
                    "gcp.metrics.database.disk.write_ops_count.value",
                    "gcp.cloudsql_mysql.database.disk.write_ops.count",
                )?;
            }

            if event.has("gcp.metrics.database.instance_state.value") {
                event.rename(
                    "gcp.metrics.database.instance_state.value",
                    "gcp.cloudsql_mysql.database.instance_state",
                )?;
            }

            if event.has("gcp.metrics.database.memory.quota.value") {
                event.rename(
                    "gcp.metrics.database.memory.quota.value",
                    "gcp.cloudsql_mysql.database.memory.quota.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.memory.total_usage.value") {
                event.rename(
                    "gcp.metrics.database.memory.total_usage.value",
                    "gcp.cloudsql_mysql.database.memory.total_usage.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.memory.usage.value") {
                event.rename(
                    "gcp.metrics.database.memory.usage.value",
                    "gcp.cloudsql_mysql.database.memory.usage.bytes",
                )?;
            }

            if event.has("gcp.metrics.database.memory.utilization.value") {
                event.rename(
                    "gcp.metrics.database.memory.utilization.value",
                    "gcp.cloudsql_mysql.database.memory.utilization.pct",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_buffer_pool_pages_dirty.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_buffer_pool_pages_dirty.value",
                    "gcp.cloudsql_mysql.database.innodb_buffer_pool_pages_dirty.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_buffer_pool_pages_free.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_buffer_pool_pages_free.value",
                    "gcp.cloudsql_mysql.database.innodb_buffer_pool_pages_free.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_buffer_pool_pages_total.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_buffer_pool_pages_total.value",
                    "gcp.cloudsql_mysql.database.innodb_buffer_pool_pages_total.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_data_fsyncs.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_data_fsyncs.value",
                    "gcp.cloudsql_mysql.database.innodb_data_fsyncs.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_os_log_fsyncs.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_os_log_fsyncs.value",
                    "gcp.cloudsql_mysql.database.innodb_os_log_fsyncs.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_pages_read.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_pages_read.value",
                    "gcp.cloudsql_mysql.database.innodb_pages_read.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.innodb_pages_written.value") {
                event.rename(
                    "gcp.metrics.database.mysql.innodb_pages_written.value",
                    "gcp.cloudsql_mysql.database.innodb_pages_written.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.queries.value") {
                event.rename(
                    "gcp.metrics.database.mysql.queries.value",
                    "gcp.cloudsql_mysql.database.queries.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.questions.value") {
                event.rename(
                    "gcp.metrics.database.mysql.questions.value",
                    "gcp.cloudsql_mysql.database.questions.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.received_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.mysql.received_bytes_count.value",
                    "gcp.cloudsql_mysql.database.received_bytes.count",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.last_io_errno.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.last_io_errno.value",
                    "gcp.cloudsql_mysql.database.replication.last_io_errno",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.last_sql_errno.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.last_sql_errno.value",
                    "gcp.cloudsql_mysql.database.replication.last_sql_errno",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.seconds_behind_master.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.seconds_behind_master.value",
                    "gcp.cloudsql_mysql.database.replication.seconds_behind_master.sec",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.slave_io_running.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.slave_io_running.value",
                    "gcp.cloudsql_mysql.database.replication.slave_io_running",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.slave_io_running_state.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.slave_io_running_state.value",
                    "gcp.cloudsql_mysql.database.replication.slave_io_running_state",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.slave_sql_running.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.slave_sql_running.value",
                    "gcp.cloudsql_mysql.database.replication.slave_sql_running",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.replication.slave_sql_running_state.value") {
                event.rename(
                    "gcp.metrics.database.mysql.replication.slave_sql_running_state.value",
                    "gcp.cloudsql_mysql.database.replication.slave_sql_running_state",
                )?;
            }

            if event.has("gcp.metrics.database.mysql.sent_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.mysql.sent_bytes_count.value",
                    "gcp.cloudsql_mysql.database.sent_bytes.count",
                )?;
            }

            if event.has("gcp.metrics.database.network.connections.value") {
                event.rename(
                    "gcp.metrics.database.network.connections.value",
                    "gcp.cloudsql_mysql.database.network.connections.count",
                )?;
            }

            if event.has("gcp.metrics.database.network.received_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.network.received_bytes_count.value",
                    "gcp.cloudsql_mysql.database.network.received_bytes.count",
                )?;
            }

            if event.has("gcp.metrics.database.network.sent_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.network.sent_bytes_count.value",
                    "gcp.cloudsql_mysql.database.network.sent_bytes.count",
                )?;
            }

            if event.has("gcp.metrics.database.replication.network_lag.value") {
                event.rename(
                    "gcp.metrics.database.replication.network_lag.value",
                    "gcp.cloudsql_mysql.database.replication.network_lag.sec",
                )?;
            }

            if event.has("gcp.metrics.database.replication.replica_lag.value") {
                event.rename(
                    "gcp.metrics.database.replication.replica_lag.value",
                    "gcp.cloudsql_mysql.database.replication.replica_lag.sec",
                )?;
            }

            if event.has("gcp.metrics.database.up.value") {
                event.rename(
                    "gcp.metrics.database.up.value",
                    "gcp.cloudsql_mysql.database.up",
                )?;
            }

            if event.has("gcp.metrics.database.uptime.value") {
                event.rename(
                    "gcp.metrics.database.uptime.value",
                    "gcp.cloudsql_mysql.database.uptime.sec",
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
