// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `system_stats` pipeline.
pub struct SystemStats;

impl Transform for SystemStats {
    fn name(&self) -> &str {
        "system_stats"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("arista.activeHosts") {
                    event.rename("arista.activeHosts", "arista.hosts.active")?;
                }

                if event.has_value("arista.diskTotal") {
                    event.rename("arista.diskTotal", "arista.disk.total.bytes")?;
                }

                if event.has_value("arista.diskUsed") {
                    event.rename("arista.diskUsed", "arista.disk.used.bytes")?;
                }

                if event.has_value("arista.diskFree") {
                    event.rename("arista.diskFree", "arista.disk.free.bytes")?;
                }

                if event.has_value("arista.diskUsedPercent") {
                    event.rename("arista.diskUsedPercent", "arista.disk.used.pct")?;
                }

                if event.has_value("arista.diskFreePercent") {
                    event.rename("arista.diskFreePercent", "arista.disk.free.pct")?;
                }

                if event.has_value("arista.cpuSystem") {
                    event.rename("arista.cpuSystem", "arista.cpu.system.pct")?;
                }

                if event.has_value("arista.cpuUser") {
                    event.rename("arista.cpuUser", "arista.cpu.user.pct")?;
                }

            let _cond = { event.has_value("arista.cpu.system.pct") && event.has_value("arista.cpu.user.pct") };
            if _cond {
                // Painless script
                // Source: if (ctx.arista?.cpu?.total == null) {\n  Map map = new HashMap();\n  ctx.arista.cpu.put('total', map);\n}\nif (ctx.arista?.cpu?.system?.pct != null && ctx.arista?.cpu?.user?.pct != null) {\n  ctx.arista.cpu.total.pct = (ctx.arista.cpu.system.pct + ctx.arista.cpu.user.pct);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.arista?.cpu?.total == null) {\n  Map map = new HashMap();\n  ctx.arista.cpu.put('total', map);\n}\nif (ctx.arista?.cpu?.system?.pct != null && ctx.arista?.cpu?.user?.pct != null) {\n  ctx.arista.cpu.total.pct = (ctx.arista.cpu.system.pct + ctx.arista.cpu.user.pct);\n}"#))?;
            }

                if event.has_value("arista.load1") {
                    event.rename("arista.load1", "arista.cpu.load.1")?;
                }

                if event.has_value("arista.load5") {
                    event.rename("arista.load5", "arista.cpu.load.5")?;
                }

                if event.has_value("arista.load15") {
                    event.rename("arista.load15", "arista.cpu.load.15")?;
                }

                if event.has_value("arista.memTotal") {
                    event.rename("arista.memTotal", "arista.memory.total.bytes")?;
                }

                if event.has_value("arista.memUsed") {
                    event.rename("arista.memUsed", "arista.memory.used.bytes")?;
                }

                if event.has_value("arista.memFree") {
                    event.rename("arista.memFree", "arista.memory.free.bytes")?;
                }

                if event.has_value("arista.memCache") {
                    event.rename("arista.memCache", "arista.memory.cache.bytes")?;
                }

                if event.has_value("arista.memUsedPercent") {
                    event.rename("arista.memUsedPercent", "arista.memory.used.pct")?;
                }

                if event.has_value("arista.memFreePercent") {
                    event.rename("arista.memFreePercent", "arista.memory.free.pct")?;
                }

                if event.has_value("arista.memBuffers") {
                    event.rename("arista.memBuffers", "arista.memory.buffers")?;
                }

                if event.has_value("arista.swapTotal") {
                    event.rename("arista.swapTotal", "arista.memory.swap.total.bytes")?;
                }

                if event.has_value("arista.swapUsed") {
                    event.rename("arista.swapUsed", "arista.memory.swap.used.bytes")?;
                }

                if event.has_value("arista.swapFree") {
                    event.rename("arista.swapFree", "arista.memory.swap.free.bytes")?;
                }

                if event.has_value("arista.swapUsedPercent") {
                    event.rename("arista.swapUsedPercent", "arista.memory.swap.used.pct")?;
                }

                if event.has_value("arista.swapFreePercent") {
                    event.rename("arista.swapFreePercent", "arista.memory.swap.free.pct")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
