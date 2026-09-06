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
                event.rename("response.name", "rubrik.cluster.name")?;
            }

            if event.has_value("response.id") {
                event.rename("response.id", "rubrik.cluster.id")?;
            }

            if event.has_value("response.type") {
                event.rename("response.type", "rubrik.cluster.type")?;
            }

            if event.has_value("response.estimatedRunway") {
                event.rename(
                    "response.estimatedRunway",
                    "rubrik.cluster_performance.estimated_runway",
                )?;
            }

            if event.has_value("response.metric.usedCapacity") {
                event.rename(
                    "response.metric.usedCapacity",
                    "rubrik.cluster_performance.used_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.availableCapacity") {
                event.rename(
                    "response.metric.availableCapacity",
                    "rubrik.cluster_performance.available_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.totalCapacity") {
                event.rename(
                    "response.metric.totalCapacity",
                    "rubrik.cluster_performance.total_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.averageDailyGrowth") {
                event.rename(
                    "response.metric.averageDailyGrowth",
                    "rubrik.cluster_performance.average_daily_growth.bytes",
                )?;
            }

            if event.has_value("response.metric.cdpCapacity") {
                event.rename(
                    "response.metric.cdpCapacity",
                    "rubrik.cluster_performance.cdp_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.liveMountCapacity") {
                event.rename(
                    "response.metric.liveMountCapacity",
                    "rubrik.cluster_performance.live_mount_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.miscellaneousCapacity") {
                event.rename(
                    "response.metric.miscellaneousCapacity",
                    "rubrik.cluster_performance.miscellaneous_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.pendingSnapshotCapacity") {
                event.rename(
                    "response.metric.pendingSnapshotCapacity",
                    "rubrik.cluster_performance.pending_snapshot_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.snapshotCapacity") {
                event.rename(
                    "response.metric.snapshotCapacity",
                    "rubrik.cluster_performance.snapshot_capacity.bytes",
                )?;
            }

            if event.has_value("response.metric.ingestedSnapshotStorage") {
                event.rename(
                    "response.metric.ingestedSnapshotStorage",
                    "rubrik.cluster_performance.ingested_snapshot_storage.bytes",
                )?;
            }

            if event.has_value("response.metric.physicalSnapshotStorage") {
                event.rename(
                    "response.metric.physicalSnapshotStorage",
                    "rubrik.cluster_performance.physical_snapshot_storage.bytes",
                )?;
            }

            if event.has_value("response.status") {
                event.rename("response.status", "rubrik.cluster_performance.status")?;
            }

            event.remove("response");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);     \n
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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'   ",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
