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

            event.set("event.category", json!(["network"]))?;

            event.set("event.provider", json!("firewall"))?;

            event.set("observer.vendor", json!("Cisco"))?;

            event.set("observer.product", json!("IOS"))?;

            event.set("observer.type", json!("firewall"))?;

            event.set("event.type", json!(["info"]))?;

            if !event.has("event.original") {
                event.set(
                    "event.original",
                    event.get("message").cloned().unwrap_or(Value::Null),
                )?;
            }

            event.remove("message");

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} %{IP} (?P<log_syslog_hostname>(?:[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?)): (?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:[0-9a-zA-Z]+))|(?:[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: (?P<_temp__tz>(?:[a-zA-Z]{1,4})))?)): %{GREEDYDATA:_temp_.message}$
                if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} %{IP} (?P<log_syslog_hostname>(?:[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?)): (?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:[0-9a-zA-Z]+))|(?:[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: (?P<_temp__tz>(?:[a-zA-Z]{1,4})))?)): %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                        // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} (?:%{IP}|(?P<log_syslog_hostname>(?:[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?))) %{NUMBER:cisco.ios.sequence}: (?:(?P<cisco_ios_uptime>(?:[0-9a-zA-Z]+))|(?:[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: (?P<_temp__tz>(?:[a-zA-Z]{1,4})))?)): %{GREEDYDATA:_temp_.message}$
                        if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} (?:%{IP}|(?P<log_syslog_hostname>(?:[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?))) %{NUMBER:cisco.ios.sequence}: (?:(?P<cisco_ios_uptime>(?:[0-9a-zA-Z]+))|(?:[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: (?P<_temp__tz>(?:[a-zA-Z]{1,4})))?)): %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                            // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:(?:(?P<log_syslog_hostname>(?:[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?))|%{IP})[:]? )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:[0-9a-zA-Z]+))|(?:[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: (?P<_temp__tz>(?:[a-zA-Z]{1,4})))?)): %{GREEDYDATA:_temp_.message}$
                            if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:(?:(?P<log_syslog_hostname>(?:[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?))|%{IP})[:]? )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:[0-9a-zA-Z]+))|(?:[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: (?P<_temp__tz>(?:[a-zA-Z]{1,4})))?)): %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                            }
                        }
                    }
            }

            if let Some(input) = event.get_string("_temp_.message") {
                // Grok pattern: ^%%{GREEDYDATA:message}$
                if !cached_grok!("^%%{GREEDYDATA:message}$").extract_into(&input, event)? {
                    // Grok pattern: ^%{GREEDYDATA:_temp_.generic_message}$
                    if !cached_grok!("^%{GREEDYDATA:_temp_.generic_message}$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = { event.has("cisco.ios.sequence") };
            if _cond {
                event.set(
                    "event.sequence",
                    event
                        .get("cisco.ios.sequence")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("cisco.ios.message_count") };
            if _cond {
                if let Some(val) = event.get("cisco.ios.message_count") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "cisco.ios.message_count".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "cisco.ios.message_count".into(),
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
                                path: "cisco.ios.message_count".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("cisco.ios.message_count", converted)?;
                }
            }

            let _cond = { event.has("cisco.ios.message_count") && !event.has("event.sequence") };
            if _cond {
                event.set(
                    "event.sequence",
                    event
                        .get("cisco.ios.message_count")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            if event.has("_temp_.cisco_timestamp") {
                if let Some(s) = event.get_string("_temp_.cisco_timestamp") {
                    let re = cached_regex!(" {2,}");
                    let replaced = re.replace_all(&s, " ").into_owned();
                    event.set("_temp_.cisco_timestamp", replaced)?;
                }
            }

            // Painless script
            // Source: if (ctx._temp_?.tz != null && ctx._conf?.tz_map != null) {\n  for (def item : ctx._conf.tz_map) {\n    if (item.tz_short == ctx._temp_.tz) {\n      ctx.event.timezone = item.tz_long;\n      return;\n    }\n  }\n}\nif (ctx._conf?.tz_offset != null) {\n  ctx.event.timezone = ctx._conf.tz_offset;\n}\nif (ctx.event?.timezone == null) {\n  ctx.event.timezone = 'UTC';\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"if (ctx._temp_?.tz != null && ctx._conf?.tz_map != null) {\n  for (def item : ctx._conf.tz_map) {\n    if (item.tz_short == ctx._temp_.tz) {\n      ctx.event.timezone = item.tz_long;\n      return;\n    }\n  }\n}\nif (ctx._conf?.tz_offset != null) {\n  ctx.event.timezone = ctx._conf.tz_offset;\n}\nif (ctx.event?.timezone == null) {\n  ctx.event.timezone = 'UTC';\n}"#,
            )?;

            let _cond = { event.has("_temp_.cisco_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_temp_.cisco_timestamp") {
                    // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss.SSS z\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d yyyy HH:mm:ss.SSS z\")")
                    // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss.SSS\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d yyyy HH:mm:ss.SSS\")")
                    // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss z\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d yyyy HH:mm:ss z\")")
                    // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d yyyy HH:mm:ss\")")
                    // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS z\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS z\")")
                    // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
                    // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss z\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss z\")")
                    // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                    // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                }
            }

            if event.has("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                    if !cached_grok!("%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = {
                event.get_str("cisco.ios.facility") == Some("IOSXE")
                    && event.get_str("event.code") == Some("PLATFORM")
            };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: %%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                        if !cached_grok!("%%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = {
                event.get_str("cisco.ios.facility") == Some("FW")
                    && event.get_str("event.code") == Some("SESS_AUDIT_TRAIL")
            };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: initiator \\(%{IP:source.ip}:%{NUMBER:source.port:long}\\) sent %{NUMBER:source.bytes:long} bytes -- responder \\(%{IP:destination.ip}:%{NUMBER:destination.port:long}\\) sent %{NUMBER:destination.bytes:long} bytes, from %{NOTSPACE:cisco.ios.interface.name}
                        if !cached_grok!("initiator \\(%{IP:source.ip}:%{NUMBER:source.port:long}\\) sent %{NUMBER:source.bytes:long} bytes -- responder \\(%{IP:destination.ip}:%{NUMBER:destination.port:long}\\) sent %{NUMBER:destination.bytes:long} bytes, from %{NOTSPACE:cisco.ios.interface.name}").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = {
                event.get_str("cisco.ios.facility") == Some("FW")
                    && event.get_str("event.code") == Some("DROP_PKT")
            };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Dropping %{WORD} %{WORD} from %{NOTSPACE:cisco.ios.interface.name} %{IP:source.ip}:%{NUMBER:source.port:long} ?=> ?%{IP:destination.ip}:%{NUMBER:destination.port:long}
                        if !cached_grok!("^Dropping %{WORD} %{WORD} from %{NOTSPACE:cisco.ios.interface.name} %{IP:source.ip}:%{NUMBER:source.port:long} ?=> ?%{IP:destination.ip}:%{NUMBER:destination.port:long}").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = { event.has("_temp_.generic_message") };
            if _cond {
                event.rename("_temp_.generic_message", "message")?;
            }

            if event.has("event.severity") {
                if let Some(val) = event.get("event.severity") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "event.severity".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "event.severity".into(),
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
                                path: "event.severity".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("event.severity", converted)?;
                }
            }

            if event.has("event.sequence") {
                if let Some(val) = event.get("event.sequence") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "event.sequence".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "event.sequence".into(),
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
                                path: "event.sequence".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("event.sequence", converted)?;
                }
            }

            let _cond = {
                ["IPACCESSLOGP", "ACCESSLOGP"].contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("list ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.access_list", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("network.transport", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("(") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("(") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(") ") {
                        event.set("source.port", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(") ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("(") {
                        event.set("destination.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("(") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("), ") {
                        event.set("destination.port", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("), ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" packet") {
                        event.set("source.packets", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" packet") {
                        remaining = rest;
                    }
                }
            }

            let _cond = {
                ["IPACCESSLOGDP", "ACCESSLOGDP"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("list ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.access_list", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("network.transport", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" (") {
                        event.set("destination.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" (") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("/") {
                        event.set("icmp.type", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("/") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("), ") {
                        event.set("icmp.code", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("), ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" packet") {
                        event.set("source.packets", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" packet") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("IPACCESSLOGRP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("list ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.access_list", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("network.transport", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(", ") {
                        event.set("destination.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(", ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" packet") {
                        event.set("source.packets", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" packet") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("IPACCESSLOGSP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("list ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.access_list", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("network.transport", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" (") {
                        event.set("destination.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" (") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("), ") {
                        event.set("igmp.type", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("), ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" packet") {
                        event.set("source.packets", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" packet") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("ACCESSLOGSP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("list ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.access_list", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("network.type", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(", ") {
                        event.set("destination.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(", ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" packet") {
                        event.set("source.packets", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" packet") {
                        remaining = rest;
                    }
                }
            }

            let _cond = {
                ["IPACCESSLOGNP", "ACCESSLOGNP"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("list ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.access_list", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("network.iana_number", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(", ") {
                        event.set("destination.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(", ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" packet") {
                        event.set("source.packets", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" packet") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("LOGIN_SUCCESS") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" [user: ") {
                        event.set("_temp_.event.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" [user: ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("] [Source: ") {
                        event.set("source.user.name", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("] [Source: ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("] [localport: ") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("] [localport: ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("] at ") {
                        event.set("destination.port", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("] at ") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("LOGOUT") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("User ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" has ") {
                        event.set("source.user.name", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" has ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("cisco.ios.action", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" session ") {
                        event.set("cisco.ios.session.type", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" session ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("(") {
                        event.set("cisco.ios.session.number", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("(") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(")") {
                        event.set("source.address", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(")") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("BADAUTH") };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^(?:No|Invalid) MD5 digest from %{DATA:source.address}(\\(%{INT:source.port}\\)|\\:%{INT:source.port}) to %{DATA:destination.address}(\\(%{INT:destination.port}\\)|\\:%{INT:destination.port})(?:(?: \\(RST\\))? (?:tableid - %{DATA:cisco.ios.tableid}|%{GREEDYDATA:_temp_.rst}))?$
                        if !cached_grok!("^(?:No|Invalid) MD5 digest from %{DATA:source.address}(\\(%{INT:source.port}\\)|\\:%{INT:source.port}) to %{DATA:destination.address}(\\(%{INT:destination.port}\\)|\\:%{INT:destination.port})(?:(?: \\(RST\\))? (?:tableid - %{DATA:cisco.ios.tableid}|%{GREEDYDATA:_temp_.rst}))?$").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Received \\((?:(%{IP:cisco.ios.pim.source.ip}|%{DATA})), %{DATA:cisco.ios.pim.group.ip}\\) %{WORD:cisco.ios.action} from %{IP:source.address} for %{DATA:cisco.ios.outcome} %{IP:destination.address}
                    if !cached_grok!("Received \\((?:(%{IP:cisco.ios.pim.source.ip}|%{DATA})), %{DATA:cisco.ios.pim.group.ip}\\) %{WORD:cisco.ios.action} from %{IP:source.address} for %{DATA:cisco.ios.outcome} %{IP:destination.address}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                event.set("event.action", json!("multicast-join"))?;
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                event.set("event.reason", json!("Invalid RP"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("destination.address") {
                    if let Some(s) = event.get_string("destination.address") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "destination.address".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("destination.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip")?;
                event.set(
                    "destination.domain",
                    event
                        .get("destination.address")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("source.address") {
                    if let Some(s) = event.get_string("source.address") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "source.address".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip")?;
                event.set(
                    "source.domain",
                    event.get("source.address").cloned().unwrap_or(Value::Null),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
            }

            if event.has("cisco.ios.pim.source.ip") {
                if let Some(s) = event.get_string("cisco.ios.pim.source.ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "cisco.ios.pim.source.ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("cisco.ios.pim.source.ip", s)?;
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

            let _cond = { event.has("source.bytes") || event.has("destination.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: long n = 0;\nif (ctx.source?.bytes != null) {\n  n += ctx.source.bytes\n}\nif (ctx.destination?.bytes != null) {\n  n += ctx.destination.bytes\n}\nif (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nctx.network.bytes = n;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        r#"long n = 0;\nif (ctx.source?.bytes != null) {\n  n += ctx.source.bytes\n}\nif (ctx.destination?.bytes != null) {\n  n += ctx.destination.bytes\n}\nif (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nctx.network.bytes = n;\n"#,
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has("source.packets") };
            if _cond {
                event.set(
                    "network.packets",
                    event.get("source.packets").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = { event.has("source.ip") && !event.has("network.type") };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { event.get_str("_temp_.event.action") == Some("denied") };
            if _cond {
                event.set("event.action", json!("deny"))?;
            }

            let _cond = { event.get_str("event.action") == Some("deny") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("_temp_.event.action") == Some("permitted") };
            if _cond {
                event.set("event.action", json!("allow"))?;
            }

            let _cond = { event.get_str("event.action") == Some("allow") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(0) };
            if _cond {
                event.set("log.level", json!("emergencies"))?;
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

            let _cond = { event.has("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("source.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("destination.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("destination.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("source.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("source.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("destination.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event
                        .get("destination.domain")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("source.user.name") };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("source.user.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("source.ip") {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                    ) {
                        let src_port = event.get_i64("source.port").unwrap_or(0) as u16;
                        let dst_port = event.get_i64("destination.port").unwrap_or(0) as u16;
                        let protocol = event
                            .get_string("network.transport")
                            .or_else(|| event.get_string("network.iana_number"))
                            .unwrap_or_else(|| "tcp".to_string());
                        let cid = community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol);
                        event.set("network.community_id", json!(cid))?;
                    }
                }
                Ok(())
            })();

            event.remove("_temp_");
            event.remove("_conf");

            let _cond = {
                !event.has("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("event.original");
                    Ok(())
                })();
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.remove("_temp_");
                event.remove("_conf");
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
