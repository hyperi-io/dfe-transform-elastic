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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.was_connectionpool_jdbcOperation_time_seconds_total",
                ) {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_jdbcOperation_time_seconds_total",
                        "websphere_application_server.jdbc.connection.total.operations_seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_jdbcOperations_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_jdbcOperations_total",
                        "websphere_application_server.jdbc.connection.total.operations_calls",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_create_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_create_total",
                        "websphere_application_server.jdbc.connection.created",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_close_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_close_total",
                        "websphere_application_server.jdbc.connection.closed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_allocate_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_allocate_total",
                        "websphere_application_server.jdbc.connection.allocated",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_return_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_return_total",
                        "websphere_application_server.jdbc.connection.returned",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_free_connections") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_free_connections",
                        "websphere_application_server.jdbc.connection.free",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_fault_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_fault_total",
                        "websphere_application_server.jdbc.connection.total.fault",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_waiting_threads") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_waiting_threads",
                        "websphere_application_server.jdbc.connection.waiting_threads",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_inUse_time_seconds_total")
                {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_inUse_time_seconds_total",
                        "websphere_application_server.jdbc.connection.total.seconds_in_use",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_inUse_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_inUse_total",
                        "websphere_application_server.jdbc.connection.total.in_use",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_wait_time_seconds_total")
                {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_wait_time_seconds_total",
                        "websphere_application_server.jdbc.connection.total.wait_seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_wait_total") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_wait_total",
                        "websphere_application_server.jdbc.connection.total.wait",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_managed_connections") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_managed_connections",
                        "websphere_application_server.jdbc.connection.managed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_connectionHandles") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_connectionHandles",
                        "websphere_application_server.jdbc.connection.handles",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_size") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_size",
                        "websphere_application_server.jdbc.pool_size",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_connectionpool_utilization") {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_utilization",
                        "websphere_application_server.jdbc.percent_used",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.was_connectionpool_prepStmtCacheDiscard_total")
                {
                    event.rename(
                        "prometheus.metrics.was_connectionpool_prepStmtCacheDiscard_total",
                        "websphere_application_server.jdbc.total_cache_discarded",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels.jndiname") {
                    event.rename(
                        "prometheus.labels.jndiname",
                        "websphere_application_server.jdbc.data_source",
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
                let v = json!("web");
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
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
                let v = json!("websphere_application_server");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("info");
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                    ),
                )?;
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
