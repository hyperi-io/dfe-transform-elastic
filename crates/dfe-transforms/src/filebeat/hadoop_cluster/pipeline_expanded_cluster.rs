// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_expanded_cluster` pipeline.
pub struct PipelineExpandedCluster;

impl Transform for PipelineExpandedCluster {
    fn name(&self) -> &str {
        "pipeline_expanded_cluster"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: ctx.hadoop.cluster.temp = ctx.hadoop.cluster.clusterMetrics; ctx.hadoop.cluster.remove(\"clusterMetrics\")\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.hadoop.cluster.temp = ctx.hadoop.cluster.clusterMetrics; ctx.hadoop.cluster.remove(\"clusterMetrics\")\n"#))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.appsSubmitted") {
                    event.rename("hadoop.cluster.temp.appsSubmitted", "hadoop.cluster.applications.submitted")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.appsCompleted") {
                    event.rename("hadoop.cluster.temp.appsCompleted", "hadoop.cluster.applications.completed")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.appsPending") {
                    event.rename("hadoop.cluster.temp.appsPending", "hadoop.cluster.applications.pending")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.appsRunning") {
                    event.rename("hadoop.cluster.temp.appsRunning", "hadoop.cluster.applications.running")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.appsFailed") {
                    event.rename("hadoop.cluster.temp.appsFailed", "hadoop.cluster.applications.failed")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.appsKilled") {
                    event.rename("hadoop.cluster.temp.appsKilled", "hadoop.cluster.applications.killed")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.reservedMB") {
                    event.rename("hadoop.cluster.temp.reservedMB", "hadoop.cluster.memory.reserved")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.availableMB") {
                    event.rename("hadoop.cluster.temp.availableMB", "hadoop.cluster.memory.available")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.allocatedMB") {
                    event.rename("hadoop.cluster.temp.allocatedMB", "hadoop.cluster.memory.allocated")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.totalMB") {
                    event.rename("hadoop.cluster.temp.totalMB", "hadoop.cluster.memory.total")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.reservedVirtualCores") {
                    event.rename("hadoop.cluster.temp.reservedVirtualCores", "hadoop.cluster.virtual_cores.reserved")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.availableVirtualCores") {
                    event.rename("hadoop.cluster.temp.availableVirtualCores", "hadoop.cluster.virtual_cores.available")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.allocatedVirtualCores") {
                    event.rename("hadoop.cluster.temp.allocatedVirtualCores", "hadoop.cluster.virtual_cores.allocated")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.totalVirtualCores") {
                    event.rename("hadoop.cluster.temp.totalVirtualCores", "hadoop.cluster.virtual_cores.total")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.containersAllocated") {
                    event.rename("hadoop.cluster.temp.containersAllocated", "hadoop.cluster.containers.allocated")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.containersReserved") {
                    event.rename("hadoop.cluster.temp.containersReserved", "hadoop.cluster.containers.reserved")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.containersPending") {
                    event.rename("hadoop.cluster.temp.containersPending", "hadoop.cluster.containers.pending")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.totalNodes") {
                    event.rename("hadoop.cluster.temp.totalNodes", "hadoop.cluster.nodes.total")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.activeNodes") {
                    event.rename("hadoop.cluster.temp.activeNodes", "hadoop.cluster.nodes.active")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.lostNodes") {
                    event.rename("hadoop.cluster.temp.lostNodes", "hadoop.cluster.nodes.lost")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.unhealthyNodes") {
                    event.rename("hadoop.cluster.temp.unhealthyNodes", "hadoop.cluster.nodes.unhealthy")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.decommissioningNodes") {
                    event.rename("hadoop.cluster.temp.decommissioningNodes", "hadoop.cluster.nodes.decommissioning")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.decommissionedNodes") {
                    event.rename("hadoop.cluster.temp.decommissionedNodes", "hadoop.cluster.nodes.decommissioned")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.rebootedNodes") {
                    event.rename("hadoop.cluster.temp.rebootedNodes", "hadoop.cluster.nodes.rebooted")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.shutdownNodes") {
                    event.rename("hadoop.cluster.temp.shutdownNodes", "hadoop.cluster.nodes.shutdown")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("hadoop.cluster.temp");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
