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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("prometheus.labels.name") {
                    if let Some(input) = event.get_string("prometheus.labels.name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("\"") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\"") else {
                                break 'dissect false;
                            };
                            captured
                                .push(("apache_tomcat.request.nio_connector", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\"") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "prometheus.labels.name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_nio_connector")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("prometheus.metrics.Catalina_GlobalRequestProcessor_requestCount") {
                event.rename(
                    "prometheus.metrics.Catalina_GlobalRequestProcessor_requestCount",
                    "apache_tomcat.request.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_GlobalRequestProcessor_errorCount") {
                event.rename(
                    "prometheus.metrics.Catalina_GlobalRequestProcessor_errorCount",
                    "apache_tomcat.request.error.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_GlobalRequestProcessor_maxTime") {
                event.rename(
                    "prometheus.metrics.Catalina_GlobalRequestProcessor_maxTime",
                    "apache_tomcat.request.time.max",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_GlobalRequestProcessor_bytesReceived") {
                event.rename(
                    "prometheus.metrics.Catalina_GlobalRequestProcessor_bytesReceived",
                    "apache_tomcat.request.received.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_GlobalRequestProcessor_bytesSent") {
                event.rename(
                    "prometheus.metrics.Catalina_GlobalRequestProcessor_bytesSent",
                    "apache_tomcat.request.sent.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_GlobalRequestProcessor_processingTime")
            {
                event.rename(
                    "prometheus.metrics.Catalina_GlobalRequestProcessor_processingTime",
                    "apache_tomcat.request.time.total",
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

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
