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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^(?P<first_char>(?:.))
                let _ = cached_grok!("^(?P<first_char>(?:.))").extract_into(&input, event)?;
            }

            let _cond = { event.get_str("first_char") != Some("{") };
            if _cond {
                // Begin nested pipeline: "pipeline-plaintext"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[%{TIMESTAMP_ISO8601:logstash.log.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_log_module>(?:[\\w\\.]+))\\s*\\]\\[%{NOTSPACE:logstash.log.pipeline_id}\\]\\[%{NOTSPACE:logstash.log.plugin_id}\\] (?P<message>(?:(.|\n)*))
                    // Grok pattern: \\[%{TIMESTAMP_ISO8601:logstash.log.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_log_module>(?:[\\w\\.]+))\\s*\\]\\[%{NOTSPACE:logstash.log.pipeline_id}\\] (?P<message>(?:(.|\n)*))
                    // Grok pattern: \\[%{TIMESTAMP_ISO8601:logstash.log.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_log_module>(?:[\\w\\.]+))\\s*\\] (?P<message>(?:(.|\n)*))
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "\\[%{TIMESTAMP_ISO8601:logstash.log.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_log_module>(?:[\\w\\.]+))\\s*\\]\\[%{NOTSPACE:logstash.log.pipeline_id}\\]\\[%{NOTSPACE:logstash.log.plugin_id}\\] (?P<message>(?:(.|\n)*))",
                                [
                                    ("log_level", "log.level"),
                                    ("logstash_log_module", "logstash.log.module")
                                ]
                            ),
                            cached_grok_mapped!(
                                "\\[%{TIMESTAMP_ISO8601:logstash.log.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_log_module>(?:[\\w\\.]+))\\s*\\]\\[%{NOTSPACE:logstash.log.pipeline_id}\\] (?P<message>(?:(.|\n)*))",
                                [
                                    ("log_level", "log.level"),
                                    ("logstash_log_module", "logstash.log.module")
                                ]
                            ),
                            cached_grok_mapped!(
                                "\\[%{TIMESTAMP_ISO8601:logstash.log.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_log_module>(?:[\\w\\.]+))\\s*\\] (?P<message>(?:(.|\n)*))",
                                [
                                    ("log_level", "log.level"),
                                    ("logstash_log_module", "logstash.log.module")
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
                let _cond = { !event.has_value("event.timezone") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("logstash.log.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["yyyy-MM-dd'T'HH:mm:ss,SSS"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "logstash.log.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("event.timezone") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("logstash.log.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["yyyy-MM-dd'T'HH:mm:ss,SSS"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "logstash.log.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.remove("logstash.log.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "logstash.log.timestamp".into(),
                    });
                }
                event.set("event.kind", json!("event"))?;
                // Painless script
                // Source: def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}"#
                    ),
                )?;
                // End nested pipeline: "pipeline-plaintext"
            }

            let _cond = { event.get_str("first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "pipeline-json"
                parse_json_field(event, "message", "logstash.log")?;
                if let Some(val) = event.get("logstash.log.timeMillis") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "logstash.log.timeMillis".into(),
                            message,
                        }
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
                    return Err(TransformError::FieldNotFound {
                        path: "message".into(),
                    });
                }
                if event.remove("logstash.log.timeMillis").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "logstash.log.timeMillis".into(),
                    });
                }
                event.rename("logstash.log.logEvent.message", "message")?;
                event.rename("logstash.log.logEvent", "logstash.log.log_event")?;
                event.rename("logstash.log.level", "log.level")?;
                let _cond = {
                    event
                        .get("logstash.log.log_event.action")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // Painless script
                    // Source: def items = [];\nctx.logstash.log.log_event.action.forEach(v -> {\n    items.add(v.toString());\n});\nctx.logstash.log.log_event.action = items;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def items = [];\nctx.logstash.log.log_event.action.forEach(v -> {\n    items.add(v.toString());\n});\nctx.logstash.log.log_event.action = items;\n"#
                        ),
                    )?;
                }
                event.set("event.kind", json!("event"))?;
                // Painless script
                // Source: def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}"#
                    ),
                )?;
                // End nested pipeline: "pipeline-json"
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            if event.remove("first_char").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "first_char".into(),
                });
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
