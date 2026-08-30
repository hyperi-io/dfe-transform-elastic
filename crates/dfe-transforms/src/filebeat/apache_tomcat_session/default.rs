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

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("apache_tomcat"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_Manager_persistAuthentication")
                    == Some(1)
            };
            if _cond {
                event.set("apache_tomcat.session.persist_authentication", json!(true))?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_Manager_persistAuthentication")
                    == Some(0)
            };
            if _cond {
                event.set("apache_tomcat.session.persist_authentication", json!(false))?;
            }

            if event.has_value("prometheus.labels.context") {
                event.rename(
                    "prometheus.labels.context",
                    "apache_tomcat.session.application_name",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_maxActiveSessions") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_maxActiveSessions",
                    "apache_tomcat.session.active.allowed.max",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_maxActive") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_maxActive",
                    "apache_tomcat.session.active.max",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_activeSessions") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_activeSessions",
                    "apache_tomcat.session.active.total",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_sessionAverageAliveTime") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_sessionAverageAliveTime",
                    "apache_tomcat.session.alive_time.avg",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_sessionMaxAliveTime") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_sessionMaxAliveTime",
                    "apache_tomcat.session.alive_time.max",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_sessionCreateRate") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_sessionCreateRate",
                    "apache_tomcat.session.create.rate",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_sessionCounter") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_sessionCounter",
                    "apache_tomcat.session.create.total",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_duplicates") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_duplicates",
                    "apache_tomcat.session.duplicate_ids.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_sessionExpireRate") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_sessionExpireRate",
                    "apache_tomcat.session.expire.rate",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_expiredSessions") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_expiredSessions",
                    "apache_tomcat.session.expire.total",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_rejectedSessions") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_rejectedSessions",
                    "apache_tomcat.session.rejected.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_processExpiresFrequency") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_processExpiresFrequency",
                    "apache_tomcat.session.process_expires_frequency.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_Manager_processingTime") {
                event.rename(
                    "prometheus.metrics.Catalina_Manager_processingTime",
                    "apache_tomcat.session.processing_time",
                )?;
            }

            event.remove("prometheus");

            // Painless script
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#
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
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
