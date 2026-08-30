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

            event.set(
                "event.type",
                Value::Array(vec![json!("access"), json!("info")]),
            )?;

            if event.has_value("groupId") {
                event.rename("groupId", "mongodb_atlas.group_id")?;
            }

            let v = json!(
                event
                    .get("mongodb_atlas.group_id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("group.id", v)?;
            }

            if event.has_value("hardware.FTS_DISK_USAGE") {
                event.rename(
                    "hardware.FTS_DISK_USAGE",
                    "mongodb_atlas.hardware.fts.disk_usage.bytes",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_CPU_KERNEL") {
                event.rename(
                    "hardware.FTS_PROCESS_CPU_KERNEL",
                    "mongodb_atlas.hardware.fts.process.cpu.kernel.pct",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_CPU_USER") {
                event.rename(
                    "hardware.FTS_PROCESS_CPU_USER",
                    "mongodb_atlas.hardware.fts.process.cpu.user.pct",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_RESIDENT_MEMORY") {
                event.rename(
                    "hardware.FTS_PROCESS_RESIDENT_MEMORY",
                    "mongodb_atlas.hardware.fts.process.memory.resident.bytes",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_SHARED_MEMORY") {
                event.rename(
                    "hardware.FTS_PROCESS_SHARED_MEMORY",
                    "mongodb_atlas.hardware.fts.process.memory.shared.bytes",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_VIRTUAL_MEMORY") {
                event.rename(
                    "hardware.FTS_PROCESS_VIRTUAL_MEMORY",
                    "mongodb_atlas.hardware.fts.process.memory.virtual.bytes",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_NORMALIZED_CPU_KERNEL") {
                event.rename(
                    "hardware.FTS_PROCESS_NORMALIZED_CPU_KERNEL",
                    "mongodb_atlas.hardware.fts.process.normalized.cpu.kernel.pct",
                )?;
            }

            if event.has_value("hardware.FTS_PROCESS_NORMALIZED_CPU_USER") {
                event.rename(
                    "hardware.FTS_PROCESS_NORMALIZED_CPU_USER",
                    "mongodb_atlas.hardware.fts.process.normalized.cpu.user.pct",
                )?;
            }

            if event.has_value("status.JVM_MAX_MEMORY") {
                event.rename(
                    "status.JVM_MAX_MEMORY",
                    "mongodb_atlas.hardware.status.jvm.memory.heap.available.mb",
                )?;
            }

            if event.has_value("status.JVM_CURRENT_MEMORY") {
                event.rename(
                    "status.JVM_CURRENT_MEMORY",
                    "mongodb_atlas.hardware.status.jvm.memory.heap.used.mb",
                )?;
            }

            if event.has_value("status.PAGE_FAULTS") {
                event.rename(
                    "status.PAGE_FAULTS",
                    "mongodb_atlas.hardware.status.page_faults",
                )?;
            }

            if event.has_value("processId") {
                event.rename("processId", "mongodb_atlas.process_id")?;
            }

            event.remove("end");
            event.remove("granularity");
            event.remove("hardware");
            event.remove("start");
            event.remove("status");

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
