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

            event.set("event.module", json!("ceph"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // Painless script
            // Source: if ((ctx.json?.client_io_rate?.write_op_per_sec != null && ctx.json?.client_io_rate?.write_op_per_sec!=\"\")||(ctx.json?.client_io_rate?.read_op_per_sec != null && ctx.json?.client_io_rate?.read_op_per_sec!=\"\")){\n    def totalOperation = 0;\n    ctx.total_activity = totalOperation + ctx.json?.client_io_rate?.write_op_per_sec + ctx.json?.client_io_rate?.read_op_per_sec;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if ((ctx.json?.client_io_rate?.write_op_per_sec != null && ctx.json?.client_io_rate?.write_op_per_sec!=\"\")||(ctx.json?.client_io_rate?.read_op_per_sec != null && ctx.json?.client_io_rate?.read_op_per_sec!=\"\")){\n    def totalOperation = 0;\n    ctx.total_activity = totalOperation + ctx.json?.client_io_rate?.write_op_per_sec + ctx.json?.client_io_rate?.read_op_per_sec;\n}\n"#
                ),
            )?;

            if event.has_value("total_activity") {
                event.rename("total_activity", "ceph.osd_pool_stats.client_io_rate.count")?;
            }

            if event.has_value("json.client_io_rate.read_bytes_sec") {
                event.rename(
                    "json.client_io_rate.read_bytes_sec",
                    "ceph.osd_pool_stats.client_io_rate.read.bytes",
                )?;
            }

            if event.has_value("json.client_io_rate.read_op_per_sec") {
                event.rename(
                    "json.client_io_rate.read_op_per_sec",
                    "ceph.osd_pool_stats.client_io_rate.read.count",
                )?;
            }

            if event.has_value("json.client_io_rate.write_bytes_sec") {
                event.rename(
                    "json.client_io_rate.write_bytes_sec",
                    "ceph.osd_pool_stats.client_io_rate.write.bytes",
                )?;
            }

            if event.has_value("json.client_io_rate.write_op_per_sec") {
                event.rename(
                    "json.client_io_rate.write_op_per_sec",
                    "ceph.osd_pool_stats.client_io_rate.write.count",
                )?;
            }

            if event.has_value("json.pool_id") {
                event.rename("json.pool_id", "ceph.osd_pool_stats.pool_id")?;
            }

            if event.has_value("json.pool_name") {
                event.rename("json.pool_name", "ceph.osd_pool_stats.pool_name")?;
            }

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            event.remove("json");

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
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
