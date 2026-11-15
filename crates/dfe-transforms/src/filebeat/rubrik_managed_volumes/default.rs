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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "945fc79b613fc0f04d10f42c1fb7a15b5cc3eaa3c9a3244f4d70c04dd73d873f",
                    r#"{"params":null,"known":[]}"#
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
