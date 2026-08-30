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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("mongodb_atlas"))?;

            event.set("event.category", Value::Array(vec![json!("process")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if event.has_value("groupId") {
                event.rename("groupId", "mongodb_atlas.group_id")?;
            }

            let v = json!(
                event
                    .get("mongodb_atlas.group_id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("group.id", v)?;
            }

            if event.has_value("response.ASSERT_MSG") {
                event.rename("response.ASSERT_MSG", "mongodb_atlas.process.assert.msg")?;
            }

            if event.has_value("response.ASSERT_REGULAR") {
                event.rename(
                    "response.ASSERT_REGULAR",
                    "mongodb_atlas.process.assert.regular",
                )?;
            }

            if event.has_value("response.ASSERT_USER") {
                event.rename("response.ASSERT_USER", "mongodb_atlas.process.assert.user")?;
            }

            if event.has_value("response.ASSERT_WARNING") {
                event.rename(
                    "response.ASSERT_WARNING",
                    "mongodb_atlas.process.assert.warning",
                )?;
            }

            if event.has_value("response.BACKGROUND_FLUSH_AVG") {
                event.rename(
                    "response.BACKGROUND_FLUSH_AVG",
                    "mongodb_atlas.process.background_flush.avg",
                )?;
            }

            if event.has_value("response.CACHE_DIRTY_BYTES") {
                event.rename(
                    "response.CACHE_DIRTY_BYTES",
                    "mongodb_atlas.process.cache.dirty.bytes",
                )?;
            }

            if event.has_value("response.CACHE_BYTES_READ_INTO") {
                event.rename(
                    "response.CACHE_BYTES_READ_INTO",
                    "mongodb_atlas.process.cache.read.bytes",
                )?;
            }

            if event.has_value("response.CACHE_USED_BYTES") {
                event.rename(
                    "response.CACHE_USED_BYTES",
                    "mongodb_atlas.process.cache.used.total.bytes",
                )?;
            }

            if event.has_value("response.CACHE_BYTES_WRITTEN_FROM") {
                event.rename(
                    "response.CACHE_BYTES_WRITTEN_FROM",
                    "mongodb_atlas.process.cache.write.bytes",
                )?;
            }

            if event.has_value("response.CONNECTIONS") {
                event.rename("response.CONNECTIONS", "mongodb_atlas.process.connections")?;
            }

            if event.has_value("response.MAX_PROCESS_NORMALIZED_CPU_CHILDREN_KERNEL") {
                event.rename(
                    "response.MAX_PROCESS_NORMALIZED_CPU_CHILDREN_KERNEL",
                    "mongodb_atlas.process.cpu.normalized.children.kernel.max.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_CPU_CHILDREN_KERNEL") {
                event.rename(
                    "response.MAX_PROCESS_CPU_CHILDREN_KERNEL",
                    "mongodb_atlas.process.cpu.children.kernel.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_CPU_CHILDREN_KERNEL") {
                event.rename(
                    "response.PROCESS_CPU_CHILDREN_KERNEL",
                    "mongodb_atlas.process.cpu.children.kernel.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_CPU_CHILDREN_USER") {
                event.rename(
                    "response.MAX_PROCESS_CPU_CHILDREN_USER",
                    "mongodb_atlas.process.cpu.children.user.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_CPU_CHILDREN_USER") {
                event.rename(
                    "response.PROCESS_CPU_CHILDREN_USER",
                    "mongodb_atlas.process.cpu.children.user.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_CPU_KERNEL") {
                event.rename(
                    "response.MAX_PROCESS_CPU_KERNEL",
                    "mongodb_atlas.process.cpu.kernel.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_CPU_KERNEL") {
                event.rename(
                    "response.PROCESS_CPU_KERNEL",
                    "mongodb_atlas.process.cpu.kernel.pct",
                )?;
            }

            if event.has_value("response.PROCESS_NORMALIZED_CPU_CHILDREN_KERNEL") {
                event.rename(
                    "response.PROCESS_NORMALIZED_CPU_CHILDREN_KERNEL",
                    "mongodb_atlas.process.cpu.normalized.children.kernel.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_NORMALIZED_CPU_CHILDREN_USER") {
                event.rename(
                    "response.MAX_PROCESS_NORMALIZED_CPU_CHILDREN_USER",
                    "mongodb_atlas.process.cpu.normalized.children.user.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_NORMALIZED_CPU_CHILDREN_USER") {
                event.rename(
                    "response.PROCESS_NORMALIZED_CPU_CHILDREN_USER",
                    "mongodb_atlas.process.cpu.normalized.children.user.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_NORMALIZED_CPU_KERNEL") {
                event.rename(
                    "response.MAX_PROCESS_NORMALIZED_CPU_KERNEL",
                    "mongodb_atlas.process.cpu.normalized.kernel.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_NORMALIZED_CPU_KERNEL") {
                event.rename(
                    "response.PROCESS_NORMALIZED_CPU_KERNEL",
                    "mongodb_atlas.process.cpu.normalized.kernel.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_NORMALIZED_CPU_USER") {
                event.rename(
                    "response.MAX_PROCESS_NORMALIZED_CPU_USER",
                    "mongodb_atlas.process.cpu.normalized.user.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_NORMALIZED_CPU_USER") {
                event.rename(
                    "response.PROCESS_NORMALIZED_CPU_USER",
                    "mongodb_atlas.process.cpu.normalized.user.pct",
                )?;
            }

            if event.has_value("response.MAX_PROCESS_CPU_USER") {
                event.rename(
                    "response.MAX_PROCESS_CPU_USER",
                    "mongodb_atlas.process.cpu.user.max.pct",
                )?;
            }

            if event.has_value("response.PROCESS_CPU_USER") {
                event.rename(
                    "response.PROCESS_CPU_USER",
                    "mongodb_atlas.process.cpu.user.pct",
                )?;
            }

            if event.has_value("response.CURSORS_TOTAL_OPEN") {
                event.rename(
                    "response.CURSORS_TOTAL_OPEN",
                    "mongodb_atlas.process.cursor.open.total",
                )?;
            }

            if event.has_value("response.CURSORS_TOTAL_TIMED_OUT") {
                event.rename(
                    "response.CURSORS_TOTAL_TIMED_OUT",
                    "mongodb_atlas.process.cursor.timed_out.total",
                )?;
            }

            if event.has_value("response.DB_DATA_SIZE_TOTAL") {
                event.rename(
                    "response.DB_DATA_SIZE_TOTAL",
                    "mongodb_atlas.process.database.size.total.bytes",
                )?;
            }

            if event.has_value("response.DB_STORAGE_TOTAL") {
                event.rename(
                    "response.DB_STORAGE_TOTAL",
                    "mongodb_atlas.process.database.storage.total.bytes",
                )?;
            }

            if event.has_value("response.DOCUMENT_METRICS_DELETED") {
                event.rename(
                    "response.DOCUMENT_METRICS_DELETED",
                    "mongodb_atlas.process.document.deleted",
                )?;
            }

            if event.has_value("response.DOCUMENT_METRICS_INSERTED") {
                event.rename(
                    "response.DOCUMENT_METRICS_INSERTED",
                    "mongodb_atlas.process.document.inserted",
                )?;
            }

            if event.has_value("response.DOCUMENT_METRICS_RETURNED") {
                event.rename(
                    "response.DOCUMENT_METRICS_RETURNED",
                    "mongodb_atlas.process.document.returned",
                )?;
            }

            if event.has_value("response.DOCUMENT_METRICS_UPDATED") {
                event.rename(
                    "response.DOCUMENT_METRICS_UPDATED",
                    "mongodb_atlas.process.document.updated",
                )?;
            }

            if event.has_value("response.FTS_PROCESS_CPU_KERNEL") {
                event.rename(
                    "response.FTS_PROCESS_CPU_KERNEL",
                    "mongodb_atlas.process.fts.cpu.kernel.pct",
                )?;
            }

            if event.has_value("response.FTS_PROCESS_NORMALIZED_CPU_KERNEL") {
                event.rename(
                    "response.FTS_PROCESS_NORMALIZED_CPU_KERNEL",
                    "mongodb_atlas.process.fts.cpu.normalized.kernel.pct",
                )?;
            }

            if event.has_value("response.FTS_PROCESS_NORMALIZED_CPU_USER") {
                event.rename(
                    "response.FTS_PROCESS_NORMALIZED_CPU_USER",
                    "mongodb_atlas.process.fts.cpu.normalized.user.pct",
                )?;
            }

            if event.has_value("response.FTS_PROCESS_CPU_USER") {
                event.rename(
                    "response.FTS_PROCESS_CPU_USER",
                    "mongodb_atlas.process.fts.cpu.user.pct",
                )?;
            }

            if event.has_value("response.FTS_DISK_UTILIZATION") {
                event.rename(
                    "response.FTS_DISK_UTILIZATION",
                    "mongodb_atlas.process.fts.disk.utilization.total.bytes",
                )?;
            }

            if event.has_value("response.FTS_MEMORY_MAPPED") {
                event.rename(
                    "response.FTS_MEMORY_MAPPED",
                    "mongodb_atlas.process.fts.memory.mapped.total.bytes",
                )?;
            }

            if event.has_value("response.FTS_MEMORY_RESIDENT") {
                event.rename(
                    "response.FTS_MEMORY_RESIDENT",
                    "mongodb_atlas.process.fts.memory.resident.total.bytes",
                )?;
            }

            if event.has_value("response.FTS_MEMORY_VIRTUAL") {
                event.rename(
                    "response.FTS_MEMORY_VIRTUAL",
                    "mongodb_atlas.process.fts.memory.virtual.total.bytes",
                )?;
            }

            if event.has_value("response.GLOBAL_ACCESSES_NOT_IN_MEMORY") {
                event.rename(
                    "response.GLOBAL_ACCESSES_NOT_IN_MEMORY",
                    "mongodb_atlas.process.global.access.not_in_memory",
                )?;
            }

            if event.has_value("response.GLOBAL_LOCK_CURRENT_QUEUE_READERS") {
                event.rename(
                    "response.GLOBAL_LOCK_CURRENT_QUEUE_READERS",
                    "mongodb_atlas.process.global.lock.current_queue.reader.count",
                )?;
            }

            if event.has_value("response.GLOBAL_LOCK_CURRENT_QUEUE_TOTAL") {
                event.rename(
                    "response.GLOBAL_LOCK_CURRENT_QUEUE_TOTAL",
                    "mongodb_atlas.process.global.lock.current_queue.total",
                )?;
            }

            if event.has_value("response.GLOBAL_LOCK_CURRENT_QUEUE_WRITERS") {
                event.rename(
                    "response.GLOBAL_LOCK_CURRENT_QUEUE_WRITERS",
                    "mongodb_atlas.process.global.lock.current_queue.writer.count",
                )?;
            }

            if event.has_value("response.GLOBAL_PAGE_FAULT_EXCEPTIONS_THROWN") {
                event.rename(
                    "response.GLOBAL_PAGE_FAULT_EXCEPTIONS_THROWN",
                    "mongodb_atlas.process.global.page_fault.exception_thrown",
                )?;
            }

            if event.has_value("response.EXTRA_INFO_PAGE_FAULTS") {
                event.rename(
                    "response.EXTRA_INFO_PAGE_FAULTS",
                    "mongodb_atlas.process.host.page_faults",
                )?;
            }

            if event.has_value("response.INDEX_COUNTERS_BTREE_ACCESSES") {
                event.rename(
                    "response.INDEX_COUNTERS_BTREE_ACCESSES",
                    "mongodb_atlas.process.index.btree.access.count",
                )?;
            }

            if event.has_value("response.INDEX_COUNTERS_BTREE_HITS") {
                event.rename(
                    "response.INDEX_COUNTERS_BTREE_HITS",
                    "mongodb_atlas.process.index.btree.hits.count",
                )?;
            }

            if event.has_value("response.INDEX_COUNTERS_BTREE_MISS_RATIO") {
                event.rename(
                    "response.INDEX_COUNTERS_BTREE_MISS_RATIO",
                    "mongodb_atlas.process.index.btree.miss_ratio.count",
                )?;
            }

            if event.has_value("response.INDEX_COUNTERS_BTREE_MISSES") {
                event.rename(
                    "response.INDEX_COUNTERS_BTREE_MISSES",
                    "mongodb_atlas.process.index.btree.miss.count",
                )?;
            }

            if event.has_value("response.JOURNALING_COMMITS_IN_WRITE_LOCK") {
                event.rename(
                    "response.JOURNALING_COMMITS_IN_WRITE_LOCK",
                    "mongodb_atlas.process.journaling.commits.write_lock",
                )?;
            }

            if event.has_value("response.JOURNALING_MB") {
                event.rename(
                    "response.JOURNALING_MB",
                    "mongodb_atlas.process.journaling.mb",
                )?;
            }

            if event.has_value("response.JOURNALING_WRITE_DATA_FILES_MB") {
                event.rename(
                    "response.JOURNALING_WRITE_DATA_FILES_MB",
                    "mongodb_atlas.process.journaling.write.data_files.mb",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_USER") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_USER",
                    "mongodb_atlas.process.system.normalized.cpu.user.max.pct",
                )?;
            }

            if event.has_value("response.COMPUTED_MEMORY") {
                event.rename(
                    "response.COMPUTED_MEMORY",
                    "mongodb_atlas.process.memory.computed.mb",
                )?;
            }

            if event.has_value("response.MEMORY_MAPPED") {
                event.rename(
                    "response.MEMORY_MAPPED",
                    "mongodb_atlas.process.memory.mapped.mb",
                )?;
            }

            if event.has_value("response.MEMORY_RESIDENT") {
                event.rename(
                    "response.MEMORY_RESIDENT",
                    "mongodb_atlas.process.memory.resident.mb",
                )?;
            }

            if event.has_value("response.MEMORY_VIRTUAL") {
                event.rename(
                    "response.MEMORY_VIRTUAL",
                    "mongodb_atlas.process.memory.virtual.mb",
                )?;
            }

            if event.has_value("response.NETWORK_BYTES_IN") {
                event.rename(
                    "response.NETWORK_BYTES_IN",
                    "mongodb_atlas.process.network.in",
                )?;
            }

            if event.has_value("response.NETWORK_BYTES_OUT") {
                event.rename(
                    "response.NETWORK_BYTES_OUT",
                    "mongodb_atlas.process.network.out",
                )?;
            }

            if event.has_value("response.NETWORK_NUM_REQUESTS") {
                event.rename(
                    "response.NETWORK_NUM_REQUESTS",
                    "mongodb_atlas.process.network.request.total",
                )?;
            }

            if event.has_value("response.OPCOUNTER_CMD") {
                event.rename(
                    "response.OPCOUNTER_CMD",
                    "mongodb_atlas.process.opcounter.cmd",
                )?;
            }

            if event.has_value("response.OPCOUNTER_DELETE") {
                event.rename(
                    "response.OPCOUNTER_DELETE",
                    "mongodb_atlas.process.opcounter.delete",
                )?;
            }

            if event.has_value("response.OPCOUNTER_GETMORE") {
                event.rename(
                    "response.OPCOUNTER_GETMORE",
                    "mongodb_atlas.process.opcounter.getmore",
                )?;
            }

            if event.has_value("response.OPCOUNTER_INSERT") {
                event.rename(
                    "response.OPCOUNTER_INSERT",
                    "mongodb_atlas.process.opcounter.insert",
                )?;
            }

            if event.has_value("response.OPCOUNTER_QUERY") {
                event.rename(
                    "response.OPCOUNTER_QUERY",
                    "mongodb_atlas.process.opcounter.query",
                )?;
            }

            if event.has_value("response.OPCOUNTER_REPL_CMD") {
                event.rename(
                    "response.OPCOUNTER_REPL_CMD",
                    "mongodb_atlas.process.opcounter.repl.cmd",
                )?;
            }

            if event.has_value("response.OPCOUNTER_REPL_DELETE") {
                event.rename(
                    "response.OPCOUNTER_REPL_DELETE",
                    "mongodb_atlas.process.opcounter.repl.delete",
                )?;
            }

            if event.has_value("response.OPCOUNTER_REPL_INSERT") {
                event.rename(
                    "response.OPCOUNTER_REPL_INSERT",
                    "mongodb_atlas.process.opcounter.repl.insert",
                )?;
            }

            if event.has_value("response.OPCOUNTER_REPL_UPDATE") {
                event.rename(
                    "response.OPCOUNTER_REPL_UPDATE",
                    "mongodb_atlas.process.opcounter.repl.update",
                )?;
            }

            if event.has_value("response.OPCOUNTER_UPDATE") {
                event.rename(
                    "response.OPCOUNTER_UPDATE",
                    "mongodb_atlas.process.opcounter.update",
                )?;
            }

            if event.has_value("response.OP_EXECUTION_TIME_COMMANDS") {
                event.rename(
                    "response.OP_EXECUTION_TIME_COMMANDS",
                    "mongodb_atlas.process.operation.execution.time.cmd.avg.ms",
                )?;
            }

            if event.has_value("response.OP_EXECUTION_TIME_READS") {
                event.rename(
                    "response.OP_EXECUTION_TIME_READS",
                    "mongodb_atlas.process.operation.execution.time.read.avg.ms",
                )?;
            }

            if event.has_value("response.OP_EXECUTION_TIME_WRITES") {
                event.rename(
                    "response.OP_EXECUTION_TIME_WRITES",
                    "mongodb_atlas.process.operation.execution.time.write.avg.ms",
                )?;
            }

            if event.has_value("response.OPERATIONS_SCAN_AND_ORDER") {
                event.rename(
                    "response.OPERATIONS_SCAN_AND_ORDER",
                    "mongodb_atlas.process.operation.scan_and_order",
                )?;
            }

            if event.has_value("response.OPLOG_MASTER_LAG_TIME_DIFF") {
                event.rename(
                    "response.OPLOG_MASTER_LAG_TIME_DIFF",
                    "mongodb_atlas.process.oplog.master.lag.time_diff.s",
                )?;
            }

            if event.has_value("response.OPLOG_MASTER_TIME") {
                event.rename(
                    "response.OPLOG_MASTER_TIME",
                    "mongodb_atlas.process.oplog.master.time.s",
                )?;
            }

            if event.has_value("response.OPLOG_RATE_GB_PER_HOUR") {
                event.rename(
                    "response.OPLOG_RATE_GB_PER_HOUR",
                    "mongodb_atlas.process.oplog.rate.gb_per_hour",
                )?;
            }

            if event.has_value("response.OPLOG_REPLICATION_LAG") {
                event.rename(
                    "response.OPLOG_REPLICATION_LAG",
                    "mongodb_atlas.process.oplog.repl_lag.s",
                )?;
            }

            if event.has_value("response.OPLOG_SLAVE_LAG_MASTER_TIME") {
                event.rename(
                    "response.OPLOG_SLAVE_LAG_MASTER_TIME",
                    "mongodb_atlas.process.oplog.slave.lag.master.time.s",
                )?;
            }

            if event.has_value("response.QUERY_EXECUTOR_SCANNED") {
                event.rename(
                    "response.QUERY_EXECUTOR_SCANNED",
                    "mongodb_atlas.process.query.executor.scanned",
                )?;
            }

            if event.has_value("response.QUERY_EXECUTOR_SCANNED_OBJECTS") {
                event.rename(
                    "response.QUERY_EXECUTOR_SCANNED_OBJECTS",
                    "mongodb_atlas.process.query.executor.scanned_objects",
                )?;
            }

            if event.has_value("response.QUERY_TARGETING_SCANNED_OBJECTS_PER_RETURNED") {
                event.rename(
                    "response.QUERY_TARGETING_SCANNED_OBJECTS_PER_RETURNED",
                    "mongodb_atlas.process.query.targeting.scanned_objects_per_returned",
                )?;
            }

            if event.has_value("response.QUERY_TARGETING_SCANNED_PER_RETURNED") {
                event.rename(
                    "response.QUERY_TARGETING_SCANNED_PER_RETURNED",
                    "mongodb_atlas.process.query.targeting.scanned_per_returned",
                )?;
            }

            if event.has_value("response.RESTARTS_IN_LAST_HOUR") {
                event.rename(
                    "response.RESTARTS_IN_LAST_HOUR",
                    "mongodb_atlas.process.restart.in_last_hour",
                )?;
            }

            if event.has_value("response.MAX_SWAP_USAGE_FREE") {
                event.rename(
                    "response.MAX_SWAP_USAGE_FREE",
                    "mongodb_atlas.process.swap.usage.free.max.kb",
                )?;
            }

            if event.has_value("response.SWAP_USAGE_FREE") {
                event.rename(
                    "response.SWAP_USAGE_FREE",
                    "mongodb_atlas.process.swap.usage.total.free",
                )?;
            }

            if event.has_value("response.SWAP_USAGE_USED") {
                event.rename(
                    "response.SWAP_USAGE_USED",
                    "mongodb_atlas.process.swap.usage.total.used",
                )?;
            }

            if event.has_value("response.MAX_SWAP_USAGE_USED") {
                event.rename(
                    "response.MAX_SWAP_USAGE_USED",
                    "mongodb_atlas.process.swap.usage.used.max.kb",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_GUEST") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_GUEST",
                    "mongodb_atlas.process.system.cpu.guest.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_GUEST") {
                event.rename(
                    "response.SYSTEM_CPU_GUEST",
                    "mongodb_atlas.process.system.cpu.guest.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_IOWAIT") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_IOWAIT",
                    "mongodb_atlas.process.system.cpu.iowait.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_IOWAIT") {
                event.rename(
                    "response.SYSTEM_CPU_IOWAIT",
                    "mongodb_atlas.process.system.cpu.iowait.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_IRQ") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_IRQ",
                    "mongodb_atlas.process.system.cpu.irq.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_IRQ") {
                event.rename(
                    "response.SYSTEM_CPU_IRQ",
                    "mongodb_atlas.process.system.cpu.irq.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_KERNEL") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_KERNEL",
                    "mongodb_atlas.process.system.cpu.kernel.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_KERNEL") {
                event.rename(
                    "response.SYSTEM_CPU_KERNEL",
                    "mongodb_atlas.process.system.cpu.kernel.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_NICE") {
                event.rename(
                    "response.SYSTEM_CPU_NICE",
                    "mongodb_atlas.process.system.cpu.nice.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_SOFTIRQ") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_SOFTIRQ",
                    "mongodb_atlas.process.system.cpu.softirq.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_SOFTIRQ") {
                event.rename(
                    "response.SYSTEM_CPU_SOFTIRQ",
                    "mongodb_atlas.process.system.cpu.softirq.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_STEAL") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_STEAL",
                    "mongodb_atlas.process.system.cpu.steal.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_STEAL") {
                event.rename(
                    "response.SYSTEM_CPU_STEAL",
                    "mongodb_atlas.process.system.cpu.steal.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_CPU_USER") {
                event.rename(
                    "response.MAX_SYSTEM_CPU_USER",
                    "mongodb_atlas.process.system.cpu.user.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_CPU_USER") {
                event.rename(
                    "response.SYSTEM_CPU_USER",
                    "mongodb_atlas.process.system.cpu.user.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_MEMORY_AVAILABLE") {
                event.rename(
                    "response.SYSTEM_MEMORY_AVAILABLE",
                    "mongodb_atlas.process.system.memory.available.kb",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_MEMORY_AVAILABLE") {
                event.rename(
                    "response.MAX_SYSTEM_MEMORY_AVAILABLE",
                    "mongodb_atlas.process.system.memory.available.max.kb",
                )?;
            }

            if event.has_value("response.SYSTEM_MEMORY_FREE") {
                event.rename(
                    "response.SYSTEM_MEMORY_FREE",
                    "mongodb_atlas.process.system.memory.free.kb",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_MEMORY_FREE") {
                event.rename(
                    "response.MAX_SYSTEM_MEMORY_FREE",
                    "mongodb_atlas.process.system.memory.free.max.kb",
                )?;
            }

            if event.has_value("response.SYSTEM_MEMORY_USED") {
                event.rename(
                    "response.SYSTEM_MEMORY_USED",
                    "mongodb_atlas.process.system.memory.used.kb",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_MEMORY_USED") {
                event.rename(
                    "response.MAX_SYSTEM_MEMORY_USED",
                    "mongodb_atlas.process.system.memory.used.max.kb",
                )?;
            }

            if event.has_value("response.SYSTEM_NETWORK_IN") {
                event.rename(
                    "response.SYSTEM_NETWORK_IN",
                    "mongodb_atlas.process.system.network.in",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NETWORK_IN") {
                event.rename(
                    "response.MAX_SYSTEM_NETWORK_IN",
                    "mongodb_atlas.process.system.network.max.in",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NETWORK_OUT") {
                event.rename(
                    "response.MAX_SYSTEM_NETWORK_OUT",
                    "mongodb_atlas.process.system.network.max.out",
                )?;
            }

            if event.has_value("response.SYSTEM_NETWORK_OUT") {
                event.rename(
                    "response.SYSTEM_NETWORK_OUT",
                    "mongodb_atlas.process.system.network.out",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_GUEST") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_GUEST",
                    "mongodb_atlas.process.system.normalized.cpu.guest.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_GUEST") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_GUEST",
                    "mongodb_atlas.process.system.normalized.cpu.guest.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_IOWAIT") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_IOWAIT",
                    "mongodb_atlas.process.system.normalized.cpu.iowait.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_IOWAIT") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_IOWAIT",
                    "mongodb_atlas.process.system.normalized.cpu.iowait.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_IRQ") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_IRQ",
                    "mongodb_atlas.process.system.normalized.cpu.irq.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_IRQ") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_IRQ",
                    "mongodb_atlas.process.system.normalized.cpu.irq.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_KERNEL") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_KERNEL",
                    "mongodb_atlas.process.system.normalized.cpu.kernel.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_KERNEL") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_KERNEL",
                    "mongodb_atlas.process.system.normalized.cpu.kernel.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_NICE") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_NICE",
                    "mongodb_atlas.process.system.normalized.cpu.nice.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_NICE") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_NICE",
                    "mongodb_atlas.process.system.normalized.cpu.nice.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_SOFTIRQ") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_SOFTIRQ",
                    "mongodb_atlas.process.system.normalized.cpu.softirq.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_SOFTIRQ") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_SOFTIRQ",
                    "mongodb_atlas.process.system.normalized.cpu.softirq.pct",
                )?;
            }

            if event.has_value("response.MAX_SYSTEM_NORMALIZED_CPU_STEAL") {
                event.rename(
                    "response.MAX_SYSTEM_NORMALIZED_CPU_STEAL",
                    "mongodb_atlas.process.system.normalized.cpu.steal.max.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_STEAL") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_STEAL",
                    "mongodb_atlas.process.system.normalized.cpu.steal.pct",
                )?;
            }

            if event.has_value("response.SYSTEM_NORMALIZED_CPU_USER") {
                event.rename(
                    "response.SYSTEM_NORMALIZED_CPU_USER",
                    "mongodb_atlas.process.system.normalized.cpu.user.pct",
                )?;
            }

            if event.has_value("response.TICKETS_AVAILABLE_READS") {
                event.rename(
                    "response.TICKETS_AVAILABLE_READS",
                    "mongodb_atlas.process.ticket.available.read.count",
                )?;
            }

            if event.has_value("response.TICKETS_AVAILABLE_WRITE") {
                event.rename(
                    "response.TICKETS_AVAILABLE_WRITE",
                    "mongodb_atlas.process.ticket.available.write.count",
                )?;
            }

            if event.has_value("groupId") {
                event.rename("groupId", "mongodb_atlas.group_id")?;
            }

            if event.has_value("processId") {
                event.rename("processId", "mongodb_atlas.process_id")?;
            }

            if event.has_value("hostId") {
                event.rename("hostId", "mongodb_atlas.host_id")?;
            }

            event.remove("response");
            event.remove("start");
            event.remove("granularity");
            event.remove("end");
            event.remove("databaseName");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
