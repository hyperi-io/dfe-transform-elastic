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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}%{LOGLEVEL:log.level}%{SPACE}%{DATA:process.name}(?:\\\\[%{GREEDYDATA:process.entrypoint}\\\\])?:%{SPACE}%{GREEDYDATA:message}%{SPACE}(?P<process_message>(?:(.|\\n)*))
                    if !cached_grok_mapped!("%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}%{LOGLEVEL:log.level}%{SPACE}%{DATA:process.name}(?:\\\\[%{GREEDYDATA:process.entrypoint}\\\\])?:%{SPACE}%{GREEDYDATA:message}%{SPACE}(?P<process_message>(?:(.|\\n)*))", [("process_message", "process.message")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd HH:mm:ss,SSS"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("_tmp");

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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
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
