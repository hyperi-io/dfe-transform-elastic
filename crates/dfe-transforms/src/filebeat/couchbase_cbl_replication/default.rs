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
                if event.has_value("prometheus.labels.database") {
                    event.rename(
                        "prometheus.labels.database",
                        "couchbase.cbl_replication.database.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_conflict_write_count") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_conflict_write_count",
                        "couchbase.cbl_replication.push.conflict.write.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_write_processing_time")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_write_processing_time",
                        "couchbase.cbl_replication.push.write.processing.time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_doc_push_count") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_doc_push_count",
                        "couchbase.cbl_replication.push.doc.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_sync_function_time") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_sync_function_time",
                        "couchbase.cbl_replication.push.sync.function.time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_propose_change_time") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_propose_change_time",
                        "couchbase.cbl_replication.push.propose.change.time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_propose_change_count") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_propose_change_count",
                        "couchbase.cbl_replication.push.propose.change.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_attachment_push_count")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_attachment_push_count",
                        "couchbase.cbl_replication.push.attachment.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_push_attachment_push_bytes")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_push_attachment_push_bytes",
                        "couchbase.cbl_replication.push.attachment.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_pull_request_changes_time") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_request_changes_time",
                        "couchbase.cbl_replication.pull.request.changes.time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_pull_rev_send_latency") {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_rev_send_latency",
                        "couchbase.cbl_replication.pull.rev.latency.send",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_pull_attachment_pull_count")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_attachment_pull_count",
                        "couchbase.cbl_replication.pull.attachment.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_replication_pull_attachment_pull_bytes")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_attachment_pull_bytes",
                        "couchbase.cbl_replication.pull.attachment.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_replication_pull_num_pull_repl_total_one_shot",
                ) {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_num_pull_repl_total_one_shot",
                        "couchbase.cbl_replication.pull.num.one_shot.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_replication_pull_num_pull_repl_total_continuous",
                ) {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_num_pull_repl_total_continuous",
                        "couchbase.cbl_replication.pull.num.continuous.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_replication_pull_num_pull_repl_active_one_shot",
                ) {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_num_pull_repl_active_one_shot",
                        "couchbase.cbl_replication.pull.num.one_shot.active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.sgw_replication_pull_num_pull_repl_active_continuous",
                ) {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_num_pull_repl_active_continuous",
                        "couchbase.cbl_replication.pull.num.continuous.active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_replication_pull_num_pull_repl_since_zero")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_num_pull_repl_since_zero",
                        "couchbase.cbl_replication.pull.num.since_zero",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.sgw_replication_pull_num_pull_repl_caught_up")
                {
                    event.rename(
                        "prometheus.metrics.sgw_replication_pull_num_pull_repl_caught_up",
                        "couchbase.cbl_replication.pull.num.caught_up",
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
