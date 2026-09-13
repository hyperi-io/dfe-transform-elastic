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

            let _cond = { event.get_str("gcp.labels.cloudsql.name") != Some("sqlserver") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has_value("gcp.metrics.database.auto_failover_request_count.value") {
                event.rename(
                    "gcp.metrics.database.auto_failover_request_count.value",
                    "gcp.cloudsql_sqlserver.database.auto_failover_request.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.available_for_failover.value") {
                event.rename(
                    "gcp.metrics.database.available_for_failover.value",
                    "gcp.cloudsql_sqlserver.database.available_for_failover",
                )?;
            }

            if event.has_value("gcp.metrics.database.cpu.reserved_cores.value") {
                event.rename(
                    "gcp.metrics.database.cpu.reserved_cores.value",
                    "gcp.cloudsql_sqlserver.database.cpu.reserved_cores.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.cpu.usage_time.value") {
                event.rename(
                    "gcp.metrics.database.cpu.usage_time.value",
                    "gcp.cloudsql_sqlserver.database.cpu.usage_time.sec",
                )?;
            }

            if event.has_value("gcp.metrics.database.cpu.utilization.value") {
                event.rename(
                    "gcp.metrics.database.cpu.utilization.value",
                    "gcp.cloudsql_sqlserver.database.cpu.utilization.pct",
                )?;
            }

            if event.has_value("gcp.metrics.database.disk.bytes_used.value") {
                event.rename(
                    "gcp.metrics.database.disk.bytes_used.value",
                    "gcp.cloudsql_sqlserver.database.disk.bytes_used.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.database.disk.quota.value") {
                event.rename(
                    "gcp.metrics.database.disk.quota.value",
                    "gcp.cloudsql_sqlserver.database.disk.quota.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.database.disk.read_ops_count.value") {
                event.rename(
                    "gcp.metrics.database.disk.read_ops_count.value",
                    "gcp.cloudsql_sqlserver.database.disk.read_ops.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.disk.utilization.value") {
                event.rename(
                    "gcp.metrics.database.disk.utilization.value",
                    "gcp.cloudsql_sqlserver.database.disk.utilization.pct",
                )?;
            }

            if event.has_value("gcp.metrics.database.disk.write_ops_count.value") {
                event.rename(
                    "gcp.metrics.database.disk.write_ops_count.value",
                    "gcp.cloudsql_sqlserver.database.disk.write_ops.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.instance_state.value") {
                event.rename(
                    "gcp.metrics.database.instance_state.value",
                    "gcp.cloudsql_sqlserver.database.instance_state",
                )?;
            }

            if event.has_value("gcp.metrics.database.memory.quota.value") {
                event.rename(
                    "gcp.metrics.database.memory.quota.value",
                    "gcp.cloudsql_sqlserver.database.memory.quota.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.database.memory.total_usage.value") {
                event.rename(
                    "gcp.metrics.database.memory.total_usage.value",
                    "gcp.cloudsql_sqlserver.database.memory.total_usage.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.database.memory.usage.value") {
                event.rename(
                    "gcp.metrics.database.memory.usage.value",
                    "gcp.cloudsql_sqlserver.database.memory.usage.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.database.memory.utilization.value") {
                event.rename(
                    "gcp.metrics.database.memory.utilization.value",
                    "gcp.cloudsql_sqlserver.database.memory.utilization.pct",
                )?;
            }

            if event.has_value("gcp.metrics.database.network.connections.value") {
                event.rename(
                    "gcp.metrics.database.network.connections.value",
                    "gcp.cloudsql_sqlserver.database.network.connections.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.network.received_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.network.received_bytes_count.value",
                    "gcp.cloudsql_sqlserver.database.network.received_bytes.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.network.sent_bytes_count.value") {
                event.rename(
                    "gcp.metrics.database.network.sent_bytes_count.value",
                    "gcp.cloudsql_sqlserver.database.network.sent_bytes.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.replication.network_lag.value") {
                event.rename(
                    "gcp.metrics.database.replication.network_lag.value",
                    "gcp.cloudsql_sqlserver.database.replication.network_lag.sec",
                )?;
            }

            if event.has_value("gcp.metrics.database.replication.replica_lag.value") {
                event.rename(
                    "gcp.metrics.database.replication.replica_lag.value",
                    "gcp.cloudsql_sqlserver.database.replication.replica_lag.sec",
                )?;
            }

            if event.has_value("gcp.metrics.database.sqlserver.audits_size.value") {
                event.rename(
                    "gcp.metrics.database.sqlserver.audits_size.value",
                    "gcp.cloudsql_sqlserver.database.audits_size.bytes",
                )?;
            }

            if event.has_value("gcp.metrics.database.sqlserver.audits_upload_count.value") {
                event.rename(
                    "gcp.metrics.database.sqlserver.audits_upload_count.value",
                    "gcp.cloudsql_sqlserver.database.audits_upload.count",
                )?;
            }

            if event.has_value("gcp.metrics.database.up.value") {
                event.rename(
                    "gcp.metrics.database.up.value",
                    "gcp.cloudsql_sqlserver.database.up",
                )?;
            }

            if event.has_value("gcp.metrics.database.uptime.value") {
                event.rename(
                    "gcp.metrics.database.uptime.value",
                    "gcp.cloudsql_sqlserver.database.uptime.sec",
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

        Ok(TransformResult::Continue)
    }
}
