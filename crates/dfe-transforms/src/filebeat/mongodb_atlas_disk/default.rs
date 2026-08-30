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

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("mongodb_atlas"))?;

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("error message set and no data to process.").to_string(),
                });
            }

            if event.has_value("groupId") {
                event.rename("groupId", "group.id")?;
            }

            if event.has_value("processId") {
                event.rename("processId", "mongodb_atlas.process_id")?;
            }

            if event.has_value("partitionName") {
                event.rename("partitionName", "mongodb_atlas.partition_name")?;
            }

            if event.has_value("hostId") {
                event.rename("hostId", "mongodb_atlas.host_id")?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_IOPS_READ") {
                event.rename(
                    "response.MAX_DISK_PARTITION_IOPS_READ",
                    "mongodb_atlas.disk.read.iops.max.throughput",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_IOPS_READ") {
                event.rename(
                    "response.DISK_PARTITION_IOPS_READ",
                    "mongodb_atlas.disk.read.iops.throughput",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_LATENCY_READ") {
                event.rename(
                    "response.MAX_DISK_PARTITION_LATENCY_READ",
                    "mongodb_atlas.disk.read.latency.max.ms",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_LATENCY_READ") {
                event.rename(
                    "response.DISK_PARTITION_LATENCY_READ",
                    "mongodb_atlas.disk.read.latency.ms",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_IOPS_WRITE") {
                event.rename(
                    "response.MAX_DISK_PARTITION_IOPS_WRITE",
                    "mongodb_atlas.disk.write.iops.max.throughput",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_IOPS_WRITE") {
                event.rename(
                    "response.DISK_PARTITION_IOPS_WRITE",
                    "mongodb_atlas.disk.write.iops.throughput",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_LATENCY_WRITE") {
                event.rename(
                    "response.MAX_DISK_PARTITION_LATENCY_WRITE",
                    "mongodb_atlas.disk.write.latency.max.ms",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_LATENCY_WRITE") {
                event.rename(
                    "response.DISK_PARTITION_LATENCY_WRITE",
                    "mongodb_atlas.disk.write.latency.ms",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_SPACE_FREE") {
                event.rename(
                    "response.DISK_PARTITION_SPACE_FREE",
                    "mongodb_atlas.disk.space.free.bytes",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_SPACE_FREE") {
                event.rename(
                    "response.MAX_DISK_PARTITION_SPACE_FREE",
                    "mongodb_atlas.disk.space.free.max.bytes",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_SPACE_PERCENT_FREE") {
                event.rename(
                    "response.MAX_DISK_PARTITION_SPACE_PERCENT_FREE",
                    "mongodb_atlas.disk.space.free.max.pct",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_SPACE_PERCENT_FREE") {
                event.rename(
                    "response.DISK_PARTITION_SPACE_PERCENT_FREE",
                    "mongodb_atlas.disk.space.free.pct",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_SPACE_USED") {
                event.rename(
                    "response.DISK_PARTITION_SPACE_USED",
                    "mongodb_atlas.disk.space.used.bytes",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_SPACE_USED") {
                event.rename(
                    "response.MAX_DISK_PARTITION_SPACE_USED",
                    "mongodb_atlas.disk.space.used.max.bytes",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_SPACE_PERCENT_USED") {
                event.rename(
                    "response.MAX_DISK_PARTITION_SPACE_PERCENT_USED",
                    "mongodb_atlas.disk.space.used.max.pct",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_SPACE_PERCENT_USED") {
                event.rename(
                    "response.DISK_PARTITION_SPACE_PERCENT_USED",
                    "mongodb_atlas.disk.space.used.pct",
                )?;
            }

            if event.has_value("response.MAX_DISK_PARTITION_IOPS_TOTAL") {
                event.rename(
                    "response.MAX_DISK_PARTITION_IOPS_TOTAL",
                    "mongodb_atlas.disk.total.iops.max.throughput",
                )?;
            }

            if event.has_value("response.DISK_PARTITION_IOPS_TOTAL") {
                event.rename(
                    "response.DISK_PARTITION_IOPS_TOTAL",
                    "mongodb_atlas.disk.total.iops.throughput",
                )?;
            }

            event.remove("response");
            event.remove("start");
            event.remove("granularity");
            event.remove("end");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
                event.append(
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
