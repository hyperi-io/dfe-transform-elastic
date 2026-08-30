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

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http") {
                    event.rename("http", "hadoop")?;
                }
                Ok(())
            })();

            // Painless script
            // Source: ctx.hadoop.namenode.temp = ctx.hadoop.namenode.beans[0]; ctx.hadoop.namenode.remove(\"beans\")\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.hadoop.namenode.temp = ctx.hadoop.namenode.beans[0]; ctx.hadoop.namenode.remove(\"beans\")\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.CapacityTotal") {
                    event.rename(
                        "hadoop.namenode.temp.CapacityTotal",
                        "hadoop.namenode.capacity.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.CapacityUsed") {
                    event.rename(
                        "hadoop.namenode.temp.CapacityUsed",
                        "hadoop.namenode.capacity.used",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.CapacityRemaining") {
                    event.rename(
                        "hadoop.namenode.temp.CapacityRemaining",
                        "hadoop.namenode.capacity.remaining",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.TotalLoad") {
                    event.rename(
                        "hadoop.namenode.temp.TotalLoad",
                        "hadoop.namenode.total_load",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.LockQueueLength") {
                    event.rename(
                        "hadoop.namenode.temp.LockQueueLength",
                        "hadoop.namenode.lock_queue_length",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.BlocksTotal") {
                    event.rename(
                        "hadoop.namenode.temp.BlocksTotal",
                        "hadoop.namenode.blocks.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.CorruptBlocks") {
                    event.rename(
                        "hadoop.namenode.temp.CorruptBlocks",
                        "hadoop.namenode.blocks.corrupt",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.FilesTotal") {
                    event.rename(
                        "hadoop.namenode.temp.FilesTotal",
                        "hadoop.namenode.files_total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.PendingReplicationBlocks") {
                    event.rename(
                        "hadoop.namenode.temp.PendingReplicationBlocks",
                        "hadoop.namenode.blocks.pending_replication",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.UnderReplicatedBlocks") {
                    event.rename(
                        "hadoop.namenode.temp.UnderReplicatedBlocks",
                        "hadoop.namenode.blocks.under_replicated",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.ScheduledReplicationBlocks") {
                    event.rename(
                        "hadoop.namenode.temp.ScheduledReplicationBlocks",
                        "hadoop.namenode.blocks.scheduled_replication",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.PendingDeletionBlocks") {
                    event.rename(
                        "hadoop.namenode.temp.PendingDeletionBlocks",
                        "hadoop.namenode.blocks.pending_deletion",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.NumLiveDataNodes") {
                    event.rename(
                        "hadoop.namenode.temp.NumLiveDataNodes",
                        "hadoop.namenode.nodes.num_live_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.NumDeadDataNodes") {
                    event.rename(
                        "hadoop.namenode.temp.NumDeadDataNodes",
                        "hadoop.namenode.nodes.num_dead_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.NumDecomLiveDataNodes") {
                    event.rename(
                        "hadoop.namenode.temp.NumDecomLiveDataNodes",
                        "hadoop.namenode.nodes.num_decom_live_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.NumDecomDeadDataNodes") {
                    event.rename(
                        "hadoop.namenode.temp.NumDecomDeadDataNodes",
                        "hadoop.namenode.nodes.num_decom_dead_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.VolumeFailuresTotal") {
                    event.rename(
                        "hadoop.namenode.temp.VolumeFailuresTotal",
                        "hadoop.namenode.volume_failures_total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.EstimatedCapacityLostTotal") {
                    event.rename(
                        "hadoop.namenode.temp.EstimatedCapacityLostTotal",
                        "hadoop.namenode.estimated_capacity_lost_total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.NumDecommissioningDataNodes") {
                    event.rename(
                        "hadoop.namenode.temp.NumDecommissioningDataNodes",
                        "hadoop.namenode.nodes.num_decommissioning_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.StaleDataNodes") {
                    event.rename(
                        "hadoop.namenode.temp.StaleDataNodes",
                        "hadoop.namenode.stale_data_nodes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.NumStaleStorages") {
                    event.rename(
                        "hadoop.namenode.temp.NumStaleStorages",
                        "hadoop.namenode.num_stale_storages",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.namenode.temp.MissingReplOneBlocks") {
                    event.rename(
                        "hadoop.namenode.temp.MissingReplOneBlocks",
                        "hadoop.namenode.blocks.missing_repl_one",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("hadoop.namenode.temp");
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
