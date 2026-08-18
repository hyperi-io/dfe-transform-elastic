// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
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

        // Pattern definitions for grok
        // CISCO_PRIORITY_MSGCOUNT = <%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?
        // CISCO_HOSTNAME = [a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?
        // CISCO_TIMESTAMP = [*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: %{CISCO_TZ:_temp_.tz})?
        // CISCO_UPTIME = [0-9a-zA-Z]+
        // CISCO_TZ = [a-zA-Z]{1,4}
        if let Some(input) = event.get_str("event.original").map(String::from) {
            let input = input.as_str();
            // Grok pattern: ^%{CISCO_PRIORITY_MSGCOUNT}?%{SYSLOGTIMESTAMP} %{IP} %{CISCO_HOSTNAME:log.syslog.hostname}: (?:%{NUMBER:cisco.ios.sequence}: )?(?:%{CISCO_UPTIME:cisco.ios.uptime}|%{CISCO_TIMESTAMP}): %{GREEDYDATA:_temp_.message}$
            // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
            let grok_re = regex::Regex::new(&grok_to_regex("^%{CISCO_PRIORITY_MSGCOUNT}?%{SYSLOGTIMESTAMP} %{IP} %{CISCO_HOSTNAME:log.syslog.hostname}: (?:%{NUMBER:cisco.ios.sequence}: )?(?:%{CISCO_UPTIME:cisco.ios.uptime}|%{CISCO_TIMESTAMP}): %{GREEDYDATA:_temp_.message}$")).unwrap();
            if let Some(caps) = grok_re.captures(input) {
                for name in grok_re.capture_names().flatten() {
                    if let Some(m) = caps.name(name) {
                        event.set(name, m.as_str())?;
                    }
                }
            }
            // Additional grok pattern 1: ^%{CISCO_PRIORITY_MSGCOUNT}?%{SYSLOGTIMESTAMP} (?:%{IP}|%{CISCO_HOSTNAME:log.syslog.hostname}) %{NUMBER:cisco.ios.sequence}: (?:%{CISCO_UPTIME:cisco.ios.uptime}|%{CISCO_TIMESTAMP}): %{GREEDYDATA:_temp_.message}$
            // Additional grok pattern 2: ^%{CISCO_PRIORITY_MSGCOUNT}?(?:(?:%{CISCO_HOSTNAME:log.syslog.hostname}|%{IP})[:]? )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:%{CISCO_UPTIME:cisco.ios.uptime}|%{CISCO_TIMESTAMP}): %{GREEDYDATA:_temp_.message}$
        }

        if let Some(input) = event.get_str("_temp_.message").map(String::from) {
            let input = input.as_str();
            // Grok pattern: ^%%{GREEDYDATA:message}$
            // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
            let grok_re = regex::Regex::new(&grok_to_regex("^%%{GREEDYDATA:message}$")).unwrap();
            if let Some(caps) = grok_re.captures(input) {
                for name in grok_re.capture_names().flatten() {
                    if let Some(m) = caps.name(name) {
                        event.set(name, m.as_str())?;
                    }
                }
            }
            // Additional grok pattern 1: ^%{GREEDYDATA:_temp_.generic_message}$
        }

        // TODO: conditional: ctx.cisco?.ios?.sequence != null
        {
            event.set(
                "event.sequence",
                event
                    .get("cisco.ios.sequence")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.cisco?.ios?.message_count != null
        {
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

        // TODO: conditional: ctx.cisco?.ios?.message_count != null && ctx.event?.sequence == null
        {
            event.set(
                "event.sequence",
                event
                    .get("cisco.ios.message_count")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("_temp_.cisco_timestamp") {
            if let Some(s) = event.get_str("_temp_.cisco_timestamp").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new(" {2,}").unwrap();
                let replaced = re.replace_all(s, " ").into_owned();
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

        // TODO: conditional: ctx?._temp_.cisco_timestamp != null
        {
            if let Some(date_str) = event.get_str("_temp_.cisco_timestamp").map(String::from) {
                let date_str = date_str.as_str();
                // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss.SSS z\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d yyyy HH:mm:ss.SSS z\")")
                // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d yyyy HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss z\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d yyyy HH:mm:ss z\")")
                // Try Java datetime format: CustomTime(\"MMM d yyyy HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d yyyy HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS z\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d HH:mm:ss.SSS z\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss z\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d HH:mm:ss z\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(date_str, "CustomTime(\"MMM d HH:mm:ss\")")
            }
        }

        if event.has("message") {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                // Grok pattern: %{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.cisco?.ios?.facility == 'IOSXE' && ctx.event?.code == 'PLATFORM'
        {
            if event.has("message") {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: %%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("%%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.cisco?.ios?.facility == 'FW' && ctx.event?.code == 'SESS_AUDIT_TRAIL'
        {
            if event.has("message") {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: initiator \\(%{IP:source.ip}:%{NUMBER:source.port:long}\\) sent %{NUMBER:source.bytes:long} bytes -- responder \\(%{IP:destination.ip}:%{NUMBER:destination.port:long}\\) sent %{NUMBER:destination.bytes:long} bytes, from %{NOTSPACE:cisco.ios.interface.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("initiator \\(%{IP:source.ip}:%{NUMBER:source.port:long}\\) sent %{NUMBER:source.bytes:long} bytes -- responder \\(%{IP:destination.ip}:%{NUMBER:destination.port:long}\\) sent %{NUMBER:destination.bytes:long} bytes, from %{NOTSPACE:cisco.ios.interface.name}")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.cisco?.ios?.facility == 'FW' && ctx.event?.code == 'DROP_PKT'
        {
            if event.has("message") {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^Dropping %{WORD} %{WORD} from %{NOTSPACE:cisco.ios.interface.name} %{IP:source.ip}:%{NUMBER:source.port:long} ?=> ?%{IP:destination.ip}:%{NUMBER:destination.port:long}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^Dropping %{WORD} %{WORD} from %{NOTSPACE:cisco.ios.interface.name} %{IP:source.ip}:%{NUMBER:source.port:long} ?=> ?%{IP:destination.ip}:%{NUMBER:destination.port:long}")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx._temp_?.generic_message != null
        {
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

        // TODO: conditional: ['IPACCESSLOGP', 'ACCESSLOGP'].contains(ctx.event?.code)
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ['IPACCESSLOGDP', 'ACCESSLOGDP'].contains(ctx.event?.code)
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx.event?.code == 'IPACCESSLOGRP'
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx.event?.code == 'IPACCESSLOGSP'
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx.event?.code == 'ACCESSLOGSP'
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ['IPACCESSLOGNP', 'ACCESSLOGNP'].contains(ctx.event?.code)
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx.event?.code == 'LOGIN_SUCCESS'
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx.event?.code == 'LOGOUT'
        {
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx.event?.code == 'BADAUTH'
        {
            if event.has("message") {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^(?:No|Invalid) MD5 digest from %{DATA:source.address}(\\(%{INT:source.port}\\)|\\:%{INT:source.port}) to %{DATA:destination.address}(\\(%{INT:destination.port}\\)|\\:%{INT:destination.port})(?:(?: \\(RST\\))? (?:tableid - %{DATA:cisco.ios.tableid}|%{GREEDYDATA:_temp_.rst}))?$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^(?:No|Invalid) MD5 digest from %{DATA:source.address}(\\(%{INT:source.port}\\)|\\:%{INT:source.port}) to %{DATA:destination.address}(\\(%{INT:destination.port}\\)|\\:%{INT:destination.port})(?:(?: \\(RST\\))? (?:tableid - %{DATA:cisco.ios.tableid}|%{GREEDYDATA:_temp_.rst}))?$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.event?.code == 'INVALID_RP_JOIN'
        {
            // Pattern definitions for grok
            // PIM_SOURCE = (%{IP:cisco.ios.pim.source.ip}|%{DATA})
            if let Some(input) = event.get_str("message").map(String::from) {
                let input = input.as_str();
                // Grok pattern: Received \\(%{PIM_SOURCE}, %{DATA:cisco.ios.pim.group.ip}\\) %{WORD:cisco.ios.action} from %{IP:source.address} for %{DATA:cisco.ios.outcome} %{IP:destination.address}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("Received \\(%{PIM_SOURCE}, %{DATA:cisco.ios.pim.group.ip}\\) %{WORD:cisco.ios.action} from %{IP:source.address} for %{DATA:cisco.ios.outcome} %{IP:destination.address}")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.event?.code == "INVALID_RP_JOIN"
        {
            event.set("event.action", json!("multicast-join"))?;
        }

        // TODO: conditional: ctx.event?.code == "INVALID_RP_JOIN"
        {
            event.set("event.outcome", json!("failure"))?;
        }

        // TODO: conditional: ctx.event?.code == "INVALID_RP_JOIN"
        {
            event.set("event.reason", json!("Invalid RP"))?;
        }

        if event.has("destination.address") {
            if let Some(s) = event.get_str("destination.address").map(String::from) {
                let s = s.as_str();
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

        if event.has("source.address") {
            if let Some(s) = event.get_str("source.address").map(String::from) {
                let s = s.as_str();
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

        if event.has("cisco.ios.pim.source.ip") {
            if let Some(s) = event.get_str("cisco.ios.pim.source.ip").map(String::from) {
                let s = s.as_str();
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

        // TODO: conditional: ctx.source?.bytes != null || ctx.destination?.bytes != null
        {
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

        // TODO: conditional: ctx.source?.packets != null
        {
            event.set(
                "network.packets",
                event.get("source.packets").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.source?.ip != null && ctx.source?.ip.contains('.')
        {
            event.set("network.type", json!("ipv4"))?;
        }

        // TODO: conditional: ctx.source?.ip != null && ctx.network?.type == null
        {
            event.set("network.type", json!("ipv6"))?;
        }

        // TODO: conditional: ctx._temp_?.event?.action == 'denied'
        {
            event.set("event.action", json!("deny"))?;
        }

        // TODO: conditional: ctx.event?.action == 'deny'
        {
            event.append("event.type", json!("denied"))?;
        }

        // TODO: conditional: ctx._temp_?.event?.action == 'permitted'
        {
            event.set("event.action", json!("allow"))?;
        }

        // TODO: conditional: ctx.event?.action == 'allow'
        {
            event.append("event.type", json!("allowed"))?;
        }

        // TODO: conditional: ctx.event.severity == 0
        {
            event.set("log.level", json!("emergencies"))?;
        }

        // TODO: conditional: ctx.event.severity == 1
        {
            event.set("log.level", json!("alert"))?;
        }

        // TODO: conditional: ctx.event.severity == 2
        {
            event.set("log.level", json!("critical"))?;
        }

        // TODO: conditional: ctx.event.severity == 3
        {
            event.set("log.level", json!("error"))?;
        }

        // TODO: conditional: ctx.event.severity == 4
        {
            event.set("log.level", json!("warning"))?;
        }

        // TODO: conditional: ctx.event.severity == 5
        {
            event.set("log.level", json!("notification"))?;
        }

        // TODO: conditional: ctx.event.severity == 6
        {
            event.set("log.level", json!("informational"))?;
        }

        // TODO: conditional: ctx.event.severity == 7
        {
            event.set("log.level", json!("debug"))?;
        }

        if event.has("source.ip") {
            if let Some(ip_str) = event.get_str("source.ip").map(String::from) {
                let ip_str = ip_str.as_str();
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
            if let Some(ip_str) = event.get_str("destination.ip").map(String::from) {
                let ip_str = ip_str.as_str();
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
            if let Some(ip_str) = event.get_str("source.ip").map(String::from) {
                let ip_str = ip_str.as_str();
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
            if let Some(ip_str) = event.get_str("destination.ip").map(String::from) {
                let ip_str = ip_str.as_str();
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

        // TODO: conditional: ctx.source?.ip != null
        {
            event.append(
                "related.ip",
                event.get("source.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.destination?.ip != null
        {
            event.append(
                "related.ip",
                event.get("destination.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.source?.domain != null
        {
            event.append(
                "related.hosts",
                event.get("source.domain").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.destination?.domain != null
        {
            event.append(
                "related.hosts",
                event
                    .get("destination.domain")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.source?.user?.name != null
        {
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
                if let (Some(src_ip), Some(dst_ip)) =
                    (event.get_str("source.ip"), event.get_str("destination.ip"))
                {
                    let src_ip = src_ip.to_string();
                    let dst_ip = dst_ip.to_string();
                    let src_port = event.get_i64("source.port").unwrap_or(0) as u16;
                    let dst_port = event.get_i64("destination.port").unwrap_or(0) as u16;
                    let protocol = event
                        .get_str("network.transport")
                        .or_else(|| event.get_str("network.iana_number"))
                        .unwrap_or("tcp")
                        .to_string();
                    let cid = community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol);
                    event.set("network.community_id", json!(cid))?;
                }
            }
            Ok(())
        })();

        event.remove("_temp_");
        event.remove("_conf");

        // TODO: conditional: ctx.tags == null || !(ctx.tags.contains('preserve_original_event'))
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.original");
                Ok(())
            })();
        }

        Ok(TransformResult::Continue)
    }
}
