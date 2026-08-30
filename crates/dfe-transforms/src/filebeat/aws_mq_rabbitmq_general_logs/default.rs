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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                event.rename("message", "event.original")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("cloud.service.name", json!("amazonmq_rabbitmq"))?;

            event.set("cloud.provider", json!("aws"))?;

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{TIMESTAMP_ISO8601:timestamp} \\[%{WORD:log.level}\\] (?P<rabbitmq_log_pid>(?:\\<%{INT}\\.%{INT}\\.%{INT}\\>))\\s* (?P<message>(?:(.|\n)*))
                    let _ = cached_grok_mapped!("%{TIMESTAMP_ISO8601:timestamp} \\[%{WORD:log.level}\\] (?P<rabbitmq_log_pid>(?:\\<%{INT}\\.%{INT}\\.%{INT}\\>))\\s* (?P<message>(?:(.|\n)*))", [("rabbitmq_log_pid", "rabbitmq.log.pid")]).extract_into(&input, event)?;
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss.SSSSSSZZZZZ"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss.SSSSSSZZZZZ"],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.remove("timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp".into(),
                });
            }

            let _cond = { event.has_value("log.level") };
            if _cond {
                // Painless script
                // Source: def err_levels = [\"FATAL\", \"ERROR\", \"WARN\"]; if (err_levels.contains(ctx.log.level)) {\n  ctx.event.type = [\"error\"];\n} else {\n  ctx.event.type = [\"info\"];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def err_levels = [\"FATAL\", \"ERROR\", \"WARN\"]; if (err_levels.contains(ctx.log.level)) {\n  ctx.event.type = [\"error\"];\n} else {\n  ctx.event.type = [\"info\"];\n}"#
                    ),
                )?;
            }

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.set(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
