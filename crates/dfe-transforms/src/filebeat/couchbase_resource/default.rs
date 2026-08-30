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
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("database")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("couchbase");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels.instance") {
                    event.rename("prometheus.labels.instance", "server.address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_resource_utilization_process_cpu_percent_utilization",
                ) {
                    event.rename("prometheus.metrics.sgw_resource_utilization_process_cpu_percent_utilization", "couchbase.resource.process.cpu.pct")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_resource_utilization_process_memory_resident",
                ) {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_process_memory_resident",
                        "couchbase.resource.process.memory.resident",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_resource_utilization_admin_net_bytes_recv")
                {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_admin_net_bytes_recv",
                        "couchbase.resource.admin_net.bytes.received",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_resource_utilization_admin_net_bytes_sent")
                {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_admin_net_bytes_sent",
                        "couchbase.resource.admin_net.bytes.sent",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_resource_utilization_go_memstats_heapalloc")
                {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_go_memstats_heapalloc",
                        "couchbase.resource.go_memstats.heap.alloc",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_resource_utilization_go_memstats_heapinuse")
                {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_go_memstats_heapinuse",
                        "couchbase.resource.go_memstats.heap.in_use",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_resource_utilization_go_memstats_heapidle")
                {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_go_memstats_heapidle",
                        "couchbase.resource.go_memstats.heap.idle",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_resource_utilization_go_memstats_heapreleased",
                ) {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_go_memstats_heapreleased",
                        "couchbase.resource.go_memstats.heap.released",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_resource_utilization_go_memstats_stackinuse")
                {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_go_memstats_stackinuse",
                        "couchbase.resource.go_memstats.stack.in_use",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_resource_utilization_error_count") {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_error_count",
                        "couchbase.resource.error.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_resource_utilization_warn_count") {
                    event.rename(
                        "prometheus.metrics.sgw_resource_utilization_warn_count",
                        "couchbase.resource.warn.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.go_memstats_last_gc_time_seconds") {
                    event.rename(
                        "prometheus.metrics.go_memstats_last_gc_time_seconds",
                        "couchbase.resource.last_gc",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.tags == null) {\n    ctx.tags = new ArrayList();\n}\nctx.tags.add(ctx.prometheus.labels.job)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.tags == null) {\n    ctx.tags = new ArrayList();\n}\nctx.tags.add(ctx.prometheus.labels.job)\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("prometheus");
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
