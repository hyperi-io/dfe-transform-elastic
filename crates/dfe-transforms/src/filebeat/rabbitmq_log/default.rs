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

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{TIMESTAMP_ISO8601:timestamp} \\[%{WORD:log.level}\\] (?P<rabbitmq_log_pid>(?:\\<%{INT}+\\.%{INT}+\\.%{INT}+\\>)) (?P<message>(?:(.|\n)*))
                    let _ = cached_grok_mapped!("%{TIMESTAMP_ISO8601:timestamp} \\[%{WORD:log.level}\\] (?P<rabbitmq_log_pid>(?:\\<%{INT}+\\.%{INT}+\\.%{INT}+\\>)) (?P<message>(?:(.|\n)*))", [("rabbitmq_log_pid", "rabbitmq.log.pid")]).extract_into(&input, event)?;
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
