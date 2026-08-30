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
                let v = json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.ingested", v)?;
                }
                Ok(())
            })();

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
                let v = json!("cassandra");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{LOGLEVEL:log.level}%{SPACE}\\[%{GREEDYDATA:process.thread.name}\\]%{SPACE}%{TIMESTAMP_ISO8601:cassandra.log.timestamp}%{SPACE}%{DATA:log.origin.file.name}\\:%{INT:log.origin.file.line:long}%{SPACE}\\-%{SPACE}%{GREEDYDATA:message}(?P<cassandra_log_meta>(?:(.|\\n|\\t)*))
                    let _ = cached_grok_mapped!("%{LOGLEVEL:log.level}%{SPACE}\\[%{GREEDYDATA:process.thread.name}\\]%{SPACE}%{TIMESTAMP_ISO8601:cassandra.log.timestamp}%{SPACE}%{DATA:log.origin.file.name}\\:%{INT:log.origin.file.line:long}%{SPACE}\\-%{SPACE}%{GREEDYDATA:message}(?P<cassandra_log_meta>(?:(.|\\n|\\t)*))", [("cassandra_log_meta", "cassandra.log.meta")]).extract_into(&input, event)?;
                }
            }

            if let Some(date_str) = event.get_as_string("cassandra.log.timestamp") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss,SSS"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "cassandra.log.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("cassandra.log.timestamp");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("event");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            event.append("event.category", json!("database"))?;

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
