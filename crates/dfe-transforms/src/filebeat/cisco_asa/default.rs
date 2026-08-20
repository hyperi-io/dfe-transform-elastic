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
                    }
                }
            }

            let _cond = { event.has_value("_temp_.full_message") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: ^(?P<_temp__raw_date>(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:[A-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{DATA:_temp_.full_message}$
                    if !cached_grok_mapped!("^(?P<_temp__raw_date>(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:[A-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{DATA:_temp_.full_message}$", [("_temp__raw_date", "_temp_.raw_date"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                        // Grok pattern: %{GREEDYDATA:_temp_.full_message}
                        if !cached_grok!("%{GREEDYDATA:_temp_.full_message}").extract_into(&input, event)? {
                        }
                    }
                }
            }

            // Painless script
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
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
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx._temp_.full_message = ctx._temp_.message_prefix + ctx._temp_.extracted_message;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.full_message") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: (?:%{DATA}%(?:[A-Z]+))-(?:(?P<_temp__cisco_suffix>(?:[^0-9-]+))-)?%{NONNEGINT:event.severity:int}-%{POSINT:_temp_.cisco.message_id}?:?\\s*%{GREEDYDATA:message}
                    if !cached_grok_mapped!("(?:%{DATA}%(?:[A-Z]+))-(?:(?P<_temp__cisco_suffix>(?:[^0-9-]+))-)?%{NONNEGINT:event.severity:int}-%{POSINT:_temp_.cisco.message_id}?:?\\s*%{GREEDYDATA:message}", [("_temp__cisco_suffix", "_temp_.cisco.suffix")]).extract_into(&input, event)? {
                        // Grok pattern: %{GREEDYDATA:message}
                        if !cached_grok!("%{GREEDYDATA:message}").extract_into(&input, event)? {
                        }
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
                    && event.get_str("_temp_.tz").is_some_and(|s| !s.is_empty())
                    && event.has_value("_conf.tz_map")
            };
            if _cond {
                // Painless script
                // Source: for (def item : ctx._conf.tz_map) {\n  if (item.tz_short == ctx._temp_.tz) {\n    ctx._temp_.tz = item.tz_long;\n    break;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
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
                        if let Some(parsed) = parse_date_out(
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
                            event.set("@timestamp", parsed)?;
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
                            if let Some(parsed) = parse_date_out(
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
                                event.set("@timestamp", parsed)?;
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
                    if !cached_grok!("Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{POSINT:source.port} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}/%{POSINT:destination.port}(%{GREEDYDATA})?").extract_into(&input, event)? {
                        // Grok pattern: Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}(/%{POSINT:source.port})? (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}(/%{POSINT:destination.port})?(%{GREEDYDATA})?
                        if !cached_grok!("Deny %{NOTSPACE:network.direction} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}(/%{POSINT:source.port})? (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}(/%{POSINT:destination.port})?(%{GREEDYDATA})?").extract_into(&input, event)? {
                        }
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
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106015") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Deny %{NOTSPACE:network.transport} %{NOTSPACE} %{NOTSPACE} from %{IPORHOST:source.address}/%{POSINT:source.port} to %{IPORHOST:destination.address}/%{POSINT:destination.port} flags %{DATA} on interface %{NOTSPACE:_temp_.cisco.source_interface}
                    if !cached_grok!("Deny %{NOTSPACE:network.transport} %{NOTSPACE} %{NOTSPACE} from %{IPORHOST:source.address}/%{POSINT:source.port} to %{IPORHOST:destination.address}/%{POSINT:destination.port} flags %{DATA} on interface %{NOTSPACE:_temp_.cisco.source_interface}").extract_into(&input, event)? {
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
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("111010") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: User '%{NOTSPACE:server.user.name}', running %{QUOTEDSTRING} from IP %{IP:source.address}, executed %{QUOTEDSTRING:_temp_.cisco.command_line_arguments}
                    if !cached_grok!("User '%{NOTSPACE:server.user.name}', running %{QUOTEDSTRING} from IP %{IP:source.address}, executed %{QUOTEDSTRING:_temp_.cisco.command_line_arguments}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user %{DATA:_temp_.cisco.aaa_type} Successful(%{SPACE})?: server =(%{SPACE})?%{IPORHOST:destination.address} [:,] [Uu]ser = (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))
                    if !cached_grok_mapped!("AAA user %{DATA:_temp_.cisco.aaa_type} Successful(%{SPACE})?: server =(%{SPACE})?%{IPORHOST:destination.address} [:,] [Uu]ser = (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user (?:(authentication|authorization)) Rejected(%{SPACE})?: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|Account has been locked out|Users account has expired|User was not found)))(%{SPACE})?: server = %{IPORHOST:destination.address}(%{SPACE})?: user = ?((?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))|)(%{SPACE})?: user IP = (?:(%{IP:source.address}|None))
                    if !cached_grok_mapped!("AAA user (?:(authentication|authorization)) Rejected(%{SPACE})?: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|Account has been locked out|Users account has expired|User was not found)))(%{SPACE})?: server = %{IPORHOST:destination.address}(%{SPACE})?: user = ?((?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:%{HOSTNAME}\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@%{HOSTNAME})?(?:, *%{NUMBER})?))))|)(%{SPACE})?: user IP = (?:(%{IP:source.address}|None))", [("_temp__cisco_rejection_reason", "_temp_.cisco.rejection_reason"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113008") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA transaction status ACCEPT(%{SPACE})?: user = %{GREEDYDATA:source.user.name}
                    if !cached_grok!("AAA transaction status ACCEPT(%{SPACE})?: user = %{GREEDYDATA:source.user.name}").extract_into(&input, event)? {
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
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113015") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = %{DATA:source.user.name}%{SPACE}: [Uu]ser IP = (?:(%{IP:source.address}|None))%{SPACE}$
                    if !cached_grok_mapped!("^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = %{DATA:source.user.name}%{SPACE}: [Uu]ser IP = (?:(%{IP:source.address}|None))%{SPACE}$", [("_temp__cisco_rejection_reason", "_temp_.cisco.rejection_reason")]).extract_into(&input, event)? {
                        // Grok pattern: ^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = (?P<source_user_name>(?:[^:]+?))%{SPACE}$
                        if !cached_grok_mapped!("^AAA user authentication Rejected%{SPACE}: reason = (?P<_temp__cisco_rejection_reason>(?:(AAA failure|Account has been disabled|Invalid password|Password is expiring|Password has expired|Password malformed|Unspecified|User was not found)))%{SPACE}: local database%{SPACE}: [Uu]ser = (?P<source_user_name>(?:[^:]+?))%{SPACE}$", [("_temp__cisco_rejection_reason", "_temp_.cisco.rejection_reason"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        }
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
                    if !cached_grok!("Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}>").extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:source.user.group.name} User (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:, *%{NUMBER})?)))) IP %{IP:source.address}
                        if !cached_grok_mapped!("Group %{NOTSPACE:source.user.group.name} User (?P<source_user_name>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:, *%{NUMBER})?)))) IP %{IP:source.address}", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        }
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
                    {}
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
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("302020") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Built %{NOTSPACE:network.direction} %{NOTSPACE:network.type} connection for faddr (?:(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?gaddr (?:(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))|(?:[^:]*):(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER} laddr (?:(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?(type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?
                    if !cached_grok_mapped!("Built %{NOTSPACE:network.direction} %{NOTSPACE:network.type} connection for faddr (?:(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?gaddr (?:(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))|(?:[^:]*):(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))/%{NUMBER} laddr (?:(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\) )?(type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("destination_domain", "destination.domain"), ("destination_domain", "destination.domain"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("source_domain", "source.domain"), ("source_domain", "source.domain"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt")]).extract_into(&input, event)? {
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
                    if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("source_address", "source.address"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                        // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: protocol %{NUMBER:_temp_.cisco.original_iana_number} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                        if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: protocol %{NUMBER:_temp_.cisco.original_iana_number} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("source_address", "source.address"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                            // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.group.name}\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                            if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.group.name}\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_user_domain", "source.user.domain"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("source_address", "source.address"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                                // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                                if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))\\((?P<source_user_domain>(?:[^:]*))\\\\%{NOTSPACE:source.user.name}\\) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_user_domain", "source.user.domain"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("source_address", "source.address"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                                    // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?
                                    if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: %{NOTSPACE:input.type} src (?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})? dst (?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("source_address", "source.address"), ("destination_address", "destination.address"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                                        // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: <unknown>[.]?
                                        if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))) \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: <unknown>[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface")]).extract_into(&input, event)? {
                                        }
                                    }
                                }
                            }
                        }
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
                    if !cached_grok!("SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) disconnected by SSH server, reason: %{GREEDYDATA:event.reason}").extract_into(&input, event)? {
                        // Grok pattern: SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) terminated normally
                        if !cached_grok!("SSH session from (?:%{IP:source.ip} )?on interface %{NOTSPACE:_temp_.cisco.source_interface} for user (?:\\\"?(?:\\*{5}|%{USERNAME:source.user.name})\\\"?) terminated normally").extract_into(&input, event)? {
                        }
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                        .map_or_else(String::new, painless_to_string)
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
                    if !cached_grok!("Group = %{NOTSPACE}, Username = %{NOTSPACE:user.name}, IP = %{IP:source.address}, Security negotiation complete for User (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}").extract_into(&input, event)? {
                        // Grok pattern: Group = %{NOTSPACE}, IP = %{IP:source.address}, Security negotiation complete [a-z\\s]+ (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}
                        if !cached_grok!("Group = %{NOTSPACE}, IP = %{IP:source.address}, Security negotiation complete [a-z\\s]+ (%{DATA}) %{DATA}, Inbound SPI = %{DATA}, Outbound SPI = %{DATA}").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <%{DATA:_temp_.cisco.webvpn.group_name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> WebVPN session terminated: %{GREEDYDATA:event.reason}.
                    if !cached_grok!("Group <%{DATA:_temp_.cisco.webvpn.group_name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> WebVPN session terminated: %{GREEDYDATA:event.reason}.").extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} WebVPN session terminated: %{GREEDYDATA:event.reason}.
                        if !cached_grok!("Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} WebVPN session terminated: %{GREEDYDATA:event.reason}.").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> WebVPN access GRANTED: \"?%{DATA:url.original}\"?$
                    if !cached_grok_mapped!("^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> WebVPN access GRANTED: \"?%{DATA:url.original}\"?$", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name"), ("source_address", "source.address")]).extract_into(&input, event)? {
                        // Grok pattern: ^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} WebVPN access GRANTED: \"?%{DATA:url.original}\"?$
                        if !cached_grok!("^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} WebVPN access GRANTED: \"?%{DATA:url.original}\"?$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716058") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))>
                    if !cached_grok_mapped!("^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))>", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name"), ("source_address", "source.address")]).extract_into(&input, event)? {
                        // Grok pattern: ^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address}
                        if !cached_grok!("^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address}").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716059") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User( <(?P<source_user_name>(?:[^<>]+))>)? IP <(?P<destination_address>(?:[^<>]+))> AnyConnect session (resumed connection|resumed. Connection) from( IP)? <%{NOTSPACE:source.address}>\\.$
                    if !cached_grok_mapped!("^Group <(?P<_temp__cisco_webvpn_group_name>(?:[^<>]+))> User( <(?P<source_user_name>(?:[^<>]+))>)? IP <(?P<destination_address>(?:[^<>]+))> AnyConnect session (resumed connection|resumed. Connection) from( IP)? <%{NOTSPACE:source.address}>\\.$", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                        // Grok pattern: ^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User( %{NOTSPACE:source.user.name})? IP %{NOTSPACE:destination.address} AnyConnect session (resumed connection|resumed. Connection) from( IP)? %{NOTSPACE:source.address}\\.$
                        if !cached_grok!("^Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User( %{NOTSPACE:source.user.name})? IP %{NOTSPACE:destination.address} AnyConnect session (resumed connection|resumed. Connection) from( IP)? %{NOTSPACE:source.address}\\.$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("717022") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Certificate was successfully validated. serial number:%{SPACE}%{DATA:_temp_.cisco.serial_number}, subject name:%{SPACE}%{DATA:_temp_.cisco.distinguished_name}\\.
                    if !cached_grok!("Certificate was successfully validated. serial number:%{SPACE}%{DATA:_temp_.cisco.serial_number}, subject name:%{SPACE}%{DATA:_temp_.cisco.distinguished_name}\\.").extract_into(&input, event)? {
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
                        captured.push(("?", &remaining[..pos]));
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
                        captured.push(("?", &remaining[..pos]));
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
                    if !cached_grok_mapped!("^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> %{GREEDYDATA:event.reason}$", [("source_user_group_name", "source.user.group.name"), ("source_user_name", "source.user.name"), ("source_address", "source.address")]).extract_into(&input, event)? {
                        // Grok pattern: ^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} %{GREEDYDATA:event.reason}$
                        if !cached_grok!("^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} %{GREEDYDATA:event.reason}$").extract_into(&input, event)? {
                        }
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
                    if !cached_grok_mapped!("^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))> Client Type: %{GREEDYDATA:user_agent.original}$", [("source_user_group_name", "source.user.group.name"), ("source_user_name", "source.user.name"), ("source_address", "source.address")]).extract_into(&input, event)? {
                        // Grok pattern: ^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} Client Type: %{GREEDYDATA:user_agent.original}$
                        if !cached_grok!("^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address} Client Type: %{GREEDYDATA:user_agent.original}$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version} session
                    if !cached_grok!("^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version} session").extract_into(&input, event)? {
                        // Grok pattern: ^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} for %{NOTSPACE:_temp_.cisco.tls_version} session
                        if !cached_grok!("^Starting SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} for %{NOTSPACE:_temp_.cisco.tls_version} session").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version}
                    if !cached_grok!("^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} for %{NOTSPACE:_temp_.cisco.tls_version}").extract_into(&input, event)? {
                        // Grok pattern: ^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port}$
                        if !cached_grok!("^Device completed SSL handshake with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port}$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725007") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^SSL session with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} terminated
                    if !cached_grok!("^SSL session with %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port} terminated").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("725016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Device selects trust-point %{DATA:_temp_.cisco.trustpoint} for %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port}$
                    if !cached_grok!("^Device selects trust-point %{DATA:_temp_.cisco.trustpoint} for %{NOTSPACE:_temp_.cisco.peer_type} %{DATA:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{NOTSPACE:source.port} to %{NOTSPACE:destination.address}/%{NOTSPACE:destination.port}$").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("733100") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[(%{SPACE})?%{DATA:_temp_.cisco.burst.object}\\] drop %{NOTSPACE:_temp_.cisco.burst.id} exceeded. Current burst rate is %{INT:_temp_.cisco.burst.current_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_rate}; Current average rate is %{INT:_temp_.cisco.burst.avg_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_avg_rate}; Cumulative total count is %{INT:_temp_.cisco.burst.cumulative_count}
                    if !cached_grok!("\\[(%{SPACE})?%{DATA:_temp_.cisco.burst.object}\\] drop %{NOTSPACE:_temp_.cisco.burst.id} exceeded. Current burst rate is %{INT:_temp_.cisco.burst.current_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_rate}; Current average rate is %{INT:_temp_.cisco.burst.avg_rate} per second, max configured rate is %{INT:_temp_.cisco.burst.configured_avg_rate}; Cumulative total count is %{INT:_temp_.cisco.burst.cumulative_count}").extract_into(&input, event)? {
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
                    if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$").extract_into(&input, event)? {
                        // Grok pattern: ^IPAA: DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                        if !cached_grok!("^IPAA: DHCP configured, no viable servers found for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737006") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                    if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$").extract_into(&input, event)? {
                        // Grok pattern: ^IPAA: Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$
                        if !cached_grok!("^IPAA: Local pool request succeeded for tunnel-group %{NOTSPACE:_temp_.cisco.tunnel_group}$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$
                    if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$").extract_into(&input, event)? {
                        // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool address %{NOTSPACE:_temp_.cisco.pool_address}$
                        if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Freeing local pool address %{NOTSPACE:_temp_.cisco.pool_address}$").extract_into(&input, event)? {
                            // Grok pattern: ^IPAA: Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$
                            if !cached_grok!("^IPAA: Freeing local pool %{NOTSPACE:_temp_.cisco.pool_name} address %{NOTSPACE:_temp_.cisco.pool_address}$").extract_into(&input, event)? {
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737026") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned from local pool %{IP:_temp_.cisco.pool_address}$
                    if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned from local pool %{IP:_temp_.cisco.pool_address}$").extract_into(&input, event)? {
                        // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$
                        if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$").extract_into(&input, event)? {
                            // Grok pattern: ^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$
                            if !cached_grok!("^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool %{NOTSPACE:_temp_.cisco.pool_name}$").extract_into(&input, event)? {
                                // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$
                                if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$").extract_into(&input, event)? {
                                    // Grok pattern: ^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$
                                    if !cached_grok!("^IPAA: Client assigned %{NOTSPACE:_temp_.cisco.pool_address} from local pool$").extract_into(&input, event)? {
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("737034") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, %{GREEDYDATA:event.reason}$
                    if !cached_grok!("^IPAA: Session=%{NOTSPACE:_temp_.cisco.session_id}, %{GREEDYDATA:event.reason}$").extract_into(&input, event)? {
                        // Grok pattern: ^IPAA: %{GREEDYDATA:event.reason}$
                        if !cached_grok!("^IPAA: %{GREEDYDATA:event.reason}$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("751025") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA} Username:%{USER:source.user.name}\\s+%{GREEDYDATA}$
                    if !cached_grok!("^%{DATA} Username:%{USER:source.user.name}\\s+%{GREEDYDATA}$")
                        .extract_into(&input, event)?
                    {}
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

            if event.has("_temp_.cisco.dap_records") {
                if let Some(s) = event.get_string("_temp_.cisco.dap_records") {
                    let parts: Vec<Value> = cached_regex!(",\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
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
                    if !cached_grok!("Authentication: rejected, group = %{NOTSPACE:source.user.group.name} user = %{USER:source.user.name} , Session Type: %{NOTSPACE:_temp_.cisco.session_type}").extract_into(&input, event)? {
                        // Grok pattern: Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> Authentication: rejected, Session Type: %{NOTSPACE:_temp_.cisco.session_type}\\.
                        if !cached_grok!("Group <%{DATA:source.user.group.name}> User <%{DATA:source.user.name}> IP <%{IP:source.address}> Authentication: rejected, Session Type: %{NOTSPACE:_temp_.cisco.session_type}\\.").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("750003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Local:%{IPORHOST:source.address}:%{NUMBER:source.port} Remote:%{IPORHOST:destination.address}:%{NUMBER:destination.port} Username:%{DATA:user.name} %{GREEDYDATA:event.reason}
                    if !cached_grok!("Local:%{IPORHOST:source.address}:%{NUMBER:source.port} Remote:%{IPORHOST:destination.address}:%{NUMBER:destination.port} Username:%{DATA:user.name} %{GREEDYDATA:event.reason}").extract_into(&input, event)? {
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
                    if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_termination_initiator", "_temp_.cisco.termination_initiator"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                        // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*))
                        if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*))", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_termination_initiator", "_temp_.cisco.termination_initiator"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                            // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)
                            if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                                // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)
                                if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) \\((?P<_temp__cisco_termination_user>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))\\)", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                                    // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*))
                                    if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*))", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                                        // Grok pattern: ^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes})
                                        if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?duration (?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes})", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
                                            // Grok pattern: ^Teardown %{NOTSPACE:network.transport} connection for faddr (?:(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?gaddr (?:(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))))|(?P<_temp__cisco_gaddr_interface>(?:[^:]*)):(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))))/%{NUMBER} laddr (?:(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?(\\s*type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?
                                            if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} connection for faddr (?:(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:destination.address}|%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\(?(?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\)? )?gaddr (?:(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))))|(?P<_temp__cisco_gaddr_interface>(?:[^:]*)):(?:(?:%{IPV6:_temp_.natsrcip}|(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))))/%{NUMBER} laddr (?:(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))|(?P<_temp__cisco_source_interface>(?:[^:]*)):(?:(?:%{IPV6:source.address}|%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))))))/%{NUMBER}\\s*(?:\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))?(\\s*type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_gaddr_interface", "_temp_.cisco.gaddr_interface"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("destination_domain", "destination.domain"), ("destination_domain", "destination.domain"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__natsrcip", "_temp_.natsrcip"), ("_temp__natsrcip", "_temp_.natsrcip"), ("source_domain", "source.domain"), ("source_domain", "source.domain"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt")]).extract_into(&input, event)? {
                                                // Grok pattern: ^Teardown %{NOTSPACE:network.transport} connection %{NOTSPACE:_temp_.cisco.connection_id} from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}(?:%{NUMBER} %{NUMBER})?
                                                if !cached_grok_mapped!("^Teardown %{NOTSPACE:network.transport} connection %{NOTSPACE:_temp_.cisco.connection_id} from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}(?:%{NUMBER} %{NUMBER})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms")]).extract_into(&input, event)? {
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
                                event.set(&format!("_temp_.orig_security.{}", key), value)?;
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
                                event.set(&format!("_temp_.cisco.dn_parts.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            // Painless script
            // Source: if (ctx._temp_?.cisco?.dn_parts == null) {\n  return;\n}\ndef parts = [:];\nctx._temp_.cisco.dn_parts.forEach((k,v) -> {\n  if (params.containsKey(k)) {\n    parts[params[k]] = (v instanceof List) ? v : [v];   // `[v]` is a Painless list literal\n  } else {\n    return false;\n  }\n});\nctx._temp_.cisco.dn_parts = parts;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"if (ctx._temp_?.cisco?.dn_parts == null) {\n  return;\n}\ndef parts = [:];\nctx._temp_.cisco.dn_parts.forEach((k,v) -> {\n  if (params.containsKey(k)) {\n    parts[params[k]] = (v instanceof List) ? v : [v];   // `[v]` is a Painless list literal\n  } else {\n    return false;\n  }\n});\nctx._temp_.cisco.dn_parts = parts;\n"#
                ),
                cached_params!(
                    "{\"C\":\"country\",\"CN\":\"common_name\",\"L\":\"locality\",\"O\":\"organization\",\"OU\":\"organizational_unit\",\"P\":\"state_or_province\",\"S\":\"state_or_province\",\"ST\":\"state_or_province\"}"
                ),
            )?;

            let _cond = {
                event.has_value("_temp_.cisco.distinguished_name")
                    && event
                        .get("_temp_.cisco.distinguished_name")
                        .is_none_or(|v| match v {
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
                        .is_none_or(|v| match v {
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
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"boolean isEmpty(def value) {\n  return (value instanceof AbstractList? value.size() : value.length()) == 0;\n}\ndef appendOrCreate(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n  dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n String key = path[path.length - 1];\n def existing = dest.get(key);\n return existing == null?\n  dest.put(key, value)\n  : existing instanceof AbstractList?\n    existing.add(value)\n    : dest.put(key, new ArrayList([existing, value]));\n}\ndef msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\ndef dest = new HashMap();\nctx._temp_.cisco['security'] = dest;\nfor (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n if (param == null) {\n   continue;\n }\n param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, field.splitOnToken('.'), entry.getValue()) );\n  dest[param.target] = entry.getValue();\n }\n}\nif (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\nfor (entry in counters.entrySet()) {\n if (best == null || best.getValue() < entry.getValue()) best = entry;\n}\nif (best != null) ctx._temp_.cisco.message_id = best.getKey();\n"#
                    ),
                    cached_params!(
                        "{\"ACPolicy\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"ac_policy\"},\"AccessControlRuleAction\":{\"ecs\":[\"event.outcome\"],\"id\":[\"430002\",\"430003\"],\"target\":\"access_control_rule_action\"},\"AccessControlRuleName\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430002\",\"430003\"],\"target\":\"access_control_rule_name\"},\"AccessControlRuleReason\":{\"id\":[\"430002\",\"430003\"],\"target\":\"access_control_rule_reason\"},\"ApplicationProtocol\":{\"ecs\":[\"network.protocol\"],\"target\":\"application_protocol\"},\"ArchiveDepth\":{\"id\":[\"430004\",\"430005\"],\"target\":\"archive_depth\"},\"ArchiveFileName\":{\"ecs\":[\"file.name\"],\"id\":[\"430004\",\"430005\"],\"target\":\"archive_file_name\"},\"ArchiveFileStatus\":{\"id\":[\"430004\",\"430005\"],\"target\":\"archive_file_status\"},\"ArchiveSHA256\":{\"ecs\":[\"file.hash.sha256\"],\"id\":[\"430004\",\"430005\"],\"target\":\"archive_sha256\"},\"Classification\":{\"id\":[\"430001\"],\"target\":\"classification\"},\"Client\":{\"ecs\":[\"network.application\"],\"target\":\"client\"},\"ClientVersion\":{\"id\":[\"430002\",\"430003\"],\"target\":\"client_version\"},\"ConnectionDuration\":{\"ecs\":[\"event.duration\"],\"id\":[\"430003\"],\"target\":\"connection_duration\"},\"DNSQuery\":{\"ecs\":[\"dns.question.name\"],\"id\":[\"430002\",\"430003\"],\"target\":\"dns_query\"},\"DNSRecordType\":{\"ecs\":[\"dns.question.type\"],\"id\":[\"430002\",\"430003\"],\"target\":\"dns_record_type\"},\"DNSResponseType\":{\"ecs\":[\"dns.response_code\"],\"id\":[\"430002\",\"430003\"],\"target\":\"dns_response_type\"},\"DNSSICategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"dnssi_category\"},\"DNS_Sinkhole\":{\"id\":[\"430002\",\"430003\"],\"target\":\"dns_sinkhole\"},\"DNS_TTL\":{\"id\":[\"430002\",\"430003\"],\"target\":\"dns_ttl\"},\"DstIP\":{\"ecs\":[\"destination.address\"],\"target\":\"dst_ip\"},\"DstPort\":{\"ecs\":[\"destination.port\"],\"target\":\"dst_port\"},\"EgressInterface\":{\"ecs\":[\"_temp_.cisco.destination_interface\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"egress_interface\"},\"EgressZone\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"egress_zone\"},\"Endpoint Profile\":{\"id\":[\"430002\",\"430003\"],\"target\":\"endpoint_profile\"},\"FileAction\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_action\"},\"FileCount\":{\"id\":[\"430002\",\"430003\"],\"target\":\"file_count\"},\"FileDirection\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_direction\"},\"FileName\":{\"ecs\":[\"file.name\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_name\"},\"FilePolicy\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_policy\"},\"FileSHA256\":{\"ecs\":[\"file.hash.sha256\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_sha256\"},\"FileSandboxStatus\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_sandbox_status\"},\"FileSize\":{\"ecs\":[\"file.size\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_size\"},\"FileStorageStatus\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_storage_status\"},\"FileType\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_type\"},\"FirstPacketSecond\":{\"ecs\":[\"event.start\"],\"id\":[\"430004\",\"430005\"],\"target\":\"first_packet_second\"},\"GID\":{\"ecs\":[\"service.id\"],\"id\":[\"430001\"],\"target\":\"gid\"},\"HTTPReferer\":{\"ecs\":[\"http.request.referrer\"],\"id\":[\"430002\",\"430003\"],\"target\":\"http_referer\"},\"HTTPResponse\":{\"ecs\":[\"http.response.status_code\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"http_response\"},\"ICMPCode\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"icmp_code\"},\"ICMPType\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"icmp_type\"},\"IPReputationSICategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ip_reputation_si_category\"},\"IPSCount\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ips_count\"},\"IngressInterface\":{\"ecs\":[\"_temp_.cisco.source_interface\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"ingress_interface\"},\"IngressZone\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"ingress_zone\"},\"InitiatorBytes\":{\"ecs\":[\"source.bytes\"],\"id\":[\"430003\"],\"target\":\"initiator_bytes\"},\"InitiatorPackets\":{\"ecs\":[\"source.packets\"],\"id\":[\"430003\"],\"target\":\"initiator_packets\"},\"InlineResult\":{\"ecs\":[\"event.outcome\"],\"id\":[\"430001\"],\"target\":\"inline_result\"},\"IntrusionPolicy\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430001\"],\"target\":\"intrusion_policy\"},\"MPLS_Label\":{\"id\":[\"430001\"],\"target\":\"mpls_label\"},\"Message\":{\"ecs\":[\"message\"],\"id\":[\"430001\"],\"target\":\"message\"},\"NAPPolicy\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"nap_policy\"},\"NetBIOSDomain\":{\"ecs\":[\"host.hostname\"],\"id\":[\"430002\",\"430003\"],\"target\":\"net_bios_domain\"},\"NumIOC\":{\"id\":[\"430001\"],\"target\":\"num_ioc\"},\"Prefilter Policy\":{\"id\":[\"430002\",\"430003\"],\"target\":\"prefilter_policy\"},\"Priority\":{\"id\":[\"430001\"],\"target\":\"priority\"},\"Protocol\":{\"ecs\":[\"network.transport\"],\"target\":\"protocol\"},\"ReferencedHost\":{\"ecs\":[\"url.domain\"],\"id\":[\"430002\",\"430003\"],\"target\":\"referenced_host\"},\"ResponderBytes\":{\"ecs\":[\"destination.bytes\"],\"id\":[\"430003\"],\"target\":\"responder_bytes\"},\"ResponderPackets\":{\"ecs\":[\"destination.packets\"],\"id\":[\"430003\"],\"target\":\"responder_packets\"},\"Revision\":{\"id\":[\"430001\"],\"target\":\"revision\"},\"SHA_Disposition\":{\"id\":[\"430004\",\"430005\"],\"target\":\"sha_disposition\"},\"SID\":{\"id\":[\"430001\"],\"target\":\"sid\"},\"SSLActualAction\":{\"ecs\":[\"event.outcome\"],\"target\":\"ssl_actual_action\"},\"SSLCertificate\":{\"id\":[\"430002\",\"430003\",\"430004\",\"430005\"],\"target\":\"ssl_certificate\"},\"SSLExpectedAction\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_expected_action\"},\"SSLFlowStatus\":{\"id\":[\"430002\",\"430003\",\"430004\",\"430005\"],\"target\":\"ssl_flow_status\"},\"SSLPolicy\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_policy\"},\"SSLRuleName\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_rule_name\"},\"SSLServerCertStatus\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_server_cert_status\"},\"SSLServerName\":{\"ecs\":[\"server.domain\"],\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_server_name\"},\"SSLSessionID\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_session_id\"},\"SSLTicketID\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_ticket_id\"},\"SSLURLCategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"sslurl_category\"},\"SSLVersion\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_version\"},\"SSSLCipherSuite\":{\"id\":[\"430002\",\"430003\"],\"target\":\"sssl_cipher_suite\"},\"SecIntMatchingIP\":{\"id\":[\"430002\",\"430003\"],\"target\":\"sec_int_matching_ip\"},\"Security Group\":{\"id\":[\"430002\",\"430003\"],\"target\":\"security_group\"},\"SperoDisposition\":{\"id\":[\"430004\",\"430005\"],\"target\":\"spero_disposition\"},\"SrcIP\":{\"ecs\":[\"source.address\"],\"target\":\"src_ip\"},\"SrcPort\":{\"ecs\":[\"source.port\"],\"target\":\"src_port\"},\"TCPFlags\":{\"id\":[\"430002\",\"430003\"],\"target\":\"tcp_flags\"},\"ThreatName\":{\"ecs\":[\"_temp_.cisco.threat_category\"],\"id\":[\"430005\"],\"target\":\"threat_name\"},\"ThreatScore\":{\"ecs\":[\"_temp_.cisco.threat_level\"],\"id\":[\"430005\"],\"target\":\"threat_score\"},\"Tunnel or Prefilter Rule\":{\"id\":[\"430002\",\"430003\"],\"target\":\"tunnel_or_prefilter_rule\"},\"URI\":{\"ecs\":[\"url.original\"],\"id\":[\"430004\",\"430005\"],\"target\":\"uri\"},\"URL\":{\"ecs\":[\"url.original\"],\"id\":[\"430002\",\"430003\"],\"target\":\"url\"},\"URLCategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"url_category\"},\"URLReputation\":{\"id\":[\"430002\",\"430003\"],\"target\":\"url_reputation\"},\"URLSICategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"urlsi_category\"},\"User\":{\"ecs\":[\"user.id\",\"user.name\"],\"target\":\"user\"},\"UserAgent\":{\"ecs\":[\"user_agent.original\"],\"id\":[\"430002\",\"430003\"],\"target\":\"user_agent\"},\"VLAN_ID\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"vlan_id\"},\"WebApplication\":{\"ecs\":[\"network.application\"],\"target\":\"web_application\"},\"originalClientSrcIP\":{\"ecs\":[\"client.address\"],\"id\":[\"430002\",\"430003\"],\"target\":\"original_client_src_ip\"}}"
                    ),
                )?;
            }

            // Painless script
            // Source: def getField(Map src, String[] path) {\n for (int i=0; i<path.length-1; i++) {\n  src = src.getOrDefault(path[i], null);\n  if (src == null || !(src instanceof Map)) {\n    return null;\n  }\n }\n return src[path[path.length-1]];\n}\ndef setField(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n   dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n dest[path[path.length-1]] = value;\n}\nfor (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  def param = entry.getValue();\n  String oldVal = getField(ctx, srcField.splitOnToken('.'));\n  if (oldVal == null) continue;\n  def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"def getField(Map src, String[] path) {\n for (int i=0; i<path.length-1; i++) {\n  src = src.getOrDefault(path[i], null);\n  if (src == null || !(src instanceof Map)) {\n    return null;\n  }\n }\n return src[path[path.length-1]];\n}\ndef setField(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n   dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n dest[path[path.length-1]] = value;\n}\nfor (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  def param = entry.getValue();\n  String oldVal = getField(ctx, srcField.splitOnToken('.'));\n  if (oldVal == null) continue;\n  def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"ctx._temp_.cisco.message_id\":{\"map\":{\"430001\":\"intrusion-detected\",\"430002\":\"connection-started\",\"430003\":\"connection-finished\",\"430004\":\"file-detected\",\"430005\":\"malware-detected\"},\"target\":\"event.action\"},\"dns.question.type\":{\"map\":{\"a domain name pointer\":\"PTR\",\"a host address\":\"A\",\"an authoritative name server\":\"NS\",\"ip6 address\":\"AAAA\",\"mail exchange\":\"MX\",\"marks the start of a zone of authority\":\"SOA\",\"server selection\":\"SRV\",\"text strings\":\"TXT\",\"the canonical name for an alias\":\"CNAME\"}},\"dns.response_code\":{\"map\":{\"no error\":\"NOERROR\",\"non-existent domain\":\"NXDOMAIN\",\"query refused\":\"REFUSED\",\"server failure\":\"SERVFAIL\"}}}"
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
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("_temp_.duration_hms", v)?;
            }

            let _cond = { event.has_value("_temp_.duration_hms") };
            if _cond {
                // Painless script
                // Source: long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        }\n    }\n    return total + cur;\n}\nif (ctx?.event == null) {\n    ctx['event'] = new HashMap();\n}\nlong nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\nctx.event['duration'] = nanos;\nif (ctx['@timestamp'] != null) {\n    String end = ctx['@timestamp'];\n    ctx.event['end'] = end;\n    try {\n        ctx.event['start'] = ZonedDateTime.ofInstant(\n            Instant.parse(end).minusNanos(nanos),\n            ZoneOffset.UTC);\n    } catch (Exception e) {\n        // If timestamp parsing fails, just set duration\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        }\n    }\n    return total + cur;\n}\nif (ctx?.event == null) {\n    ctx['event'] = new HashMap();\n}\nlong nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\nctx.event['duration'] = nanos;\nif (ctx['@timestamp'] != null) {\n    String end = ctx['@timestamp'];\n    ctx.event['end'] = end;\n    try {\n        ctx.event['start'] = ZonedDateTime.ofInstant(\n            Instant.parse(end).minusNanos(nanos),\n            ZoneOffset.UTC);\n    } catch (Exception e) {\n        // If timestamp parsing fails, just set duration\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.source_user_or_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.source_user_or_sgt") {
                    // Grok pattern: (?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))))
                    if !cached_grok_mapped!("(?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.source_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.source_user_security_group_tag_name}))))", [("_temp__cisco_source_username", "_temp_.cisco.source_username")]).extract_into(&input, event)? {
                        // Grok pattern: (?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?
                        if !cached_grok_mapped!("(?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?", [("_temp__cisco_source_username", "_temp_.cisco.source_username")]).extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.has_value("_temp_.cisco.source_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.source_sgt") {
                    // Grok pattern: (?:(, *)?(%{NUMBER:_temp_.cisco.source_user_security_group_tag})?:?%{WORD:_temp_.cisco.source_user_security_group_tag_name}?)
                    if !cached_grok!("(?:(, *)?(%{NUMBER:_temp_.cisco.source_user_security_group_tag})?:?%{WORD:_temp_.cisco.source_user_security_group_tag_name}?)").extract_into(&input, event)? {
                    }
                }
            }

            if event.has("_temp_.cisco.source_user_security_group_tag") {
                if let Some(val) = event.get("_temp_.cisco.source_user_security_group_tag") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.source_user_security_group_tag".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.source_user_security_group_tag".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.source_user_security_group_tag".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.source_user_security_group_tag", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.destination_user_or_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.destination_user_or_sgt") {
                    // Grok pattern: (?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))))
                    if !cached_grok_mapped!("(?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?(?:(, *((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))|(?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag}))))|((?:(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})):(?:(%{WORD:_temp_.cisco.destination_user_security_group_tag_name}))))", [("_temp__cisco_destination_username", "_temp_.cisco.destination_username")]).extract_into(&input, event)? {
                        // Grok pattern: (?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?
                        if !cached_grok_mapped!("(?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:[^,$)]*)))\\$?\\)?", [("_temp__cisco_destination_username", "_temp_.cisco.destination_username")]).extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.has_value("_temp_.cisco.destination_sgt") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.destination_sgt") {
                    // Grok pattern: (?:(, *)?(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})?:?%{WORD:_temp_.cisco.destination_user_security_group_tag_name}?)
                    if !cached_grok!("(?:(, *)?(%{NUMBER:_temp_.cisco.destination_user_security_group_tag})?:?%{WORD:_temp_.cisco.destination_user_security_group_tag_name}?)").extract_into(&input, event)? {
                    }
                }
            }

            if event.has("_temp_.cisco.destination_user_security_group_tag") {
                if let Some(val) = event.get("_temp_.cisco.destination_user_security_group_tag") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.destination_user_security_group_tag"
                                            .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.destination_user_security_group_tag".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.destination_user_security_group_tag".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
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
                    && event
                        .get_str("_temp_.cisco.source_username")
                        .is_none_or(|s| s.is_empty())
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
                    && event
                        .get_str("_temp_.cisco.destination_username")
                        .is_none_or(|s| s.is_empty())
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                if let Some(input) = event.get_string("source.user.name") {
                    // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?P<source_user_email>(?:(?:(?P<source_user_name>(?:[^@$]+)))@%{HOSTNAME:source.user.domain}))
                    if !cached_grok_mapped!("((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?P<source_user_email>(?:(?:(?P<source_user_name>(?:[^@$]+)))@%{HOSTNAME:source.user.domain}))", [("source_user_email", "source.user.email"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?:(?P<source_user_name>(?:[^@$]+)))
                        if !cached_grok_mapped!("((?:(LOCAL\\\\)?(%{HOSTNAME:source.user.domain}\\\\)?))?(?:(?P<source_user_name>(?:[^@$]+)))", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                            // Grok pattern: \\*+
                            if !cached_grok!("\\*+").extract_into(&input, event)? {
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                if let Some(input) = event.get_string("destination.user.name") {
                    // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?P<destination_user_email>(?:(?:(?P<destination_user_name>(?:[^@$]+)))@%{HOSTNAME:destination.user.domain}))
                    if !cached_grok_mapped!("((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?P<destination_user_email>(?:(?:(?P<destination_user_name>(?:[^@$]+)))@%{HOSTNAME:destination.user.domain}))", [("destination_user_email", "destination.user.email"), ("destination_user_name", "destination.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?:(?P<destination_user_name>(?:[^@$]+)))
                        if !cached_grok_mapped!("((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?:(?P<destination_user_name>(?:[^@$]+)))", [("destination_user_name", "destination.user.name")]).extract_into(&input, event)? {
                        }
                    }
                }
            }

            if event.has("network.transport") {
                if let Some(s) = event.get_string("network.transport") {
                    let lowered = s.to_lowercase();
                    event.set("network.transport", lowered)?;
                }
            }

            if event.has("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            if event.has("network.application") {
                if let Some(s) = event.get_string("network.application") {
                    let lowered = s.to_lowercase();
                    event.set("network.application", lowered)?;
                }
            }

            if event.has("file.type") {
                if let Some(s) = event.get_string("file.type") {
                    let lowered = s.to_lowercase();
                    event.set("file.type", lowered)?;
                }
            }

            if event.has("network.direction") {
                if let Some(s) = event.get_string("network.direction") {
                    let lowered = s.to_lowercase();
                    event.set("network.direction", lowered)?;
                }
            }

            if event.has("network.type") {
                if let Some(s) = event.get_string("network.type") {
                    let lowered = s.to_lowercase();
                    event.set("network.type", lowered)?;
                }
            }

            let _cond = { event.has_value("network.transport") };
            if _cond {
                // Painless script
                // Source: def net = ctx.network; def iana = params[net.transport]; if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} def reverse = new HashMap(); def[] arr = new def[] { null }; for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  reverse.put(String.format(\"%d\", arr), entry.getKey());\n} def trans = reverse[net.transport]; if (trans != null) {\n  net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"def net = ctx.network; def iana = params[net.transport]; if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} def reverse = new HashMap(); def[] arr = new def[] { null }; for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  reverse.put(String.format(\"%d\", arr), entry.getKey());\n} def trans = reverse[net.transport]; if (trans != null) {\n  net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n"#
                    ),
                    cached_params!(
                        "{\"dccp\":33,\"egp\":8,\"esp\":50,\"gre\":47,\"icmp\":1,\"idpr\":35,\"igmp\":2,\"igp\":9,\"ipv4\":4,\"ipv6\":41,\"ipv6-frag\":44,\"ipv6-icmp\":58,\"ipv6-nonxt\":59,\"ipv6-opts\":60,\"ipv6-route\":43,\"irtp\":28,\"pup\":12,\"rdp\":27,\"rsvp\":46,\"tcp\":6,\"udp\":17}"
                    ),
                )?;
            }

            let _cond = { event.get_str("network.transport") == Some("icmpv6") };
            if _cond {
                event.set("network.transport", json!("ipv6-icmp"))?;
            }

            if event.has("_temp_.outcome") {
                if let Some(s) = event.get_string("_temp_.outcome") {
                    let lowered = s.to_lowercase();
                    event.set("_temp_.outcome", lowered)?;
                }
            }

            if event.has("event.outcome") {
                if let Some(s) = event.get_string("event.outcome") {
                    let lowered = s.to_lowercase();
                    event.set("event.outcome", lowered)?;
                }
            }

            if event.has("source.port") {
                if let Some(val) = event.get("source.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "source.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "source.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "source.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
            }

            if event.has("destination.port") {
                if let Some(val) = event.get("destination.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "destination.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "destination.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "destination.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("destination.port", converted)?;
                }
            }

            if event.has("source.bytes") {
                if let Some(val) = event.get("source.bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "source.bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "source.bytes".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "source.bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.bytes", converted)?;
                }
            }

            if event.has("destination.bytes") {
                if let Some(val) = event.get("destination.bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "destination.bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "destination.bytes".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "destination.bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("destination.bytes", converted)?;
                }
            }

            if event.has("network.bytes") {
                if let Some(val) = event.get("network.bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "network.bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "network.bytes".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "network.bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("network.bytes", converted)?;
                }
            }

            if event.has("source.packets") {
                if let Some(val) = event.get("source.packets") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "source.packets".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "source.packets".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "source.packets".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.packets", converted)?;
                }
            }

            if event.has("destination.packets") {
                if let Some(val) = event.get("destination.packets") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "destination.packets".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "destination.packets".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "destination.packets".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("destination.packets", converted)?;
                }
            }

            if event.has("_temp_.cisco.mapped_source_port") {
                if let Some(val) = event.get("_temp_.cisco.mapped_source_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.mapped_source_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.mapped_source_port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.mapped_source_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.mapped_source_port", converted)?;
                }
            }

            if event.has("_temp_.cisco.mapped_destination_port") {
                if let Some(val) = event.get("_temp_.cisco.mapped_destination_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.mapped_destination_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.mapped_destination_port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.mapped_destination_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.mapped_destination_port", converted)?;
                }
            }

            if event.has("_temp_.cisco.icmp_code") {
                if let Some(val) = event.get("_temp_.cisco.icmp_code") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.icmp_code".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.icmp_code".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.icmp_code".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.icmp_code", converted)?;
                }
            }

            if event.has("_temp_.cisco.icmp_type") {
                if let Some(val) = event.get("_temp_.cisco.icmp_type") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.icmp_type".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.icmp_type".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.icmp_type".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.icmp_type", converted)?;
                }
            }

            if event.has("_temp_.cisco.original_iana_number") {
                if let Some(val) = event.get("_temp_.cisco.original_iana_number") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.original_iana_number".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.original_iana_number".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.original_iana_number".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.original_iana_number", converted)?;
                }
            }

            if event.has("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "http.response.status_code".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "http.response.status_code".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "http.response.status_code".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("http.response.status_code", converted)?;
                }
            }

            if event.has("file.size") {
                if let Some(val) = event.get("file.size") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "file.size".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "file.size".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "file.size".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("file.size", converted)?;
                }
            }

            if event.has("network.iana_number") {
                if let Some(val) = event.get("network.iana_number") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("network.iana_number", converted)?;
                }
            }

            if event.has("sip.to.uri.port") {
                if let Some(val) = event.get("sip.to.uri.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "sip.to.uri.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "sip.to.uri.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "sip.to.uri.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("sip.to.uri.port", converted)?;
                }
            }

            if event.has("_temp_.cisco.connections_in_use") {
                if let Some(val) = event.get("_temp_.cisco.connections_in_use") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.connections_in_use".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.connections_in_use".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.connections_in_use".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.connections_in_use", converted)?;
                }
            }

            if event.has("_temp_.cisco.connections_most_used") {
                if let Some(val) = event.get("_temp_.cisco.connections_most_used") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.connections_most_used".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.connections_most_used".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.cisco.connections_most_used".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.connections_most_used", converted)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(?:%{IP:source.ip}|%{GREEDYDATA:source.domain})$
                    if !cached_grok!("^(?:%{IP:source.ip}|%{GREEDYDATA:source.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: ^(?:%{IP:destination.ip}|%{GREEDYDATA:destination.domain})$
                    if !cached_grok!("^(?:%{IP:destination.ip}|%{GREEDYDATA:destination.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = { event.has_value("client.address") };
            if _cond {
                if let Some(input) = event.get_string("client.address") {
                    // Grok pattern: ^(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})$
                    if !cached_grok!("^(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = { event.has_value("server.address") };
            if _cond {
                if let Some(input) = event.get_string("server.address") {
                    // Grok pattern: ^(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})$
                    if !cached_grok!("^(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            if event.has("source.ip") {
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

            if event.has("destination.ip") {
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

            if event.has("source.ip") {
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

            if event.has("destination.ip") {
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has("destination.as.organization_name") {
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
                    }
                }
            }

            let _cond = { event.has_value("_temp_.natdstip") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.natdstip") {
                    // Grok pattern: ^(?:%{IP:_temp_.cisco.mapped_destination_ip}|%{GREEDYDATA:_temp_.cisco.mapped_destination_host})$
                    if !cached_grok!("^(?:%{IP:_temp_.cisco.mapped_destination_ip}|%{GREEDYDATA:_temp_.cisco.mapped_destination_host})$").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = {
                event
                    .get("_temp_.cisco.mapped_source_ip")
                    .filter(|v| !v.is_null())
                    != event.get("source.ip").filter(|v| !v.is_null())
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_source_ip")
                        .map_or_else(String::new, painless_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.nat.ip", v)?;
                }
            }

            if event.has("source.nat.ip") {
                if let Some(s) = event.get_string("source.nat.ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "source.nat.ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("source.nat.ip", s)?;
                }
            }

            let _cond = {
                event
                    .get("_temp_.cisco.mapped_source_port")
                    .filter(|v| !v.is_null())
                    != event.get("source.port").filter(|v| !v.is_null())
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_source_port")
                        .map_or_else(String::new, painless_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.nat.port", v)?;
                }
            }

            if event.has("source.nat.port") {
                if let Some(val) = event.get("source.nat.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "source.nat.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "source.nat.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "source.nat.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.nat.port", converted)?;
                }
            }

            let _cond = {
                event
                    .get("_temp_.cisco.mapped_destination_ip")
                    .filter(|v| !v.is_null())
                    != event.get("destination.ip").filter(|v| !v.is_null())
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_destination_ip")
                        .map_or_else(String::new, painless_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.nat.ip", v)?;
                }
            }

            if event.has("destination.nat.ip") {
                if let Some(s) = event.get_string("destination.nat.ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "destination.nat.ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("destination.nat.ip", s)?;
                }
            }

            let _cond = {
                event
                    .get("_temp_.cisco.mapped_destination_port")
                    .filter(|v| !v.is_null())
                    != event.get("destination.port").filter(|v| !v.is_null())
            };
            if _cond {
                let v = json!(
                    event
                        .get("_temp_.cisco.mapped_destination_port")
                        .map_or_else(String::new, painless_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.nat.port", v)?;
                }
            }

            if event.has("destination.nat.port") {
                if let Some(val) = event.get("destination.nat.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "destination.nat.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "destination.nat.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "destination.nat.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.tls_version") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.cisco.tls_version") {
                    // Grok pattern: (?P<tls_version_protocol>(?:[A-Z]+))v%{NUMBER:tls.version}
                    if !cached_grok_mapped!(
                        "(?P<tls_version_protocol>(?:[A-Z]+))v%{NUMBER:tls.version}",
                        [("tls_version_protocol", "tls.version_protocol")]
                    )
                    .extract_into(&input, event)?
                    {
                        // Grok pattern: (?P<tls_version_protocol>(?:[A-Z]+))
                        if !cached_grok_mapped!(
                            "(?P<tls_version_protocol>(?:[A-Z]+))",
                            [("tls_version_protocol", "tls.version_protocol")]
                        )
                        .extract_into(&input, event)?
                        {}
                    }
                }
            }

            if event.has("tls.version_protocol") {
                if let Some(s) = event.get_string("tls.version_protocol") {
                    let lowered = s.to_lowercase();
                    event.set("tls.version_protocol", lowered)?;
                }
            }

            event.remove("_temp_.cisco.tls_version");

            let _cond = {
                event.has_value("_temp_.cisco.message_id")
                    && event
                        .get_str("_temp_.cisco.message_id")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.rename("_temp_.cisco.message_id", "event.code")?;
            }

            let _cond = { event.has_value("_temp_.cisco") };
            if _cond {
                event.rename("_temp_.cisco", "cisco.asa")?;
            }

            if event.has("cisco.asa.list_id") {
                event.rename("cisco.asa.list_id", "cisco.asa.rule_name")?;
            }

            // Painless script
            // Source: params.get(ctx.event.code)?.get(ctx._temp_.outcome)?.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"params.get(ctx.event.code)?.get(ctx._temp_.outcome)?.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"106100\":{\"denied\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"est-allowed\":{\"action\":\"firewall-rule\",\"outcome\":\"success\",\"type\":[\"connection\",\"allowed\"]},\"permitted\":{\"action\":\"firewall-rule\",\"outcome\":\"success\",\"type\":[\"connection\",\"allowed\"]}},\"106102\":{\"denied\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"permitted\":{\"action\":\"firewall-rule\",\"outcome\":\"success\",\"type\":[\"connection\",\"allowed\"]}},\"111004\":{\"failed\":{\"action\":\"configuration\",\"category\":[\"configuration\"],\"outcome\":\"failure\",\"type\":[\"info\"]},\"ok\":{\"action\":\"configuration\",\"category\":[\"configuration\"],\"outcome\":\"success\",\"type\":[\"change\"]}}}"
                ),
            )?;

            // Painless script
            // Source: params.get(ctx.event.code)?.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"params.get(ctx.event.code)?.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"106001\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106002\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106006\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106007\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106010\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106012\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106013\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106014\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106015\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106016\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106017\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106018\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106020\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106021\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106022\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106023\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106027\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"106103\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"110002\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"info\"]},\"111007\":{\"action\":\"configuration\",\"category\":[\"configuration\"],\"outcome\":\"success\",\"type\":[\"info\"]},\"111009\":{\"action\":\"configuration\",\"category\":[\"configuration\"],\"outcome\":\"success\",\"type\":[\"info\"]},\"111010\":{\"action\":\"configuration\",\"category\":[\"configuration\"],\"outcome\":\"success\",\"type\":[\"change\"]},\"113004\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"113005\":{\"action\":\"logon-failed\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"failure\",\"type\":[\"denied\",\"info\"]},\"113008\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"113009\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"113011\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"113012\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"113015\":{\"action\":\"logon-failed\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"failure\",\"type\":[\"denied\",\"info\"]},\"113019\":{\"action\":\"client-vpn-disconnected\",\"type\":[\"connection\",\"end\"]},\"113021\":{\"action\":\"logon-failed\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"failure\",\"type\":[\"denied\",\"info\"]},\"113022\":{\"action\":\"server-failed\",\"outcome\":\"failure\",\"type\":[\"info\"]},\"113023\":{\"action\":\"server-active\",\"outcome\":\"success\",\"type\":[\"info\"]},\"113029\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113030\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113031\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113032\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113033\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113034\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113035\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113036\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113037\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113038\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"113039\":{\"action\":\"client-vpn-connected\",\"category\":[\"network\",\"session\"],\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"113040\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"302013\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"302014\":{\"action\":\"flow-expiration\",\"type\":[\"connection\",\"end\"]},\"302015\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"302016\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"302018\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"302020\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"302021\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"302022\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"302023\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"302024\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"302025\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"302026\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"302027\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"302036\":{\"action\":\"flow-expiration\",\"type\":[\"connection\",\"end\"]},\"302304\":{\"action\":\"flow-expiration\",\"type\":[\"connection\",\"end\"]},\"302306\":{\"action\":\"flow-expiration\",\"type\":[\"connection\",\"end\"]},\"303002\":{\"action\":\"ftp\",\"category\":[\"network\",\"file\"],\"outcome\":\"success\",\"type\":[\"access\"]},\"304001\":{\"action\":\"url-access\",\"outcome\":\"success\",\"type\":[\"access\",\"allowed\"]},\"304002\":{\"action\":\"url-access\",\"outcome\":\"failure\",\"type\":[\"access\",\"denied\"]},\"305011\":{\"action\":\"nat-slot\",\"category\":[\"network\",\"configuration\"],\"outcome\":\"success\",\"type\":[\"creation\"]},\"305012\":{\"action\":\"nat-slot\",\"category\":[\"network\",\"configuration\"],\"outcome\":\"success\",\"type\":[\"deletion\"]},\"313001\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"313004\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"313005\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"313008\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"313009\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"315011\":{\"action\":\"ssh-session-ended\",\"type\":[\"connection\",\"end\"]},\"322001\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338001\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338002\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338003\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338004\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338005\":{\"action\":\"dynamic-filter\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338006\":{\"action\":\"dynamic-filter\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338007\":{\"action\":\"dynamic-filter\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338008\":{\"action\":\"dynamic-filter\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338101\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338102\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338103\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338104\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338201\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338202\":{\"action\":\"dynamic-filter\",\"type\":[\"connection\",\"info\"]},\"338203\":{\"action\":\"dynamic-filter\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338204\":{\"action\":\"dynamic-filter\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"338301\":{\"action\":\"firewall-rule\",\"type\":[\"connection\",\"info\"]},\"419002\":{\"action\":\"firewall-rule\",\"type\":[\"connection\",\"info\"]},\"425005\":{\"action\":\"interface-switchover\",\"type\":[\"info\"]},\"434002\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"434004\":{\"action\":\"bypass\",\"type\":[\"info\"]},\"502103\":{\"action\":\"privilege-level-changed\",\"category\":[\"iam\"],\"outcome\":\"success\",\"type\":[\"user\",\"change\"]},\"507003\":{\"action\":\"flow-termination\",\"type\":[\"connection\",\"end\"]},\"602303\":{\"action\":\"sa-created\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"602304\":{\"action\":\"sa-deleted\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"605004\":{\"action\":\"logon-failed\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"failure\",\"type\":[\"denied\",\"info\"]},\"605005\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"607001\":{\"action\":\"firewall-rule\",\"outcome\":\"success\",\"type\":[\"info\"]},\"609001\":{\"action\":\"flow-creation\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"609002\":{\"action\":\"flow-expiration\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"611101\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"611102\":{\"action\":\"logon-failed\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"failure\",\"type\":[\"denied\",\"info\"]},\"611103\":{\"action\":\"logged-out\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"info\"]},\"710003\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"710005\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"710006\":{\"action\":\"firewall-rule\",\"outcome\":\"failure\",\"type\":[\"connection\",\"denied\"]},\"713049\":{\"action\":\"firewall-rule\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"713120\":{\"action\":\"firewall-rule\",\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"713202\":{\"action\":\"firewall-rule\",\"type\":[\"connection\",\"info\"]},\"713901\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"info\"]},\"713902\":{\"action\":\"client-vpn-error\",\"outcome\":\"failure\",\"type\":[\"info\"]},\"713903\":{\"type\":[\"info\"]},\"713904\":{\"type\":[\"info\"]},\"713905\":{\"type\":[\"info\"]},\"713906\":{\"type\":[\"info\"]},\"716002\":{\"action\":\"client-vpn-disconnected\",\"type\":[\"connection\",\"end\"]},\"716003\":{\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"716039\":{\"action\":\"logon-failed\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"failure\",\"type\":[\"denied\",\"info\"]},\"716058\":{\"action\":\"client-vpn-disconnected\",\"type\":[\"connection\",\"end\"]},\"716059\":{\"action\":\"client-vpn-resumed\",\"type\":[\"connection\",\"start\"]},\"721016\":{\"action\":\"client-vpn-connected\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"721018\":{\"action\":\"client-vpn-disconnected\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"722011\":{\"type\":[\"info\"]},\"722022\":{\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"722023\":{\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"722028\":{\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]},\"722032\":{\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"722033\":{\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"722034\":{\"type\":[\"connection\",\"info\"]},\"722035\":{\"outcome\":\"failure\",\"type\":[\"connection\",\"info\"]},\"722037\":{\"type\":[\"connection\",\"end\"]},\"722041\":{\"outcome\":\"failure\",\"type\":[\"connection\",\"info\"]},\"722051\":{\"action\":\"address-assigned\",\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"722055\":{\"type\":[\"connection\",\"info\"]},\"725001\":{\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"725002\":{\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"725007\":{\"outcome\":\"failure\",\"type\":[\"connection\",\"end\"]},\"725016\":{\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"733100\":{\"type\":[\"info\"]},\"734001\":{\"action\":\"logged-in\",\"category\":[\"authentication\",\"network\"],\"outcome\":\"success\",\"type\":[\"allowed\",\"info\"]},\"737003\":{\"outcome\":\"failure\",\"type\":[\"connection\",\"info\"]},\"737006\":{\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"737016\":{\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"737026\":{\"outcome\":\"success\",\"type\":[\"connection\",\"info\"]},\"737034\":{\"outcome\":\"failure\",\"type\":[\"connection\",\"info\"]},\"750002\":{\"action\":\"connection-started\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"750003\":{\"outcome\":\"failure\",\"type\":[\"connection\",\"start\"]},\"805001\":{\"action\":\"flow-offload-started\",\"outcome\":\"success\",\"type\":[\"connection\",\"start\"]},\"805002\":{\"action\":\"flow-offload-ended\",\"outcome\":\"success\",\"type\":[\"connection\",\"end\"]}}"
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
                        .map_or_else(String::new, painless_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                if let Some(s) = event.get_string("user.name") {
                    let re = cached_regex!("^['\"]|['\"]$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("user.name", replaced)?;
                }
            }

            let v = json!(
                event
                    .get("host.hostname")
                    .map_or_else(String::new, painless_to_string)
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
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("observer.egress.interface.name", v)?;
            }

            let v = json!(
                event
                    .get("cisco.asa.source_interface")
                    .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event.get_str("user.name").is_some_and(|s| !s.is_empty())
                    && event.get_str("user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("server.user.name")
                    && event
                        .get_str("server.user.name")
                        .is_some_and(|s| !s.is_empty())
                    && event.get_str("server.user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("server.user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .is_some_and(|s| !s.is_empty())
                    && event.get_str("source.user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.user.name")
                    && event
                        .get_str("destination.user.name")
                        .is_some_and(|s| !s.is_empty())
                    && event.get_str("destination.user.name") != Some("*****")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("host.hostname")
                    && event
                        .get_str("host.hostname")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("observer.hostname")
                    && event
                        .get_str("observer.hostname")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.domain")
                    && event
                        .get_str("destination.domain")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.domain")
                    && event
                        .get_str("source.domain")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.domain")
                    && event
                        .get_str("source.user.domain")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.user.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.user.domain")
                    && event
                        .get_str("destination.user.domain")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.user.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            // SKIPPED: condition not transpiled: ctx.cisco?.asa instanceof Map && ctx.cisco.asa.size() == 0
            #[allow(unreachable_code, unused_variables)]
            if false {
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

            // SKIPPED: condition not transpiled: ctx.cisco instanceof Map && ctx.cisco.size() == 0
            #[allow(unreachable_code, unused_variables)]
            if false {
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
                if event.has("source.ip") {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                        event
                            .get_string("network.transport")
                            .or_else(|| event.get_string("network.iana_number")),
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
                if event.has("_temp_.cisco") {
                    event.rename("_temp_.cisco", "cisco.asa")?;
                }
                event.remove("_temp_");
                event.remove("_conf");
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
