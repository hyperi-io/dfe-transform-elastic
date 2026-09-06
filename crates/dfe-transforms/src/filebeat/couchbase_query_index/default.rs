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
                // Painless script
                // Source: def getLastElement(def arr) {\n  def last_index = arr.length - 1;\n  return arr[last_index];\n}\nif (ctx.http?.query_index?.op?.samples != null) {\n  if (ctx.http.query_index.op.samples.index_remaining_ram != null) {\n    ctx.index_remaining_ram = getLastElement(ctx.http.query_index.op.samples.index_remaining_ram);\n  }\n  if (ctx.http.query_index.op.samples.index_ram_percent != null) {\n    ctx.index_ram_percent = getLastElement(ctx.http.query_index.op.samples.index_ram_percent);\n  }\n  if (ctx.http.query_index.op.samples.query_avg_req_time != null) {\n    ctx.query_avg_req_time = getLastElement(ctx.http.query_index.op.samples.query_avg_req_time);\n  }\n  if (ctx.http.query_index.op.samples.query_requests != null) {\n    ctx.query_requests = getLastElement(ctx.http.query_index.op.samples.query_requests);\n  }\n  if (ctx.http.query_index.op.samples.query_result_count != null) {\n    ctx.query_result_count = getLastElement(ctx.http.query_index.op.samples.query_result_count);\n  }\n  if (ctx.http.query_index.op.samples['eventing/failed_count'] != null) {\n    ctx.eventing_failed_count = getLastElement(ctx.http.query_index.op.samples['eventing/failed_count']);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def getLastElement(def arr) {\n  def last_index = arr.length - 1;\n  return arr[last_index];\n}\nif (ctx.http?.query_index?.op?.samples != null) {\n  if (ctx.http.query_index.op.samples.index_remaining_ram != null) {\n    ctx.index_remaining_ram = getLastElement(ctx.http.query_index.op.samples.index_remaining_ram);\n  }\n  if (ctx.http.query_index.op.samples.index_ram_percent != null) {\n    ctx.index_ram_percent = getLastElement(ctx.http.query_index.op.samples.index_ram_percent);\n  }\n  if (ctx.http.query_index.op.samples.query_avg_req_time != null) {\n    ctx.query_avg_req_time = getLastElement(ctx.http.query_index.op.samples.query_avg_req_time);\n  }\n  if (ctx.http.query_index.op.samples.query_requests != null) {\n    ctx.query_requests = getLastElement(ctx.http.query_index.op.samples.query_requests);\n  }\n  if (ctx.http.query_index.op.samples.query_result_count != null) {\n    ctx.query_result_count = getLastElement(ctx.http.query_index.op.samples.query_result_count);\n  }\n  if (ctx.http.query_index.op.samples['eventing/failed_count'] != null) {\n    ctx.eventing_failed_count = getLastElement(ctx.http.query_index.op.samples['eventing/failed_count']);\n  }\n}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("index_remaining_ram") {
                    event.rename("index_remaining_ram", "couchbase.query_index.ram.remaining")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("index_ram_percent") {
                    event.rename("index_ram_percent", "couchbase.query_index.ram.pct")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("query_avg_req_time") {
                    event.rename(
                        "query_avg_req_time",
                        "couchbase.query_index.query.request_time.avg",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("query_requests") {
                    event.rename("query_requests", "couchbase.query_index.query.requests")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("query_result_count") {
                    event.rename(
                        "query_result_count",
                        "couchbase.query_index.query.result.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("eventing_failed_count") {
                    event.rename(
                        "eventing_failed_count",
                        "couchbase.query_index.eventing.failed.count",
                    )?;
                }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("http");
                Ok(())
            })();

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
