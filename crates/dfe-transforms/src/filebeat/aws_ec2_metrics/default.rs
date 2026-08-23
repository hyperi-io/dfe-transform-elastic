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

            if let Some(v) = event
                .get("cloud.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("cloud.account.name") {
                    event.set("cloud.account.name", v)?;
                }
            }

            // Painless script
            // Source: if(ctx.aws?.ec2?.metrics?.CPUUtilization?.avg != null && ctx.host?.cpu?.usage == null) {\n    ctx.aws.ec2.metrics.CPUUtilization.avg = ctx.aws.ec2.metrics.CPUUtilization.avg / 100;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if(ctx.aws?.ec2?.metrics?.CPUUtilization?.avg != null && ctx.host?.cpu?.usage == null) {\n    ctx.aws.ec2.metrics.CPUUtilization.avg = ctx.aws.ec2.metrics.CPUUtilization.avg / 100;\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.CPUUtilization.avg") {
                    event.rename("aws.ec2.metrics.CPUUtilization.avg", "host.cpu.usage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.NetworkIn.sum") {
                    event.rename(
                        "aws.ec2.metrics.NetworkIn.sum",
                        "host.network.ingress.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.NetworkOut.sum") {
                    event.rename(
                        "aws.ec2.metrics.NetworkOut.sum",
                        "host.network.egress.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.NetworkPacketsIn.sum") {
                    event.rename(
                        "aws.ec2.metrics.NetworkPacketsIn.sum",
                        "host.network.ingress.packets",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.NetworkPacketsOut.sum") {
                    event.rename(
                        "aws.ec2.metrics.NetworkPacketsOut.sum",
                        "host.network.egress.packets",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.DiskReadBytes.sum") {
                    event.rename("aws.ec2.metrics.DiskReadBytes.sum", "host.disk.read.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("aws.ec2.metrics.DiskWriteBytes.sum") {
                    event.rename(
                        "aws.ec2.metrics.DiskWriteBytes.sum",
                        "host.disk.write.bytes",
                    )?;
                }
                Ok(())
            })();

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

        Ok(TransformResult::Continue)
    }
}
