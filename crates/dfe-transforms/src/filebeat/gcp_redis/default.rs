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

            if event.has("gcp.metrics.clients.blocked.value") {
                event.rename(
                    "gcp.metrics.clients.blocked.value",
                    "gcp.redis.clients.blocked.count",
                )?;
            }

            if event.has("gcp.metrics.clients.connected.value") {
                event.rename(
                    "gcp.metrics.clients.connected.value",
                    "gcp.redis.clients.connected.count",
                )?;
            }

            if event.has("gcp.metrics.commands.calls.value") {
                event.rename(
                    "gcp.metrics.commands.calls.value",
                    "gcp.redis.commands.calls.count",
                )?;
            }

            if event.has("gcp.metrics.commands.total_time.value") {
                event.rename(
                    "gcp.metrics.commands.total_time.value",
                    "gcp.redis.commands.total_time.us",
                )?;
            }

            if event.has("gcp.metrics.commands.usec_per_call.value") {
                event.rename(
                    "gcp.metrics.commands.usec_per_call.value",
                    "gcp.redis.commands.usec_per_call.sec",
                )?;
            }

            if event.has("gcp.metrics.keyspace.avg_ttl.value") {
                event.rename(
                    "gcp.metrics.keyspace.avg_ttl.value",
                    "gcp.redis.keyspace.avg_ttl.sec",
                )?;
            }

            if event.has("gcp.metrics.keyspace.keys.value") {
                event.rename(
                    "gcp.metrics.keyspace.keys.value",
                    "gcp.redis.keyspace.keys.count",
                )?;
            }

            if event.has("gcp.metrics.keyspace.keys_with_expiration.value") {
                event.rename(
                    "gcp.metrics.keyspace.keys_with_expiration.value",
                    "gcp.redis.keyspace.keys_with_expiration.count",
                )?;
            }

            if event.has("gcp.metrics.persistence.rdb.bgsave_in_progress.value") {
                event.rename(
                    "gcp.metrics.persistence.rdb.bgsave_in_progress.value",
                    "gcp.redis.persistence.rdb.bgsave_in_progress",
                )?;
            }

            if event.has("gcp.metrics.replication.master.slaves.lag.value") {
                event.rename(
                    "gcp.metrics.replication.master.slaves.lag.value",
                    "gcp.redis.replication.master.slaves.lag.sec",
                )?;
            }

            if event.has("gcp.metrics.replication.master.slaves.offset.value") {
                event.rename(
                    "gcp.metrics.replication.master.slaves.offset.value",
                    "gcp.redis.replication.master.slaves.offset.bytes",
                )?;
            }

            if event.has("gcp.metrics.replication.master_repl_offset.value") {
                event.rename(
                    "gcp.metrics.replication.master_repl_offset.value",
                    "gcp.redis.replication.master_repl_offset.bytes",
                )?;
            }

            if event.has("gcp.metrics.replication.offset_diff.value") {
                event.rename(
                    "gcp.metrics.replication.offset_diff.value",
                    "gcp.redis.replication.offset_diff.bytes",
                )?;
            }

            if event.has("gcp.metrics.replication.role.value") {
                event.rename(
                    "gcp.metrics.replication.role.value",
                    "gcp.redis.replication.role",
                )?;
            }

            if event.has("gcp.metrics.server.uptime.value") {
                event.rename(
                    "gcp.metrics.server.uptime.value",
                    "gcp.redis.server.uptime.sec",
                )?;
            }

            if event.has("gcp.metrics.stats.cache_hit_ratio.value") {
                event.rename(
                    "gcp.metrics.stats.cache_hit_ratio.value",
                    "gcp.redis.stats.cache_hit_ratio",
                )?;
            }

            if event.has("gcp.metrics.stats.connections.total.value") {
                event.rename(
                    "gcp.metrics.stats.connections.total.value",
                    "gcp.redis.stats.connections.total.count",
                )?;
            }

            if event.has("gcp.metrics.stats.cpu_utilization.value") {
                event.rename(
                    "gcp.metrics.stats.cpu_utilization.value",
                    "gcp.redis.stats.cpu_utilization.sec",
                )?;
            }

            if event.has("gcp.metrics.stats.evicted_keys.value") {
                event.rename(
                    "gcp.metrics.stats.evicted_keys.value",
                    "gcp.redis.stats.evicted_keys.count",
                )?;
            }

            if event.has("gcp.metrics.stats.expired_keys.value") {
                event.rename(
                    "gcp.metrics.stats.expired_keys.value",
                    "gcp.redis.stats.expired_keys.count",
                )?;
            }

            if event.has("gcp.metrics.stats.keyspace_hits.value") {
                event.rename(
                    "gcp.metrics.stats.keyspace_hits.value",
                    "gcp.redis.stats.keyspace_hits.count",
                )?;
            }

            if event.has("gcp.metrics.stats.keyspace_misses.value") {
                event.rename(
                    "gcp.metrics.stats.keyspace_misses.value",
                    "gcp.redis.stats.keyspace_misses.count",
                )?;
            }

            if event.has("gcp.metrics.stats.memory.maxmemory.value") {
                event.rename(
                    "gcp.metrics.stats.memory.maxmemory.value",
                    "gcp.redis.stats.memory.maxmemory.mb",
                )?;
            }

            if event.has("gcp.metrics.stats.memory.system_memory_overload_duration.value") {
                event.rename(
                    "gcp.metrics.stats.memory.system_memory_overload_duration.value",
                    "gcp.redis.stats.memory.system_memory_overload_duration.us",
                )?;
            }

            if event.has("gcp.metrics.stats.memory.system_memory_usage_ratio.value") {
                event.rename(
                    "gcp.metrics.stats.memory.system_memory_usage_ratio.value",
                    "gcp.redis.stats.memory.system_memory_usage_ratio",
                )?;
            }

            if event.has("gcp.metrics.stats.memory.usage.value") {
                event.rename(
                    "gcp.metrics.stats.memory.usage.value",
                    "gcp.redis.stats.memory.usage.bytes",
                )?;
            }

            if event.has("gcp.metrics.stats.memory.usage_ratio.value") {
                event.rename(
                    "gcp.metrics.stats.memory.usage_ratio.value",
                    "gcp.redis.stats.memory.usage_ratio",
                )?;
            }

            if event.has("gcp.metrics.stats.network_traffic.value") {
                event.rename(
                    "gcp.metrics.stats.network_traffic.value",
                    "gcp.redis.stats.network_traffic.bytes",
                )?;
            }

            if event.has("gcp.metrics.stats.pubsub.channels.value") {
                event.rename(
                    "gcp.metrics.stats.pubsub.channels.value",
                    "gcp.redis.stats.pubsub.channels.count",
                )?;
            }

            if event.has("gcp.metrics.stats.pubsub.patterns.value") {
                event.rename(
                    "gcp.metrics.stats.pubsub.patterns.value",
                    "gcp.redis.stats.pubsub.patterns.count",
                )?;
            }

            if event.has("gcp.metrics.stats.reject_connections_count.value") {
                event.rename(
                    "gcp.metrics.stats.reject_connections_count.value",
                    "gcp.redis.stats.reject_connections.count",
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
