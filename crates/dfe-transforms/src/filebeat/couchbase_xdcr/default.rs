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
                // Source: def getLastElement(def x) {\n  def index = x.length - 1;\n  return x[index];\n}\nif (ctx.http?.xdcr?.op?.samples != null) {\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_backoff != null) {\n    ctx.backoff = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_backoff);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_total_bytes != null) {\n    ctx.bytes_total = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_total_bytes);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_count != null) {\n    ctx.count = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_count);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_remaining != null) {\n    ctx.items_remaining = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_remaining);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_sent != null) {\n    ctx.items_sent = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_sent);\n  }\n  if (ctx.http.xdcr.op.samples.ep_oom_errors != null) {\n    ctx.oom_errors = getLastElement(ctx.http.xdcr.op.samples.ep_oom_errors);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_producer_count != null) {\n    ctx.producer_count = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_producer_count);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def getLastElement(def x) {\n  def index = x.length - 1;\n  return x[index];\n}\nif (ctx.http?.xdcr?.op?.samples != null) {\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_backoff != null) {\n    ctx.backoff = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_backoff);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_total_bytes != null) {\n    ctx.bytes_total = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_total_bytes);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_count != null) {\n    ctx.count = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_count);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_remaining != null) {\n    ctx.items_remaining = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_remaining);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_sent != null) {\n    ctx.items_sent = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_items_sent);\n  }\n  if (ctx.http.xdcr.op.samples.ep_oom_errors != null) {\n    ctx.oom_errors = getLastElement(ctx.http.xdcr.op.samples.ep_oom_errors);\n  }\n  if (ctx.http.xdcr.op.samples.ep_dcp_xdcr_producer_count != null) {\n    ctx.producer_count = getLastElement(ctx.http.xdcr.op.samples.ep_dcp_xdcr_producer_count);\n  }\n}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("backoff") {
                    event.rename("backoff", "couchbase.xdcr.backoff")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("bytes_total") {
                    event.rename("bytes_total", "couchbase.xdcr.bytes.total")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("count") {
                    event.rename("count", "couchbase.xdcr.count")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("items_remaining") {
                    event.rename("items_remaining", "couchbase.xdcr.items.remaining")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("items_sent") {
                    event.rename("items_sent", "couchbase.xdcr.items.sent")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oom_errors") {
                    event.rename("oom_errors", "couchbase.xdcr.errors.out_of_memory")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("producer_count") {
                    event.rename("producer_count", "couchbase.xdcr.producer.count")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("http");
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
