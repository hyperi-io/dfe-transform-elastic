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
                let v = json!("activemq");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("event.original") };
            if _cond {
                let v = json!(
                    event
                        .get("message")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.original", v)?;
                }
            }

            if event.has_value("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{LOGLEVEL:log.level}%{SPACE}\\|%{SPACE}%{WORD:user.name}%{SPACE}(?P<message>(?:(\\n|(?! \\|).)*))%{SPACE}\\|%{SPACE}(?P<activemq_audit_thread>(?:((?! \n).)*))
                    let _ = cached_grok_mapped!("%{LOGLEVEL:log.level}%{SPACE}\\|%{SPACE}%{WORD:user.name}%{SPACE}(?P<message>(?:(\\n|(?! \\|).)*))%{SPACE}\\|%{SPACE}(?P<activemq_audit_thread>(?:((?! \n).)*))", [("activemq_audit_thread", "activemq.audit.thread")]).extract_into(&input, event)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("event");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
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
