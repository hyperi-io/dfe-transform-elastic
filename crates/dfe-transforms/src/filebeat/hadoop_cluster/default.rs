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

            let _cond = { event.has("hadoop.cluster.beans") };
            if _cond {
                // Begin nested pipeline: "pipeline-cluster"
                // Painless script
                // Source: ctx.hadoop.cluster.temp = ctx.hadoop.cluster.beans[0]; ctx.hadoop.cluster.remove(\"beans\")\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.hadoop.cluster.temp = ctx.hadoop.cluster.beans[0]; ctx.hadoop.cluster.remove(\"beans\")\n"#
                    ),
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.AMLaunchDelayAvgTime") {
                        event.rename(
                            "hadoop.cluster.temp.AMLaunchDelayAvgTime",
                            "hadoop.cluster.application_main.launch_delay_avg_time",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.AMLaunchDelayNumOps") {
                        event.rename(
                            "hadoop.cluster.temp.AMLaunchDelayNumOps",
                            "hadoop.cluster.application_main.launch_delay_num_ops",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.AMRegisterDelayAvgTime") {
                        event.rename(
                            "hadoop.cluster.temp.AMRegisterDelayAvgTime",
                            "hadoop.cluster.application_main.register_delay_avg_time",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.AMRegisterDelayNumOps") {
                        event.rename(
                            "hadoop.cluster.temp.AMRegisterDelayNumOps",
                            "hadoop.cluster.application_main.register_delay_num_ops",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.NumActiveNMs") {
                        event.rename(
                            "hadoop.cluster.temp.NumActiveNMs",
                            "hadoop.cluster.node_managers.num_active",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.NumDecommissionedNMs") {
                        event.rename(
                            "hadoop.cluster.temp.NumDecommissionedNMs",
                            "hadoop.cluster.node_managers.num_decommissioned",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.NumLostNMs") {
                        event.rename(
                            "hadoop.cluster.temp.NumLostNMs",
                            "hadoop.cluster.node_managers.num_lost",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.NumRebootedNMs") {
                        event.rename(
                            "hadoop.cluster.temp.NumRebootedNMs",
                            "hadoop.cluster.node_managers.num_rebooted",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.NumUnhealthyNMs") {
                        event.rename(
                            "hadoop.cluster.temp.NumUnhealthyNMs",
                            "hadoop.cluster.node_managers.num_unhealthy",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("hadoop.cluster.temp");
                    Ok(())
                })();
                // End nested pipeline: "pipeline-cluster"
            }

            let _cond = { event.has("hadoop.cluster.clusterMetrics") };
            if _cond {
                // Begin nested pipeline: "pipeline-expanded-cluster"
                // Painless script
                // Source: ctx.hadoop.cluster.temp = ctx.hadoop.cluster.clusterMetrics; ctx.hadoop.cluster.remove(\"clusterMetrics\")\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.hadoop.cluster.temp = ctx.hadoop.cluster.clusterMetrics; ctx.hadoop.cluster.remove(\"clusterMetrics\")\n"#
                    ),
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.appsSubmitted") {
                        event.rename(
                            "hadoop.cluster.temp.appsSubmitted",
                            "hadoop.cluster.applications.submitted",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.appsCompleted") {
                        event.rename(
                            "hadoop.cluster.temp.appsCompleted",
                            "hadoop.cluster.applications.completed",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.appsPending") {
                        event.rename(
                            "hadoop.cluster.temp.appsPending",
                            "hadoop.cluster.applications.pending",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.appsRunning") {
                        event.rename(
                            "hadoop.cluster.temp.appsRunning",
                            "hadoop.cluster.applications.running",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.appsFailed") {
                        event.rename(
                            "hadoop.cluster.temp.appsFailed",
                            "hadoop.cluster.applications.failed",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.appsKilled") {
                        event.rename(
                            "hadoop.cluster.temp.appsKilled",
                            "hadoop.cluster.applications.killed",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.reservedMB") {
                        event.rename(
                            "hadoop.cluster.temp.reservedMB",
                            "hadoop.cluster.memory.reserved",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.availableMB") {
                        event.rename(
                            "hadoop.cluster.temp.availableMB",
                            "hadoop.cluster.memory.available",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.allocatedMB") {
                        event.rename(
                            "hadoop.cluster.temp.allocatedMB",
                            "hadoop.cluster.memory.allocated",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.totalMB") {
                        event
                            .rename("hadoop.cluster.temp.totalMB", "hadoop.cluster.memory.total")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.reservedVirtualCores") {
                        event.rename(
                            "hadoop.cluster.temp.reservedVirtualCores",
                            "hadoop.cluster.virtual_cores.reserved",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.availableVirtualCores") {
                        event.rename(
                            "hadoop.cluster.temp.availableVirtualCores",
                            "hadoop.cluster.virtual_cores.available",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.allocatedVirtualCores") {
                        event.rename(
                            "hadoop.cluster.temp.allocatedVirtualCores",
                            "hadoop.cluster.virtual_cores.allocated",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.totalVirtualCores") {
                        event.rename(
                            "hadoop.cluster.temp.totalVirtualCores",
                            "hadoop.cluster.virtual_cores.total",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.containersAllocated") {
                        event.rename(
                            "hadoop.cluster.temp.containersAllocated",
                            "hadoop.cluster.containers.allocated",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.containersReserved") {
                        event.rename(
                            "hadoop.cluster.temp.containersReserved",
                            "hadoop.cluster.containers.reserved",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.containersPending") {
                        event.rename(
                            "hadoop.cluster.temp.containersPending",
                            "hadoop.cluster.containers.pending",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.totalNodes") {
                        event.rename(
                            "hadoop.cluster.temp.totalNodes",
                            "hadoop.cluster.nodes.total",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.activeNodes") {
                        event.rename(
                            "hadoop.cluster.temp.activeNodes",
                            "hadoop.cluster.nodes.active",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.lostNodes") {
                        event
                            .rename("hadoop.cluster.temp.lostNodes", "hadoop.cluster.nodes.lost")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.unhealthyNodes") {
                        event.rename(
                            "hadoop.cluster.temp.unhealthyNodes",
                            "hadoop.cluster.nodes.unhealthy",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.decommissioningNodes") {
                        event.rename(
                            "hadoop.cluster.temp.decommissioningNodes",
                            "hadoop.cluster.nodes.decommissioning",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.decommissionedNodes") {
                        event.rename(
                            "hadoop.cluster.temp.decommissionedNodes",
                            "hadoop.cluster.nodes.decommissioned",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.rebootedNodes") {
                        event.rename(
                            "hadoop.cluster.temp.rebootedNodes",
                            "hadoop.cluster.nodes.rebooted",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.cluster.temp.shutdownNodes") {
                        event.rename(
                            "hadoop.cluster.temp.shutdownNodes",
                            "hadoop.cluster.nodes.shutdown",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("hadoop.cluster.temp");
                    Ok(())
                })();
                // End nested pipeline: "pipeline-expanded-cluster"
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
