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
            event.set("event.module", json!("etcd"))?;

            event.set("ecs.version", json!("8.11.0"))?;

            if event.has_value("prometheus.etcd_mvcc_db_total_size_in_bytes.value") {
                event.rename(
                    "prometheus.etcd_mvcc_db_total_size_in_bytes.value",
                    "etcd.disk.mvcc_db_total_size.bytes",
                )?;
            }

            if event.has_value("prometheus.etcd_disk_wal_fsync_duration_seconds.histogram") {
                event.rename(
                    "prometheus.etcd_disk_wal_fsync_duration_seconds.histogram",
                    "etcd.disk.wal_fsync_duration_seconds.histogram",
                )?;
            }

            if event.has_value("prometheus.etcd_disk_backend_commit_duration_seconds.histogram") {
                event.rename(
                    "prometheus.etcd_disk_backend_commit_duration_seconds.histogram",
                    "etcd.disk.backend_commit_duration_seconds.histogram",
                )?;
            }

            if event.has_value("prometheus.go_memstats_alloc_bytes.value") {
                event.rename(
                    "prometheus.go_memstats_alloc_bytes.value",
                    "etcd.memory.go_memstats_alloc.bytes",
                )?;
            }

            if event.has_value("prometheus.go_memstats_alloc_bytes_total.counter") {
                event.rename(
                    "prometheus.go_memstats_alloc_bytes_total.counter",
                    "etcd.memory.go_memstats_alloc.total.bytes",
                )?;
            }

            if event.has_value("prometheus.etcd_network_client_grpc_sent_bytes_total.counter") {
                event.rename(
                    "prometheus.etcd_network_client_grpc_sent_bytes_total.counter",
                    "etcd.network.client_grpc_sent.bytes",
                )?;
            }

            if event.has_value("prometheus.etcd_network_client_grpc_received_bytes_total.counter") {
                event.rename(
                    "prometheus.etcd_network_client_grpc_received_bytes_total.counter",
                    "etcd.network.client_grpc_received.bytes",
                )?;
            }

            if event.has_value("prometheus.etcd_server_has_leader.value") {
                event.rename(
                    "prometheus.etcd_server_has_leader.value",
                    "etcd.server.has_leader.count",
                )?;
            }

            if event.has_value("prometheus.etcd_server_leader_changes_seen_total.counter") {
                event.rename(
                    "prometheus.etcd_server_leader_changes_seen_total.counter",
                    "etcd.server.leader_changes.count",
                )?;
            }

            if event.has_value("prometheus.etcd_server_proposals_committed_total.value") {
                event.rename(
                    "prometheus.etcd_server_proposals_committed_total.value",
                    "etcd.server.proposals_committed.count",
                )?;
            }

            if event.has_value("prometheus.etcd_server_proposals_pending.value") {
                event.rename(
                    "prometheus.etcd_server_proposals_pending.value",
                    "etcd.server.proposals_pending.count",
                )?;
            }

            if event.has_value("prometheus.etcd_server_proposals_failed_total.counter") {
                event.rename(
                    "prometheus.etcd_server_proposals_failed_total.counter",
                    "etcd.server.proposals_failed.count",
                )?;
            }

            if event.has_value("prometheus.grpc_server_started_total.counter") {
                event.rename(
                    "prometheus.grpc_server_started_total.counter",
                    "etcd.server.grpc_started.count",
                )?;
            }

            if event.has_value("prometheus.grpc_server_handled_total.counter") {
                event.rename(
                    "prometheus.grpc_server_handled_total.counter",
                    "etcd.server.grpc_handled.count",
                )?;
            }

            if event.has_value("prometheus.process_start_time_seconds.value") {
                event.rename(
                    "prometheus.process_start_time_seconds.value",
                    "etcd.process_start_time.sec",
                )?;
            }

            if event.has_value("prometheus.etcd_network_peer_round_trip_time_seconds.histogram") {
                event.rename(
                    "prometheus.etcd_network_peer_round_trip_time_seconds.histogram",
                    "etcd.network.peer_round_trip_time_seconds.histogram",
                )?;
            }

            if event.has_value("prometheus.etcd_server_proposals_applied_total.value") {
                event.rename(
                    "prometheus.etcd_server_proposals_applied_total.value",
                    "etcd.server.proposals_applied_total",
                )?;
            }

            if event.has_value("prometheus.etcd_network_peer_received_bytes_total.counter") {
                event.rename(
                    "prometheus.etcd_network_peer_received_bytes_total.counter",
                    "etcd.network.peer_received_bytes_total",
                )?;
            }

            if event.has_value("prometheus.etcd_network_peer_sent_bytes_total.counter") {
                event.rename(
                    "prometheus.etcd_network_peer_sent_bytes_total.counter",
                    "etcd.network.peer_sent_bytes_total",
                )?;
            }

            if event.has_value("prometheus.etcd_network_peer_sent_failures_total.counter") {
                event.rename(
                    "prometheus.etcd_network_peer_sent_failures_total.counter",
                    "etcd.network.peer_sent_failures_total",
                )?;
            }

            if event.has_value("prometheus.etcd_network_peer_received_failures_total.counter") {
                event.rename(
                    "prometheus.etcd_network_peer_received_failures_total.counter",
                    "etcd.network.peer_received_failures_total",
                )?;
            }

            if event.has_value("prometheus.etcd_debugging_store_writes_total.counter") {
                event.rename(
                    "prometheus.etcd_debugging_store_writes_total.counter",
                    "etcd.store.writes_total",
                )?;
            }

            if event.has_value("prometheus.etcd_debugging_store_expires_total.counter") {
                event.rename(
                    "prometheus.etcd_debugging_store_expires_total.counter",
                    "etcd.store.expires_total",
                )?;
            }

            if event.has_value("prometheus.etcd_debugging_store_reads_total.counter") {
                event.rename(
                    "prometheus.etcd_debugging_store_reads_total.counter",
                    "etcd.store.reads_total",
                )?;
            }

            if event.has_value("prometheus.etcd_debugging_store_watchers.value") {
                event.rename(
                    "prometheus.etcd_debugging_store_watchers.value",
                    "etcd.store.watchers",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels") {
                    event.rename("prometheus.labels", "etcd.labels")?;
                }
                Ok(())
            })();

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("etcd.labels") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "etcd.labels.fingerprint",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            event.remove("prometheus");

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
                event.set(
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
