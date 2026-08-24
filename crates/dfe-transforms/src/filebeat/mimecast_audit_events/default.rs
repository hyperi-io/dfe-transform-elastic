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

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

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

            parse_json_field(event, "event.original", "mimecast")?;

            let _cond = {
                !event.has_value("mimecast.eventTime")
                    || (event.has_value("mimecast.data")
                        && event.get("mimecast.data").is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        }))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("mimecast.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            if let Some(date_str) = event.get_as_string("mimecast.eventTime") {
                match parse_date_out(
                    &date_str,
                    &[
                        "yyyy-MM-dd'T'HH:mm:ssz",
                        "yyyy-MM-dd'T'HH:mm:ssZ",
                        "yyyy-MM-dd'T'HH:mm:ss.Sz",
                        "yyyy-MM-dd'T'HH:mm:ss.SZ",
                        "yyyy-MM-dd'T'HH:mm:ss.SSz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSZ",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSZ",
                        "yyyy-MM-dd'T'HH:mm:ss z",
                    ],
                    Some("UTC"),
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "mimecast.eventTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("mimecast.auditType") {
                map_strings(
                    event,
                    "mimecast.auditType",
                    "mimecast.auditType",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("mimecast.auditType") {
                gsub_field(
                    event,
                    "mimecast.auditType",
                    "mimecast.auditType",
                    cached_regex!(" "),
                    "-",
                )?;
            }

            if event.has_value("mimecast.auditType") {
                event.rename("mimecast.auditType", "event.action")?;
            }

            if event.has_value("mimecast.user") {
                event.rename("mimecast.user", "user.email")?;
            }

            if event.has_value("mimecast.id") {
                event.rename("mimecast.id", "event.id")?;
            }

            let _cond = { event.get_str("event.action") == Some("logon-authentication-failed") };
            if _cond {
                if event.has_value("mimecast.eventInfo") {
                    if let Some(input) = event.get_string("mimecast.eventInfo") {
                        // Grok pattern: ^%{GREEDYDATA:mimecast.info},\\sDate:\\s(?P<mimecast_date>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY})),\\sTime:\\s%{TIME:mimecast.time} (?P<mimecast_timezone>(?:(?:[A-Z]{3,4}|(?:GMT)?[-+][0-9]{2}:?[0-9]{2}))),\\sIP:\\s%{IP:client.ip},\\sApplication:\\s%{NOTSPACE:mimecast.application},(?:\\sMethod:\\s%{DATA:mimecast.method},)?\\sReason:\\s%{DATA:event.reason}$
                        // Grok pattern: ^%{GREEDYDATA:mimecast.info},\\sDate:\\s(?P<mimecast_date>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY})),\\sTime:\\s%{TIME:mimecast.time}(?P<mimecast_timezone>(?:(?:[A-Z]{3,4}|(?:GMT)?[-+][0-9]{2}:?[0-9]{2}))),\\sIP:\\s%{IP:client.ip},\\sApplication:\\s%{NOTSPACE:mimecast.application},\\sRemote IP is %{IP:mimecast.remote_ip}$
                        // Grok pattern: ^%{GREEDYDATA:mimecast.info},\\s%{WORD} ?: ?%{DATA:mimecast.email.address}\\[%{DATA:mimecast.email.metadata}\\] remote IP ?: ?%{IP:mimecast.remote_ip} application ?: ?%{NOTSPACE:mimecast.application}$
                        // Grok pattern: ^%{GREEDYDATA:mimecast.info},\\sRemote IP is %{IP:mimecast.remote_ip}$
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^%{GREEDYDATA:mimecast.info},\\sDate:\\s(?P<mimecast_date>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY})),\\sTime:\\s%{TIME:mimecast.time} (?P<mimecast_timezone>(?:(?:[A-Z]{3,4}|(?:GMT)?[-+][0-9]{2}:?[0-9]{2}))),\\sIP:\\s%{IP:client.ip},\\sApplication:\\s%{NOTSPACE:mimecast.application},(?:\\sMethod:\\s%{DATA:mimecast.method},)?\\sReason:\\s%{DATA:event.reason}$",
                                    [
                                        ("mimecast_date", "mimecast.date"),
                                        ("mimecast_timezone", "mimecast.timezone")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^%{GREEDYDATA:mimecast.info},\\sDate:\\s(?P<mimecast_date>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY})),\\sTime:\\s%{TIME:mimecast.time}(?P<mimecast_timezone>(?:(?:[A-Z]{3,4}|(?:GMT)?[-+][0-9]{2}:?[0-9]{2}))),\\sIP:\\s%{IP:client.ip},\\sApplication:\\s%{NOTSPACE:mimecast.application},\\sRemote IP is %{IP:mimecast.remote_ip}$",
                                    [
                                        ("mimecast_date", "mimecast.date"),
                                        ("mimecast_timezone", "mimecast.timezone")
                                    ]
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:mimecast.info},\\s%{WORD} ?: ?%{DATA:mimecast.email.address}\\[%{DATA:mimecast.email.metadata}\\] remote IP ?: ?%{IP:mimecast.remote_ip} application ?: ?%{NOTSPACE:mimecast.application}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:mimecast.info},\\sRemote IP is %{IP:mimecast.remote_ip}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = { !event.has_value("mimecast.info") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.eventInfo") {
                        if let Some(input) = event.get_string("mimecast.eventInfo") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(">, ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.info", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(">, ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("mimecast.rest_of_event_info", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("mimecast.info") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.eventInfo") {
                        if let Some(input) = event.get_string("mimecast.eventInfo") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(", ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.info", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("mimecast.rest_of_event_info", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.action") == Some("folder-log-entry")
                    || event.get_str("event.action") == Some("custom-report-definition-created")
                    || event.get_str("event.action") == Some("mimecast-support-login")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.eventInfo") {
                        if let Some(input) = event.get_string("mimecast.eventInfo") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(" - ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" - ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("<") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.info", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("<") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("> ") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.email", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("> ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.date", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.time", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.timezone", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("client.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("mimecast.application", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mimecast.rest_of_event_info") {
                    if let Some(kv_str) = event.get_string("mimecast.rest_of_event_info") {
                        for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(": ") else {
                                return Err(TransformError::ParseError {
                                    path: "mimecast.rest_of_event_info".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("mimecast.event_info_parts.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.get_str("event.action") == Some("user-logged-on")
                    && !event.has_value("mimecast.event_info_parts.IP")
            };
            if _cond {
                event.set(
                    "mimecast.remote",
                    json!(
                        event
                            .get("mimecast.rest_of_event_info")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mimecast.remote") {
                    if let Some(input) = event.get_string("mimecast.remote") {
                        // Grok pattern: %{IP:mimecast.remote_ip}
                        let _ =
                            cached_grok!("%{IP:mimecast.remote_ip}").extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("mimecast.event_info_parts.Date") {
                event.rename("mimecast.event_info_parts.Date", "mimecast.date")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mimecast.event_info_parts.Time") {
                    if let Some(input) = event.get_string("mimecast.event_info_parts.Time") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("mimecast.event_info_parts.Time", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("mimecast.timezone", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("mimecast.event_info_parts.Time") {
                event.rename("mimecast.event_info_parts.Time", "mimecast.time")?;
            }

            if event.has_value("mimecast.event_info_parts.IP") {
                event.rename("mimecast.event_info_parts.IP", "client.ip")?;
            }

            if event.has_value("mimecast.event_info_parts.Application") {
                event.rename(
                    "mimecast.event_info_parts.Application",
                    "mimecast.application",
                )?;
            }

            if event.has_value("mimecast.event_info_parts.Method") {
                event.rename("mimecast.event_info_parts.Method", "mimecast.method")?;
            }

            if event.has_value("mimecast.event_info_parts.Reason") {
                event.rename("mimecast.event_info_parts.Reason", "event.reason")?;
            }

            let _cond = { event.get_str("event.action") == Some("threat-intel-feed-download") };
            if _cond {
                if event.has_value("mimecast.info") {
                    event.rename("mimecast.info", "mimecast.filename")?;
                }
            }

            if event.has_value("mimecast.event_info_parts.Processed") {
                event.rename(
                    "mimecast.event_info_parts.Processed",
                    "email.origination_timestamp",
                )?;
            }

            if event.has_value("mimecast.event_info_parts.Subject") {
                event.rename("mimecast.event_info_parts.Subject", "email.subject")?;
            }

            if event.has_value("mimecast.event_info_parts.2FA") {
                event.rename("mimecast.event_info_parts.2FA", "mimecast.2FA")?;
            }

            let _cond = { event.get_str("event.action") == Some("message-action") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.event_info_parts.From") {
                        if let Some(input) = event.get_string("mimecast.event_info_parts.From") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("<") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("> ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("> ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("email.from.address", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.action") == Some("message-action") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.event_info_parts.To") {
                        if let Some(input) = event.get_string("mimecast.event_info_parts.To") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("<") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("> ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("> ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("email.to.address", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.action") == Some("page-data-exports") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.eventInfo") {
                        if let Some(input) = event.get_string("mimecast.eventInfo") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("[") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" : ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" : ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.export_type", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" :") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" :") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.export_name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" :") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" :") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.email", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" :") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" :") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.weekday", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.month", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.monthday", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.time", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.timezone", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.year", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" :") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" :") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("client.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" :") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" :") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.columns_exported", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" : ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" : ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("file.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(",") else {
                                    break 'dissect false;
                                };
                                captured.push(("file.size", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(",") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" : ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" : ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("], ") else {
                                    break 'dissect false;
                                };
                                captured.push(("file.extension", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("], ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.date", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", ") else {
                                    break 'dissect false;
                                };
                                captured.push(("mimecast.time", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", ") else {
                                    break 'dissect false;
                                };
                                captured.push(("client.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("mimecast.application", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.action") == Some("user-logged-on")
                    && !event.has_value("mimecast.event_info_parts.IP")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("mimecast.rest_of_event_info") {
                        if let Some(input) = event.get_string("mimecast.rest_of_event_info") {
                            // Grok pattern: %{IP:client.ip}
                            let _ = cached_grok!("%{IP:client.ip}").extract_into(&input, event)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("email.from.address") };
            if _cond {
                event.set(
                    "email.from.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.from.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = { event.has_value("email.to.address") };
            if _cond {
                event.set(
                    "email.to.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.to.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.from.address") {
                    if let Some(input) = event.get_string("email.from.address") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("<") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(">") else {
                                break 'dissect false;
                            };
                            captured.push(("email.from.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(">") else {
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
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.to.address") {
                    if let Some(input) = event.get_string("email.to.address") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("<") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(">") else {
                                break 'dissect false;
                            };
                            captured.push(("email.to.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(">") else {
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
                }
                Ok(())
            })();

            if event.has_value("file.size") {
                if let Some(val) = event.get("file.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "file.size".into(),
                            message,
                        }
                    })?;
                    event.set("file.size", converted)?;
                }
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                if let Some(s) = event.get_string("user.email") {
                    let parts: Vec<Value> = s.split("@").map(|p| json!(p)).collect();
                    event.set("user.parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("user.parts") && event.get("user.parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                if let Some(v) = event.get("user.parts.0").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.has_value("user.parts") && event.get("user.parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                if let Some(v) = event.get("user.parts.1").cloned() {
                    event.set("user.domain", v)?;
                }
            }

            let _cond = {
                event.has_value("mimecast.filename")
                    && event.get_str("event.action") == Some("threat-intel-feed-download")
            };
            if _cond {
                if event.has_value("mimecast.filename") {
                    event.rename("mimecast.filename", "file.name")?;
                }
            }

            let _cond = {
                event.has_value("file.name")
                    && event.get_str("event.action") == Some("threat-intel-feed-download")
            };
            if _cond {
                if let Some(s) = event.get_string("file.name") {
                    let parts: Vec<Value> = cached_regex!("\\.")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("file.parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("file.parts") && event.get("file.parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                // Painless script
                // Source: ctx.file.extension = ctx.file.parts[ctx.file.parts.length-1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.file.extension = ctx.file.parts[ctx.file.parts.length-1];\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("mimecast.date") && event.has_value("mimecast.time") };
            if _cond {
                event.set(
                    "event.created",
                    json!(format!(
                        "{} {}",
                        event
                            .get("mimecast.date")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("mimecast.time")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("event.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("event.created") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ssZ",
                            "yyyy-MM-dd HH:mm:ss z",
                            "yyyy-MM-dd HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.Sz",
                            "yyyy-MM-dd'T'HH:mm:ss.SZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSZ",
                            "yyyy-MM-dd'T'HH:mm:ss z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "event.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mimecast.remote_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("mimecast.remote_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
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

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("email.direction") {
                map_strings(
                    event,
                    "email.direction",
                    "email.direction",
                    str::to_lowercase,
                )?;
            }

            event.remove("mimecast.eventTime");
            event.remove("user.parts");
            event.remove("mimecast.date");
            event.remove("mimecast.time");
            event.remove("file.parts");
            event.remove("mimecast.info");
            event.remove("mimecast.type");
            event.remove("mimecast.search");
            event.remove("mimecast.description");
            event.remove("mimecast.product");
            event.remove("mimecast.provider");
            event.remove("mimecast.filename");
            event.remove("mimecast.criteria");
            event.remove("mimecast.viewed");
            event.remove("mimecast.byuser");
            event.remove("mimecast.export_type");
            event.remove("mimecast.export_name");
            event.remove("mimecast.weekday");
            event.remove("mimecast.month");
            event.remove("mimecast.monthday");
            event.remove("mimecast.year");
            event.remove("mimecast.columns_exported");
            event.remove("mimecast.as.asn");
            event.remove("mimecast.organization_name");
            event.remove("mimecast.event_info_parts");
            event.remove("mimecast.rest_of_event_info");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
