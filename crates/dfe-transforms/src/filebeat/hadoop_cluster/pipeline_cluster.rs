// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_cluster` pipeline.
pub struct PipelineCluster;

impl Transform for PipelineCluster {
    fn name(&self) -> &str {
        "pipeline_cluster"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: ctx.hadoop.cluster.temp = ctx.hadoop.cluster.beans[0]; ctx.hadoop.cluster.remove(\"beans\")\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.hadoop.cluster.temp = ctx.hadoop.cluster.beans[0]; ctx.hadoop.cluster.remove(\"beans\")\n"#))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.AMLaunchDelayAvgTime") {
                    event.rename("hadoop.cluster.temp.AMLaunchDelayAvgTime", "hadoop.cluster.application_main.launch_delay_avg_time")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.AMLaunchDelayNumOps") {
                    event.rename("hadoop.cluster.temp.AMLaunchDelayNumOps", "hadoop.cluster.application_main.launch_delay_num_ops")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.AMRegisterDelayAvgTime") {
                    event.rename("hadoop.cluster.temp.AMRegisterDelayAvgTime", "hadoop.cluster.application_main.register_delay_avg_time")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.AMRegisterDelayNumOps") {
                    event.rename("hadoop.cluster.temp.AMRegisterDelayNumOps", "hadoop.cluster.application_main.register_delay_num_ops")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.NumActiveNMs") {
                    event.rename("hadoop.cluster.temp.NumActiveNMs", "hadoop.cluster.node_managers.num_active")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.NumDecommissionedNMs") {
                    event.rename("hadoop.cluster.temp.NumDecommissionedNMs", "hadoop.cluster.node_managers.num_decommissioned")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.NumLostNMs") {
                    event.rename("hadoop.cluster.temp.NumLostNMs", "hadoop.cluster.node_managers.num_lost")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.NumRebootedNMs") {
                    event.rename("hadoop.cluster.temp.NumRebootedNMs", "hadoop.cluster.node_managers.num_rebooted")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.cluster.temp.NumUnhealthyNMs") {
                    event.rename("hadoop.cluster.temp.NumUnhealthyNMs", "hadoop.cluster.node_managers.num_unhealthy")?;
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
