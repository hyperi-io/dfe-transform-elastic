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
                if event.has_value("prometheus.labels.database") {
                    event.rename(
                        "prometheus.labels.database",
                        "couchbase.miscellaneous.database.name",
                    )?;
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
                if event.has_value("prometheus.metrics.sgw_delta_sync_delta_cache_hit") {
                    event.rename(
                        "prometheus.metrics.sgw_delta_sync_delta_cache_hit",
                        "couchbase.miscellaneous.delta_sync.cache.hits",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_delta_sync_deltas_sent") {
                    event.rename(
                        "prometheus.metrics.sgw_delta_sync_deltas_sent",
                        "couchbase.miscellaneous.delta_sync.sent",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_delta_sync_deltas_requested") {
                    event.rename(
                        "prometheus.metrics.sgw_delta_sync_deltas_requested",
                        "couchbase.miscellaneous.delta_sync.requested",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_delta_sync_delta_pull_replication_count")
                {
                    event.rename(
                        "prometheus.metrics.sgw_delta_sync_delta_pull_replication_count",
                        "couchbase.miscellaneous.delta_sync.pull.replications",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_delta_sync_delta_push_doc_count") {
                    event.rename(
                        "prometheus.metrics.sgw_delta_sync_delta_push_doc_count",
                        "couchbase.miscellaneous.delta_sync.push.documents",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_shared_bucket_import_import_count") {
                    event.rename(
                        "prometheus.metrics.sgw_shared_bucket_import_import_count",
                        "couchbase.miscellaneous.shared_bucket.import.documents.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_shared_bucket_import_import_error_count")
                {
                    event.rename(
                        "prometheus.metrics.sgw_shared_bucket_import_import_error_count",
                        "couchbase.miscellaneous.shared_bucket.import.documents.errors.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_gsi_views_channels_count") {
                    event.rename(
                        "prometheus.metrics.sgw_gsi_views_channels_count",
                        "couchbase.miscellaneous.gsi_views.channels.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_gsi_views_access_count") {
                    event.rename(
                        "prometheus.metrics.sgw_gsi_views_access_count",
                        "couchbase.miscellaneous.gsi_views.access.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_gsi_views_allDocs_count") {
                    event.rename(
                        "prometheus.metrics.sgw_gsi_views_allDocs_count",
                        "couchbase.miscellaneous.gsi_views.all_docs.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_gsi_views_roleAccess_count") {
                    event.rename(
                        "prometheus.metrics.sgw_gsi_views_roleAccess_count",
                        "couchbase.miscellaneous.gsi_views.role_access.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_security_num_docs_rejected") {
                    event.rename(
                        "prometheus.metrics.sgw_security_num_docs_rejected",
                        "couchbase.miscellaneous.security.documents.rejected.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_security_num_access_errors") {
                    event.rename(
                        "prometheus.metrics.sgw_security_num_access_errors",
                        "couchbase.miscellaneous.security.access.errors.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.sgw_security_auth_failed_count") {
                    event.rename(
                        "prometheus.metrics.sgw_security_auth_failed_count",
                        "couchbase.miscellaneous.security.authentications.failed.count",
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
