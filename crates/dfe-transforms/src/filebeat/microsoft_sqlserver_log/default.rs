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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?P<date>(?:%{DATA} %{DATA})) %{DATA:microsoft_sqlserver.log.origin} [ ]*(?P<message>(?:(.|\\n)*))
                    // Grok pattern: (?P<message>(?:(.|\\n)*))
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "(?P<date>(?:%{DATA} %{DATA})) %{DATA:microsoft_sqlserver.log.origin} [ ]*(?P<message>(?:(.|\\n)*))"
                            ),
                            cached_grok!("(?P<message>(?:(.|\\n)*))"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("date") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SS"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "date".into(),
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

            event.remove("date");

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "")?;
            }

            if event.has_value("host.mac") {
                gsub_field(
                    event,
                    "host.mac",
                    "host.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("database"))?;

            event.append("event.type", json!("info"))?;

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
