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

            event.set("event.dataset", json!("golang.expvar"))?;

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

            if event.has_value("json.memstats.BuckHashSys") {
                event.rename(
                    "json.memstats.BuckHashSys",
                    "golang.expvar.buck_hash_sys.bytes",
                )?;
            }

            if event.has_value("json.cmdline") {
                event.rename("json.cmdline", "golang.expvar.cmdline")?;
            }

            if event.has_value("json.memstats.LastGC") {
                event.rename("json.memstats.LastGC", "golang.expvar.gc.last_finished.ns")?;
            }

            if event.has_value("json.memstats.NumForcedGC") {
                event.rename("json.memstats.NumForcedGC", "golang.expvar.gc.forced")?;
            }

            if event.has_value("json.memstats.GCSys") {
                event.rename(
                    "json.memstats.GCSys",
                    "golang.expvar.gc.metadata.memory.bytes",
                )?;
            }

            if event.has_value("json.memstats.MCacheInuse") {
                event.rename(
                    "json.memstats.MCacheInuse",
                    "golang.expvar.mcache.allocated.bytes",
                )?;
            }

            if event.has_value("json.memstats.MCacheSys") {
                event.rename(
                    "json.memstats.MCacheSys",
                    "golang.expvar.mcache.obtained.bytes",
                )?;
            }

            if event.has_value("json.memstats.MSpanInuse") {
                event.rename(
                    "json.memstats.MSpanInuse",
                    "golang.expvar.mspan.allocated.bytes",
                )?;
            }

            if event.has_value("json.memstats.MSpanSys") {
                event.rename(
                    "json.memstats.MSpanSys",
                    "golang.expvar.mspan.obtained.bytes",
                )?;
            }

            if event.has_value("json.memstats.OtherSys") {
                event.rename(
                    "json.memstats.OtherSys",
                    "golang.expvar.obtained.miscellaneous.bytes",
                )?;
            }

            if event.has_value("json.memstats.Sys") {
                event.rename("json.memstats.Sys", "golang.expvar.obtained.total.bytes")?;
            }

            if event.has_value("json.memstats.Lookups") {
                event.rename("json.memstats.Lookups", "golang.expvar.pointer.lookups")?;
            }

            if event.has_value("json.memstats.StackInuse") {
                event.rename("json.memstats.StackInuse", "golang.expvar.stack.bytes")?;
            }

            event.remove("json.memstats");

            if event.has_value("json") {
                event.rename("json", "golang.expvar.custom")?;
            }

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
