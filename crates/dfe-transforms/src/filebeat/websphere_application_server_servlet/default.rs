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
                if event.has_value("prometheus.metrics.was_servlet_requests_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_requests_total",
                        "websphere_application_server.servlet.requests.processed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_response_time_seconds_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_response_time_seconds_total",
                        "websphere_application_server.servlet.response_time_seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_responses_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_responses_total",
                        "websphere_application_server.servlet.responses.processed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_loaded_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_loaded_total",
                        "websphere_application_server.servlet.loaded",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_reload_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_reload_total",
                        "websphere_application_server.servlet.reloaded",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_errors_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_errors_total",
                        "websphere_application_server.servlet.errors",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_concurrent_requests") {
                    event.rename(
                        "prometheus.metrics.was_servlet_concurrent_requests",
                        "websphere_application_server.servlet.requests.concurrent",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.was_servlet_asyncContext_response_time_seconds_total",
                ) {
                    event.rename(
                        "prometheus.metrics.was_servlet_asyncContext_response_time_seconds_total",
                        "websphere_application_server.servlet.async_context.response_time_seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_asyncContext_responses_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_asyncContext_responses_total",
                        "websphere_application_server.servlet.async_context.responses.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_uri_requests_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_uri_requests_total",
                        "websphere_application_server.servlet.uri.requests.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_uri_concurrent_requests") {
                    event.rename(
                        "prometheus.metrics.was_servlet_uri_concurrent_requests",
                        "websphere_application_server.servlet.uri.requests.concurrent",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_uri_response_time_seconds_total")
                {
                    event.rename(
                        "prometheus.metrics.was_servlet_uri_response_time_seconds_total",
                        "websphere_application_server.servlet.uri.response_time_seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_servlet_uri_responses_total") {
                    event.rename(
                        "prometheus.metrics.was_servlet_uri_responses_total",
                        "websphere_application_server.servlet.uri.responses.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.was_servlet_uri_asyncContext_response_time_seconds_total",
                ) {
                    event.rename("prometheus.metrics.was_servlet_uri_asyncContext_response_time_seconds_total", "websphere_application_server.servlet.uri.async_context.response_time_seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.was_servlet_uri_asyncContext_responses_total")
                {
                    event.rename(
                        "prometheus.metrics.was_servlet_uri_asyncContext_responses_total",
                        "websphere_application_server.servlet.uri.async_context.responses.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels.appname") {
                    event.rename(
                        "prometheus.labels.appname",
                        "websphere_application_server.servlet.app_name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels.uri") {
                    event.rename("prometheus.labels.uri", "url.path")?;
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
