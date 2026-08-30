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
                    // Grok pattern: (?:(?:(?P<_tmp_local_timestamp>(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}|%{NUMBER})%{SPACE}%{TIME}))|%{TIMESTAMP_ISO8601:_tmp.timestamp}))%{SPACE}(%{NUMBER:mysql.thread_id:long}%{SPACE})?(\\[%{DATA:log.level}\\]%{SPACE})?(?P<message>(?:(.|\n)+))
                    // Grok pattern: %{GREEDYDATA:message}
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?:(?:(?P<_tmp_local_timestamp>(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}|%{NUMBER})%{SPACE}%{TIME}))|%{TIMESTAMP_ISO8601:_tmp.timestamp}))%{SPACE}(%{NUMBER:mysql.thread_id:long}%{SPACE})?(\\[%{DATA:log.level}\\]%{SPACE})?(?P<message>(?:(.|\n)+))",
                                [("_tmp_local_timestamp", "_tmp.local_timestamp")]
                            ),
                            cached_grok!("%{GREEDYDATA:message}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: (\\[%{DATA:event.code}\\])%{SPACE}(\\[%{DATA:event.provider}\\])%{SPACE}(?:(.|\n)+)
                        // Grok pattern: %{GREEDYDATA}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "(\\[%{DATA:event.code}\\])%{SPACE}(\\[%{DATA:event.provider}\\])%{SPACE}(?:(.|\n)+)"
                                ),
                                cached_grok!("%{GREEDYDATA}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })();

            event.rename("@timestamp", "event.created")?;

            let _cond =
                { event.has_value("_tmp.local_timestamp") && !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.local_timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyMMdd H:m:s",
                            "yyMMdd  H:m:s",
                            "yyyy-MM-dd H:m:s",
                            "yyyy-MM-dd  H:m:s",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.local_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond =
                { event.has_value("_tmp.local_timestamp") && event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.local_timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyMMdd H:m:s",
                            "yyMMdd  H:m:s",
                            "yyyy-MM-dd H:m:s",
                            "yyyy-MM-dd  H:m:s",
                        ],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.local_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("_tmp");

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("database"))?;

            event.append("event.type", json!("info"))?;

            let _cond = {
                event.has_value("log.level")
                    && event
                        .get_str("log.level")
                        .is_some_and(|s| s.to_lowercase() == "error")
            };
            if _cond {
                event.append("event.type", json!("error"))?;
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
