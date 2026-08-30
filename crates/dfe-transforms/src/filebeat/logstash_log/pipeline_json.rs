// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_json` pipeline.
pub struct PipelineJson;

impl Transform for PipelineJson {
    fn name(&self) -> &str {
        "pipeline_json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                parse_json_field(event, "message", "logstash.log")?;

                if let Some(val) = event.get("logstash.log.timeMillis") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "logstash.log.timeMillis".into(),
                            message,
                        })?;
                    event.set("logstash.log.timeMillis", converted)?;
                }

                if let Some(date_str) = event.get_as_string("logstash.log.timeMillis") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "logstash.log.timeMillis".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

                event.rename("logstash.log.loggerName", "logstash.log.module")?;

                if event.remove("message").is_none() {
                    return Err(TransformError::FieldNotFound { path: "message".into() });
                }
                if event.remove("logstash.log.timeMillis").is_none() {
                    return Err(TransformError::FieldNotFound { path: "logstash.log.timeMillis".into() });
                }

                event.rename("logstash.log.logEvent.message", "message")?;

                event.rename("logstash.log.logEvent", "logstash.log.log_event")?;

                event.rename("logstash.log.level", "log.level")?;

            let _cond = { event.get("logstash.log.log_event.action").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def items = [];\nctx.logstash.log.log_event.action.forEach(v -> {\n    items.add(v.toString());\n});\nctx.logstash.log.log_event.action = items;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def items = [];\nctx.logstash.log.log_event.action.forEach(v -> {\n    items.add(v.toString());\n});\nctx.logstash.log.log_event.action = items;\n"#))?;
            }

            event.set("event.kind", json!("event"))?;

                // Painless script
                // Source: def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}"#))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
