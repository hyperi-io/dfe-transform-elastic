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
            let _cond = { event.get_str("input.type") == Some("log") };
            if _cond {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: %{SYSLOGTIMESTAMP:system.syslog.timestamp} %{SYSLOGHOST:host.hostname} %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: (?P<system_syslog_message>(?:(.|\n)*))
                        // Grok pattern: %{SYSLOGTIMESTAMP:system.syslog.timestamp} (?P<system_syslog_message>(?:(.|\n)*))
                        // Grok pattern: %{TIMESTAMP_ISO8601:system.syslog.timestamp} %{SYSLOGHOST:host.hostname} %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: (?P<system_syslog_message>(?:(.|\n)*))
                        // Grok pattern: %{TIMESTAMP_ISO8601:system.syslog.timestamp} (?P<system_syslog_message>(?:(.|\n)*))
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "%{SYSLOGTIMESTAMP:system.syslog.timestamp} %{SYSLOGHOST:host.hostname} %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: (?P<system_syslog_message>(?:(.|\n)*))",
                                    [("system_syslog_message", "system.syslog.message")]
                                ),
                                cached_grok_mapped!(
                                    "%{SYSLOGTIMESTAMP:system.syslog.timestamp} (?P<system_syslog_message>(?:(.|\n)*))",
                                    [("system_syslog_message", "system.syslog.message")]
                                ),
                                cached_grok_mapped!(
                                    "%{TIMESTAMP_ISO8601:system.syslog.timestamp} %{SYSLOGHOST:host.hostname} %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: (?P<system_syslog_message>(?:(.|\n)*))",
                                    [("system_syslog_message", "system.syslog.message")]
                                ),
                                cached_grok_mapped!(
                                    "%{TIMESTAMP_ISO8601:system.syslog.timestamp} (?P<system_syslog_message>(?:(.|\n)*))",
                                    [("system_syslog_message", "system.syslog.message")]
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.get_str("input.type") == Some("journald") };
            if _cond {
                // Begin nested pipeline: "journald"
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("event.ingested", v)?;
                }
                if let Some(v) = event.get("event.original").cloned() {
                    event.set("message", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("journald.pid").cloned() {
                        event.set("process.pid", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "process.name",
                        json!(
                            event
                                .get("journald.process.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    event.has_value("host.hostname") && event.get_str("host.hostname") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.remove("journald");
                event.remove("process.thread");
                event.remove("syslog");
                event.remove("systemd");
                event.remove("message_id");
                // End nested pipeline: "journald"
            }

            let _cond = { event.get_str("input.type") == Some("log") };
            if _cond {
                // Begin nested pipeline: "log"
                let _cond = { event.has_value("event.original") };
                if _cond {
                    event.remove("message");
                }
                if event.has_value("system.syslog.message") {
                    event.rename("system.syslog.message", "message")?;
                }
                let _cond = { !event.has_value("event.timezone") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("system.syslog.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "MMM  d HH:mm:ss",
                                    "MMM dd HH:mm:ss",
                                    "MMM d HH:mm:ss",
                                    "ISO8601",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "system.syslog.timestamp".into(),
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
                        if let Some(date_str) = event.get_as_string("system.syslog.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "MMM  d HH:mm:ss",
                                    "MMM dd HH:mm:ss",
                                    "MMM d HH:mm:ss",
                                    "ISO8601",
                                ],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "system.syslog.timestamp".into(),
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
                if event.remove("system.syslog.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "system.syslog.timestamp".into(),
                    });
                }
                event.set("ecs.version", json!("8.11.0"))?;
                // End nested pipeline: "log"
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
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
