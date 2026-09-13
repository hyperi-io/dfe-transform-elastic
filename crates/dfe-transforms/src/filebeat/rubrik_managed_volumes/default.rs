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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("event.kind", json!("metric"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "response")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "error.message",
                        json!("Received invalid JSON. Unable to parse the source log message"),
                    )?;
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("response.name") {
                event.rename("response.name", "rubrik.managed_volumes.name")?;
            }

            if event.has_value("response.state") {
                event.rename("response.state", "rubrik.managed_volumes.state")?;
            }

            if event.has_value("response.provisionedSize") {
                event.rename(
                    "response.provisionedSize",
                    "rubrik.managed_volumes.volume_size.bytes",
                )?;
            }

            if event.has_value("response.mainMount.logicalUsedSize") {
                event.rename(
                    "response.mainMount.logicalUsedSize",
                    "rubrik.managed_volumes.used_size.bytes",
                )?;
            }

            if event.has_value("response.snapshotDistribution.totalCount") {
                event.rename(
                    "response.snapshotDistribution.totalCount",
                    "rubrik.managed_volumes.total_snapshots.count",
                )?;
            }

            if event.has_value("response.numChannels") {
                event.rename(
                    "response.numChannels",
                    "rubrik.managed_volumes.num_channels.count",
                )?;
            }

            if event.has_value("response.cluster.id") {
                event.rename("response.cluster.id", "rubrik.cluster.id")?;
            }

            if event.has_value("response.cluster.name") {
                event.rename("response.cluster.name", "rubrik.cluster.name")?;
            }

            if event.has_value("response.effectiveSlaDomain.id") {
                event.rename(
                    "response.effectiveSlaDomain.id",
                    "rubrik.effective_sla_domain.id",
                )?;
            }

            if event.has_value("response.effectiveSlaDomain.name") {
                event.rename(
                    "response.effectiveSlaDomain.name",
                    "rubrik.effective_sla_domain.name",
                )?;
            }

            // Painless script
            // Source: if (ctx.rubrik.managed_volumes?.volume_size?.bytes != null && ctx.rubrik.managed_volumes?.used_size?.bytes != null) {\n    ctx.rubrik.managed_volumes.free_size = [:];\n    ctx.rubrik.managed_volumes.free_size.bytes = ctx.rubrik.managed_volumes.volume_size.bytes - ctx.rubrik.managed_volumes.used_size.bytes;\n} else {\n    ctx.rubrik.managed_volumes.free_size = [:];\n    ctx.rubrik.managed_volumes.free_size.bytes = 0;\n}\n\n// Calculate pending snapshots which is the difference of the total snapshot count and the scheduled snapshot count.\n if (ctx.response?.snapshotDistribution != null) {\n    def snapshotDistribution = ctx.response.snapshotDistribution;\n\n    def totalCount = snapshotDistribution.totalCount != null ? snapshotDistribution.totalCount : 0;\n    def scheduledCount = snapshotDistribution.scheduledCount != null ? snapshotDistribution.scheduledCount : 0;\n\n    // Initialize pending_snapshots field and set the count\n    ctx.rubrik.managed_volumes.pending_snapshots = [:];\n    ctx.rubrik.managed_volumes.pending_snapshots.count = totalCount - scheduledCount;\n} else {\n    // If snapshotDistribution is missing, initialize pending_snapshots with a default value\n    ctx.rubrik.managed_volumes.pending_snapshots = [:];\n    ctx.rubrik.managed_volumes.pending_snapshots.count = 0;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.rubrik.managed_volumes?.volume_size?.bytes != null && ctx.rubrik.managed_volumes?.used_size?.bytes != null) {\n    ctx.rubrik.managed_volumes.free_size = [:];\n    ctx.rubrik.managed_volumes.free_size.bytes = ctx.rubrik.managed_volumes.volume_size.bytes - ctx.rubrik.managed_volumes.used_size.bytes;\n} else {\n    ctx.rubrik.managed_volumes.free_size = [:];\n    ctx.rubrik.managed_volumes.free_size.bytes = 0;\n}\n\n// Calculate pending snapshots which is the difference of the total snapshot count and the scheduled snapshot count.\n if (ctx.response?.snapshotDistribution != null) {\n    def snapshotDistribution = ctx.response.snapshotDistribution;\n\n    def totalCount = snapshotDistribution.totalCount != null ? snapshotDistribution.totalCount : 0;\n    def scheduledCount = snapshotDistribution.scheduledCount != null ? snapshotDistribution.scheduledCount : 0;\n\n    // Initialize pending_snapshots field and set the count\n    ctx.rubrik.managed_volumes.pending_snapshots = [:];\n    ctx.rubrik.managed_volumes.pending_snapshots.count = totalCount - scheduledCount;\n} else {\n    // If snapshotDistribution is missing, initialize pending_snapshots with a default value\n    ctx.rubrik.managed_volumes.pending_snapshots = [:];\n    ctx.rubrik.managed_volumes.pending_snapshots.count = 0;\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("rubrik.managed_volumes") };
            if _cond {
                event.remove("response");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
