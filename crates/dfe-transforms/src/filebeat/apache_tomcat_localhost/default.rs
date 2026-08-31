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

            event.set("event.kind", json!("event"))?;

            event.set("event.module", json!("apache_tomcat"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?P<_tmp_timestamp>(?:%{MONTHDAY}-%{MONTH}-%{YEAR} %{TIME})) %{DATA:log.level} \\[%{DATA:apache_tomcat.localhost.subsystem}\\] %{GREEDYDATA:message}(?P<error_stack_trace>(?:(.|\\n\\t|\\n\\t\\t|\\n)*))$
                    // Grok pattern: ^(?P<_tmp_timestamp>(?:%{MONTHDAY}-%{MONTH}-%{YEAR} %{TIME})) %{DATA:log.level} \\[%{DATA:apache_tomcat.localhost.subsystem}\\] %{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^(?P<_tmp_timestamp>(?:%{MONTHDAY}-%{MONTH}-%{YEAR} %{TIME})) %{DATA:log.level} \\[%{DATA:apache_tomcat.localhost.subsystem}\\] %{GREEDYDATA:message}(?P<error_stack_trace>(?:(.|\\n\\t|\\n\\t\\t|\\n)*))$",
                                [
                                    ("_tmp_timestamp", "_tmp.timestamp"),
                                    ("error_stack_trace", "error.stack_trace")
                                ]
                            ),
                            cached_grok_mapped!(
                                "^(?P<_tmp_timestamp>(?:%{MONTHDAY}-%{MONTH}-%{YEAR} %{TIME})) %{DATA:log.level} \\[%{DATA:apache_tomcat.localhost.subsystem}\\] %{GREEDYDATA:message}$",
                                [("_tmp_timestamp", "_tmp.timestamp")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_parse_log")?;
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

            let _cond = { event.get_str("error.stack_trace") != Some("") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("error")]))?;
            }

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
            }

            if !event.has("event.timezone") {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("_tmp.timestamp") && event.get_str("_tmp.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["dd-MMM-yyyy HH:mm:ss.SSS"],
                            event.get_str("event.timezone"),
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
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_timestamp")?;
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
            }

            if event.has_value("log.level") {
                map_strings(event, "log.level", "log.level", str::to_lowercase)?;
            }

            event.remove("_tmp");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
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
