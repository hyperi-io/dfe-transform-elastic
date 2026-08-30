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
                if event.has_value("prometheus.labels.appname") {
                    event.rename(
                        "prometheus.labels.appname",
                        "websphere_application_server.session_manager.app_name",
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
                if event.has_value("prometheus.metrics.was_session_activateNonExistSession_total") {
                    event.rename("prometheus.metrics.was_session_activateNonExistSession_total", "websphere_application_server.session_manager.activated_non_existent_sessions")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_active_sessions") {
                    event.rename(
                        "prometheus.metrics.was_session_active_sessions",
                        "websphere_application_server.session_manager.sessions.active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_affinityBreak_total") {
                    event.rename(
                        "prometheus.metrics.was_session_affinityBreak_total",
                        "websphere_application_server.session_manager.affinity_breaks",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_cacheDiscard_total") {
                    event.rename(
                        "prometheus.metrics.was_session_cacheDiscard_total",
                        "websphere_application_server.session_manager.cache_discarded",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_create_total") {
                    event.rename(
                        "prometheus.metrics.was_session_create_total",
                        "websphere_application_server.session_manager.sessions.created",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_live_sessions") {
                    event.rename(
                        "prometheus.metrics.was_session_live_sessions",
                        "websphere_application_server.session_manager.sessions.current",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_externalRead_bytes_total") {
                    event.rename(
                        "prometheus.metrics.was_session_externalRead_bytes_total",
                        "websphere_application_server.session_manager.external.bytes.read",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_externalRead_seconds_total") {
                    event.rename(
                        "prometheus.metrics.was_session_externalRead_seconds_total",
                        "websphere_application_server.session_manager.external.time_seconds.read",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_externalWrite_bytes_total") {
                    event.rename(
                        "prometheus.metrics.was_session_externalWrite_bytes_total",
                        "websphere_application_server.session_manager.external.bytes.written",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_externalWrite_seconds_total") {
                    event.rename("prometheus.metrics.was_session_externalWrite_seconds_total", "websphere_application_server.session_manager.external.time_seconds.written")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_invalidated_total") {
                    event.rename(
                        "prometheus.metrics.was_session_invalidated_total",
                        "websphere_application_server.session_manager.sessions.invalidated.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_invalidatedByTimeout_total") {
                    event.rename("prometheus.metrics.was_session_invalidatedByTimeout_total", "websphere_application_server.session_manager.sessions.invalidated.by_timeouts")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_noRoomForNewSession_total") {
                    event.rename(
                        "prometheus.metrics.was_session_noRoomForNewSession_total",
                        "websphere_application_server.session_manager.no_room_for_new_sessions",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_externalRead_total") {
                    event.rename(
                        "prometheus.metrics.was_session_externalRead_total",
                        "websphere_application_server.session_manager.persistent_stores.data_read",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_externalWrite_total") {
                    event.rename("prometheus.metrics.was_session_externalWrite_total", "websphere_application_server.session_manager.persistent_stores.data_written")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.was_session_life_time_seconds_total") {
                    event.rename(
                        "prometheus.metrics.was_session_life_time_seconds_total",
                        "websphere_application_server.session_manager.sessions.life_time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.was_session_timeSinceLastActivated_seconds_total",
                ) {
                    event.rename("prometheus.metrics.was_session_timeSinceLastActivated_seconds_total", "websphere_application_server.session_manager.time_since_session_last_activated")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.tags == null) {\n  ctx.tags = new ArrayList();\n}\nctx.tags.add(ctx.prometheus.labels.job)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.tags == null) {\n  ctx.tags = new ArrayList();\n}\nctx.tags.add(ctx.prometheus.labels.job)\n"#
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
