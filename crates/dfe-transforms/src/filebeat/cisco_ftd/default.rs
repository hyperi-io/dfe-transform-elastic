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
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:(?:(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)\\s*)?(?:(?P<_temp__raw_date>(?:(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?(?P<_temp__tz>(?:(?:Z|[+-]%{HOUR}(?::?%{MINUTE}))))?)|(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:(?:[APMCE][SD]T|UTC))))?)))):?\\s+)?(?:(?:(?:(?P<process_name>(?:(?:[^%\\s:\\[]+))):\\s%{SYSLOGHOST:host.name}))|(?:(?:%{SYSLOGHOST:host.hostname}:?\\s+)?(?:(?P<process_name>(?:(?:[^%\\s:\\[]+)))?(?:\\[%{POSINT:process.pid:long}\\])?)?))(?:{DATA})?(?:(?:(:|\\s)\\s+))?))?\\s*%{GREEDYDATA:_temp_.full_message}
                if !cached_grok_mapped!("(?:(?:(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)\\s*)?(?:(?P<_temp__raw_date>(?:(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?(?P<_temp__tz>(?:(?:Z|[+-]%{HOUR}(?::?%{MINUTE}))))?)|(?:(?:%{DAY} )?%{MONTH}  *%{MONTHDAY}(?: %{YEAR})? %{TIME}(?: (?P<_temp__tz>(?:(?:[APMCE][SD]T|UTC))))?)))):?\\s+)?(?:(?:(?:(?P<process_name>(?:(?:[^%\\s:\\[]+))):\\s%{SYSLOGHOST:host.name}))|(?:(?:%{SYSLOGHOST:host.hostname}:?\\s+)?(?:(?P<process_name>(?:(?:[^%\\s:\\[]+)))?(?:\\[%{POSINT:process.pid:long}\\])?)?))(?:{DATA})?(?:(?:(:|\\s)\\s+))?))?\\s*%{GREEDYDATA:_temp_.full_message}", [("_temp__raw_date", "_temp_.raw_date"), ("process_name", "process.name"), ("process_name", "process.name"), ("_temp__tz", "_temp_.tz"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
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

            if let Some(input) = event.get_string("_temp_.full_message") {
                // Grok pattern: (?:%{DATA}%(?:[A-Z]+))-(?:(?P<_temp__cisco_suffix>(?:[^0-9-]+))?-)?%{NONNEGINT:event.severity:int}-%{POSINT:_temp_.cisco.message_id}?:?\\s*%{GREEDYDATA:message}
                if !cached_grok_mapped!("(?:%{DATA}%(?:[A-Z]+))-(?:(?P<_temp__cisco_suffix>(?:[^0-9-]+))?-)?%{NONNEGINT:event.severity:int}-%{POSINT:_temp_.cisco.message_id}?:?\\s*%{GREEDYDATA:message}", [("_temp__cisco_suffix", "_temp_.cisco.suffix")]).extract_into(&input, event)? {
                        // Grok pattern: %{GREEDYDATA:message}
                        if !cached_grok!("%{GREEDYDATA:message}").extract_into(&input, event)? {
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
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "EEE MMM  d HH:mm:ss",
                                "EEE MMM dd HH:mm:ss",
                                "MMM  d HH:mm:ss z",
                                "MMM dd HH:mm:ss z",
                                "EEE MMM  d HH:mm:ss z",
                                "EEE MMM dd HH:mm:ss z",
                                "MMM  d yyyy HH:mm:ss",
                                "MMM dd yyyy HH:mm:ss",
                                "EEE MMM  d yyyy HH:mm:ss",
                                "EEE MMM dd yyyy HH:mm:ss",
                                "MMM  d yyyy HH:mm:ss z",
                                "MMM dd yyyy HH:mm:ss z",
                                "EEE MMM  d yyyy HH:mm:ss z",
                                "EEE MMM dd yyyy HH:mm:ss z",
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__raw_date_a2ad7d0e",
                    )?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("_temp_.raw_date") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &[
                                        "ISO8601",
                                        "MMM  d HH:mm:ss",
                                        "MMM dd HH:mm:ss",
                                        "EEE MMM  d HH:mm:ss",
                                        "EEE MMM dd HH:mm:ss",
                                        "MMM  d HH:mm:ss z",
                                        "MMM dd HH:mm:ss z",
                                        "EEE MMM  d HH:mm:ss z",
                                        "EEE MMM dd HH:mm:ss z",
                                        "MMM  d yyyy HH:mm:ss",
                                        "MMM dd yyyy HH:mm:ss",
                                        "EEE MMM  d yyyy HH:mm:ss",
                                        "EEE MMM dd yyyy HH:mm:ss",
                                        "MMM  d yyyy HH:mm:ss z",
                                        "MMM dd yyyy HH:mm:ss z",
                                        "EEE MMM  d yyyy HH:mm:ss z",
                                        "EEE MMM dd yyyy HH:mm:ss z",
                                    ],
                                    None,
                                    None,
                                ) {
                                    event.set("@timestamp", parsed)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date__temp__raw_date_4577a3bd",
                            )?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.get_str("_temp_.cisco.message_id") != Some("") };
            if _cond {
                event.set("event.action", json!("firewall-rule"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") != Some("") };
            if _cond {
                // Painless script
                // Source: def messageID = Long.parseLong(ctx._temp_.cisco.message_id);\nif (messageID >= 101001 && messageID <= 105052) {\n ctx.event[\"reason\"] = ctx.message;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def messageID = Long.parseLong(ctx._temp_.cisco.message_id);\nif (messageID >= 101001 && messageID <= 105052) {\n ctx.event[\"reason\"] = ctx.message;\n}"#
                    ),
                )?;
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
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" connection ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connection ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        let Some(pos) = remaining.find(" Connection ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Connection ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" by ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" by ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list ") else {
                            break 'dissect false;
                        };
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                    // Grok pattern: %{NOTSPACE:event.outcome} %{NOTSPACE} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{POSINT:source.port} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}/%{POSINT:destination.port}(%{GREEDYDATA})?
                    if !cached_grok!("%{NOTSPACE:event.outcome} %{NOTSPACE} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address}/%{POSINT:source.port} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}/%{POSINT:destination.port}(%{GREEDYDATA})?").extract_into(&input, event)? {
                        // Grok pattern: %{NOTSPACE:event.outcome} %{NOTSPACE} protocol %{POSINT:network.iana_number} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}(%{GREEDYDATA})?
                        if !cached_grok!("%{NOTSPACE:event.outcome} %{NOTSPACE} protocol %{POSINT:network.iana_number} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{NOTSPACE:destination.address}(%{GREEDYDATA})?").extract_into(&input, event)? {
                        }
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
                    // Grok pattern: %{NOTSPACE:event.outcome} %{NOTSPACE} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:(?P<destination_address>[^ (]*)(%{GREEDYDATA})?
                    if !cached_grok_mapped!("%{NOTSPACE:event.outcome} %{NOTSPACE} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:(?P<destination_address>[^ (]*)(%{GREEDYDATA})?", [("destination_address", "destination.address")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106015") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{NOTSPACE:event.outcome} %{NOTSPACE:network.transport} %{NOTSPACE} %{NOTSPACE} from %{IP:source.address}/%{POSINT:source.port} to %{IPORHOST:destination.address}/%{POSINT:destination.port} flags %{DATA} on interface %{NOTSPACE:_temp_.cisco.source_interface}
                    if !cached_grok!("%{NOTSPACE:event.outcome} %{NOTSPACE:network.transport} %{NOTSPACE} %{NOTSPACE} from %{IP:source.address}/%{POSINT:source.port} to %{IPORHOST:destination.address}/%{POSINT:destination.port} flags %{DATA} on interface %{NOTSPACE:_temp_.cisco.source_interface}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106016") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" IP spoof from (") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IP spoof from (") else {
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
                        let Some(pos) = remaining.find(" IP due to Land Attack from ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IP due to Land Attack from ")
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.icmp_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" by ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" by ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" list ") else {
                            break 'dissect false;
                        };
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
                        let Some(pos) = remaining.find(" IP teardrop fragment (size = ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IP teardrop fragment (size = ")
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                    // Grok pattern: ^%{NOTSPACE:event.outcome} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})?\\s*(\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\) )?dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?%{DATA}by access-group \"%{NOTSPACE:_temp_.cisco.list_id}\"
                    if !cached_grok_mapped!("^%{NOTSPACE:event.outcome} ((protocol %{POSINT:network.iana_number})|%{NOTSPACE:network.transport}) src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})?\\s*(\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\) )?dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?%{DATA}by access-group \"%{NOTSPACE:_temp_.cisco.list_id}\"", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("106027") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" src ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" src ") else {
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
                        captured.push(("event.outcome", &remaining[..pos]));
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

            let _cond = {
                event.get_str("_temp_.cisco.message_id") == Some("106102")
                    || event.get_str("_temp_.cisco.message_id") == Some("106103")
            };
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
                        captured.push(("event.outcome", &remaining[..pos]));
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

            let _cond = {
                ["109201", "109202", "109204", "109207", "109210"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: UAUTH: Session(=| )%{DATA}, User(=| )(?P<user_name>(?:[^,]+)), Assigned IP(=| )%{IP:source.address}, Succe%{GREEDYDATA}
                    if !cached_grok_mapped!("UAUTH: Session(=| )%{DATA}, User(=| )(?P<user_name>(?:[^,]+)), Assigned IP(=| )%{IP:source.address}, Succe%{GREEDYDATA}", [("user_name", "user.name")]).extract_into(&input, event)? {
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
                        captured.push(("_temp_.cisco.cli_outcome", remaining));
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
                event.get_str("_temp_.cisco.message_id") == Some("111004")
                    && event.get_str("_temp_.cisco.cli_outcome") == Some("OK")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("_temp_.cisco.message_id") == Some("111004")
                    && event.get_str("_temp_.cisco.cli_outcome") == Some("FAILED")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            event.remove("_temp_.cisco.cli_outcome");

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("111004") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                ["111008", "111009"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{NOTSPACE} '%{NOTSPACE:server.user.name}' executed %{NOTSPACE} %{GREEDYDATA:_temp_.cisco.command_line_arguments}
                    if !cached_grok!("^%{NOTSPACE} '%{NOTSPACE:server.user.name}' executed %{NOTSPACE} %{GREEDYDATA:_temp_.cisco.command_line_arguments}").extract_into(&input, event)? {
                        // Grok pattern: ^%{NOTSPACE} '%{NOTSPACE:server.user.name}' executed the '%{DATA}' command
                        if !cached_grok!("^%{NOTSPACE} '%{NOTSPACE:server.user.name}' executed the '%{DATA}' command").extract_into(&input, event)? {
                        }
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
                    // Grok pattern: AAA user %{DATA:_temp_.cisco.aaa_type} Successful(%{SPACE})?: server =(%{SPACE})?%{IP:destination.address} [:,] [Uu]ser = (?P<source_user_name>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))
                    if !cached_grok_mapped!("AAA user %{DATA:_temp_.cisco.aaa_type} Successful(%{SPACE})?: server =(%{SPACE})?%{IP:destination.address} [:,] [Uu]ser = (?P<source_user_name>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113005") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user (authentication|authorization) Rejected(%{SPACE})?: reason = (?P<event_reason>(?:(AAA failure|Account has been disabled|Account has been locked out|Invalid password|Password has expired|Password is expiring|Password malformed|Unspecified|Users account has expired)))(%{SPACE})?: server = %{IP:destination.address}(%{SPACE})?: user = ?((?P<source_user_name>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?: user IP = %{IP:source.address}
                    if !cached_grok_mapped!("AAA user (authentication|authorization) Rejected(%{SPACE})?: reason = (?P<event_reason>(?:(AAA failure|Account has been disabled|Account has been locked out|Invalid password|Password has expired|Password is expiring|Password malformed|Unspecified|Users account has expired)))(%{SPACE})?: server = %{IP:destination.address}(%{SPACE})?: user = ?((?P<source_user_name>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?: user IP = %{IP:source.address}", [("event_reason", "event.reason"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113008") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA transaction status ACCEPT(%{SPACE})?: user = ?((?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?
                    if !cached_grok_mapped!("AAA transaction status ACCEPT(%{SPACE})?: user = ?((?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113009") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA retrieved default group policy \\(%{DATA:source.user.group.name}\\) for user = ?((?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?
                    if !cached_grok_mapped!("AAA retrieved default group policy \\(%{DATA:source.user.group.name}\\) for user = ?((?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113012") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA user authentication Successful(%{SPACE})?: local database(%{SPACE})?: [Uu]ser = (?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))
                    if !cached_grok_mapped!("AAA user authentication Successful(%{SPACE})?: local database(%{SPACE})?: [Uu]ser = (?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113014") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AAA authentication server not accessible(%{SPACE})?: server =(%{SPACE})?%{IP:destination.address}(%{SPACE})?: [Uu]ser = ((?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?
                    if !cached_grok_mapped!("AAA authentication server not accessible(%{SPACE})?: server =(%{SPACE})?%{IP:destination.address}(%{SPACE})?: [Uu]ser = ((?P<source_user_name>(?:((LOCAL\\\\+)?(%{HOSTNAME}\\\\+)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))|\\*+)(%{SPACE})?", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113040") };
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
                    // Grok pattern: ^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))>
                    if !cached_grok_mapped!("^Group <(?P<source_user_group_name>(?:[^<>]+))> User <(?P<source_user_name>(?:[^<>]+))> IP <(?P<source_address>(?:[^<>]+))>", [("source_user_group_name", "source.user.group.name"), ("source_user_name", "source.user.name"), ("source_address", "source.address")]).extract_into(&input, event)? {
                        // Grok pattern: ^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address}
                        if !cached_grok!("^Group %{NOTSPACE:source.user.group.name} User %{NOTSPACE:source.user.name} IP %{NOTSPACE:source.address}").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("113042") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Non-HTTP connection from (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:source.port} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:destination.port} denied by redirect filter; only HTTP connections are supported for redirection.$
                    if !cached_grok_mapped!("^Non-HTTP connection from (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:source.port} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:destination.port} denied by redirect filter; only HTTP connections are supported for redirection.$", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("destination_address", "destination.address")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("210007") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^LU allocate xlate failed for (?P<_temp__cisco_translation_type>(?:%{WORD}-%{WORD})) %{WORD:network.protocol} translation from (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:source.port} \\((?P<_temp__cisco_mapped_source_ip>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:_temp_.cisco.mapped_source_port}\\) to (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:destination.port} \\((?P<_temp__cisco_mapped_destination_ip>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)
                    if !cached_grok_mapped!("^LU allocate xlate failed for (?P<_temp__cisco_translation_type>(?:%{WORD}-%{WORD})) %{WORD:network.protocol} translation from (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:source.port} \\((?P<_temp__cisco_mapped_source_ip>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:_temp_.cisco.mapped_source_port}\\) to (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:destination.port} \\((?P<_temp__cisco_mapped_destination_ip>(?:(?:%{IP}|%{HOSTNAME})))/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)", [("_temp__cisco_translation_type", "_temp_.cisco.translation_type"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__cisco_mapped_source_ip", "_temp_.cisco.mapped_source_ip"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("destination_address", "destination.address"), ("_temp__cisco_mapped_destination_ip", "_temp_.cisco.mapped_destination_ip")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("210022") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("LU missed ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" updates") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.missed_updates_count", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" updates") else {
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

            let _cond = { event.has_value("_temp_.cisco.missed_updates_count") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.missed_updates_count") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.missed_updates_count".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.missed_updates_count".into(),
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
                                path: "_temp_.cisco.missed_updates_count".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.missed_updates_count", converted)?;
                }
            }

            let _cond = {
                ["302013", "302015"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Built %{NOTSPACE} (?:Probe )?%{NOTSPACE:network.transport} connection %{NUMBER:_temp_.cisco.connection_id} for (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER:source.port} \\((?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER:_temp_.cisco.mapped_source_port}\\)(\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{NOTSPACE:destination.address}/%{NUMBER:destination.port} \\(%{NOTSPACE:_temp_.natdstip}/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)(\\((?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))?( \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))?%{GREEDYDATA}
                    if !cached_grok_mapped!("Built %{NOTSPACE} (?:Probe )?%{NOTSPACE:network.transport} connection %{NUMBER:_temp_.cisco.connection_id} for (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER:source.port} \\((?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER:_temp_.cisco.mapped_source_port}\\)(\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{NOTSPACE:destination.address}/%{NUMBER:destination.port} \\(%{NOTSPACE:_temp_.natdstip}/%{NUMBER:_temp_.cisco.mapped_destination_port}\\)(\\((?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))?( \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))?%{GREEDYDATA}", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__natsrcip", "_temp_.natsrcip"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user")]).extract_into(&input, event)? {
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("305006") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{NOTSPACE:_temp_.cisco.translation_type} translation creation failed for %{NOTSPACE:network.protocol} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|%{HOSTNAME})))(/%{NUMBER:source.port})?(\\s*\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|%{HOSTNAME})))(/%{NUMBER:destination.port})?(\\s*\\((?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\)$
                    if !cached_grok_mapped!("^%{NOTSPACE:_temp_.cisco.translation_type} translation creation failed for %{NOTSPACE:network.protocol} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|%{HOSTNAME})))(/%{NUMBER:source.port})?(\\s*\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|%{HOSTNAME})))(/%{NUMBER:destination.port})?(\\s*\\((?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?(%{HOSTNAME}\\\\)?%{USERNAME}(@%{HOSTNAME})?(, *%{NUMBER})?)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\)$", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("destination_address", "destination.address"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("305012") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Teardown %{DATA} %{NOTSPACE:network.transport} translation from (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER:source.port}(\\s*\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\+)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\+)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{IP:destination.address}/%{NUMBER:destination.port} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND}))
                    if !cached_grok_mapped!("Teardown %{DATA} %{NOTSPACE:network.transport} translation from (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER:source.port}(\\s*\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\+)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\+)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))? to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{IP:destination.address}/%{NUMBER:destination.port} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND}))", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond =
                { ["302020"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.action", json!("flow-creation"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("302020") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Built %{NOTSPACE} %{NOTSPACE:network.protocol} connection for faddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\((?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\) )?gaddr (?:(?:[^:]*):)?(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))))/%{NUMBER} laddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\) )?(type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?
                    if !cached_grok_mapped!("Built %{NOTSPACE} %{NOTSPACE:network.protocol} connection for faddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\((?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\) )?gaddr (?:(?:[^:]*):)?(?:(?:%{DATA:_temp_.natsrcip}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))))/%{NUMBER} laddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\) )?(type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("destination_domain", "destination.domain"), ("source_domain", "source.domain")]).extract_into(&input, event)? {
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
                        let Some(rest) = remaining.strip_prefix("Teardown ") else {
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("304001") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("304002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Access ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" URL ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" URL ") else {
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
                    // Grok pattern: Built %{NOTSPACE} %{NOTSPACE:network.transport} translation from %{NOTSPACE:_temp_.cisco.source_interface}:%{IPORHOST:source.address}/%{NUMBER:source.port}(\\(%{NOTSPACE:source.user.name}\\))? to %{NOTSPACE:_temp_.cisco.destination_interface}:%{IP:destination.address}/%{NUMBER:destination.port}
                    if !cached_grok!("Built %{NOTSPACE} %{NOTSPACE:network.transport} translation from %{NOTSPACE:_temp_.cisco.source_interface}:%{IPORHOST:source.address}/%{NUMBER:source.port}(\\(%{NOTSPACE:source.user.name}\\))? to %{NOTSPACE:_temp_.cisco.destination_interface}:%{IP:destination.address}/%{NUMBER:destination.port}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("305013") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Asymmetric NAT rules matched for forward and reverse flows; Connection for protocol %{NOTSPACE:network.iana_number} src (?P<_temp__cisco_source_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:source.port})?(?:\\(%{NOTSPACE:source.user.name}\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:destination.port})?(?:\\s*\\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\))? denied due to NAT reverse path failure
                    if !cached_grok_mapped!("^Asymmetric NAT rules matched for forward and reverse flows; Connection for protocol %{NOTSPACE:network.iana_number} src (?P<_temp__cisco_source_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:source.port})?(?:\\(%{NOTSPACE:source.user.name}\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:destination.port})?(?:\\s*\\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\))? denied due to NAT reverse path failure", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface")]).extract_into(&input, event)? {
                        // Grok pattern: ^Asymmetric NAT rules matched for forward and reverse flows; Connection for %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:source.port})?(?:\\(%{NOTSPACE:source.user.name}\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:destination.port})?(?:\\s*\\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\))? denied due to NAT reverse path failure
                        if !cached_grok_mapped!("^Asymmetric NAT rules matched for forward and reverse flows; Connection for %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:source.port})?(?:\\(%{NOTSPACE:source.user.name}\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{IPORHOST}(?:/%{NUMBER:destination.port})?(?:\\s*\\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\))? denied due to NAT reverse path failure", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface")]).extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("313001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                                        // Grok pattern: No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})?(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: <%{NOTSPACE:input.type}>[.]?
                                        if !cached_grok_mapped!("No matching connection for ICMP error message: %{NOTSPACE:network.transport} src (?P<_temp__cisco_source_interface>(?:[^:]*)):(?P<source_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:source.port})?(\\((?:(?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_source_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_source_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? dst (?P<_temp__cisco_destination_interface>(?:[^:]*)):(?P<destination_address>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))))(/%{NUMBER:destination.port})?(\\((?:(?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?))|\\((?:(?P<_temp__cisco_destination_user_or_sgt>(?:(?:\\*\\*\\*\\*\\*|(?:(?:LOCAL\\\\)?(?:(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b))\\\\)?(?:[a-zA-Z0-9._'-]+)\\$?(?:@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z\\-_]{0,62}))*(\\.?|\\b)))?(?:(?:, *)?%{NUMBER}(?::%{WORD})?)?)|[^$]+)))|(?P<_temp__cisco_destination_user_or_sgt>(?:(?:, *)?%{NUMBER}(?::%{WORD})?)))\\)))\\))? \\(type %{NUMBER:_temp_.cisco.icmp_type}, code %{NUMBER:_temp_.cisco.icmp_code}\\) on (?:[^:]*) interface.%{SPACE}Original IP payload: <%{NOTSPACE:input.type}>[.]?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("source_address", "source.address"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("destination_address", "destination.address"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_source_user_or_sgt", "_temp_.cisco.source_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt"), ("_temp__cisco_destination_user_or_sgt", "_temp_.cisco.destination_user_or_sgt")]).extract_into(&input, event)? {
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
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
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
                        let Some(pos) = remaining.find(" invalid ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" invalid ") else {
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("322001") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" MAC address ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" MAC address ") else {
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("401004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Shunned packet: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ==> ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ==> ") else {
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
                        captured.push(("_temp_.cisco.destination_interface", remaining));
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("500004") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Invalid transport field for protocol=%{NOTSPACE:network.transport}, from %{IPORHOST:source.address}/%{NUMBER:source.port} to %{IPORHOST:destination.address}/%{NUMBER:destination.port}$
                    if !cached_grok!("^Invalid transport field for protocol=%{NOTSPACE:network.transport}, from %{IPORHOST:source.address}/%{NUMBER:source.port} to %{IPORHOST:destination.address}/%{NUMBER:destination.port}$").extract_into(&input, event)? {
                    }
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("502103") };
            if _cond {
                event.append("event.type", json!("group"))?;
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("502103") };
            if _cond {
                event.append("event.category", json!("iam"))?;
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

            let _cond =
                { ["602101"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("PMTU-D packet ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" bytes greater than effective mtu ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.bytes", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" bytes greater than effective mtu ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", dest_addr=") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.cisco.effective_mtu", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", dest_addr=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", src_addr=") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", src_addr=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", prot=") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", prot=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("network.protocol", remaining));
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
                        captured.push(("event.outcome", &remaining[..pos]));
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.cisco.connection_type") {
                        // Grok pattern: (?:(?:(?P<network_transport>(?:(?:UDP|TCP)))|(?P<network_protocol>(?:(?:RTP|RTCP)))))
                        if !cached_grok_mapped!("(?:(?:(?P<network_transport>(?:(?:UDP|TCP)))|(?P<network_protocol>(?:(?:RTP|RTCP)))))", [("network_transport", "network.transport"), ("network_protocol", "network.protocol")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
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
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User authentication ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(": IP address: ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": IP address: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Uname: ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Uname: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("server.user.name", remaining));
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

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("710003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" access ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" access ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" by ACL from ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" by ACL from ") else {
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
                        let Some(pos) = remaining.find(" request ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" request ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                            let Some(pos) = remaining
                                .find(", Security negotiation complete for LAN-to-LAN Group (")
                            else {
                                break 'dissect false;
                            };
                            captured.push(("source.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(
                                ", Security negotiation complete for LAN-to-LAN Group (",
                            ) else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(") ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(") ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(", Inbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", Inbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(", Outbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", Outbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("713049") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", Username = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(", IP = ") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", IP = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) =
                                remaining.find(", Security negotiation complete for User (")
                            else {
                                break 'dissect false;
                            };
                            captured.push(("source.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining
                                .strip_prefix(", Security negotiation complete for User (")
                            else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(") ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(") ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(", Inbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", Inbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(", Outbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", Outbound SPI = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716002") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <%{NOTSPACE:_temp_.cisco.webvpn.group_name}> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> WebVPN session terminated: %{GREEDYDATA:event.reason}.
                    if !cached_grok_mapped!("Group <%{NOTSPACE:_temp_.cisco.webvpn.group_name}> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> WebVPN session terminated: %{GREEDYDATA:event.reason}.", [("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} WebVPN session terminated: %{GREEDYDATA:event.reason}.
                        if !cached_grok!("Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} WebVPN session terminated: %{GREEDYDATA:event.reason}.").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722051") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> IPv4 [Aa]ddress <%{IP:_temp_.cisco.assigned_ip}> IPv6 [Aa]ddress <%{IP:_temp_.cisco.assigned_ipv6}> %{GREEDYDATA}
                    if !cached_grok_mapped!("Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> IPv4 [Aa]ddress <%{IP:_temp_.cisco.assigned_ip}> IPv6 [Aa]ddress <%{IP:_temp_.cisco.assigned_ipv6}> %{GREEDYDATA}", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} IPv4 [Aa]ddress %{IP:_temp_.cisco.assigned_ip} IPv6 [Aa]ddress %{IP:_temp_.cisco.assigned_ipv6} %{GREEDYDATA}
                        if !cached_grok!("Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} IPv4 [Aa]ddress %{IP:_temp_.cisco.assigned_ip} IPv6 [Aa]ddress %{IP:_temp_.cisco.assigned_ipv6} %{GREEDYDATA}").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("722055") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> Client [Tt]ype: %{GREEDYDATA:user_agent.original}
                    if !cached_grok_mapped!("Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> Client [Tt]ype: %{GREEDYDATA:user_agent.original}", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} Client [Tt]ype: %{GREEDYDATA:user_agent.original}
                        if !cached_grok!("Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} Client [Tt]ype: %{GREEDYDATA:user_agent.original}").extract_into(&input, event)? {
                        }
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

            let _cond = {
                ["746012", "746013"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("user-identity: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" IP-User mapping ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IP-User mapping ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" - ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" - ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" - ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.operation.status", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" - ") else {
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
                        let Some(rest) = remaining.strip_prefix("SFR requested to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
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
                        let Some(rest) = remaining.strip_prefix("SFR requested ASA to ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" further packet redirection and process ")
                        else {
                            break 'dissect false;
                        };
                        captured.push(("event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" further packet redirection and process ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" flow from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.protocol", &remaining[..pos]));
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
                        captured.push(("network.protocol", &remaining[..pos]));
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
                        captured.push(("event.action", &remaining[..pos]));
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
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Local:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Remote:") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Remote:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Username:") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Username:") else {
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
                        // Grok pattern: Group <%{NOTSPACE:source.user.group.name}> User <%{NOTSPACE:source.user.name}> IP <%{IP:source.address}> Authentication: rejected, Session Type: %{NOTSPACE:_temp_.cisco.session_type}\\.
                        if !cached_grok!("Group <%{NOTSPACE:source.user.group.name}> User <%{NOTSPACE:source.user.name}> IP <%{IP:source.address}> Authentication: rejected, Session Type: %{NOTSPACE:_temp_.cisco.session_type}\\.").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716058") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> AnyConnect session lost connection. Waiting to resume.
                    if !cached_grok_mapped!("Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> AnyConnect session lost connection. Waiting to resume.", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} AnyConnect session lost connection. Waiting to resume.
                        if !cached_grok!("Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} AnyConnect session lost connection. Waiting to resume.").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("716059") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> AnyConnect session resumed(. C| c)onnection from IP <%{IP:_temp_.cisco.mapped_source_ip}>.
                    if !cached_grok_mapped!("Group <(?P<_temp__cisco_webvpn_group_name>(?:[^>]+))> User <(?P<source_user_name>(?:[^>]+))> IP <%{IP:source.address}> AnyConnect session resumed(. C| c)onnection from IP <%{IP:_temp_.cisco.mapped_source_ip}>.", [("_temp__cisco_webvpn_group_name", "_temp_.cisco.webvpn.group_name"), ("source_user_name", "source.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} AnyConnect session resumed(. C| c)onnection from IP %{IP:_temp_.cisco.mapped_source_ip}.
                        if !cached_grok!("Group %{NOTSPACE:_temp_.cisco.webvpn.group_name} User %{NOTSPACE:source.user.name} IP %{IP:source.address} AnyConnect session resumed(. C| c)onnection from IP %{IP:_temp_.cisco.mapped_source_ip}.").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("750003") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Local:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Remote:") else {
                            break 'dissect false;
                        };
                        captured.push(("source.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Remote:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Username:") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Username:") else {
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
                        let Some(pos) = remaining.find(" ERROR:") else {
                            break 'dissect false;
                        };
                        captured.push(("event.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ERROR:") else {
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

            let _cond = {
                ["434002", "434004"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond =
                { ["419002"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("network.protocol", json!("tcp"))?;
            }

            let _cond =
                { ["110002"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond =
                { ["713120"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["746012", "746013"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
                    && event.get_str("_temp_.operation.status") == Some("Succeeded")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["746012", "746013"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
                    && event.get_str("_temp_.operation.status") != Some("Succeeded")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["113004", "113008", "113009", "113012"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["113002", "113005", "113014", "113021"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["602303", "602304", "611101"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["605004", "611102"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond =
                { ["734001"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond =
                { ["716039"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond =
                { ["710005"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["713901", "713902", "713903", "713904", "713905"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["113039", "746012"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("client-vpn-connected"))?;
            }

            let _cond =
                { ["602101"].contains(&event.get_str("_temp_.cisco.message_id").unwrap_or("")) };
            if _cond {
                event.set("event.action", json!("vpn-pmtu-d"))?;
            }

            let _cond = {
                [
                    "113029", "113030", "113031", "113032", "113033", "113034", "113035", "113036",
                    "113037", "113038", "113040",
                ]
                .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("client-vpn-error"))?;
            }

            let _cond = {
                ["113019", "746013"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("client-vpn-disconnected"))?;
            }

            let _cond = {
                ["750002", "750003"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("connection-started"))?;
            }

            let _cond = {
                ["750003", "713905", "713904", "713906", "713902", "713901"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("error"))?;
            }

            let _cond = {
                ["113005", "113014", "113021", "605004", "611102", "716039"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("logon-failed"))?;
            }

            let _cond = {
                ["113004", "113008", "113009", "113012", "611101", "734001"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("logged-in"))?;
            }

            let _cond = {
                ["750003", "713905", "713904", "713906", "713902", "713901"]
                    .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            let _cond = {
                [
                    "305012", "302014", "302016", "302018", "302021", "302036", "302304", "302306",
                    "609001", "609002",
                ]
                .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("flow-expiration"))?;
            }

            let _cond = {
                [
                    "302014", "302016", "302018", "302021", "302036", "302304", "302306",
                ]
                .contains(&event.get_str("_temp_.cisco.message_id").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)
                    if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_termination_initiator", "_temp_.cisco.termination_initiator"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user")]).extract_into(&input, event)? {
                        // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*))
                        if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) from (?P<_temp__cisco_termination_initiator>(?:[^:]*))", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_termination_initiator", "_temp_.cisco.termination_initiator")]).extract_into(&input, event)? {
                            // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)
                            if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*)) \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user")]).extract_into(&input, event)? {
                                // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)
                                if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) \\((?P<_temp__cisco_termination_user>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__duration_hms", "_temp_.duration_hms"), ("_temp__cisco_termination_user", "_temp_.cisco.termination_user")]).extract_into(&input, event)? {
                                    // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*))
                                    if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}) (?P<event_reason>(?:[^:]*))", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__duration_hms", "_temp_.duration_hms"), ("event_reason", "event.reason")]).extract_into(&input, event)? {
                                        // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes})
                                        if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} (?:state-bypass )?connection %{NOTSPACE:_temp_.cisco.connection_id} (?:for|from) (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address}/%{NUMBER:source.port:int}\\s*(?:\\(?(?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?duration\\s+(?:(?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes})", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__duration_hms", "_temp_.duration_hms")]).extract_into(&input, event)? {
                                            // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} connection %{NOTSPACE:_temp_.cisco.connection_id} from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}(?:\\s+%{NUMBER}\\s+%{NUMBER})?
                                            if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} connection %{NOTSPACE:_temp_.cisco.connection_id} from (?P<_temp__cisco_source_interface>(?:[^:]*)):%{DATA:source.address} to (?P<_temp__cisco_destination_interface>(?:[^:]*)):%{DATA:destination.address}/%{NUMBER:destination.port:int} duration (?P<_temp__duration_hms>(?:%{INT}:%{MINUTE}:%{SECOND})) bytes %{NUMBER:network.bytes}(?:\\s+%{NUMBER}\\s+%{NUMBER})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_interface", "_temp_.cisco.destination_interface"), ("_temp__duration_hms", "_temp_.duration_hms")]).extract_into(&input, event)? {
                                                // Grok pattern: ^Teardown (?:Probe )?%{NOTSPACE:network.transport} connection for faddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?gaddr (?:(?:[^:]*):)?(?:(?:(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))))/%{NUMBER} laddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))?(\\s*type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?
                                                if !cached_grok_mapped!("^Teardown (?:Probe )?%{NOTSPACE:network.transport} connection for faddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:destination.address}|(?P<destination_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\(?(?P<_temp__cisco_destination_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\)? )?gaddr (?:(?:[^:]*):)?(?:(?:(?P<_temp__natsrcip>(?:(?:%{IP}|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))|(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))))/%{NUMBER} laddr (?:(?P<_temp__cisco_source_interface>(?:[^:]*)):)?(?:(?:%{IP:source.address}|(?P<source_domain>(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))))/%{NUMBER}\\s*(?:\\((?P<_temp__cisco_source_username>(?:((LOCAL\\\\)?((?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b))\\\\)?%{USERNAME}(@(?:\\b(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62})(?:\\.(?:[0-9A-Za-z][0-9A-Za-z_-]{0,62}))*(\\.?|\\b)))?(, *%{NUMBER})?)))\\))?(\\s*type %{NUMBER:_temp_.cisco.icmp_type} code %{NUMBER:_temp_.cisco.icmp_code})?", [("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_destination_username", "_temp_.cisco.destination_username"), ("_temp__cisco_source_interface", "_temp_.cisco.source_interface"), ("_temp__cisco_source_username", "_temp_.cisco.source_username"), ("destination_domain", "destination.domain"), ("_temp__natsrcip", "_temp_.natsrcip"), ("source_domain", "source.domain")]).extract_into(&input, event)? {
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
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                    Ok(())
                })();
            }

            event.remove("message");
            event.remove("_temp_.full_message");
            event.remove("_conf");

            let _cond = { event.has_value("_temp_.orig_security") };
            if _cond {
                // Painless script
                // Source: boolean isEmpty(def value) {\n  return (value instanceof AbstractList ? value.size() : value.length()) == 0;\n}\ndef appendOrCreate(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n  dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n String key = path[path.length - 1];\n def existing = dest.get(key);\n return existing == null ?\n  dest.put(key, value)\n  : existing instanceof AbstractList ?\n    existing.add(value)\n    : dest.put(key, new ArrayList([existing, value]));\n}\ndef msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\ndef dest = new HashMap();\ndef dest_event = new HashMap();\ndef security_event_list = new ArrayList(['ac_policy',\n                            'access_control_rule_action',\n                            'access_control_rule_name',\n                            'access_control_rule_reason',\n                            'application_protocol',\n                            'client',\n                            'client_version',\n                            'connection_duration',\n                            'dns_query',\n                            'dns_record_type',\n                            'dns_response_type',\n                            'dns_ttl',\n                            'destination_ip_dynamic_attribute',\n                            'destination_security_group',\n                            'destination_security_group_tag',\n                            'dst_ip',\n                            'dst_port',\n                            'egress_interface',\n                            'egress_zone',\n                            'encrypt_peer_ip',\n                            'file_action',\n                            'file_count',\n                            'file_direction',\n                            'file_name',\n                            'file_policy',\n                            'file_sandbox_status',\n                            'file_sha256',\n                            'file_size',\n                            'file_type',\n                            'first_packet_second',\n                            'http_referer',\n                            'http_response',\n                            'icmp_code',\n                            'icmp_type',\n                            'ingress_interface',\n                            'ingress_zone',\n                            'initiator_bytes',\n                            'initiator_packets',\n                            'nap_policy',\n                            'prefilter_policy',\n                            'protocol',\n                            'referenced_host',\n                            'responder_bytes',\n                            'responder_packets',\n                            'sha_disposition',\n                            'source_security_group',\n                            'source_security_group_tag',\n                            'source_security_group_type',\n                            'spero_disposition',\n                            'src_ip',\n                            'src_port',\n                            'ssl_actual_action',\n                            'ssl_certificate',\n                            'ssl_expected_action',\n                            'ssl_flow_status',\n                            'ssl_policy',\n                            'ssl_rule_name',\n                            'ssl_server_cert_status',\n                            'ssl_server_name',\n                            'ssl_session_id',\n                            'ssl_ticket_id',\n                            'ssl_version',\n                            'sslurl_category',\n                            'tunnel_or_prefilter_rule',\n                            'uri',\n                            'url',\n                            'url_category',\n                            'url_reputation',\n                            'user',\n                            'user_agent',\n                            'vpn_action',\n                            'web_application']);\nctx._temp_.cisco['security'] = dest;\nctx._temp_.cisco['security_event'] = dest_event;\nfor (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n if (param == null) {\n   continue;\n }\n param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, field.splitOnToken('.'), entry.getValue()) );\n  if (security_event_list.contains(param.target)){\n    dest_event[param.target] = entry.getValue();\n  }\n  else{\n    dest[param.target] = entry.getValue();\n  }\n }\n}\nif (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\nfor (entry in counters.entrySet()) {\n if (best == null || best.getValue() < entry.getValue()) best = entry;\n}\nif (best != null) ctx._temp_.cisco.message_id = best.getKey();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"boolean isEmpty(def value) {\n  return (value instanceof AbstractList ? value.size() : value.length()) == 0;\n}\ndef appendOrCreate(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n  dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n String key = path[path.length - 1];\n def existing = dest.get(key);\n return existing == null ?\n  dest.put(key, value)\n  : existing instanceof AbstractList ?\n    existing.add(value)\n    : dest.put(key, new ArrayList([existing, value]));\n}\ndef msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\ndef dest = new HashMap();\ndef dest_event = new HashMap();\ndef security_event_list = new ArrayList(['ac_policy',\n                            'access_control_rule_action',\n                            'access_control_rule_name',\n                            'access_control_rule_reason',\n                            'application_protocol',\n                            'client',\n                            'client_version',\n                            'connection_duration',\n                            'dns_query',\n                            'dns_record_type',\n                            'dns_response_type',\n                            'dns_ttl',\n                            'destination_ip_dynamic_attribute',\n                            'destination_security_group',\n                            'destination_security_group_tag',\n                            'dst_ip',\n                            'dst_port',\n                            'egress_interface',\n                            'egress_zone',\n                            'encrypt_peer_ip',\n                            'file_action',\n                            'file_count',\n                            'file_direction',\n                            'file_name',\n                            'file_policy',\n                            'file_sandbox_status',\n                            'file_sha256',\n                            'file_size',\n                            'file_type',\n                            'first_packet_second',\n                            'http_referer',\n                            'http_response',\n                            'icmp_code',\n                            'icmp_type',\n                            'ingress_interface',\n                            'ingress_zone',\n                            'initiator_bytes',\n                            'initiator_packets',\n                            'nap_policy',\n                            'prefilter_policy',\n                            'protocol',\n                            'referenced_host',\n                            'responder_bytes',\n                            'responder_packets',\n                            'sha_disposition',\n                            'source_security_group',\n                            'source_security_group_tag',\n                            'source_security_group_type',\n                            'spero_disposition',\n                            'src_ip',\n                            'src_port',\n                            'ssl_actual_action',\n                            'ssl_certificate',\n                            'ssl_expected_action',\n                            'ssl_flow_status',\n                            'ssl_policy',\n                            'ssl_rule_name',\n                            'ssl_server_cert_status',\n                            'ssl_server_name',\n                            'ssl_session_id',\n                            'ssl_ticket_id',\n                            'ssl_version',\n                            'sslurl_category',\n                            'tunnel_or_prefilter_rule',\n                            'uri',\n                            'url',\n                            'url_category',\n                            'url_reputation',\n                            'user',\n                            'user_agent',\n                            'vpn_action',\n                            'web_application']);\nctx._temp_.cisco['security'] = dest;\nctx._temp_.cisco['security_event'] = dest_event;\nfor (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n if (param == null) {\n   continue;\n }\n param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, field.splitOnToken('.'), entry.getValue()) );\n  if (security_event_list.contains(param.target)){\n    dest_event[param.target] = entry.getValue();\n  }\n  else{\n    dest[param.target] = entry.getValue();\n  }\n }\n}\nif (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\nfor (entry in counters.entrySet()) {\n if (best == null || best.getValue() < entry.getValue()) best = entry;\n}\nif (best != null) ctx._temp_.cisco.message_id = best.getKey();\n"#
                    ),
                    cached_params!(
                        "{\"ACPolicy\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"ac_policy\"},\"AccessControlRuleAction\":{\"ecs\":[\"event.outcome\"],\"id\":[\"430002\",\"430003\"],\"target\":\"access_control_rule_action\"},\"AccessControlRuleName\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430002\",\"430003\"],\"target\":\"access_control_rule_name\"},\"AccessControlRuleReason\":{\"id\":[\"430002\",\"430003\"],\"target\":\"access_control_rule_reason\"},\"ApplicationProtocol\":{\"ecs\":[\"network.protocol\"],\"target\":\"application_protocol\"},\"ArchiveDepth\":{\"id\":[\"430004\",\"430005\"],\"target\":\"archive_depth\"},\"ArchiveFileName\":{\"ecs\":[\"file.name\"],\"id\":[\"430004\",\"430005\"],\"target\":\"archive_file_name\"},\"ArchiveFileStatus\":{\"id\":[\"430004\",\"430005\"],\"target\":\"archive_file_status\"},\"ArchiveSHA256\":{\"ecs\":[\"file.hash.sha256\"],\"id\":[\"430004\",\"430005\"],\"target\":\"archive_sha256\"},\"Classification\":{\"id\":[\"430001\"],\"target\":\"classification\"},\"Client\":{\"ecs\":[\"network.application\"],\"target\":\"client\"},\"ClientVersion\":{\"id\":[\"430002\",\"430003\"],\"target\":\"client_version\"},\"ConnectionDuration\":{\"ecs\":[\"event.duration\"],\"id\":[\"430003\"],\"target\":\"connection_duration\"},\"DNSQuery\":{\"ecs\":[\"dns.question.name\"],\"id\":[\"430002\",\"430003\"],\"target\":\"dns_query\"},\"DNSRecordType\":{\"ecs\":[\"dns.question.type\"],\"id\":[\"430002\",\"430003\"],\"target\":\"dns_record_type\"},\"DNSResponseType\":{\"ecs\":[\"dns.response_code\"],\"id\":[\"430002\",\"430003\"],\"target\":\"dns_response_type\"},\"DNSSICategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"dnssi_category\"},\"DNS_Sinkhole\":{\"id\":[\"430002\",\"430003\"],\"target\":\"dns_sinkhole\"},\"DNS_TTL\":{\"id\":[\"430002\",\"430003\"],\"target\":\"dns_ttl\"},\"DestinationIP_DynamicAttribute\":{\"id\":[\"430002\",\"430003\"],\"target\":\"destination_ip_dynamic_attribute\"},\"DestinationSecurityGroup\":{\"id\":[\"430002\",\"430003\"],\"target\":\"destination_security_group\"},\"DestinationSecurityGroupTag\":{\"id\":[\"430002\",\"430003\"],\"target\":\"destination_security_group_tag\"},\"DstIP\":{\"ecs\":[\"destination.address\"],\"target\":\"dst_ip\"},\"DstPort\":{\"ecs\":[\"destination.port\"],\"target\":\"dst_port\"},\"EgressInterface\":{\"ecs\":[\"_temp_.cisco.destination_interface\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"egress_interface\"},\"EgressZone\":{\"ecs\":[\"observer.egress.zone\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"egress_zone\"},\"EncryptPeerIP\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"encrypt_peer_ip\"},\"Endpoint Profile\":{\"ecs\":[\"_temp_.host.type\"],\"id\":[\"430002\",\"430003\"],\"target\":\"endpoint_profile\"},\"FileAction\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_action\"},\"FileCount\":{\"id\":[\"430002\",\"430003\"],\"target\":\"file_count\"},\"FileDirection\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_direction\"},\"FileName\":{\"ecs\":[\"file.name\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_name\"},\"FilePolicy\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_policy\"},\"FileSHA256\":{\"ecs\":[\"file.hash.sha256\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_sha256\"},\"FileSandboxStatus\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_sandbox_status\"},\"FileSize\":{\"ecs\":[\"file.size\"],\"id\":[\"430004\",\"430005\"],\"target\":\"file_size\"},\"FileStorageStatus\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_storage_status\"},\"FileType\":{\"id\":[\"430004\",\"430005\"],\"target\":\"file_type\"},\"FirstPacketSecond\":{\"ecs\":[\"event.start\"],\"id\":[\"430004\",\"430005\"],\"target\":\"first_packet_second\"},\"GID\":{\"ecs\":[\"service.id\"],\"id\":[\"430001\"],\"target\":\"gid\"},\"HTTPReferer\":{\"ecs\":[\"http.request.referrer\"],\"id\":[\"430002\",\"430003\"],\"target\":\"http_referer\"},\"HTTPResponse\":{\"ecs\":[\"http.response.status_code\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"http_response\"},\"ICMPCode\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"icmp_code\"},\"ICMPType\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"icmp_type\"},\"IPReputationSICategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ip_reputation_si_category\"},\"IPSCount\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ips_count\"},\"IngressInterface\":{\"ecs\":[\"_temp_.cisco.source_interface\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"ingress_interface\"},\"IngressZone\":{\"ecs\":[\"observer.ingress.zone\"],\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"ingress_zone\"},\"InitiatorBytes\":{\"ecs\":[\"source.bytes\"],\"id\":[\"430003\"],\"target\":\"initiator_bytes\"},\"InitiatorPackets\":{\"ecs\":[\"source.packets\"],\"id\":[\"430003\"],\"target\":\"initiator_packets\"},\"InlineResult\":{\"ecs\":[\"event.outcome\"],\"id\":[\"430001\"],\"target\":\"inline_result\"},\"IntrusionPolicy\":{\"ecs\":[\"_temp_.cisco.rule_name\"],\"id\":[\"430001\"],\"target\":\"intrusion_policy\"},\"MPLS_Label\":{\"id\":[\"430001\"],\"target\":\"mpls_label\"},\"Message\":{\"ecs\":[\"message\"],\"id\":[\"430001\"],\"target\":\"message\"},\"NAPPolicy\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"nap_policy\"},\"NAT_InitiatorIP\":{\"ecs\":[\"source.nat.ip\"],\"id\":[\"430002\",\"430003\"],\"target\":\"nat_src_ip\"},\"NAT_InitiatorPort\":{\"ecs\":[\"source.nat.port\"],\"id\":[\"430002\",\"430003\"],\"target\":\"nat_src_port\"},\"NAT_ResponderIP\":{\"ecs\":[\"destination.nat.ip\"],\"id\":[\"430002\",\"430003\"],\"target\":\"nat_dst_ip\"},\"NAT_ResponderPort\":{\"ecs\":[\"destination.nat.port\"],\"id\":[\"430002\",\"430003\"],\"target\":\"nat_dst_port\"},\"NetBIOSDomain\":{\"ecs\":[\"host.hostname\"],\"id\":[\"430002\",\"430003\"],\"target\":\"net_bios_domain\"},\"NumIOC\":{\"id\":[\"430001\"],\"target\":\"num_ioc\"},\"Prefilter Policy\":{\"id\":[\"430002\",\"430003\"],\"target\":\"prefilter_policy\"},\"Priority\":{\"id\":[\"430001\"],\"target\":\"priority\"},\"Protocol\":{\"ecs\":[\"network.transport\"],\"target\":\"protocol\"},\"ReferencedHost\":{\"ecs\":[\"url.domain\"],\"id\":[\"430002\",\"430003\"],\"target\":\"referenced_host\"},\"ResponderBytes\":{\"ecs\":[\"destination.bytes\"],\"id\":[\"430003\"],\"target\":\"responder_bytes\"},\"ResponderPackets\":{\"ecs\":[\"destination.packets\"],\"id\":[\"430003\"],\"target\":\"responder_packets\"},\"Revision\":{\"ecs\":[\"rule.version\"],\"id\":[\"430001\"],\"target\":\"revision\"},\"SHA_Disposition\":{\"id\":[\"430004\",\"430005\"],\"target\":\"sha_disposition\"},\"SID\":{\"ecs\":[\"rule.id\"],\"id\":[\"430001\"],\"target\":\"sid\"},\"SSLActualAction\":{\"ecs\":[\"event.outcome\"],\"target\":\"ssl_actual_action\"},\"SSLCertificate\":{\"id\":[\"430002\",\"430003\",\"430004\",\"430005\"],\"target\":\"ssl_certificate\"},\"SSLExpectedAction\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_expected_action\"},\"SSLFlowStatus\":{\"id\":[\"430002\",\"430003\",\"430004\",\"430005\"],\"target\":\"ssl_flow_status\"},\"SSLPolicy\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_policy\"},\"SSLRuleName\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_rule_name\"},\"SSLServerCertStatus\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_server_cert_status\"},\"SSLServerName\":{\"ecs\":[\"server.domain\"],\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_server_name\"},\"SSLSessionID\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_session_id\"},\"SSLTicketID\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_ticket_id\"},\"SSLURLCategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"sslurl_category\"},\"SSLVersion\":{\"id\":[\"430002\",\"430003\"],\"target\":\"ssl_version\"},\"SSSLCipherSuite\":{\"id\":[\"430002\",\"430003\"],\"target\":\"sssl_cipher_suite\"},\"SecIntMatchingIP\":{\"id\":[\"430002\",\"430003\"],\"target\":\"sec_int_matching_ip\"},\"Security Group\":{\"id\":[\"430002\",\"430003\"],\"target\":\"security_group\"},\"SourceSecurityGroup\":{\"id\":[\"430002\",\"430003\"],\"target\":\"source_security_group\"},\"SourceSecurityGroupTag\":{\"id\":[\"430002\",\"430003\"],\"target\":\"source_security_group_tag\"},\"SourceSecurityGroupType\":{\"id\":[\"430002\",\"430003\"],\"target\":\"source_security_group_type\"},\"SperoDisposition\":{\"id\":[\"430004\",\"430005\"],\"target\":\"spero_disposition\"},\"SrcIP\":{\"ecs\":[\"source.address\"],\"target\":\"src_ip\"},\"SrcPort\":{\"ecs\":[\"source.port\"],\"target\":\"src_port\"},\"TCPFlags\":{\"id\":[\"430002\",\"430003\"],\"target\":\"tcp_flags\"},\"ThreatName\":{\"ecs\":[\"_temp_.cisco.threat_category\"],\"id\":[\"430005\"],\"target\":\"threat_name\"},\"ThreatScore\":{\"ecs\":[\"_temp_.cisco.threat_level\"],\"id\":[\"430005\"],\"target\":\"threat_score\"},\"Tunnel or Prefilter Rule\":{\"id\":[\"430002\",\"430003\"],\"target\":\"tunnel_or_prefilter_rule\"},\"URI\":{\"ecs\":[\"url.original\"],\"id\":[\"430004\",\"430005\"],\"target\":\"uri\"},\"URL\":{\"ecs\":[\"url.original\"],\"id\":[\"430002\",\"430003\"],\"target\":\"url\"},\"URLCategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"url_category\"},\"URLReputation\":{\"id\":[\"430002\",\"430003\"],\"target\":\"url_reputation\"},\"URLSICategory\":{\"id\":[\"430002\",\"430003\"],\"target\":\"urlsi_category\"},\"User\":{\"ecs\":[\"user.id\",\"user.name\"],\"target\":\"user\"},\"UserAgent\":{\"ecs\":[\"user_agent.original\"],\"id\":[\"430002\",\"430003\"],\"target\":\"user_agent\"},\"VLAN_ID\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"vlan_id\"},\"VPN_Action\":{\"id\":[\"430001\",\"430002\",\"430003\"],\"target\":\"vpn_action\"},\"WebApplication\":{\"ecs\":[\"network.application\"],\"target\":\"web_application\"},\"originalClientSrcIP\":{\"ecs\":[\"client.address\"],\"id\":[\"430002\",\"430003\"],\"target\":\"original_client_src_ip\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.connection_duration") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.connection_duration") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.connection_duration"
                                            .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.connection_duration".into(),
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
                                path: "_temp_.cisco.security_event.connection_duration".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.connection_duration", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.dns_ttl") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.dns_ttl") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.dns_ttl".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.dns_ttl".into(),
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
                                path: "_temp_.cisco.security_event.dns_ttl".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.dns_ttl", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.dst_ip") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.cisco.security_event.dst_ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp_.cisco.security_event.dst_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("_temp_.cisco.security_event.dst_ip", s)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.encrypt_peer_ip") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.cisco.security_event.encrypt_peer_ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp_.cisco.security_event.encrypt_peer_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("_temp_.cisco.security_event.encrypt_peer_ip", s)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.dst_port") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.dst_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.dst_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.dst_port".into(),
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
                                path: "_temp_.cisco.security_event.dst_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.dst_port", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.file_count") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.file_count") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.file_count".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.file_count".into(),
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
                                path: "_temp_.cisco.security_event.file_count".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.file_count", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.file_size") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.file_size") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.file_size".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.file_size".into(),
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
                                path: "_temp_.cisco.security_event.file_size".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.file_size", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.first_packet_second") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("_temp_.cisco.security_event.first_packet_second")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_first_packet_second",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.http_response") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.http_response") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.http_response".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.http_response".into(),
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
                                path: "_temp_.cisco.security_event.http_response".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.http_response", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.initiator_bytes") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.initiator_bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.initiator_bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.initiator_bytes".into(),
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
                                path: "_temp_.cisco.security_event.initiator_bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.initiator_bytes", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.initiator_packets") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.initiator_packets") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.initiator_packets"
                                            .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.initiator_packets".into(),
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
                                path: "_temp_.cisco.security_event.initiator_packets".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.initiator_packets", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.responder_bytes") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.responder_bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.responder_bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.responder_bytes".into(),
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
                                path: "_temp_.cisco.security_event.responder_bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.responder_bytes", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.responder_packets") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.responder_packets") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.responder_packets"
                                            .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.responder_packets".into(),
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
                                path: "_temp_.cisco.security_event.responder_packets".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.responder_packets", converted)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.src_ip") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.cisco.security_event.src_ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp_.cisco.security_event.src_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("_temp_.cisco.security_event.src_ip", s)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco.security_event.src_port") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.security_event.src_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.security_event.src_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.security_event.src_port".into(),
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
                                path: "_temp_.cisco.security_event.src_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.security_event.src_port", converted)?;
                }
            }

            // Painless script
            // Source: def getField(Map src, String[] path) {\n for (int i=0; i<path.length-1; i++) {\n  src = src.getOrDefault(path[i], null);\n  if (src == null || !(src instanceof Map)) {\n    return null;\n  }\n }\n return src[path[path.length-1]];\n}\ndef setField(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n   dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n dest[path[path.length-1]] = value;\n}\nfor (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  def param = entry.getValue();\n  def rawVal = getField(ctx, srcField.splitOnToken('.'));\n  if (rawVal == null) continue;\n  String oldVal;\n  if (rawVal instanceof AbstractList) {\n    if (rawVal.size() == 0) continue;\n    oldVal = rawVal[0].toString();\n  } else {\n    oldVal = rawVal.toString();\n  }\n  def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"def getField(Map src, String[] path) {\n for (int i=0; i<path.length-1; i++) {\n  src = src.getOrDefault(path[i], null);\n  if (src == null || !(src instanceof Map)) {\n    return null;\n  }\n }\n return src[path[path.length-1]];\n}\ndef setField(Map dest, String[] path, def value) {\n for (int i=0; i<path.length-1; i++) {\n   dest = dest.computeIfAbsent(path[i], _ -> new HashMap());\n }\n dest[path[path.length-1]] = value;\n}\nfor (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  def param = entry.getValue();\n  def rawVal = getField(ctx, srcField.splitOnToken('.'));\n  if (rawVal == null) continue;\n  String oldVal;\n  if (rawVal instanceof AbstractList) {\n    if (rawVal.size() == 0) continue;\n    oldVal = rawVal[0].toString();\n  } else {\n    oldVal = rawVal.toString();\n  }\n  def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n"#
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

            let _cond = { event.has_value("source.bytes") };
            if _cond {
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

            let _cond = { event.has_value("destination.bytes") };
            if _cond {
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

            let _cond = {
                event.has_value("source.bytes")
                    && event.has_value("destination.bytes")
                    && !event.has_value("network.bytes")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.network == null) {\n  ctx.network = [:];\n}\nctx.network.bytes = ctx.source.bytes + ctx.destination.bytes;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.network == null) {\n  ctx.network = [:];\n}\nctx.network.bytes = ctx.source.bytes + ctx.destination.bytes;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.duration_hms") };
            if _cond {
                // Painless script
                // Source: long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        } else if (c != (char)'h' && c == (char)'m' && c == (char)'s') {\n            return 0;\n        }\n    }\n    return total + cur;\n} if (ctx.event == null) {\n    ctx['event'] = new HashMap();\n} if (ctx?._temp_.cisco?.message_id == '430003') {\n  String start = ctx['@timestamp'];\n  ctx.event['start'] = start;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['end'] = ZonedDateTime.ofInstant(\n      Instant.parse(start).plusNanos(nanos),\n      ZoneOffset.UTC);\n} else {\n  String end = ctx['@timestamp'];\n  ctx.event['end'] = end;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['start'] = ZonedDateTime.ofInstant(\n      Instant.parse(end).minusNanos(nanos),\n      ZoneOffset.UTC);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        } else if (c != (char)'h' && c == (char)'m' && c == (char)'s') {\n            return 0;\n        }\n    }\n    return total + cur;\n} if (ctx.event == null) {\n    ctx['event'] = new HashMap();\n} if (ctx?._temp_.cisco?.message_id == '430003') {\n  String start = ctx['@timestamp'];\n  ctx.event['start'] = start;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['end'] = ZonedDateTime.ofInstant(\n      Instant.parse(start).plusNanos(nanos),\n      ZoneOffset.UTC);\n} else {\n  String end = ctx['@timestamp'];\n  ctx.event['end'] = end;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['start'] = ZonedDateTime.ofInstant(\n      Instant.parse(end).minusNanos(nanos),\n      ZoneOffset.UTC);\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco.source_username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("_temp_.cisco.source_username") {
                        let re = cached_regex!("\\\\{2,}");
                        let replaced = re.replace_all(&s, "\\\\").into_owned();
                        event.set("_temp_.cisco.source_username", replaced)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp_.cisco.source_username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.cisco.source_username") {
                        // Grok pattern: (?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:%{USERNAME}(@%{HOSTNAME})?)))(?:(, *%{NUMBER:_temp_.cisco.source_user_security_group_tag})?)
                        if !cached_grok_mapped!("(?P<_temp__cisco_source_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:%{USERNAME}(@%{HOSTNAME})?)))(?:(, *%{NUMBER:_temp_.cisco.source_user_security_group_tag})?)", [("_temp__cisco_source_username", "_temp_.cisco.source_username")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
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

            let _cond = { event.has_value("_temp_.cisco.destination_username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.cisco.destination_username") {
                        // Grok pattern: (?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:%{USERNAME}(@%{HOSTNAME})?)))(?:(, *%{NUMBER:_temp_.cisco.destination_user_security_group_tag})?)
                        if !cached_grok_mapped!("(?P<_temp__cisco_destination_username>(?:((?:(LOCAL\\\\)?(%{HOSTNAME}\\\\)?))?(?:%{USERNAME}(@%{HOSTNAME})?)))(?:(, *%{NUMBER:_temp_.cisco.destination_user_security_group_tag})?)", [("_temp__cisco_destination_username", "_temp_.cisco.destination_username")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
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

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_source_user_name")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "fail-{}",
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: (format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        ))
                        .to_string(),
                    });
                }
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("destination.user.name") {
                        // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?P<destination_user_email>(?:(?:(?P<destination_user_name>(?:[^@$]+)))@%{HOSTNAME:destination.user.domain}))
                        if !cached_grok_mapped!("((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?P<destination_user_email>(?:(?:(?P<destination_user_name>(?:[^@$]+)))@%{HOSTNAME:destination.user.domain}))", [("destination_user_email", "destination.user.email"), ("destination_user_name", "destination.user.name")]).extract_into(&input, event)? {
                        // Grok pattern: ((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?:(?P<destination_user_name>(?:[^@$]+)))
                        if !cached_grok_mapped!("((?:(LOCAL\\\\)?(%{HOSTNAME:destination.user.domain}\\\\)?))?(?:(?P<destination_user_name>(?:[^@$]+)))", [("destination_user_name", "destination.user.name")]).extract_into(&input, event)? {
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_destination_user_name",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "fail-{}",
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: (format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        ))
                        .to_string(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("network.transport") {
                    let lowered = s.to_lowercase();
                    event.set("network.transport", lowered)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("network.application") {
                    let lowered = s.to_lowercase();
                    event.set("network.application", lowered)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("file.type") {
                    let lowered = s.to_lowercase();
                    event.set("file.type", lowered)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("network.direction") {
                    let lowered = s.to_lowercase();
                    event.set("network.direction", lowered)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("network.type") {
                    let lowered = s.to_lowercase();
                    event.set("network.type", lowered)?;
                }
                Ok(())
            })();

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

            if event.has("event.outcome") {
                if let Some(s) = event.get_string("event.outcome") {
                    let lowered = s.to_lowercase();
                    event.set("event.outcome", lowered)?;
                }
            }

            let _cond = { event.get_str("event.outcome") == Some("est-allowed") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("event.outcome") == Some("permitted") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("event.outcome") == Some("allow") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("event.outcome") == Some("denied") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("event.outcome") == Some("deny") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("event.outcome") == Some("dropped") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("network.transport") == Some("icmpv6") };
            if _cond {
                event.set("network.transport", json!("ipv6-icmp"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "source.port".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "destination.port".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "source.bytes".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "destination.bytes".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "network.bytes".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "source.packets".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "destination.packets".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_temp_.cisco.mapped_source_port".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_temp_.cisco.mapped_destination_port".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_temp_.cisco.icmp_code".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_temp_.cisco.icmp_type".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(?:%{IP:source.ip}|%{GREEDYDATA:source.domain})$
                    if !cached_grok!("^(?:%{IP:source.ip}|%{GREEDYDATA:source.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: ^(?:%{IP:destination.ip}|%{GREEDYDATA:destination.domain})$
                    if !cached_grok!("^(?:%{IP:destination.ip}|%{GREEDYDATA:destination.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("client.address") {
                    // Grok pattern: ^(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})$
                    if !cached_grok!("^(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("server.address") {
                    // Grok pattern: ^(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})$
                    if !cached_grok!("^(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})$")
                        .extract_into(&input, event)?
                    {}
                }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_temp_.natsrcip") {
                    // Grok pattern: ^(?:%{IP:_temp_.cisco.mapped_source_ip}|%{GREEDYDATA:_temp_.cisco.mapped_source_host})$
                    if !cached_grok!("^(?:%{IP:_temp_.cisco.mapped_source_ip}|%{GREEDYDATA:_temp_.cisco.mapped_source_host})$").extract_into(&input, event)? {
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_temp_.natdstip") {
                    // Grok pattern: ^(?:%{IP:_temp_.cisco.mapped_destination_ip}|%{GREEDYDATA:_temp_.cisco.mapped_destination_host})$
                    if !cached_grok!("^(?:%{IP:_temp_.cisco.mapped_destination_ip}|%{GREEDYDATA:_temp_.cisco.mapped_destination_host})$").extract_into(&input, event)? {
                    }
                }
                Ok(())
            })();

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

            let _cond = { event.has_value("_temp_.cisco.effective_mtu") };
            if _cond {
                if let Some(val) = event.get("_temp_.cisco.effective_mtu") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_temp_.cisco.effective_mtu".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_temp_.cisco.effective_mtu".into(),
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
                                path: "_temp_.cisco.effective_mtu".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("_temp_.cisco.effective_mtu", converted)?;
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

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("private_is_internal"))
                    }
                    serde_json::Value::String(s) => s.contains("private_is_internal"),
                    _ => false,
                }) && event.has_value("source.ip")
                    && event.has_value("destination.ip")
                    && (!event.has_value("_temp_.external_zones")
                        || !event.has_value("_temp_.internal_zones"))
            };
            if _cond {
                // Painless script
                // Source: boolean isPrivateCIDR(def ip) {\n  CIDR class_a_network = new CIDR('10.0.0.0/8');\n  CIDR class_b_network = new CIDR('172.16.0.0/12');\n  CIDR class_c_network = new CIDR('192.168.0.0/16');\n\n  try {\n    return class_a_network.contains(ip) || class_b_network.contains(ip) || class_c_network.contains(ip);\n  } catch (IllegalArgumentException e) {\n    return false;\n  }\n}\ntry {\n  if (ctx.network == null) {\n    Map map = new HashMap();\n    ctx.put('network', map);\n  }\n\n  if (!isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'inbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'outbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'internal';\n  } else if (!isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'external';\n  } else {\n    ctx.network.direction = 'unknown';\n  }\n}\ncatch (Exception e) {\n  ctx.network.direction = null;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"boolean isPrivateCIDR(def ip) {\n  CIDR class_a_network = new CIDR('10.0.0.0/8');\n  CIDR class_b_network = new CIDR('172.16.0.0/12');\n  CIDR class_c_network = new CIDR('192.168.0.0/16');\n\n  try {\n    return class_a_network.contains(ip) || class_b_network.contains(ip) || class_c_network.contains(ip);\n  } catch (IllegalArgumentException e) {\n    return false;\n  }\n}\ntry {\n  if (ctx.network == null) {\n    Map map = new HashMap();\n    ctx.put('network', map);\n  }\n\n  if (!isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'inbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'outbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'internal';\n  } else if (!isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'external';\n  } else {\n    ctx.network.direction = 'unknown';\n  }\n}\ncatch (Exception e) {\n  ctx.network.direction = null;\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "_temp_.url_domain",
                        json!(
                            event
                                .get("url.domain")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url.original", "url", true, false)?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp_.url_domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "url.domain",
                        json!(
                            event
                                .get("_temp_.url_domain")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("_temp_.cisco.message_id", "event.code")?;
                Ok(())
            })();

            let _cond = { event.get_str("_temp_.cisco.message_id") == Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("_temp_.cisco.message_id").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_temp_.cisco.message_id".into(),
                        });
                    }
                    if event.remove("event.code").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "event.code".into(),
                        });
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("dns.question.name") {
                    if let Some(domain_str) = event.get_string("dns.question.name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            event.set(
                                "dns.question.registered_domain",
                                json!(rd.registered_domain),
                            )?;
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("dns.question.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.remove("dns.question.domain");

            let _cond = { event.get_str("_temp_.host.type") == Some("Invalid ID") };
            if _cond {
                if event.remove("_temp_.host.type").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_temp_.host.type".into(),
                    });
                }
            }

            let _cond = { event.has_value("_temp_.host.type") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.host.type") {
                    let re = cached_regex!("Device");
                    let replaced = re.replace_all(&s, " ").into_owned();
                    event.set("_temp_.host.type", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp_.host.type") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.host.type") {
                    let re = cached_regex!("^.*Macintosh-Workstation");
                    let replaced = re.replace_all(&s, "Macintosh:Mac").into_owned();
                    event.set("_temp_.host.type", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp_.host.type") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.host.type") {
                    let re = cached_regex!("^.*Microsoft-Workstation");
                    let replaced = re.replace_all(&s, "Microsoft:Microsoft").into_owned();
                    event.set("_temp_.host.type", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp_.host.type") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.host.type") {
                    let re = cached_regex!("^.*ChromeBook-Workstation");
                    let replaced = re.replace_all(&s, "ChromeBook:ChromeBook").into_owned();
                    event.set("_temp_.host.type", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp_.host.type") };
            if _cond {
                if let Some(s) = event.get_string("_temp_.host.type") {
                    let re = cached_regex!("(?:Workstation|[-_])");
                    let replaced = re.replace_all(&s, " ").into_owned();
                    event.set("_temp_.host.type", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp_.host.type") };
            if _cond {
                if let Some(csv_str) = event.get_string("_temp_.host.type") {
                    let csv_str = csv_close_quote_gap(&csv_str, ':', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b':')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("device.manufacturer", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("host.type", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("device.model.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("host.os.full", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("device.manufacturer") };
            if _cond {
                if let Some(s) = event.get_string("device.manufacturer") {
                    let trimmed = s.trim().to_string();
                    event.set("device.manufacturer", trimmed)?;
                }
            }

            let _cond = { event.has_value("device.model.name") };
            if _cond {
                if let Some(s) = event.get_string("device.model.name") {
                    let trimmed = s.trim().to_string();
                    event.set("device.model.name", trimmed)?;
                }
            }

            let _cond = { event.has_value("host.type") };
            if _cond {
                if let Some(s) = event.get_string("host.type") {
                    let trimmed = s.trim().to_string();
                    event.set("host.type", trimmed)?;
                }
            }

            let _cond = { event.has_value("host.os.full") };
            if _cond {
                if let Some(s) = event.get_string("host.os.full") {
                    let trimmed = s.trim().to_string();
                    event.set("host.os.full", trimmed)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("_temp_.cisco", "cisco.ftd")?;
                Ok(())
            })();

            event.remove("_temp_");

            if event.has("cisco.ftd.list_id") {
                event.rename("cisco.ftd.list_id", "cisco.ftd.rule_name")?;
            }

            // Painless script
            // Source: if (ctx.event?.action == null || !params.containsKey(ctx.event.action)) {\n  return;\n} ctx.event.kind = params.get(ctx.event.action).get('kind'); ctx.event.category = params.get(ctx.event.action).get('category').clone(); ctx.event.type = params.get(ctx.event.action).get('type').clone(); if (ctx.event?.outcome == null) {\n  return;\n} if (ctx.event.category.contains('network') || ctx.event.category.contains('intrusion_detection')) {\n  if (ctx.event.outcome == 'success') {\n    ctx.event.type.add('allowed');\n  }\n  if (ctx.event.outcome == 'failure') {\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'trust') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('allowed');\n  }\n  if (ctx.event.outcome == 'block') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'block with reset') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'domain not found') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'monitor') {\n    ctx.event.outcome = 'success';\n  }\n  if (ctx.event.outcome == 'monitored') {\n    ctx.event.category.add('intrusion_detection');\n    ctx.event.outcome = 'success';\n  }\n  if (ctx.event.outcome == 'pass') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('allowed');\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"if (ctx.event?.action == null || !params.containsKey(ctx.event.action)) {\n  return;\n} ctx.event.kind = params.get(ctx.event.action).get('kind'); ctx.event.category = params.get(ctx.event.action).get('category').clone(); ctx.event.type = params.get(ctx.event.action).get('type').clone(); if (ctx.event?.outcome == null) {\n  return;\n} if (ctx.event.category.contains('network') || ctx.event.category.contains('intrusion_detection')) {\n  if (ctx.event.outcome == 'success') {\n    ctx.event.type.add('allowed');\n  }\n  if (ctx.event.outcome == 'failure') {\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'trust') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('allowed');\n  }\n  if (ctx.event.outcome == 'block') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'block with reset') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'domain not found') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('denied');\n  }\n  if (ctx.event.outcome == 'monitor') {\n    ctx.event.outcome = 'success';\n  }\n  if (ctx.event.outcome == 'monitored') {\n    ctx.event.category.add('intrusion_detection');\n    ctx.event.outcome = 'success';\n  }\n  if (ctx.event.outcome == 'pass') {\n    ctx.event.outcome = 'success';\n    ctx.event.type.add('allowed');\n  }\n}"#
                ),
                cached_params!(
                    "{\"bypass\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"connection-finished\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"connection\",\"end\"]},\"connection-started\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"connection\",\"start\"]},\"creation\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\",\"connection\",\"start\"]},\"deleted\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\",\"end\"]},\"error\":{\"category\":[\"network\"],\"kind\":\"event\",\"outcome\":\"failure\",\"type\":[\"info\",\"end\"]},\"file-detected\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"firewall-rule\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"flow-creation\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"connection\",\"start\"]},\"flow-expiration\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"connection\",\"end\"]},\"intrusion-detected\":{\"category\":[\"intrusion_detection\"],\"kind\":\"alert\",\"type\":[\"info\"]},\"malware-detected\":{\"category\":[\"malware\"],\"kind\":\"event\",\"type\":[\"info\"]}}"
                ),
            )?;

            let _cond = {
                event.get_str("event.code") == Some("430005")
                    && ["Malware", "Custom Detection"].contains(
                        &event
                            .get_str("cisco.ftd.security.sha_disposition")
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
                            .get_str("cisco.ftd.security.sha_disposition")
                            .unwrap_or(""),
                    ))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.get_str("user.name") == Some("Not Found")
                    || event.get_str("user.name") == Some("No Authentication Required")
            };
            if _cond {
                if event.remove("user.id").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "user.id".into(),
                    });
                }
                if event.remove("user.name").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "user.name".into(),
                    });
                }
            }

            let _cond = {
                event.get("user.name").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
                    serde_json::Value::String(s) => s.contains("\\"),
                    _ => false,
                }) && ["430001", "430002", "430003", "430004", "430005", ""]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("user.name") {
                    // Grok pattern: (?:%{DATA}\\\\)?%{GREEDYDATA:user.name}
                    if !cached_grok!("(?:%{DATA}\\\\)?%{GREEDYDATA:user.name}")
                        .extract_into(&input, event)?
                    {}
                }
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

            let _cond = {
                event.get("user").is_some_and(|v| v.is_object()) && event.get("user").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("user").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "user".into(),
                        });
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("cisco.ftd.security_event.access_control_rule_name") };
            if _cond {
                if let Some(v) = event
                    .get("cisco.ftd.security_event.access_control_rule_name")
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            let _cond = { event.has_value("cisco.ftd.security_event.ac_policy") };
            if _cond {
                if let Some(v) = event.get("cisco.ftd.security_event.ac_policy").cloned() {
                    event.set("rule.ruleset", v)?;
                }
            }

            let _cond = {
                !event.has_value("rule.ruleset")
                    && event.has_value("cisco.ftd.security.intrusion_policy")
            };
            if _cond {
                if let Some(v) = event.get("cisco.ftd.security.intrusion_policy").cloned() {
                    event.set("rule.ruleset", v)?;
                }
            }

            let _cond = {
                !event.has_value("rule.ruleset")
                    && event.has_value("cisco.ftd.security_event.file_policy")
            };
            if _cond {
                if let Some(v) = event.get("cisco.ftd.security_event.file_policy").cloned() {
                    event.set("rule.ruleset", v)?;
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

            let v = json!("Cisco");
            if !painless_is_empty_value(&v) {
                event.set("observer.vendor", v)?;
            }

            let v = json!("idps");
            if !painless_is_empty_value(&v) {
                event.set("observer.type", v)?;
            }

            let v = json!("ftd");
            if !painless_is_empty_value(&v) {
                event.set("observer.product", v)?;
            }

            let v = json!(
                event
                    .get("cisco.ftd.destination_interface")
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("observer.egress.interface.name", v)?;
            }

            let v = json!(
                event
                    .get("cisco.ftd.source_interface")
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

            let _cond = { event.has_value("cisco.ftd.security_event.encrypt_peer_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cisco.ftd.security_event.encrypt_peer_ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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
                event.has_value("server.user.name") && event.get_str("server.user.name") != Some("")
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
                event.has_value("source.user.name") && event.get_str("source.user.name") != Some("")
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
                    && event.get_str("destination.user.name") != Some("")
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

            let _cond =
                { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
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
                    && event.get_str("observer.hostname") != Some("")
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
                    && event.get_str("destination.domain") != Some("")
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

            let _cond =
                { event.has_value("source.domain") && event.get_str("source.domain") != Some("") };
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
                    && event.get_str("source.user.domain") != Some("")
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
                    && event.get_str("destination.user.domain") != Some("")
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
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("_temp_.cisco", "cisco.ftd")?;
                    Ok(())
                })();
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
