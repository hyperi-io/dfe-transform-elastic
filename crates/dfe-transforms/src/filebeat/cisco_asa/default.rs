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
            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            if event.remove("message").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "message".into(),
                });
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            let _cond = { event.has_value("event.original") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?:(?:(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)\\s*)?(?:(?P<_temp__raw_date>(?:(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?(?P<_temp__tz>(?:(?:Z|[+-]%{HOUR}(?::?%{MINUTE}))))?)|(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:[A-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)))):?\\s+)?(?:(?:(?:(?P<process_name>(?:(?:[^%\\s:\\[]+))):\\s%{SYSLOGHOST:host.name}))|(?:(?:%{SYSLOGHOST:host.hostname}:?\\s+)?(?:(?P<process_name>(?:(?:[^%\\s:\\[]+)))?(?:\\[%{POSINT:process.pid:long}\\])?)?))(?:{DATA})?(?:(?:(:|\\s)\\s+))?))?\\s*%{GREEDYDATA:_temp_.full_message}
                    if !cached_grok_mapped!("(?:(?:(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)\\s*)?(?:(?P<_temp__raw_date>(?:(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?(?P<_temp__tz>(?:(?:Z|[+-]%{HOUR}(?::?%{MINUTE}))))?)|(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:[A-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)))):?\\s+)?(?:(?:(?:(?P<process_name>(?:(?:[^%\\s:\\[]+))):\\s%{SYSLOGHOST:host.name}))|(?:(?:%{SYSLOGHOST:host.hostname}:?\\s+)?(?:(?P<process_name>(?:(?:[^%\\s:\\[]+)))?(?:\\[%{POSINT:process.pid:long}\\])?)?))(?:{DATA})?(?:(?:(:|\\s)\\s+))?))?\\s*%{GREEDYDATA:_temp_.full_message}", [("_temp__raw_date", "_temp_.raw_date"), ("process_name", "process.name"), ("process_name", "process.name"), ("_temp__tz", "_temp_.tz"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_temp_.full_message") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: ^(?P<_temp__raw_date>(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:[A-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{DATA:_temp_.full_message}$
                    // Grok pattern: %{GREEDYDATA:_temp_.full_message}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^(?P<_temp__raw_date>(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:[A-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{DATA:_temp_.full_message}$",
                                [
                                    ("_temp__raw_date", "_temp_.raw_date"),
                                    ("_temp__tz", "_temp_.tz")
                                ]
                            ),
                            cached_grok!("%{GREEDYDATA:_temp_.full_message}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // Painless script
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("_temp_.full_message")
                    && event.get("_temp_.full_message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("message repeated"))
                        }
                        serde_json::Value::String(s) => s.contains("message repeated"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: (?P<_temp__message_prefix>.*)message repeated (?P<INT:_temp__cisco_message_repeats:int>\\d+) times: \\[\\s(?P<_temp__extracted_message>.*)\\]
                    if !cached_grok_mapped!("(?P<_temp__message_prefix>.*)message repeated (?P<INT:_temp__cisco_message_repeats:int>\\d+) times: \\[\\s(?P<_temp__extracted_message>.*)\\]", [("_temp__message_prefix", "_temp_.message_prefix"), ("INT:_temp__cisco_message_repeats:int", "INT:_temp_.cisco.message_repeats:int"), ("_temp__extracted_message", "_temp_.extracted_message")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.message_prefix")
                    && event.has_value("_temp_.extracted_message")
            };
            if _cond {
                // Painless script
                // Source: ctx._temp_.full_message = ctx._temp_.message_prefix + ctx._temp_.extracted_message;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx._temp_.full_message = ctx._temp_.message_prefix + ctx._temp_.extracted_message;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.full_message") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: (?:%{DATA}%(?:[A-Z]+))-(?:(?P<_temp__cisco_suffix>(?:[^0-9-]+))-)?%{NONNEGINT:event.severity:int}-%{POSINT:_temp_.cisco.message_id}?:?\\s*%{GREEDYDATA:message}
                    // Grok pattern: %{GREEDYDATA:message}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?:%{DATA}%(?:[A-Z]+))-(?:(?P<_temp__cisco_suffix>(?:[^0-9-]+))-)?%{NONNEGINT:event.severity:int}-%{POSINT:_temp_.cisco.message_id}?:?\\s*%{GREEDYDATA:message}",
                                [("_temp__cisco_suffix", "_temp_.cisco.suffix")]
                            ),
                            cached_grok!("%{GREEDYDATA:message}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("_temp_.cisco.message_id") };
            if _cond {
                event.set("_temp_.cisco.message_id", json!(""))?;
            }

            let _cond = { !event.has_value("event.severity") };
            if _cond {
                event.set("event.severity", json!(7))?;
            }

            let _cond = {
                event.has_value("_temp_.tz")
                    && event.get_str("_temp_.tz") != Some("")
                    && event.has_value("_conf.tz_map")
            };
            if _cond {
                // Painless script
                // Source: for (def item : ctx._conf.tz_map) {\n  if (item.tz_short == ctx._temp_.tz) {\n    ctx._temp_.tz = item.tz_long;\n    break;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (def item : ctx._conf.tz_map) {\n  if (item.tz_short == ctx._temp_.tz) {\n    ctx._temp_.tz = item.tz_long;\n    break;\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("_temp_.tz") == Some("Z") };
            if _cond {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(v) = event.get("event.timezone").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            if !event.has("_temp_.tz") {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            if let Some(v) = event.get("_temp_.tz").cloned() {
                event.set("event.timezone", v)?;
            }

            let _cond = { event.has_value("_temp_.raw_date") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSS][ z]",
                                "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSSSSS][ z]",
                                "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSSSSSSSS][ z]",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.raw_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_raw_date")?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("_temp_.raw_date") };
                    if _cond {
                        if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "ISO8601",
                                    "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSS][ z]",
                                    "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSSSSS][ z]",
                                    "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSSSSSSSS][ z]",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_temp_.raw_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_i64("event.severity") == Some(0) };
            if _cond {
                event.set("log.level", json!("unknown"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(1) };
            if _cond {
                event.set("log.level", json!("alert"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(2) };
            if _cond {
                event.set("log.level", json!("critical"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(3) };
            if _cond {
                event.set("log.level", json!("error"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(4) };
            if _cond {
                event.set("log.level", json!("warning"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(5) };
            if _cond {
                event.set("log.level", json!("notification"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(6) };
            if _cond {
                event.set("log.level", json!("informational"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(7) };
            if _cond {
                event.set("log.level", json!("debug"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.direction", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" connection denied from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connection denied from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" flags ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" flags ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Connection denied by ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Connection denied by ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.direction", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" src ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" src ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" dest ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" dest ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.address", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106006") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.direction", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106007") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.direction", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" due to ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" due to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106010") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{POSINT:source.port} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}/%{POSINT:destination.port}(%{GREEDYDATA})?
                    // Grok pattern: Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}(/%{POSINT:source.port})? (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}(/%{POSINT:destination.port})?(%{GREEDYDATA})?
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{POSINT:source.port} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}/%{POSINT:destination.port}(%{GREEDYDATA})?"
                            ),
                            cached_grok!(
                                "Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}(/%{POSINT:source.port})? (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}(/%{POSINT:destination.port})?(%{GREEDYDATA})?"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106012") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny IP from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("event.reason", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106013") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dropping echo request from ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to PAT address ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to PAT address ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.address", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106013") };
            if _cond {
                event.set("network.transport", json!("icmp"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106013") };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106014") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Deny %{NOTSPACE:network.direction} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:(?P<destination_address>[^ (]*)(%{GREEDYDATA})?
                    if !cached_grok_mapped!("Deny %{NOTSPACE:network.direction} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:(?P<destination_address>[^ (]*)(%{GREEDYDATA})?", [("destination_address", "destination.address")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106015") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Deny %{NOTSPACE:network.transport} %{NOTSPACE} %{NOTSPACE} from %{IPORHOST:source.address}/%{POSINT:source.port} to %{IPORHOST:destination.address}/%{POSINT:destination.port} flags %{DATA} on interface %{NOTSPACE:_temp_.cisco.source_interface}
                    if !cached_grok!("Deny %{NOTSPACE:network.transport} %{NOTSPACE} %{NOTSPACE} from %{IPORHOST:source.address}/%{POSINT:source.port} to %{IPORHOST:destination.address}/%{POSINT:destination.port} flags %{DATA} on interface %{NOTSPACE:_temp_.cisco.source_interface}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny IP spoof from (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106017") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny IP due to Land Attack from ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.address", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106018") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" packet type ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet type ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" denied by ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" denied by ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.direction", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" src ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" src ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" dest ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" dest ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.address", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106020") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) =
                            remaining.strip_prefix("Deny IP teardrop fragment (size = ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", offset = ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", offset = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.address", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106021") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" reverse path check from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" reverse path check from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106022") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" connection spoof from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connection spoof from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106023") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Deny ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{DATA:_temp_.cisco.source_interface}:(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})?\\s*(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?dst %{DATA:_temp_.cisco.destination_interface}:(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[(\\s]+%{DATA}by access-group \"%{NOTSPACE:_temp_.cisco.list_id}?\"
                    if !cached_grok_mapped!("^Deny ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{DATA:_temp_.cisco.source_interface}:(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})?\\s*(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?dst %{DATA:_temp_.cisco.destination_interface}:(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[(\\s]+%{DATA}by access-group \"%{NOTSPACE:_temp_.cisco.list_id}?\"", [("source_address", "source.address"), ("destination_address", "destination.address"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106027") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Deny src ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Deny src ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" dst ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" dst ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" by access-group \"") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" by access-group \"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("\"") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106100") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("access-list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("-> ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("-> ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106102") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("access-list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for user ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for user ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("-> ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("-> ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106103") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("access-list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" denied ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" denied ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for user ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for user ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("-> ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("-> ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("111004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" end configuration: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" end configuration: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.outcome", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("111007") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Begin configuration: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("111009") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{NOTSPACE} '%{NOTSPACE:server.user.name}' executed %{NOTSPACE} %{GREEDYDATA:_temp_.cisco.command_line_arguments}
                    if !cached_grok!("^%{NOTSPACE} '%{NOTSPACE:server.user.name}' executed %{NOTSPACE} %{GREEDYDATA:_temp_.cisco.command_line_arguments}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("111010") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: User '%{NOTSPACE:server.user.name}', running %{QUOTEDSTRING} from IP %{IP:source.address}, executed %{QUOTEDSTRING:_temp_.cisco.command_line_arguments}
                    if !cached_grok!("User '%{NOTSPACE:server.user.name}', running %{QUOTEDSTRING} from IP %{IP:source.address}, executed %{QUOTEDSTRING:_temp_.cisco.command_line_arguments}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user %{DATA:_temp_.cisco.aaa_type} Successful(%{SPACE})?: server =(%{SPACE})?%{IPORHOST:destination.address} [:,] [Uu]ser = (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))
                    if !cached_grok_mapped!("AAA user %{DATA:_temp_.cisco.aaa_type} Successful(%{SPACE})?: server =(%{SPACE})?%{IPORHOST:destination.address} [:,] [Uu]ser = (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user (?:(authentication|authorization)) Rejected(%{SPACE})?: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|Account has been locked out|Users account has expired|User was not found)))(%{SPACE})?: server = %{IPORHOST:destination.address}(%{SPACE})?: user = ?((?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))|)(%{SPACE})?: user IP = (?:(%{IP:source.address}|None))
                    if !cached_grok_mapped!("AAA user (?:(authentication|authorization)) Rejected(%{SPACE})?: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|Account has been locked out|Users account has expired|User was not found)))(%{SPACE})?: server = %{IPORHOST:destination.address}(%{SPACE})?: user = ?((?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))|)(%{SPACE})?: user IP = (?:(%{IP:source.address}|None))", [("_temp__cisco_rejection_reason", "_temp_.cisco.rejection_reason"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113008") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA transaction status ACCEPT(%{SPACE})?: user = %{GREEDYDATA:source.user.name}
                    if !cached_grok!("AAA transaction status ACCEPT(%{SPACE})?: user = %{GREEDYDATA:source.user.name}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113009") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) =
                            remaining.strip_prefix("AAA retrieved default group policy ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for user ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.group_policy", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for user ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("source.user.name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113011") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) =
                            remaining.strip_prefix("AAA retrieved user specific group policy (")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") for user = ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.group_policy", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") for user = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("source.user.name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113012") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user authentication Successful(%{SPACE})?: local database(%{SPACE})?: [Uu]ser = (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))
                    if !cached_grok_mapped!("AAA user authentication Successful(%{SPACE})?: local database(%{SPACE})?: [Uu]ser = (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113015") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = %{DATA:source.user.name}%{SPACE}: [Uu]ser IP = (?:(%{IP:source.address}|None))%{SPACE}$
                    // Grok pattern: ^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = (?P<source_user_name>(?:[^:]+?))%{SPACE}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = %{DATA:source.user.name}%{SPACE}: [Uu]ser IP = (?:(%{IP:source.address}|None))%{SPACE}$",
                                [(
                                    "_temp__cisco_rejection_reason",
                                    "_temp_.cisco.rejection_reason"
                                )]
                            ),
                            cached_grok_mapped!(
                                "^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = (?P<source_user_name>(?:[^:]+?))%{SPACE}$",
                                [
                                    (
                                        "_temp__cisco_rejection_reason",
                                        "_temp_.cisco.rejection_reason"
                                    ),
                                    ("source_user_name", "source.user.name")
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113019") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Group = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Username = ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.group.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Username = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IP = ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IP = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Session disconnected. Session Type: ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(", Session disconnected. Session Type: ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Duration: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.session_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Duration: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Bytes xmt: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.duration_hms", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Bytes xmt: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Bytes rcv: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.bytes", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Bytes rcv: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Reason: ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.bytes", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Reason: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("event.reason", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113021") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) =
                            remaining.strip_prefix("Attempted console login failed. User ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" did NOT have appropriate Admin Rights.")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" did NOT have appropriate Admin Rights.")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113022") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("AAA Marking ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" server ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" server ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" in aaa-server group ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" in aaa-server group ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113023") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("AAA Marking ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" server ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" server ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" in aaa-server group ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" in aaa-server group ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.cisco.message_id") == Some("113040")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("Group"))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Group <") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("> User <") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.group.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("> User <") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("> IP <") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("> IP <") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) =
                            remaining.find("> Terminating the VPN connection attempt from <")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining
                            .strip_prefix("> Terminating the VPN connection attempt from <")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) =
                            remaining.find(">. Reason: This connection is group locked to ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.tunnel_group", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining
                            .strip_prefix(">. Reason: This connection is group locked to ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(".") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.cisco.message_id") == Some("113040")
                    && !(event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("Group")))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) =
                            remaining.strip_prefix("Terminating the VPN connection attempt from ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) =
                            remaining.find(". Reason: This connection is group locked to ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.group.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(". Reason: This connection is group locked to ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(".") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                [
                    "113029", "113030", "113031", "113032", "113033", "113034", "113035", "113036",
                    "113038", "113039",
                ]
                .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}>
                    // Grok pattern: Group %{NOTSPACE:source.user.group.name} User (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:, *%{NUMBER})?)))) IP %{IP:source.address}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}>"
                            ),
                            cached_grok_mapped!(
                                "Group %{NOTSPACE:source.user.group.name} User (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:, *%{NUMBER})?)))) IP %{IP:source.address}",
                                [("source_user_name", "source.user.name")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("302010") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" in use, ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.connections_in_use", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" in use, ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" most used") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.connections_most_used", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" most used") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                ["302013", "302015"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Built %{NOTSPACE:network.direction} %{GREEDYDATA:_temp_.var_302013_302015}
                    if !cached_grok!(
                        "Built %{NOTSPACE:network.direction} %{GREEDYDATA:_temp_.var_302013_302015}"
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                ["302013", "302015"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
                    && event.get_str("network.direction") == Some("inbound")
            };
            if _cond {
                if let Some(input) = event.get_string("_temp_.var_302013_302015") {
                    // Grok pattern: ^%{NOTSPACE:network.transport} connection %{NUMBER:_temp_.cisco.connection_id} for (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:source.port} \\((?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:_temp_.cisco.mapped_source_port}\\)(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{NOTSPACE:destination.address}/%{NUMBER:destination.port} \\(%{NOTSPACE:_temp_.natdstip}/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?( \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\))?%{GREEDYDATA}
                    if !cached_grok_mapped!("^%{NOTSPACE:network.transport} connection %{NUMBER:_temp_.cisco.connection_id} for (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:source.port} \\((?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:_temp_.cisco.mapped_source_port}\\)(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{NOTSPACE:destination.address}/%{NUMBER:destination.port} \\(%{NOTSPACE:_temp_.natdstip}/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?( \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\))?%{GREEDYDATA}", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__natsrcip", "_temp_.natsrcip"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                ["302013", "302015"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
                    && event.get_str("network.direction") == Some("outbound")
            };
            if _cond {
                if let Some(input) = event.get_string("_temp_.var_302013_302015") {
                    // Grok pattern: ^%{NOTSPACE:network.transport} connection %{NUMBER:_temp_.cisco.connection_id} for (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:destination.port} \\((?P<_temp__natdstip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? to (?P<_temp__cisco_source_interface>(?:[^:]*)):%{NOTSPACE:source.address}/%{NUMBER:source.port} \\(%{NOTSPACE:_temp_.natsrcip}/%{NUMBER:_temp_.cisco.mapped_source_port}\\)(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?( \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\))?%{GREEDYDATA}
                    if !cached_grok_mapped!("^%{NOTSPACE:network.transport} connection %{NUMBER:_temp_.cisco.connection_id} for (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:destination.port} \\((?P<_temp__natdstip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? to (?P<_temp__cisco_source_interface>(?:[^:]*)):%{NOTSPACE:source.address}/%{NUMBER:source.port} \\(%{NOTSPACE:_temp_.natsrcip}/%{NUMBER:_temp_.cisco.mapped_source_port}\\)(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?( \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\))?%{GREEDYDATA}", [("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("destination_address", "destination.address"), ("_temp__natdstip", "_temp_.natdstip"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("303002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" connection from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connection from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", user ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", user ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("client.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" file ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" file ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("file.path", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("305012") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Teardown %{DATA} %{NOTSPACE:network.transport} translation from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port}(\\s*\\((?P<_temp__cisco_source_username>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:(?:[^@$]*))\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:, *%{NUMBER})?(?:%{NUMBER}:%{DATA})?))))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND}))
                    if !cached_grok_mapped!("Teardown %{DATA} %{NOTSPACE:network.transport} translation from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port}(\\s*\\((?P<_temp__cisco_source_username>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:(?:[^@$]*))\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:, *%{NUMBER})?(?:%{NUMBER}:%{DATA})?))))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND}))", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("302020") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Built %{NOTSPACE:network.direction} %{NOTSPACE:network.type} connection for faddr (?:(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?gaddr (?:(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))|(?:[^:]*):(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER} laddr (?:(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?(type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?
                    if !cached_grok_mapped!("Built %{NOTSPACE:network.direction} %{NOTSPACE:network.type} connection for faddr (?:(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?gaddr (?:(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))|(?:[^:]*):(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER} laddr (?:(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?(type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("destination_domain", "destination.domain"), ("destination_domain", "destination.domain"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("source_domain", "source.domain"), ("source_domain", "source.domain"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("302022") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Built ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" stub ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" stub ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" connection for ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connection for ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("302023") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Teardown stub ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" connection for ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connection for ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" duration ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" duration ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" forwarded bytes ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.duration_hms", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" forwarded bytes ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.bytes", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("event.reason", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("304001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (%{NOTSPACE:source.user.name}@)?%{IP:source.address}(\\(%{DATA}\\))? %{DATA} (%{NOTSPACE}@)?%{IPORHOST:destination.address}:%{GREEDYDATA:url.original}
                    if !cached_grok!("(%{NOTSPACE:source.user.name}@)?%{IP:source.address}(\\(%{DATA}\\))? %{DATA} (%{NOTSPACE}@)?%{IPORHOST:destination.address}:%{GREEDYDATA:url.original}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("304002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Access denied URL ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" SRC ") else {
                            break 'dissect false;
                        };
                        captured.push(("url.original", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" SRC ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("EST ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("EST ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("305011") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Built %{NOTSPACE} %{NOTSPACE:network.transport} translation from %{NOTSPACE:_temp_.cisco.source_interface}:%{DATA:source.address}/%{NUMBER:source.port}(\\(%{NOTSPACE:source.user.name}\\))? to %{NOTSPACE:_temp_.cisco.destination_interface}:%{DATA:destination.address}/%{NUMBER:destination.port}
                    if !cached_grok!("Built %{NOTSPACE} %{NOTSPACE:network.transport} translation from %{NOTSPACE:_temp_.cisco.source_interface}:%{DATA:source.address}/%{NUMBER:source.port}(\\(%{NOTSPACE:source.user.name}\\))? to %{NOTSPACE:_temp_.cisco.destination_interface}:%{DATA:destination.address}/%{NUMBER:destination.port}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("313001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Denied ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" type=") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" type=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", code=") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", code=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_code", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("313004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Denied ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" type=") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" type=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", from") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", from") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("addr ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("addr ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(": no matching session") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": no matching session") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("313005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: protocol %{NUMBER:_temp_.cisco.original_iana_number} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.group.name}\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: <unknown>[.]?
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("source_address", "source.address"),
                                    ("destination_address", "destination.address")
                                ]
                            ),
                            cached_grok_mapped!(
                                "No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: protocol %{NUMBER:_temp_.cisco.original_iana_number} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("source_address", "source.address"),
                                    ("destination_address", "destination.address")
                                ]
                            ),
                            cached_grok_mapped!(
                                "No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.group.name}\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    ("source_user_domain", "source.user.domain"),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("source_address", "source.address"),
                                    ("destination_address", "destination.address")
                                ]
                            ),
                            cached_grok_mapped!(
                                "No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    ("source_user_domain", "source.user.domain"),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("source_address", "source.address"),
                                    ("destination_address", "destination.address")
                                ]
                            ),
                            cached_grok_mapped!(
                                "No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("source_address", "source.address"),
                                    ("destination_address", "destination.address"),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: <unknown>[.]?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    )
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("313008") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Denied ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" type=") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" type=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", code=") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", code=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_code", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on interface ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("313009") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Denied invalid ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" code ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" code ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", for ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_code", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", for ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("315011") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) disconnected by SSH server, reason: %{GREEDYDATA:event.reason}
                    // Grok pattern: SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) terminated normally
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) disconnected by SSH server, reason: %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!(
                                "SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) terminated normally"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("322001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Deny MAC address ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", possible spoof attempt on interface ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("source.mac", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(", possible spoof attempt on interface ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.source_interface", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic filter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338001") };
            if _cond {
                let v = json!(
                    event
                        .get("source.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338002") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338005") };
            if _cond {
                let v = json!(
                    event
                        .get("source.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338006") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338006") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338007") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338008") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" black") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" black") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338101") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" white") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" white") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("source.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338101") };
            if _cond {
                let v = json!(
                    event
                        .get("source.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338102") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" white") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" white") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338102") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338103") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" white") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" white") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338104") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" white") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" white") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338201") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" grey") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" grey") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338201") };
            if _cond {
                let v = json!(
                    event
                        .get("source.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338202") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" grey") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" grey") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338202") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338203") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" grey") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" grey") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("source ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("source ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338203") };
            if _cond {
                let v = json!(
                    event
                        .get("source.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338204") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dynamic ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("ilter ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" grey") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" grey") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("d ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("d ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" traffic from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" traffic from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("destination ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("destination ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" resolved from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.list_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" list: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", threat-level: ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", threat-level: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", category: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.threat_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", category: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.threat_category", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338204") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.domain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338301") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) =
                            remaining.strip_prefix("Intercepted DNS reply for domain ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", matched ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", matched ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.list_id", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338301") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.address")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("client.address", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338301") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.port")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("client.port", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338301") };
            if _cond {
                let v = json!(
                    event
                        .get("source.address")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.address", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("338301") };
            if _cond {
                let v = json!(
                    event
                        .get("source.port")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("server.port", v)?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("502103") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User priv level changed: Uname: ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" From: ") else {
                            break 'dissect false;
                        };
                        captured.push(("server.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" From: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" To: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.privilege.old", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" To: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.privilege.new", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("507003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" flow from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" flow from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) =
                            remaining.find(" terminated by inspection engine, reason - ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" terminated by inspection engine, reason - ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                ["605004", "605005"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Login ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for user \"") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for user \"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("\"") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("609001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Built local-host ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("source.address", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("607001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Pre-allocate SIP ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" secondary channel for ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.connection_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" secondary channel for ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" message") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.message", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("607001") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.connection_type") {
                    // Grok pattern: (?:(?:(?P<network_transport>(?:(?:UDP|TCP)))|(?P<network_protocol>(?:(?:RTP|RTCP)))))
                    if !cached_grok_mapped!("(?:(?:(?P<network_transport>(?:(?:UDP|TCP)))|(?P<network_protocol>(?:(?:RTP|RTCP)))))", [("network_transport", "network.transport"), ("network_protocol", "network.protocol")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("609002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Teardown local-host ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" duration ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" duration ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.duration_hms", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                ["611102", "611101"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^User authentication %{DATA}:(?:\\s*IP address: %{IP:source.address},)? Uname: %{DATA:server.user.name}$
                    if !cached_grok!("^User authentication %{DATA}:(?:\\s*IP address: %{IP:source.address},)? Uname: %{DATA:server.user.name}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("710003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" access denied by ACL from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" access denied by ACL from ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.port", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("710005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" request discarded from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" request discarded from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.port", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("713049") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group = %{NOTSPACE}, Username = %{NOTSPACE:user.name}, IP = %{IP:source.address}, Security negotiation complete for User (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}
                    // Grok pattern: Group = %{NOTSPACE}, IP = %{IP:source.address}, Security negotiation complete [a-z\\s]+ (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Group = %{NOTSPACE}, Username = %{NOTSPACE:user.name}, IP = %{IP:source.address}, Security negotiation complete for User (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}"
                            ),
                            cached_grok!(
                                "Group = %{NOTSPACE}, IP = %{IP:source.address}, Security negotiation complete [a-z\\s]+ (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <%{DATA:_temp_.cisco.webvpn.group_name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> WebVPN session terminated: %{GREEDYDATA:event.reason}.
                    // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} WebVPN session terminated: %{GREEDYDATA:event.reason}.
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Group <%{DATA:_temp_.cisco.webvpn.group_name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> WebVPN session terminated: %{GREEDYDATA:event.reason}."
                            ),
                            cached_grok!(
                                "Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} WebVPN session terminated: %{GREEDYDATA:event.reason}."
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> WebVPN access GRANTED: \"?%{DATA:url.original}\"?$
                    // Grok pattern: ^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} WebVPN access GRANTED: \"?%{DATA:url.original}\"?$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> WebVPN access GRANTED: \"?%{DATA:url.original}\"?$",
                                [
                                    (
                                        "_temp__cisco_webvpn_group_name",
                                        "_temp_.cisco.webvpn.group_name"
                                    ),
                                    ("source_user_name", "source.user.name"),
                                    ("source_address", "source.address")
                                ]
                            ),
                            cached_grok!(
                                "^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} WebVPN access GRANTED: \"?%{DATA:url.original}\"?$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716058") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))>
                    // Grok pattern: ^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))>",
                                [
                                    (
                                        "_temp__cisco_webvpn_group_name",
                                        "_temp_.cisco.webvpn.group_name"
                                    ),
                                    ("source_user_name", "source.user.name"),
                                    ("source_address", "source.address")
                                ]
                            ),
                            cached_grok!(
                                "^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716059") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User( <(?P<source_user_name>(?:[^<>]+))>)? IP <(?P<destination_address>(?:[^<>]+))> AnyConnect session (resumed connection|resumed. Connection) from( IP)? <%{NOTSPACE:source.address}>\\.$
                    // Grok pattern: ^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User( %{NOTSPACE:source.user.name})? IP %{NOTSPACE:destination.address} AnyConnect session (resumed connection|resumed. Connection) from( IP)? %{NOTSPACE:source.address}\\.$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User( <(?P<source_user_name>(?:[^<>]+))>)? IP <(?P<destination_address>(?:[^<>]+))> AnyConnect session (resumed connection|resumed. Connection) from( IP)? <%{NOTSPACE:source.address}>\\.$",
                                [
                                    (
                                        "_temp__cisco_webvpn_group_name",
                                        "_temp_.cisco.webvpn.group_name"
                                    ),
                                    ("source_user_name", "source.user.name"),
                                    ("destination_address", "destination.address")
                                ]
                            ),
                            cached_grok!(
                                "^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User( %{NOTSPACE:source.user.name})? IP %{NOTSPACE:destination.address} AnyConnect session (resumed connection|resumed. Connection) from( IP)? %{NOTSPACE:source.address}\\.$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("717022") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Certificate was successfully validated. serial number:%{SPACE}%{DATA:_temp_.cisco.serial_number}, subject name:%{SPACE}%{DATA:_temp_.cisco.distinguished_name}\\.
                    if !cached_grok!("Certificate was successfully validated. serial number:%{SPACE}%{DATA:_temp_.cisco.serial_number}, subject name:%{SPACE}%{DATA:_temp_.cisco.distinguished_name}\\.").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("721016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") WebVPN session for client user ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.device_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(") WebVPN session for client user ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IP") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IP") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" has been created.") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" has been created.") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("721018") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") WebVPN session for client user ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.device_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(") WebVPN session for client user ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IP") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IP") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" has been deleted.") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" has been deleted.") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                [
                    "722011", "722022", "722023", "722028", "722032", "722033", "722034", "722035",
                    "722037", "722051",
                ]
                .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> %{GREEDYDATA:event.reason}$
                    // Grok pattern: ^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} %{GREEDYDATA:event.reason}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> %{GREEDYDATA:event.reason}$",
                                [
                                    ("source_user_group_name", "source.user.group.name"),
                                    ("source_user_name", "source.user.name"),
                                    ("source_address", "source.address")
                                ]
                            ),
                            cached_grok!(
                                "^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} %{GREEDYDATA:event.reason}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722033") };
            if _cond {
                if let Some(input) = event.get_string("event.reason") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("First ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) =
                            remaining.find(" SVC connection established for SVC session.")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" SVC connection established for SVC session.")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.reason".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722034") };
            if _cond {
                if let Some(input) = event.get_string("event.reason") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("New ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" SVC connection, no existing connection.")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" SVC connection, no existing connection.")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.reason".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722041") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^TunnelGroup <(?P<_temp__cisco_tunnel_group>(?:[^<>]+))> GroupPolicy <(?P<_temp__cisco_group_policy>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> %{GREEDYDATA:event.reason}$
                    if !cached_grok_mapped!("^TunnelGroup <(?P<_temp__cisco_tunnel_group>(?:[^<>]+))> GroupPolicy <(?P<_temp__cisco_group_policy>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> %{GREEDYDATA:event.reason}$", [("_temp__cisco_tunnel_group", "_temp_.cisco.tunnel_group"), ("_temp__cisco_group_policy", "_temp_.cisco.group_policy"), ("source_user_name", "source.user.name"), ("source_address", "source.address")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722051") };
            if _cond {
                if let Some(input) = event.get_string("event.reason") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IPv4 Address <") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("> ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.assigned_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("> ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.reason".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722055") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> Client Type: %{GREEDYDATA:user_agent.original}$
                    // Grok pattern: ^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} Client Type: %{GREEDYDATA:user_agent.original}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> Client Type: %{GREEDYDATA:user_agent.original}$",
                                [
                                    ("source_user_group_name", "source.user.group.name"),
                                    ("source_user_name", "source.user.name"),
                                    ("source_address", "source.address")
                                ]
                            ),
                            cached_grok!(
                                "^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} Client Type: %{GREEDYDATA:user_agent.original}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version} session
                    // Grok pattern: ^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} for %{NOTSPACE:_temp_.cisco.tls_version} session
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version} session"
                            ),
                            cached_grok!(
                                "^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} for %{NOTSPACE:_temp_.cisco.tls_version} session"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version}
                    // Grok pattern: ^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version}"
                            ),
                            cached_grok!(
                                "^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725007") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^SSL session with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} terminated
                    if !cached_grok!("^SSL session with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} terminated").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Device selects trust-point %{DATA:_temp_.cisco.trustpoint} for %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port}$
                    if !cached_grok!("^Device selects trust-point %{DATA:_temp_.cisco.trustpoint} for %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("733100") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[(%{SPACE})?%{DATA:_temp_.cisco.burst.object}\\] drop %{NOTSPACE:_temp_.cisco.burst.id} exceeded. Current burst rate is %{INT:_temp_.cisco.burst.current_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_rate}; Current average rate is %{INT:_temp_.cisco.burst.avg_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_avg_rate}; Cumulative total count is %{INT:_temp_.cisco.burst.cumulative_count}
                    if !cached_grok!("\\[(%{SPACE})?%{DATA:_temp_.cisco.burst.object}\\] drop %{NOTSPACE:_temp_.cisco.burst.id} exceeded. Current burst rate is %{INT:_temp_.cisco.burst.current_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_rate}; Current average rate is %{INT:_temp_.cisco.burst.avg_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_avg_rate}; Cumulative total count is %{INT:_temp_.cisco.burst.cumulative_count}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("734001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("DAP: User ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Addr ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.email", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Addr ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Connection ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Connection ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(
                            ": The following DAP records were selected for this connection: ",
                        ) else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.connection_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(
                            ": The following DAP records were selected for this connection: ",
                        ) else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.dap_records", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                    // Grok pattern: ^IPAA: DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$"
                            ),
                            cached_grok!(
                                "^IPAA: DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737006") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                    // Grok pattern: ^IPAA: Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$"
                            ),
                            cached_grok!(
                                "^IPAA: Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool address %{NOTSPACE:_temp_.cisco.pool_address}$
                    // Grok pattern: ^IPAA: Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$"
                            ),
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool address %{NOTSPACE:_temp_.cisco.pool_address}$"
                            ),
                            cached_grok!(
                                "^IPAA: Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737026") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned from local pool %{IP:_temp_.cisco.pool_address}$
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$
                    // Grok pattern: ^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$
                    // Grok pattern: ^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned from local pool %{IP:_temp_.cisco.pool_address}$"
                            ),
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$"
                            ),
                            cached_grok!(
                                "^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$"
                            ),
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$"
                            ),
                            cached_grok!(
                                "^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737034") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, %{GREEDYDATA:event.reason}$
                    // Grok pattern: ^IPAA: %{GREEDYDATA:event.reason}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, %{GREEDYDATA:event.reason}$"
                            ),
                            cached_grok!("^IPAA: %{GREEDYDATA:event.reason}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("751025") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA} Username:%{USER:source.user.name}\\s+%{GREEDYDATA}$
                    if !cached_grok!("^%{DATA} Username:%{USER:source.user.name}\\s+%{GREEDYDATA}$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("805001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Offloaded ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Flow for connection ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Flow for connection ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.connection_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("805002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) =
                            remaining.find(" Flow is no longer offloaded for connection ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" Flow is no longer offloaded for connection ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.connection_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natsrcip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") to ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.natdstip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.mapped_destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            if event.has_value("_temp_.cisco.dap_records") {
                if let Some(s) = event.get_string("_temp_.cisco.dap_records") {
                    let mut parts: Vec<Value> = cached_regex!(",\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_temp_.cisco.dap_records", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("434002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("SFR requested to drop ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.port", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("434004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("SFR requested ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) =
                            remaining.find(" to bypass further packet redirection and process ")
                        else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining
                            .strip_prefix(" to bypass further packet redirection and process ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" flow from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" flow from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" locally") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" locally") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("110002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" for ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("destination.port", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("419002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("from ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.source_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.destination_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("event.reason", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("425005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Interface ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" become active in redundant interface ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" become active in redundant interface ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.cisco.redundant_interface_name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("611103") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User logged out: Uname: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("user.name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                ["602303", "602304"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(": An ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": An ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.direction", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" SA (SPI= ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.tunnel_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" SA (SPI= ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") between ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") between ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" and ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" and ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (user= ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (user= ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(") has been ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") has been ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(".") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("750002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Local:%{IPORHOST:source.address}:%{NUMBER:source.port} Remote:%{IPORHOST:destination.address}:%{NUMBER:destination.port} Username:%{DATA:user.name} %{GREEDYDATA:event.reason}
                    if !cached_grok!("Local:%{IPORHOST:source.address}:%{NUMBER:source.port} Remote:%{IPORHOST:destination.address}:%{NUMBER:destination.port} Username:%{DATA:user.name} %{GREEDYDATA:event.reason}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("713120") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Group = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IP = ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IP = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (msgid=") else {
                            break 'dissect false;
                        };
                        captured.push(("event.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (msgid=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("event.id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("713202") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IP = ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(". ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(". ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet.") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet.") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716039") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Authentication: rejected, group = %{NOTSPACE:source.user.group.name} user = %{USER:source.user.name} , Session Type: %{NOTSPACE:_temp_.cisco.session_type}
                    // Grok pattern: Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> Authentication: rejected, Session Type: %{NOTSPACE:_temp_.cisco.session_type}\\.
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Authentication: rejected, group = %{NOTSPACE:source.user.group.name} user = %{USER:source.user.name} , Session Type: %{NOTSPACE:_temp_.cisco.session_type}"
                            ),
                            cached_grok!(
                                "Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> Authentication: rejected, Session Type: %{NOTSPACE:_temp_.cisco.session_type}\\."
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("750003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Local:%{IPORHOST:source.address}:%{NUMBER:source.port} Remote:%{IPORHOST:destination.address}:%{NUMBER:destination.port} Username:%{DATA:user.name} %{GREEDYDATA:event.reason}
                    if !cached_grok!("Local:%{IPORHOST:source.address}:%{NUMBER:source.port} Remote:%{IPORHOST:destination.address}:%{NUMBER:destination.port} Username:%{DATA:user.name} %{GREEDYDATA:event.reason}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                ["713905", "713904", "713906", "713902", "713901"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^(Group = %{IP}, )?(IP = %{IP:source.address}, )?%{GREEDYDATA:event.reason}$
                    if !cached_grok!("^(Group = %{IP}, )?(IP = %{IP:source.address}, )?%{GREEDYDATA:event.reason}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond =
                { ["419002"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("network.transport", json!("tcp"))?;
            }

            let _cond = {
                [
                    "302014", "302016", "302018", "302021", "302036", "302304", "302306",
                ]
                .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*))
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*))
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes})
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} connection for faddr (?:(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?gaddr (?:(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))))|(?P<_temp__cisco_gaddr_interface>(?:[^:]*)):(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))))/%{NUMBER} laddr (?:(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?(\\s*type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?
                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} connection %{NOTSPACE:_temp_.cisco.connection_id} from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}(?:%{NUMBER} %{NUMBER})?
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms"),
                                    ("event_reason", "event.reason"),
                                    (
                                        "_temp__cisco_termination_initiator",
                                        "_temp_.cisco.termination_initiator"
                                    ),
                                    (
                                        "_temp__cisco_termination_user",
                                        "_temp_.cisco.termination_user"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*))",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms"),
                                    ("event_reason", "event.reason"),
                                    (
                                        "_temp__cisco_termination_initiator",
                                        "_temp_.cisco.termination_initiator"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms"),
                                    ("event_reason", "event.reason"),
                                    (
                                        "_temp__cisco_termination_user",
                                        "_temp_.cisco.termination_user"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms"),
                                    (
                                        "_temp__cisco_termination_user",
                                        "_temp_.cisco.termination_user"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*))",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms"),
                                    ("event_reason", "event.reason"),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes})",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms"),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} connection for faddr (?:(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?gaddr (?:(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))))|(?P<_temp__cisco_gaddr_interface>(?:[^:]*)):(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))))/%{NUMBER} laddr (?:(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?(\\s*type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_gaddr_interface",
                                        "_temp_.cisco.gaddr_interface"
                                    ),
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    ("destination_domain", "destination.domain"),
                                    ("destination_domain", "destination.domain"),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_destination_user_or_sgt",
                                        "_temp_.cisco.destination_user_or_sgt"
                                    ),
                                    ("_temp__natsrcip", "_temp_.natsrcip"),
                                    ("_temp__natsrcip", "_temp_.natsrcip"),
                                    ("source_domain", "source.domain"),
                                    ("source_domain", "source.domain"),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    ),
                                    (
                                        "_temp__cisco_source_user_or_sgt",
                                        "_temp_.cisco.source_user_or_sgt"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "^Teardown %{NOTSPACE:network.transport} connection %{NOTSPACE:_temp_.cisco.connection_id} from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}(?:%{NUMBER} %{NUMBER})?",
                                [
                                    (
                                        "_temp__cisco_source_interface",
                                        "_temp_.cisco.source_interface"
                                    ),
                                    (
                                        "_temp__cisco_destination_interface",
                                        "_temp_.cisco.destination_interface"
                                    ),
                                    ("_temp__duration_hms", "_temp_.duration_hms")
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                ["430001", "430002", "430003", "430004", "430005", ""]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
                    && event.has_value("cisco.message")
                    && event.get("cisco.message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(kv_str) = event.get_string("message") {
                    for pair in cached_regex!(",(?=[A-za-z1-9\\s]+:)")
                        .split(&kv_str)
                        .into_iter()
                    {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once(":") else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            let value = value.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("_temp_.orig_security.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.cisco.distinguished_name")
                    && event
                        .get("_temp_.cisco.distinguished_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(kv_str) = event.get_string("_temp_.cisco.distinguished_name") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.distinguished_name".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            let value = value.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("_temp_.cisco.dn_parts.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            // Painless script
            // Source: if (ctx._temp_?.cisco?.dn_parts == null) {\n  return;\n}\ndef parts = [:];\nctx._temp_.cisco.dn_parts.forEach((k,v) -> {\n  if (params.containsKey(k)) {\n    parts[params[k]] = (v instanceof List) ? v : [v];   // `[v]` is a Painless list literal\n  } else {\n    return false;\n  }\n});\nctx._temp_.cisco.dn_parts = parts;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx._temp_?.cisco?.dn_parts == null) {\n  return;\n}\ndef parts = [:];\nctx._temp_.cisco.dn_parts.forEach((k,v) -> {\n  if (params.containsKey(k)) {\n    parts[params[k]] = (v instanceof List) ? v : [v];   // `[v]` is a Painless list literal\n  } else {\n    return false;\n  }\n});\nctx._temp_.cisco.dn_parts = parts;\n"#
                ),
                cached_params!(
                    "{\"ST\":\"state_or_province\",\"S\":\"state_or_province\",\"P\":\"state_or_province\",\"CN\":\"common_name\",\"C\":\"country\",\"L\":\"locality\",\"O\":\"organization\",\"OU\":\"organizational_unit\"}"
                ),
            )?;

            let _cond = {
                event.has_value("_temp_.cisco.distinguished_name")
                    && event
                        .get("_temp_.cisco.distinguished_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
            };
            if _cond {
                event.remove("_temp_.cisco.distinguished_name");
            }

            let _cond = {
                event.has_value("_temp_.cisco.serial_number")
                    && event
                        .get("_temp_.cisco.serial_number")
                        .is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
            };
            if _cond {
                event.remove("_temp_.cisco.serial_number");
            }

            let _cond = { event.has_value("_temp_.cisco.dn_parts") };
            if _cond {
                event.rename("_temp_.cisco.dn_parts", "tls.server.x509.subject")?;
            }

            let _cond = { event.has_value("_temp_.cisco.distinguished_name") };
            if _cond {
                event.rename(
                    "_temp_.cisco.distinguished_name",
                    "tls.server.x509.subject.distinguished_name",
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.serial_number") };
            if _cond {
                event.rename(
                    "_temp_.cisco.serial_number",
                    "tls.server.x509.serial_number",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("keep_message"))
                        }
                        serde_json::Value::String(s) => s.contains("keep_message"),
                        _ => false,
                    })
                    && !event.has_value("_temp_.cisco.full_message")
            };
            if _cond {
                event.rename("message", "_temp_.cisco.full_message")?;
            }

            event.remove("message");
            event.remove("_temp_.full_message");

            let _cond = { event.has_value("_temp_.orig_security") };
            if _cond {
                // Painless script
                // Source: boolean isEmpty(def value) {\n  return (value instanceof AbstractList? value.size() : value.length()) == 0;\n}\ndef appendOrCreate(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n  dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n String key = path[path.length - 1];\n def existing = dest.get(key);\n return existing == null?\n  dest.put(key, value)\n  : existing instanceof AbstractList?\n    existing.add(value)\n    : dest.put(key, new ArrayList([existing, value]));\n}\ndef msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\ndef dest = new HashMap();\nctx._temp_.cisco['security'] = dest;\nfor (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n if (param == null) {\n   continue;\n }\n param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, field.splitOnToken('.'), entry.getValue()) );\n  dest[param.target] = entry.getValue();\n }\n}\nif (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\nfor (entry in counters.entrySet()) {\n if (best == null || best.getValue() < entry.getValue()) best = entry;\n}\nif (best != null) ctx._temp_.cisco.message_id = best.getKey();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"boolean isEmpty(def value) {\n  return (value instanceof AbstractList? value.size() : value.length()) == 0;\n}\ndef appendOrCreate(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n  dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n String key = path[path.length - 1];\n def existing = dest.get(key);\n return existing == null?\n  dest.put(key, value)\n  : existing instanceof AbstractList?\n    existing.add(value)\n    : dest.put(key, new ArrayList([existing, value]));\n}\ndef msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\ndef dest = new HashMap();\nctx._temp_.cisco['security'] = dest;\nfor (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n if (param == null) {\n   continue;\n }\n param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, field.splitOnToken('.'), entry.getValue()) );\n  dest[param.target] = entry.getValue();\n }\n}\nif (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\nfor (entry in counters.entrySet()) {\n if (best == null || best.getValue() < entry.getValue()) best = entry;\n}\nif (best != null) ctx._temp_.cisco.message_id = best.getKey();\n"#
                    ),
                    cached_params!(
                        "{\"ACPolicy\":{\"target\":\"ac_policy\",\"id\":[\"430001\",\"430002\",\"430003\"],\"ecs\":[\"_temp_.cisco.rule_name\"]},\"AccessControlRuleAction\":{\"target\":\"access_control_rule_action\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"event.outcome\"]},\"AccessControlRuleName\":{\"target\":\"access_control_rule_name\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"_temp_.cisco.rule_name\"]},\"AccessControlRuleReason\":{\"target\":\"access_control_rule_reason\",\"id\":[\"430002\",\"430003\"]},\"ApplicationProtocol\":{\"target\":\"application_protocol\",\"ecs\":[\"network.protocol\"]},\"ArchiveDepth\":{\"target\":\"archive_depth\",\"id\":[\"430004\",\"430005\"]},\"ArchiveFileName\":{\"target\":\"archive_file_name\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"file.name\"]},\"ArchiveFileStatus\":{\"target\":\"archive_file_status\",\"id\":[\"430004\",\"430005\"]},\"ArchiveSHA256\":{\"target\":\"archive_sha256\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"file.hash.sha256\"]},\"Classification\":{\"target\":\"classification\",\"id\":[\"430001\"]},\"Client\":{\"target\":\"client\",\"ecs\":[\"network.application\"]},\"ClientVersion\":{\"target\":\"client_version\",\"id\":[\"430002\",\"430003\"]},\"ConnectionDuration\":{\"target\":\"connection_duration\",\"id\":[\"430003\"],\"ecs\":[\"event.duration\"]},\"DNS_Sinkhole\":{\"target\":\"dns_sinkhole\",\"id\":[\"430002\",\"430003\"]},\"DNS_TTL\":{\"target\":\"dns_ttl\",\"id\":[\"430002\",\"430003\"]},\"DNSQuery\":{\"target\":\"dns_query\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"dns.question.name\"]},\"DNSRecordType\":{\"target\":\"dns_record_type\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"dns.question.type\"]},\"DNSResponseType\":{\"target\":\"dns_response_type\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"dns.response_code\"]},\"DNSSICategory\":{\"target\":\"dnssi_category\",\"id\":[\"430002\",\"430003\"]},\"DstIP\":{\"target\":\"dst_ip\",\"ecs\":[\"destination.address\"]},\"DstPort\":{\"target\":\"dst_port\",\"ecs\":[\"destination.port\"]},\"EgressInterface\":{\"target\":\"egress_interface\",\"id\":[\"430001\",\"430002\",\"430003\"],\"ecs\":[\"_temp_.cisco.destination_interface\"]},\"EgressZone\":{\"target\":\"egress_zone\",\"id\":[\"430001\",\"430002\",\"430003\"]},\"Endpoint Profile\":{\"target\":\"endpoint_profile\",\"id\":[\"430002\",\"430003\"]},\"FileAction\":{\"target\":\"file_action\",\"id\":[\"430004\",\"430005\"]},\"FileCount\":{\"target\":\"file_count\",\"id\":[\"430002\",\"430003\"]},\"FileDirection\":{\"target\":\"file_direction\",\"id\":[\"430004\",\"430005\"]},\"FileName\":{\"target\":\"file_name\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"file.name\"]},\"FilePolicy\":{\"target\":\"file_policy\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"_temp_.cisco.rule_name\"]},\"FileSHA256\":{\"target\":\"file_sha256\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"file.hash.sha256\"]},\"FileSandboxStatus\":{\"target\":\"file_sandbox_status\",\"id\":[\"430004\",\"430005\"]},\"FileSize\":{\"target\":\"file_size\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"file.size\"]},\"FileStorageStatus\":{\"target\":\"file_storage_status\",\"id\":[\"430004\",\"430005\"]},\"FileType\":{\"target\":\"file_type\",\"id\":[\"430004\",\"430005\"]},\"FirstPacketSecond\":{\"target\":\"first_packet_second\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"event.start\"]},\"GID\":{\"target\":\"gid\",\"id\":[\"430001\"],\"ecs\":[\"service.id\"]},\"HTTPReferer\":{\"target\":\"http_referer\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"http.request.referrer\"]},\"HTTPResponse\":{\"target\":\"http_response\",\"id\":[\"430001\",\"430002\",\"430003\"],\"ecs\":[\"http.response.status_code\"]},\"ICMPCode\":{\"target\":\"icmp_code\",\"id\":[\"430001\",\"430002\",\"430003\"]},\"ICMPType\":{\"target\":\"icmp_type\",\"id\":[\"430001\",\"430002\",\"430003\"]},\"IPReputationSICategory\":{\"target\":\"ip_reputation_si_category\",\"id\":[\"430002\",\"430003\"]},\"IPSCount\":{\"target\":\"ips_count\",\"id\":[\"430002\",\"430003\"]},\"IngressInterface\":{\"target\":\"ingress_interface\",\"id\":[\"430001\",\"430002\",\"430003\"],\"ecs\":[\"_temp_.cisco.source_interface\"]},\"IngressZone\":{\"target\":\"ingress_zone\",\"id\":[\"430001\",\"430002\",\"430003\"]},\"InitiatorBytes\":{\"target\":\"initiator_bytes\",\"id\":[\"430003\"],\"ecs\":[\"source.bytes\"]},\"InitiatorPackets\":{\"target\":\"initiator_packets\",\"id\":[\"430003\"],\"ecs\":[\"source.packets\"]},\"InlineResult\":{\"target\":\"inline_result\",\"id\":[\"430001\"],\"ecs\":[\"event.outcome\"]},\"IntrusionPolicy\":{\"target\":\"intrusion_policy\",\"id\":[\"430001\"],\"ecs\":[\"_temp_.cisco.rule_name\"]},\"MPLS_Label\":{\"target\":\"mpls_label\",\"id\":[\"430001\"]},\"Message\":{\"target\":\"message\",\"id\":[\"430001\"],\"ecs\":[\"message\"]},\"NAPPolicy\":{\"target\":\"nap_policy\",\"id\":[\"430001\",\"430002\",\"430003\"]},\"NetBIOSDomain\":{\"target\":\"net_bios_domain\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"host.hostname\"]},\"NumIOC\":{\"target\":\"num_ioc\",\"id\":[\"430001\"]},\"Prefilter Policy\":{\"target\":\"prefilter_policy\",\"id\":[\"430002\",\"430003\"]},\"Priority\":{\"target\":\"priority\",\"id\":[\"430001\"]},\"Protocol\":{\"target\":\"protocol\",\"ecs\":[\"network.transport\"]},\"ReferencedHost\":{\"target\":\"referenced_host\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"url.domain\"]},\"ResponderBytes\":{\"target\":\"responder_bytes\",\"id\":[\"430003\"],\"ecs\":[\"destination.bytes\"]},\"ResponderPackets\":{\"target\":\"responder_packets\",\"id\":[\"430003\"],\"ecs\":[\"destination.packets\"]},\"Revision\":{\"target\":\"revision\",\"id\":[\"430001\"]},\"SHA_Disposition\":{\"target\":\"sha_disposition\",\"id\":[\"430004\",\"430005\"]},\"SID\":{\"target\":\"sid\",\"id\":[\"430001\"]},\"SSLActualAction\":{\"target\":\"ssl_actual_action\",\"ecs\":[\"event.outcome\"]},\"SSLCertificate\":{\"target\":\"ssl_certificate\",\"id\":[\"430002\",\"430003\",\"430004\",\"430005\"]},\"SSLExpectedAction\":{\"target\":\"ssl_expected_action\",\"id\":[\"430002\",\"430003\"]},\"SSLFlowStatus\":{\"target\":\"ssl_flow_status\",\"id\":[\"430002\",\"430003\",\"430004\",\"430005\"]},\"SSLPolicy\":{\"target\":\"ssl_policy\",\"id\":[\"430002\",\"430003\"]},\"SSLRuleName\":{\"target\":\"ssl_rule_name\",\"id\":[\"430002\",\"430003\"]},\"SSLServerCertStatus\":{\"target\":\"ssl_server_cert_status\",\"id\":[\"430002\",\"430003\"]},\"SSLServerName\":{\"target\":\"ssl_server_name\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"server.domain\"]},\"SSLSessionID\":{\"target\":\"ssl_session_id\",\"id\":[\"430002\",\"430003\"]},\"SSLTicketID\":{\"target\":\"ssl_ticket_id\",\"id\":[\"430002\",\"430003\"]},\"SSLURLCategory\":{\"target\":\"sslurl_category\",\"id\":[\"430002\",\"430003\"]},\"SSLVersion\":{\"target\":\"ssl_version\",\"id\":[\"430002\",\"430003\"]},\"SSSLCipherSuite\":{\"target\":\"sssl_cipher_suite\",\"id\":[\"430002\",\"430003\"]},\"SecIntMatchingIP\":{\"target\":\"sec_int_matching_ip\",\"id\":[\"430002\",\"430003\"]},\"Security Group\":{\"target\":\"security_group\",\"id\":[\"430002\",\"430003\"]},\"SperoDisposition\":{\"target\":\"spero_disposition\",\"id\":[\"430004\",\"430005\"]},\"SrcIP\":{\"target\":\"src_ip\",\"ecs\":[\"source.address\"]},\"SrcPort\":{\"target\":\"src_port\",\"ecs\":[\"source.port\"]},\"TCPFlags\":{\"target\":\"tcp_flags\",\"id\":[\"430002\",\"430003\"]},\"ThreatName\":{\"target\":\"threat_name\",\"id\":[\"430005\"],\"ecs\":[\"_temp_.cisco.threat_category\"]},\"ThreatScore\":{\"target\":\"threat_score\",\"id\":[\"430005\"],\"ecs\":[\"_temp_.cisco.threat_level\"]},\"Tunnel or Prefilter Rule\":{\"target\":\"tunnel_or_prefilter_rule\",\"id\":[\"430002\",\"430003\"]},\"URI\":{\"target\":\"uri\",\"id\":[\"430004\",\"430005\"],\"ecs\":[\"url.original\"]},\"URL\":{\"target\":\"url\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"url.original\"]},\"URLCategory\":{\"target\":\"url_category\",\"id\":[\"430002\",\"430003\"]},\"URLReputation\":{\"target\":\"url_reputation\",\"id\":[\"430002\",\"430003\"]},\"URLSICategory\":{\"target\":\"urlsi_category\",\"id\":[\"430002\",\"430003\"]},\"User\":{\"target\":\"user\",\"ecs\":[\"user.id\",\"user.name\"]},\"UserAgent\":{\"target\":\"user_agent\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"user_agent.original\"]},\"VLAN_ID\":{\"target\":\"vlan_id\",\"id\":[\"430001\",\"430002\",\"430003\"]},\"WebApplication\":{\"target\":\"web_application\",\"ecs\":[\"network.application\"]},\"originalClientSrcIP\":{\"target\":\"original_client_src_ip\",\"id\":[\"430002\",\"430003\"],\"ecs\":[\"client.address\"]}}"
                    ),
                )?;
            }

            // Painless script
            // Source: def getField(Map src, String[] path) {\n for (int i=0; i<path.length-1; i++) {\n  src = src.getOrDefault(path[i], null);\n  if (src == null || !(src instanceof Map)) {\n    return null;\n  }\n }\n return src[path[path.length-1]];\n}\ndef setField(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n   dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n dest[path[path.length-1]] = value;\n}\nfor (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  def param = entry.getValue();\n  String oldVal = getField(ctx, srcField.splitOnToken('.'));\n  if (oldVal == null) continue;\n  def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def getField(Map src, String[] path) {\n for (int i=0; i<path.length-1; i++) {\n  src = src.getOrDefault(path[i], null);\n  if (src == null || !(src instanceof Map)) {\n    return null;\n  }\n }\n return src[path[path.length-1]];\n}\ndef setField(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n   dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n dest[path[path.length-1]] = value;\n}\nfor (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  def param = entry.getValue();\n  String oldVal = getField(ctx, srcField.splitOnToken('.'));\n  if (oldVal == null) continue;\n  def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"ctx._temp_.cisco.message_id\":{\"target\":\"event.action\",\"map\":{\"430001\":\"intrusion-detected\",\"430002\":\"connection-started\",\"430003\":\"connection-finished\",\"430004\":\"file-detected\",\"430005\":\"malware-detected\"}},\"dns.question.type\":{\"map\":{\"a host address\":\"A\",\"ip6 address\":\"AAAA\",\"text strings\":\"TXT\",\"a domain name pointer\":\"PTR\",\"an authoritative name server\":\"NS\",\"the canonical name for an alias\":\"CNAME\",\"marks the start of a zone of authority\":\"SOA\",\"mail exchange\":\"MX\",\"server selection\":\"SRV\"}},\"dns.response_code\":{\"map\":{\"non-existent domain\":\"NXDOMAIN\",\"server failure\":\"SERVFAIL\",\"query refused\":\"REFUSED\",\"no error\":\"NOERROR\"}}}"
                ),
            )?;

            let _cond =
                { event.has_value("dns.question.type") && !event.has_value("dns.response_code") };
            if _cond {
                event.set("dns.response_code", json!("NOERROR"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("430001") };
            if _cond {
                event.set("event.action", json!("intrusion-detected"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("430002") };
            if _cond {
                event.set("event.action", json!("connection-started"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("430003") };
            if _cond {
                event.set("event.action", json!("connection-finished"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("430004") };
            if _cond {
                event.set("event.action", json!("file-detected"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("430005") };
            if _cond {
                event.set("event.action", json!("malware-detected"))?;
            }

            let v = json!(
                event
                    .get("event.duration")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("_temp_.duration_hms", v)?;
            }

            let _cond = { event.has_value("_temp_.duration_hms") };
            if _cond {
                // Painless script
                // Source: long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        }\n    }\n    return total + cur;\n}\nif (ctx?.event == null) {\n    ctx['event'] = new HashMap();\n}\nlong nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\nctx.event['duration'] = nanos;\nif (ctx['@timestamp'] != null) {\n    String end = ctx['@timestamp'];\n    ctx.event['end'] = end;\n    try {\n        ctx.event['start'] = ZonedDateTime.ofInstant(\n            Instant.parse(end).minusNanos(nanos),\n            ZoneOffset.UTC);\n    } catch (Exception e) {\n        // If timestamp parsing fails, just set duration\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        }\n    }\n    return total + cur;\n}\nif (ctx?.event == null) {\n    ctx['event'] = new HashMap();\n}\nlong nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\nctx.event['duration'] = nanos;\nif (ctx['@timestamp'] != null) {\n    String end = ctx['@timestamp'];\n    ctx.event['end'] = end;\n    try {\n        ctx.event['start'] = ZonedDateTime.ofInstant(\n            Instant.parse(end).minusNanos(nanos),\n            ZoneOffset.UTC);\n    } catch (Exception e) {\n        // If timestamp parsing fails, just set duration\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.source_user_or_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.source_user_or_sgt") {
                    // Grok pattern: (?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))))
                    // Grok pattern: (?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))))",
                                [(
                                    "_temp__cisco_source_username",
                                    "_temp_.cisco.source_username"
                                )]
                            ),
                            cached_grok_mapped!(
                                "(?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?",
                                [(
                                    "_temp__cisco_source_username",
                                    "_temp_.cisco.source_username"
                                )]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_temp_.cisco.source_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.source_sgt") {
                    // Grok pattern: (?:(, *)?(%{NUMBER:_temp_.cisco.source_user_security_group_tag})?:?%{WORD:_temp_.cisco.source_user_security_group_tag_name}?)
                    if !cached_grok!("(?:(, *)?(%{NUMBER:_temp_.cisco.source_user_security_group_tag})?:?%{WORD:_temp_.cisco.source_user_security_group_tag_name}?)").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("_temp_.cisco.source_user_security_group_tag") {
                if let Some(val) = event.get("_temp_.cisco.source_user_security_group_tag") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.source_user_security_group_tag".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.source_user_security_group_tag", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.destination_user_or_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.destination_user_or_sgt") {
                    // Grok pattern: (?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))))
                    // Grok pattern: (?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))))",
                                [(
                                    "_temp__cisco_destination_username",
                                    "_temp_.cisco.destination_username"
                                )]
                            ),
                            cached_grok_mapped!(
                                "(?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?",
                                [(
                                    "_temp__cisco_destination_username",
                                    "_temp_.cisco.destination_username"
                                )]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_temp_.cisco.destination_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.destination_sgt") {
                    // Grok pattern: (?:(, *)?(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})?:?%{WORD:_temp_.cisco.destination_user_security_group_tag_name}?)
                    if !cached_grok!("(?:(, *)?(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})?:?%{WORD:_temp_.cisco.destination_user_security_group_tag_name}?)").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("_temp_.cisco.destination_user_security_group_tag") {
                if let Some(val) = event.get("_temp_.cisco.destination_user_security_group_tag") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.destination_user_security_group_tag".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "_temp_.cisco.destination_user_security_group_tag",
                        converted,
                    )?;
                }
            }

            event.remove("_temp_.cisco.source_user_or_sgt");
            event.remove("_temp_.cisco.destination_user_or_sgt");

            let _cond = {
                event.has_value("_temp_.cisco.source_username")
                    && event.get_str("_temp_.cisco.source_username") == Some("")
            };
            if _cond {
                if event.remove("_temp_.cisco.source_username").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_temp_.cisco.source_username".into(),
                    });
                }
            }

            let _cond = {
                event.has_value("_temp_.cisco.destination_username")
                    && event.get_str("_temp_.cisco.destination_username") == Some("")
            };
            if _cond {
                if event.remove("_temp_.cisco.destination_username").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_temp_.cisco.destination_username".into(),
                    });
                }
            }

            let _cond = {
                !event.has_value("source.user.name")
                    && event.has_value("_temp_.cisco.source_username")
            };
            if _cond {
                event.set(
                    "source.user.name",
                    json!(
                        event
                            .get("_temp_.cisco.source_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("destination.user.name")
                    && event.has_value("_temp_.cisco.destination_username")
            };
            if _cond {
                event.set(
                    "destination.user.name",
                    json!(
                        event
                            .get("_temp_.cisco.destination_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name") && event.get_str("source.user.name") != Some("")
            };
            if _cond {
                if let Some(input) = event.get_string("source.user.name") {
                    // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?P<source_user_email>(?:(?:(?P<source_user_name>(?:[^@$]+)))@%{HOSTNAME:source.user.domain}))
                    // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?:(?P<source_user_name>(?:[^@$]+)))
                    // Grok pattern: \\*+
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?P<source_user_email>(?:(?:(?P<source_user_name>(?:[^@$]+)))@%{HOSTNAME:source.user.domain}))",
                                [
                                    ("source_user_email", "source.user.email"),
                                    ("source_user_name", "source.user.name")
                                ]
                            ),
                            cached_grok_mapped!(
                                "((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?:(?P<source_user_name>(?:[^@$]+)))",
                                [("source_user_name", "source.user.name")]
                            ),
                            cached_grok!("\\*+"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                if let Some(input) = event.get_string("destination.user.name") {
                    // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?P<destination_user_email>(?:(?:(?P<destination_user_name>(?:[^@$]+)))@%{HOSTNAME:destination.user.domain}))
                    // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?:(?P<destination_user_name>(?:[^@$]+)))
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?P<destination_user_email>(?:(?:(?P<destination_user_name>(?:[^@$]+)))@%{HOSTNAME:destination.user.domain}))",
                                [
                                    ("destination_user_email", "destination.user.email"),
                                    ("destination_user_name", "destination.user.name")
                                ]
                            ),
                            cached_grok_mapped!(
                                "((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?:(?P<destination_user_name>(?:[^@$]+)))",
                                [("destination_user_name", "destination.user.name")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.application") {
                map_strings(
                    event,
                    "network.application",
                    "network.application",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("file.type") {
                map_strings(event, "file.type", "file.type", str::to_lowercase)?;
            }

            if event.has_value("network.direction") {
                map_strings(
                    event,
                    "network.direction",
                    "network.direction",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.type") {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
            }

            let _cond = { event.has_value("network.transport") };
            if _cond {
                // Painless script
                // Source: def net = ctx.network; def iana = params[net.transport]; if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} def reverse = new HashMap(); def[] arr = new def[] { null }; for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  reverse.put(String.format(\"%d\", arr), entry.getKey());\n} def trans = reverse[net.transport]; if (trans != null) {\n  net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def net = ctx.network; def iana = params[net.transport]; if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} def reverse = new HashMap(); def[] arr = new def[] { null }; for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  reverse.put(String.format(\"%d\", arr), entry.getKey());\n} def trans = reverse[net.transport]; if (trans != null) {\n  net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n"#
                    ),
                    cached_params!(
                        "{\"icmp\":1,\"igmp\":2,\"ipv4\":4,\"tcp\":6,\"egp\":8,\"igp\":9,\"pup\":12,\"udp\":17,\"rdp\":27,\"irtp\":28,\"dccp\":33,\"idpr\":35,\"ipv6\":41,\"ipv6-route\":43,\"ipv6-frag\":44,\"rsvp\":46,\"gre\":47,\"esp\":50,\"ipv6-icmp\":58,\"ipv6-nonxt\":59,\"ipv6-opts\":60}"
                    ),
                )?;
            }

            let _cond = { event.get_str("network.transport") == Some("icmpv6") };
            if _cond {
                event.set("network.transport", json!("ipv6-icmp"))?;
            }

            if event.has_value("_temp_.outcome") {
                map_strings(event, "_temp_.outcome", "_temp_.outcome", str::to_lowercase)?;
            }

            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            if event.has_value("source.port") {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            if event.has_value("destination.port") {
                if let Some(val) = event.get("destination.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            if event.has_value("source.bytes") {
                if let Some(val) = event.get("source.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("source.bytes", converted)?;
                }
            }

            if event.has_value("destination.bytes") {
                if let Some(val) = event.get("destination.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("destination.bytes", converted)?;
                }
            }

            if event.has_value("network.bytes") {
                if let Some(val) = event.get("network.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "network.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("network.bytes", converted)?;
                }
            }

            if event.has_value("source.packets") {
                if let Some(val) = event.get("source.packets") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.packets".into(),
                            message,
                        }
                    })?;
                    event.set("source.packets", converted)?;
                }
            }

            if event.has_value("destination.packets") {
                if let Some(val) = event.get("destination.packets") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.packets".into(),
                            message,
                        }
                    })?;
                    event.set("destination.packets", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.mapped_source_port") {
                if let Some(val) = event.get("_temp_.cisco.mapped_source_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.mapped_source_port".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.mapped_source_port", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.mapped_destination_port") {
                if let Some(val) = event.get("_temp_.cisco.mapped_destination_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.mapped_destination_port".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.mapped_destination_port", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.icmp_code") {
                if let Some(val) = event.get("_temp_.cisco.icmp_code") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.icmp_code".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.icmp_code", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.icmp_type") {
                if let Some(val) = event.get("_temp_.cisco.icmp_type") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.icmp_type".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.icmp_type", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.original_iana_number") {
                if let Some(val) = event.get("_temp_.cisco.original_iana_number") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.original_iana_number".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.original_iana_number", converted)?;
                }
            }

            if event.has_value("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

            if event.has_value("file.size") {
                if let Some(val) = event.get("file.size") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "file.size".into(),
                            message,
                        }
                    })?;
                    event.set("file.size", converted)?;
                }
            }

            if event.has_value("network.iana_number") {
                if let Some(val) = event.get("network.iana_number") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "network.iana_number".into(),
                            message,
                        }
                    })?;
                    event.set("network.iana_number", converted)?;
                }
            }

            if event.has_value("sip.to.uri.port") {
                if let Some(val) = event.get("sip.to.uri.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "sip.to.uri.port".into(),
                            message,
                        }
                    })?;
                    event.set("sip.to.uri.port", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.connections_in_use") {
                if let Some(val) = event.get("_temp_.cisco.connections_in_use") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.connections_in_use".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.connections_in_use", converted)?;
                }
            }

            if event.has_value("_temp_.cisco.connections_most_used") {
                if let Some(val) = event.get("_temp_.cisco.connections_most_used") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_temp_.cisco.connections_most_used".into(),
                            message,
                        }
                    })?;
                    event.set("_temp_.cisco.connections_most_used", converted)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(?:%{IP:source.ip}|%{GREEDYDATA:source.domain})$
                    if !cached_grok!("^(?:%{IP:source.ip}|%{GREEDYDATA:source.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: ^(?:%{IP:destination.ip}|%{GREEDYDATA:destination.domain})$
                    if !cached_grok!("^(?:%{IP:destination.ip}|%{GREEDYDATA:destination.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("client.address") };
            if _cond {
                if let Some(input) = event.get_string("client.address") {
                    // Grok pattern: ^(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})$
                    if !cached_grok!("^(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("server.address") };
            if _cond {
                if let Some(input) = event.get_string("server.address") {
                    // Grok pattern: ^(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})$
                    if !cached_grok!("^(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { event.has_value("_temp_.natsrcip") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.natsrcip") {
                    // Grok pattern: ^(?:%{IP:_temp_.cisco.mapped_source_ip}|%{GREEDYDATA:_temp_.cisco.mapped_source_host})$
                    if !cached_grok!("^(?:%{IP:_temp_.cisco.mapped_source_ip}|%{GREEDYDATA:_temp_.cisco.mapped_source_host})$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_temp_.natdstip") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.natdstip") {
                    // Grok pattern: ^(?:%{IP:_temp_.cisco.mapped_destination_ip}|%{GREEDYDATA:_temp_.cisco.mapped_destination_host})$
                    if !cached_grok!("^(?:%{IP:_temp_.cisco.mapped_destination_ip}|%{GREEDYDATA:_temp_.cisco.mapped_destination_host})$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                !condition_eq(
                    event.get("_temp_.cisco.mapped_source_ip"),
                    event.get("source.ip"),
                )
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_source_ip")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.nat.ip", v)?;
                }
            }

            if event.has_value("source.nat.ip") {
                if let Some(val) = event.get("source.nat.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.nat.ip".into(),
                            message,
                        })?;
                    event.set("source.nat.ip", converted)?;
                }
            }

            let _cond = {
                !condition_eq(
                    event.get("_temp_.cisco.mapped_source_port"),
                    event.get("source.port"),
                )
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_source_port")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.nat.port", v)?;
                }
            }

            if event.has_value("source.nat.port") {
                if let Some(val) = event.get("source.nat.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.nat.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.nat.port", converted)?;
                }
            }

            let _cond = {
                !condition_eq(
                    event.get("_temp_.cisco.mapped_destination_ip"),
                    event.get("destination.ip"),
                )
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_destination_ip")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.nat.ip", v)?;
                }
            }

            if event.has_value("destination.nat.ip") {
                if let Some(val) = event.get("destination.nat.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.nat.ip".into(),
                            message,
                        })?;
                    event.set("destination.nat.ip", converted)?;
                }
            }

            let _cond = {
                !condition_eq(
                    event.get("_temp_.cisco.mapped_destination_port"),
                    event.get("destination.port"),
                )
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_destination_port")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.nat.port", v)?;
                }
            }

            if event.has_value("destination.nat.port") {
                if let Some(val) = event.get("destination.nat.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.nat.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.nat.port", converted)?;
                }
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("internal"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("external"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.egress.zone")
                    && event.has_value("observer.ingress.zone")
                    && ((!(event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })) && !(event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    }))) || (!(event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })) && !(event.get("_temp_.internal_zones").is_some_and(
                        |v| match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        },
                    ))))
            };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.set(
                    "_temp_.url_domain",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            let _cond = { event.has_value("_temp_.url_domain") };
            if _cond {
                event.append_unique(
                    "url.domain",
                    json!(
                        event
                            .get("_temp_.url_domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.tls_version") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.tls_version") {
                    // Grok pattern: (?P<tls_version_protocol>(?:[A-Z]+))v%{NUMBER:tls.version}
                    // Grok pattern: (?P<tls_version_protocol>(?:[A-Z]+))
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?P<tls_version_protocol>(?:[A-Z]+))v%{NUMBER:tls.version}",
                                [("tls_version_protocol", "tls.version_protocol")]
                            ),
                            cached_grok_mapped!(
                                "(?P<tls_version_protocol>(?:[A-Z]+))",
                                [("tls_version_protocol", "tls.version_protocol")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            event.remove("_temp_.cisco.tls_version");

            let _cond = {
                event.has_value("_temp_.cisco.message_id")
                    && event.get_str("_temp_.cisco.message_id") != Some("")
            };
            if _cond {
                event.rename("_temp_.cisco.message_id", "event.code")?;
            }

            let _cond = { event.has_value("_temp_.cisco") };
            if _cond {
                event.rename("_temp_.cisco", "cisco.asa")?;
            }

            if event.has_value("cisco.asa.list_id") {
                event.rename("cisco.asa.list_id", "cisco.asa.rule_name")?;
            }

            // Painless script
            // Source: params.get(ctx.event.code)?.get(ctx._temp_.outcome)?.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"params.get(ctx.event.code)?.get(ctx._temp_.outcome)?.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"106100\":{\"denied\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"permitted\":{\"type\":[\"connection\",\"allowed\"],\"outcome\":\"success\",\"action\":\"firewall-rule\"},\"est-allowed\":{\"type\":[\"connection\",\"allowed\"],\"outcome\":\"success\",\"action\":\"firewall-rule\"}},\"106102\":{\"denied\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"permitted\":{\"type\":[\"connection\",\"allowed\"],\"outcome\":\"success\",\"action\":\"firewall-rule\"}},\"111004\":{\"failed\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"failure\",\"action\":\"configuration\"},\"ok\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"action\":\"configuration\"}}}"
                ),
            )?;

            // Painless script
            // Source: params.get(ctx.event.code)?.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"params.get(ctx.event.code)?.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"106001\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106002\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106006\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106007\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106010\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106012\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106013\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106014\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106015\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106016\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106017\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106018\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106020\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106021\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106022\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106023\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106027\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"106103\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"110002\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"111007\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\",\"action\":\"configuration\"},\"111009\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\",\"action\":\"configuration\"},\"111010\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"action\":\"configuration\"},\"113004\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"113005\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"denied\",\"info\"],\"outcome\":\"failure\",\"action\":\"logon-failed\"},\"113008\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"113009\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"113011\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"113012\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"113015\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"denied\",\"info\"],\"outcome\":\"failure\",\"action\":\"logon-failed\"},\"113019\":{\"type\":[\"connection\",\"end\"],\"action\":\"client-vpn-disconnected\"},\"113021\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"denied\",\"info\"],\"outcome\":\"failure\",\"action\":\"logon-failed\"},\"113022\":{\"type\":[\"info\"],\"outcome\":\"failure\",\"action\":\"server-failed\"},\"113023\":{\"type\":[\"info\"],\"outcome\":\"success\",\"action\":\"server-active\"},\"113029\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113030\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113031\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113032\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113033\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113034\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113035\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113036\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113037\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113038\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"113039\":{\"category\":[\"network\",\"session\"],\"type\":[\"connection\",\"start\"],\"action\":\"client-vpn-connected\",\"outcome\":\"success\"},\"113040\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"302013\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"302014\":{\"type\":[\"connection\",\"end\"],\"action\":\"flow-expiration\"},\"302015\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"302016\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"302018\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"302020\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"302021\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"302022\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"302023\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"302024\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"302025\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"302026\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"302027\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"302036\":{\"type\":[\"connection\",\"end\"],\"action\":\"flow-expiration\"},\"302304\":{\"type\":[\"connection\",\"end\"],\"action\":\"flow-expiration\"},\"302306\":{\"type\":[\"connection\",\"end\"],\"action\":\"flow-expiration\"},\"303002\":{\"category\":[\"network\",\"file\"],\"type\":[\"access\"],\"outcome\":\"success\",\"action\":\"ftp\"},\"304001\":{\"type\":[\"access\",\"allowed\"],\"outcome\":\"success\",\"action\":\"url-access\"},\"304002\":{\"type\":[\"access\",\"denied\"],\"outcome\":\"failure\",\"action\":\"url-access\"},\"305011\":{\"category\":[\"network\",\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\",\"action\":\"nat-slot\"},\"305012\":{\"category\":[\"network\",\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\",\"action\":\"nat-slot\"},\"313001\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"313004\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"313005\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"313008\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"313009\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"315011\":{\"type\":[\"connection\",\"end\"],\"action\":\"ssh-session-ended\"},\"322001\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"338001\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338002\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338003\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338004\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338005\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"dynamic-filter\"},\"338006\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"dynamic-filter\"},\"338007\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"dynamic-filter\"},\"338008\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"dynamic-filter\"},\"338101\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338102\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338103\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338104\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338201\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338202\":{\"type\":[\"connection\",\"info\"],\"action\":\"dynamic-filter\"},\"338203\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"dynamic-filter\"},\"338204\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"dynamic-filter\"},\"338301\":{\"type\":[\"connection\",\"info\"],\"action\":\"firewall-rule\"},\"419002\":{\"type\":[\"connection\",\"info\"],\"action\":\"firewall-rule\"},\"425005\":{\"type\":[\"info\"],\"action\":\"interface-switchover\"},\"434002\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"434004\":{\"type\":[\"info\"],\"action\":\"bypass\"},\"502103\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\",\"action\":\"privilege-level-changed\"},\"507003\":{\"type\":[\"connection\",\"end\"],\"action\":\"flow-termination\"},\"602303\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"sa-created\"},\"602304\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"sa-deleted\"},\"605004\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"denied\",\"info\"],\"outcome\":\"failure\",\"action\":\"logon-failed\"},\"605005\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"607001\":{\"type\":[\"info\"],\"outcome\":\"success\",\"action\":\"firewall-rule\"},\"609001\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-creation\"},\"609002\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-expiration\"},\"611101\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"611102\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"denied\",\"info\"],\"outcome\":\"failure\",\"action\":\"logon-failed\"},\"611103\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"info\"],\"outcome\":\"success\",\"action\":\"logged-out\"},\"710003\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"710005\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"710006\":{\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\",\"action\":\"firewall-rule\"},\"713049\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"firewall-rule\"},\"713120\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\",\"action\":\"firewall-rule\"},\"713202\":{\"type\":[\"connection\",\"info\"],\"action\":\"firewall-rule\"},\"713901\":{\"type\":[\"info\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"713902\":{\"type\":[\"info\"],\"outcome\":\"failure\",\"action\":\"client-vpn-error\"},\"713903\":{\"type\":[\"info\"]},\"713904\":{\"type\":[\"info\"]},\"713905\":{\"type\":[\"info\"]},\"713906\":{\"type\":[\"info\"]},\"716002\":{\"type\":[\"connection\",\"end\"],\"action\":\"client-vpn-disconnected\"},\"716003\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\"},\"716039\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"denied\",\"info\"],\"action\":\"logon-failed\",\"outcome\":\"failure\"},\"716058\":{\"type\":[\"connection\",\"end\"],\"action\":\"client-vpn-disconnected\"},\"716059\":{\"type\":[\"connection\",\"start\"],\"action\":\"client-vpn-resumed\"},\"721016\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"client-vpn-connected\"},\"721018\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"client-vpn-disconnected\"},\"722011\":{\"type\":[\"info\"]},\"722022\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"722023\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\"},\"722028\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\"},\"722032\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\"},\"722033\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\"},\"722034\":{\"type\":[\"connection\",\"info\"]},\"722035\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"failure\"},\"722037\":{\"type\":[\"connection\",\"end\"]},\"722041\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"failure\"},\"722051\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\",\"action\":\"address-assigned\"},\"722055\":{\"type\":[\"connection\",\"info\"]},\"725001\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\"},\"725002\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"725007\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"failure\"},\"725016\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"733100\":{\"type\":[\"info\"]},\"734001\":{\"category\":[\"authentication\",\"network\"],\"type\":[\"allowed\",\"info\"],\"outcome\":\"success\",\"action\":\"logged-in\"},\"737003\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"failure\"},\"737006\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"737016\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"737026\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"737034\":{\"type\":[\"connection\",\"info\"],\"outcome\":\"failure\"},\"750002\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"connection-started\"},\"750003\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"failure\"},\"805001\":{\"type\":[\"connection\",\"start\"],\"outcome\":\"success\",\"action\":\"flow-offload-started\"},\"805002\":{\"type\":[\"connection\",\"end\"],\"outcome\":\"success\",\"action\":\"flow-offload-ended\"}}"
                ),
            )?;

            let _cond = {
                event.get_str("event.code") == Some("430005")
                    && ["Malware", "Custom Detection"].contains(
                        &event
                            .get_str("cisco.asa.security.sha_disposition")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("430005")
                    && !(["Malware", "Custom Detection"].contains(
                        &event
                            .get_str("cisco.asa.security.sha_disposition")
                            .unwrap_or(""),
                    ))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                let v = json!(
                    event
                        .get("destination.user.name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                gsub_field(
                    event,
                    "user.name",
                    "user.name",
                    cached_regex!("^['\"]|['\"]$"),
                    "",
                )?;
            }

            let v = json!(
                event
                    .get("host.hostname")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("observer.hostname", v)?;
            }

            event.set("observer.vendor", json!("Cisco"))?;

            event.set("observer.type", json!("firewall"))?;

            event.set("observer.product", json!("asa"))?;

            let v = json!(
                event
                    .get("cisco.asa.destination_interface")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("observer.egress.interface.name", v)?;
            }

            let v = json!(
                event
                    .get("cisco.asa.source_interface")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("observer.ingress.interface.name", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.asa.pool_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cisco.asa.pool_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event.get_str("user.name") != Some("")
                    && event.get_str("user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("server.user.name")
                    && event.get_str("server.user.name") != Some("")
                    && event.get_str("server.user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("server.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event.get_str("source.user.name") != Some("")
                    && event.get_str("source.user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.user.name")
                    && event.get_str("destination.user.name") != Some("")
                    && event.get_str("destination.user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
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

            let _cond = {
                event.has_value("observer.hostname")
                    && event.get_str("observer.hostname") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.domain")
                    && event.get_str("destination.domain") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("source.domain") && event.get_str("source.domain") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.domain")
                    && event.get_str("source.user.domain") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.user.domain")
                    && event.get_str("destination.user.domain") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            let _cond = {
                event.get("cisco.asa").is_some_and(|v| v.is_object()) && event.get("cisco.asa").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("cisco.asa").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco.asa".into(),
                        });
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("cisco").is_some_and(|v| v.is_object()) && event.get("cisco").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("cisco").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco".into(),
                        });
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.ip") {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                        event
                            .get_as_string("network.iana_number")
                            .or_else(|| event.get_as_string("network.transport")),
                    ) {
                        let icmp = matches!(
                            protocol.to_ascii_lowercase().as_str(),
                            "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                        );
                        let (src_field, dst_field) = if icmp {
                            ("icmp.type", "icmp.code")
                        } else {
                            ("source.port", "destination.port")
                        };
                        let src_port =
                            u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                        let dst_port =
                            u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                        match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                            Ok(cid) => event.set("network.community_id", cid)?,
                            Err(message) => {
                                return Err(TransformError::ParseError {
                                    path: "network.community_id".into(),
                                    message,
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.remove("_temp_");
            event.remove("_conf");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                if event.has_value("_temp_.cisco") {
                    event.rename("_temp_.cisco", "cisco.asa")?;
                }
                event.remove("_temp_");
                event.remove("_conf");
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
