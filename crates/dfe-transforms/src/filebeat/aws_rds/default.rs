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

            // Painless script
            // Source: if(ctx.agent?.type == \"firehose\" && ctx.aws?.rds?.metrics?.CPUUtilization?.avg != null && ctx.aws?.rds?.cpu?.total?.pct == null) {\n    ctx.aws.rds.metrics.CPUUtilization.avg = ctx.aws.rds.metrics.CPUUtilization.avg / 100;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if(ctx.agent?.type == \"firehose\" && ctx.aws?.rds?.metrics?.CPUUtilization?.avg != null && ctx.aws?.rds?.cpu?.total?.pct == null) {\n    ctx.aws.rds.metrics.CPUUtilization.avg = ctx.aws.rds.metrics.CPUUtilization.avg / 100;\n}\n"#
                ),
            )?;

            if event.has("aws.rds.metrics.BurstBalance.avg") {
                event.rename(
                    "aws.rds.metrics.BurstBalance.avg",
                    "aws.rds.burst_balance.pct",
                )?;
            }

            if event.has("aws.rds.metrics.CPUUtilization.avg") {
                event.rename(
                    "aws.rds.metrics.CPUUtilization.avg",
                    "aws.rds.cpu.total.pct",
                )?;
            }

            if event.has("aws.rds.metrics.CPUCreditUsage.avg") {
                event.rename(
                    "aws.rds.metrics.CPUCreditUsage.avg",
                    "aws.rds.cpu.credit_usage",
                )?;
            }

            if event.has("aws.rds.metrics.CPUCreditBalance.avg") {
                event.rename(
                    "aws.rds.metrics.CPUCreditBalance.avg",
                    "aws.rds.cpu.credit_balance",
                )?;
            }

            if event.has("aws.rds.metrics.DatabaseConnections.avg") {
                event.rename(
                    "aws.rds.metrics.DatabaseConnections.avg",
                    "aws.rds.database_connections",
                )?;
            }

            if event.has("aws.rds.metrics.DiskQueueDepth.avg") {
                event.rename(
                    "aws.rds.metrics.DiskQueueDepth.avg",
                    "aws.rds.disk_queue_depth",
                )?;
            }

            if event.has("aws.rds.metrics.FailedSQLServerAgentJobsCount.avg") {
                event.rename(
                    "aws.rds.metrics.FailedSQLServerAgentJobsCount.avg",
                    "aws.rds.failed_sql_server_agent_jobs",
                )?;
            }

            if event.has("aws.rds.metrics.FreeableMemory.avg") {
                event.rename(
                    "aws.rds.metrics.FreeableMemory.avg",
                    "aws.rds.freeable_memory.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.FreeStorageSpace.avg") {
                event.rename(
                    "aws.rds.metrics.FreeStorageSpace.avg",
                    "aws.rds.free_storage.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.MaximumUsedTransactionIDs.avg") {
                event.rename(
                    "aws.rds.metrics.MaximumUsedTransactionIDs.avg",
                    "aws.rds.maximum_used_transaction_ids",
                )?;
            }

            if event.has("aws.rds.metrics.OldestReplicationSlotLag.avg") {
                event.rename(
                    "aws.rds.metrics.OldestReplicationSlotLag.avg",
                    "aws.rds.oldest_replication_slot_lag.mb",
                )?;
            }

            if event.has("aws.rds.metrics.ReadIOPS.avg") {
                event.rename("aws.rds.metrics.ReadIOPS.avg", "aws.rds.read.iops")?;
            }

            if event.has("aws.rds.metrics.CommitThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.CommitThroughput.avg",
                    "aws.rds.throughput.commit",
                )?;
            }

            if event.has("aws.rds.metrics.DeleteThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.DeleteThroughput.avg",
                    "aws.rds.throughput.delete",
                )?;
            }

            if event.has("aws.rds.metrics.DDLThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.DDLThroughput.avg",
                    "aws.rds.throughput.ddl",
                )?;
            }

            if event.has("aws.rds.metrics.DMLThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.DMLThroughput.avg",
                    "aws.rds.throughput.dml",
                )?;
            }

            if event.has("aws.rds.metrics.InsertThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.InsertThroughput.avg",
                    "aws.rds.throughput.insert",
                )?;
            }

            if event.has("aws.rds.metrics.NetworkThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.NetworkThroughput.avg",
                    "aws.rds.throughput.network",
                )?;
            }

            if event.has("aws.rds.metrics.NetworkReceiveThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.NetworkReceiveThroughput.avg",
                    "aws.rds.throughput.network_receive",
                )?;
            }

            if event.has("aws.rds.metrics.NetworkTransmitThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.NetworkTransmitThroughput.avg",
                    "aws.rds.throughput.network_transmit",
                )?;
            }

            if event.has("aws.rds.metrics.ReadThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.ReadThroughput.avg",
                    "aws.rds.throughput.read",
                )?;
            }

            if event.has("aws.rds.metrics.SelectThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.SelectThroughput.avg",
                    "aws.rds.throughput.select",
                )?;
            }

            if event.has("aws.rds.metrics.UpdateThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.UpdateThroughput.avg",
                    "aws.rds.throughput.update",
                )?;
            }

            if event.has("aws.rds.metrics.WriteThroughput.avg") {
                event.rename(
                    "aws.rds.metrics.WriteThroughput.avg",
                    "aws.rds.throughput.write",
                )?;
            }

            if event.has("aws.rds.metrics.CommitLatency.avg") {
                event.rename(
                    "aws.rds.metrics.CommitLatency.avg",
                    "aws.rds.latency.commit",
                )?;
            }

            if event.has("aws.rds.metrics.DDLLatency.avg") {
                event.rename("aws.rds.metrics.DDLLatency.avg", "aws.rds.latency.ddl")?;
            }

            if event.has("aws.rds.metrics.DMLLatency.avg") {
                event.rename("aws.rds.metrics.DMLLatency.avg", "aws.rds.latency.dml")?;
            }

            if event.has("aws.rds.metrics.InsertLatency.avg") {
                event.rename(
                    "aws.rds.metrics.InsertLatency.avg",
                    "aws.rds.latency.insert",
                )?;
            }

            if event.has("aws.rds.metrics.ReadLatency.avg") {
                event.rename("aws.rds.metrics.ReadLatency.avg", "aws.rds.latency.read")?;
            }

            if event.has("aws.rds.metrics.SelectLatency.avg") {
                event.rename(
                    "aws.rds.metrics.SelectLatency.avg",
                    "aws.rds.latency.select",
                )?;
            }

            if event.has("aws.rds.metrics.UpdateLatency.avg") {
                event.rename(
                    "aws.rds.metrics.UpdateLatency.avg",
                    "aws.rds.latency.update",
                )?;
            }

            if event.has("aws.rds.metrics.WriteLatency.avg") {
                event.rename("aws.rds.metrics.WriteLatency.avg", "aws.rds.latency.write")?;
            }

            if event.has("aws.rds.metrics.DeleteLatency.avg") {
                event.rename(
                    "aws.rds.metrics.DeleteLatency.avg",
                    "aws.rds.latency.delete",
                )?;
            }

            if event.has("aws.rds.metrics.ReplicaLag.avg") {
                event.rename("aws.rds.metrics.ReplicaLag.avg", "aws.rds.replica_lag.sec")?;
            }

            if event.has("aws.rds.metrics.BinLogDiskUsage.avg") {
                event.rename(
                    "aws.rds.metrics.BinLogDiskUsage.avg",
                    "aws.rds.disk_usage.bin_log.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.ReplicationSlotDiskUsage.avg") {
                event.rename(
                    "aws.rds.metrics.ReplicationSlotDiskUsage.avg",
                    "aws.rds.disk_usage.replication_slot.mb",
                )?;
            }

            if event.has("aws.rds.metrics.TransactionLogsDiskUsage.avg") {
                event.rename(
                    "aws.rds.metrics.TransactionLogsDiskUsage.avg",
                    "aws.rds.disk_usage.transaction_logs.mb",
                )?;
            }

            if event.has("aws.rds.metrics.SwapUsage.avg") {
                event.rename("aws.rds.metrics.SwapUsage.avg", "aws.rds.swap_usage.bytes")?;
            }

            if event.has("aws.rds.metrics.TransactionLogsGeneration.avg") {
                event.rename(
                    "aws.rds.metrics.TransactionLogsGeneration.avg",
                    "aws.rds.transaction_logs_generation",
                )?;
            }

            if event.has("aws.rds.metrics.WriteIOPS.avg") {
                event.rename("aws.rds.metrics.WriteIOPS.avg", "aws.rds.write.iops")?;
            }

            if event.has("aws.rds.metrics.Queries.avg") {
                event.rename("aws.rds.metrics.Queries.avg", "aws.rds.queries")?;
            }

            if event.has("aws.rds.metrics.Deadlocks.avg") {
                event.rename("aws.rds.metrics.Deadlocks.avg", "aws.rds.deadlocks")?;
            }

            if event.has("aws.rds.metrics.VolumeBytesUsed.avg") {
                event.rename(
                    "aws.rds.metrics.VolumeBytesUsed.avg",
                    "aws.rds.volume_used.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.FreeLocalStorage.avg") {
                event.rename(
                    "aws.rds.metrics.FreeLocalStorage.avg",
                    "aws.rds.free_local_storage.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.ActiveTransactions.avg") {
                event.rename(
                    "aws.rds.metrics.ActiveTransactions.avg",
                    "aws.rds.transactions.active",
                )?;
            }

            if event.has("aws.rds.metrics.BlockedTransactions.avg") {
                event.rename(
                    "aws.rds.metrics.BlockedTransactions.avg",
                    "aws.rds.transactions.blocked",
                )?;
            }

            if event.has("aws.rds.metrics.LoginFailures.avg") {
                event.rename(
                    "aws.rds.metrics.LoginFailures.avg",
                    "aws.rds.login_failures",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraBinlogReplicaLag.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraBinlogReplicaLag.avg",
                    "aws.rds.aurora_bin_log_replica_lag",
                )?;
            }

            if event.has("aws.rds.metrics.aurora_bin_log_replica_lag.avg") {
                event.rename(
                    "aws.rds.metrics.aurora_bin_log_replica_lag.avg",
                    "aws.rds.aurora_global_db.replicated_write_io.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraGlobalDBDataTransferBytes.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraGlobalDBDataTransferBytes.avg",
                    "aws.rds.aurora_global_db.data_transfer.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraGlobalDBReplicationLag.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraGlobalDBReplicationLag.avg",
                    "aws.rds.aurora_global_db.replication_lag.ms",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraReplicaLag.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraReplicaLag.avg",
                    "aws.rds.aurora_replica.lag.ms",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraReplicaLagMaximum.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraReplicaLagMaximum.avg",
                    "aws.rds.aurora_replica.lag_max.ms",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraReplicaLagMinimum.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraReplicaLagMinimum.avg",
                    "aws.rds.aurora_replica.lag_min.ms",
                )?;
            }

            if event.has("aws.rds.metrics.BacktrackChangeRecordsCreationRate.avg") {
                event.rename(
                    "aws.rds.metrics.BacktrackChangeRecordsCreationRate.avg",
                    "aws.rds.backtrack_change_records.creation_rate",
                )?;
            }

            if event.has("aws.rds.metrics.BacktrackChangeRecordsStored.avg") {
                event.rename(
                    "aws.rds.metrics.BacktrackChangeRecordsStored.avg",
                    "aws.rds.backtrack_change_records.stored",
                )?;
            }

            if event.has("aws.rds.metrics.BacktrackWindowActual.avg") {
                event.rename(
                    "aws.rds.metrics.BacktrackWindowActual.avg",
                    "aws.rds.backtrack_window.actual",
                )?;
            }

            if event.has("aws.rds.metrics.BacktrackWindowAlert.avg") {
                event.rename(
                    "aws.rds.metrics.BacktrackWindowAlert.avg",
                    "aws.rds.backtrack_window.alert",
                )?;
            }

            if event.has("aws.rds.metrics.BackupRetentionPeriodStorageUsed.avg") {
                event.rename(
                    "aws.rds.metrics.BackupRetentionPeriodStorageUsed.avg",
                    "aws.rds.storage_used.backup_retention_period.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.SnapshotStorageUsed.avg") {
                event.rename(
                    "aws.rds.metrics.SnapshotStorageUsed.avg",
                    "aws.rds.storage_used.snapshot.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.BufferCacheHitRatio.avg") {
                event.rename(
                    "aws.rds.metrics.BufferCacheHitRatio.avg",
                    "aws.rds.cache_hit_ratio.buffer",
                )?;
            }

            if event.has("aws.rds.metrics.ResultSetCacheHitRatio.avg") {
                event.rename(
                    "aws.rds.metrics.ResultSetCacheHitRatio.avg",
                    "aws.rds.cache_hit_ratio.result_set",
                )?;
            }

            if event.has("aws.rds.metrics.EngineUptime.avg") {
                event.rename(
                    "aws.rds.metrics.EngineUptime.avg",
                    "aws.rds.engine_uptime.sec",
                )?;
            }

            if event.has("aws.rds.metrics.VolumeReadIOPs.avg") {
                event.rename(
                    "aws.rds.metrics.VolumeReadIOPs.avg",
                    "aws.rds.volume.read.iops",
                )?;
            }

            if event.has("aws.rds.metrics.VolumeWriteIOPs.avg") {
                event.rename(
                    "aws.rds.metrics.VolumeWriteIOPs.avg",
                    "aws.rds.volume.write.iops",
                )?;
            }

            if event.has("aws.rds.metrics.RDSToAuroraPostgreSQLReplicaLag.avg") {
                event.rename(
                    "aws.rds.metrics.RDSToAuroraPostgreSQLReplicaLag.avg",
                    "aws.rds.rds_to_aurora_postgresql_replica_lag.sec",
                )?;
            }

            if event.has("aws.rds.metrics.TotalBackupStorageBilled.avg") {
                event.rename(
                    "aws.rds.metrics.TotalBackupStorageBilled.avg",
                    "aws.rds.backup_storage_billed_total.bytes",
                )?;
            }

            if event.has("aws.rds.metrics.AuroraVolumeBytesLeftTotal.avg") {
                event.rename(
                    "aws.rds.metrics.AuroraVolumeBytesLeftTotal.avg",
                    "aws.rds.aurora_volume_left_total.bytes",
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
