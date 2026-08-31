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

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?m)%{TIMESTAMP_ISO8601:kafka.log.timestamp}. %{LOGLEVEL:log.level} +%{JAVALOGMESSAGE:message} \\(%{JAVACLASS:kafka.log.class}\\)$[ \\n]*(?'kafka.log.trace.full'.*)
                // Grok pattern: (?m)%{TIMESTAMP_ISO8601:kafka.log.timestamp} %{LOGLEVEL:log.level}\\s+%{JAVACLASS:kafka.log.class}: \\[%{NOTSPACE:kafka.log.thread}\\]: %{GREEDYDATA:message}
                // Grok pattern: (?m)\\[%{TIMESTAMP_ISO8601:kafka.log.timestamp}\\] \\[%{LOGLEVEL:log.level} ?\\] \\[%{NOTSPACE:kafka.log.thread}\\] \\[%{NOTSPACE:kafka.log.class}\\] \\- %{GREEDYDATA:message}
                if !extract_first_match_traced(
                    &[
                        cached_grok!(
                            "(?m)%{TIMESTAMP_ISO8601:kafka.log.timestamp}. %{LOGLEVEL:log.level} +%{JAVALOGMESSAGE:message} \\(%{JAVACLASS:kafka.log.class}\\)$[ \\n]*(?'kafka.log.trace.full'.*)"
                        ),
                        cached_grok!(
                            "(?m)%{TIMESTAMP_ISO8601:kafka.log.timestamp} %{LOGLEVEL:log.level}\\s+%{JAVACLASS:kafka.log.class}: \\[%{NOTSPACE:kafka.log.thread}\\]: %{GREEDYDATA:message}"
                        ),
                        cached_grok!(
                            "(?m)\\[%{TIMESTAMP_ISO8601:kafka.log.timestamp}\\] \\[%{LOGLEVEL:log.level} ?\\] \\[%{NOTSPACE:kafka.log.thread}\\] \\[%{NOTSPACE:kafka.log.class}\\] \\- %{GREEDYDATA:message}"
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[(?P<kafka_log_component>(?:[^\\]]*))\\][,:.]? +%{JAVALOGMESSAGE:message}
                    if !cached_grok_mapped!("\\[(?P<kafka_log_component>(?:[^\\]]*))\\][,:.]? +%{JAVALOGMESSAGE:message}", [("kafka_log_component", "kafka.log.component")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("kafka.log.component", json!("unknown"))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("kafka.log.trace.full") {
                    if let Some(input) = event.get_string("kafka.log.trace.full") {
                        // Grok pattern: %{JAVACLASS:kafka.log.trace.class}:\\s*%{JAVALOGMESSAGE:kafka.log.trace.message}
                        if !cached_grok!("%{JAVACLASS:kafka.log.trace.class}:\\s*%{JAVALOGMESSAGE:kafka.log.trace.message}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                if event.remove("kafka.log.trace").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "kafka.log.trace".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.remove("kafka.log.trace.full");

            event.rename("@timestamp", "event.created")?;

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("kafka.log.timestamp") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss,SSS"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "kafka.log.timestamp".into(),
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
                    if let Some(date_str) = event.get_as_string("kafka.log.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss,SSS"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "kafka.log.timestamp".into(),
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

            if event.remove("kafka.log.timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "kafka.log.timestamp".into(),
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

        event.remove("_ingest._grok_match_index");
        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
            event.remove("_ingest");
        }
        Ok(TransformResult::Continue)
    }
}
