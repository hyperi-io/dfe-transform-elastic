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

            event.set("event.dataset", json!("golang.heap"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("golang"))?;

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

            if event.has_value("json.memstats.HeapInuse") {
                event.rename(
                    "json.memstats.HeapInuse",
                    "golang.heap.allocations.active.bytes",
                )?;
            }

            if event.has_value("json.memstats.Frees") {
                event.rename("json.memstats.Frees", "golang.heap.allocations.frees.count")?;
            }

            if event.has_value("json.memstats.HeapIdle") {
                event.rename(
                    "json.memstats.HeapIdle",
                    "golang.heap.allocations.idle.bytes",
                )?;
            }

            if event.has_value("json.memstats.HeapAlloc") {
                event.rename(
                    "json.memstats.HeapAlloc",
                    "golang.heap.allocations.object.bytes",
                )?;
            }

            if event.has_value("json.memstats.HeapObjects") {
                event.rename(
                    "json.memstats.HeapObjects",
                    "golang.heap.allocations.object.count",
                )?;
            }

            if event.has_value("json.memstats.TotalAlloc") {
                event.rename(
                    "json.memstats.TotalAlloc",
                    "golang.heap.allocations.total.bytes",
                )?;
            }

            if event.has_value("json.cmdline") {
                event.rename("json.cmdline", "golang.heap.cmdline")?;
            }

            if event.has_value("json.memstats.GCCPUFraction") {
                event.rename("json.memstats.GCCPUFraction", "golang.heap.gc.cpu_fraction")?;
            }

            if event.has_value("json.memstats.NextGC") {
                event.rename("json.memstats.NextGC", "golang.heap.gc.next_gc_limit")?;
            }

            if event.has_value("json.memstats.NumGC") {
                event.rename("json.memstats.NumGC", "golang.heap.gc.total.count")?;
            }

            if event.has_value("json.memstats.PauseTotalNs") {
                event.rename(
                    "json.memstats.PauseTotalNs",
                    "golang.heap.gc.pause.total.ns",
                )?;
            }

            if event.has_value("json.memstats.Mallocs") {
                event.rename("json.memstats.Mallocs", "golang.heap.mallocs.count")?;
            }

            if event.has_value("json.memstats.HeapReleased") {
                event.rename(
                    "json.memstats.HeapReleased",
                    "golang.heap.system.released.bytes",
                )?;
            }

            if event.has_value("json.memstats.StackSys") {
                event.rename("json.memstats.StackSys", "golang.heap.system.stack.bytes")?;
            }

            if event.has_value("json.memstats.HeapSys") {
                event.rename("json.memstats.HeapSys", "golang.heap.system.total.bytes")?;
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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
