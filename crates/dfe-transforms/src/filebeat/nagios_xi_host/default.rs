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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.perfdata") {
                    if let Some(input) = event.get_string("json.perfdata") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("rta=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("ms") else {
                                break 'dissect false;
                            };
                            captured.push(("json.rta", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("ms") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("pl=") else {
                                break 'dissect false;
                            };
                            captured.push(("json.temp1", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("pl=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("%") else {
                                break 'dissect false;
                            };
                            captured.push(("json.pl", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("%") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("json.temp2", remaining));
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
                if event.has_value("json.perfdata") {
                    if let Some(input) = event.get_string("json.perfdata") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("time=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("s") else {
                                break 'dissect false;
                            };
                            captured
                                .push(("nagios_xi.host.performance_data.time", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("s") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("size=") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("size=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("B") else {
                                break 'dissect false;
                            };
                            captured
                                .push(("nagios_xi.host.performance_data.size", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("B") else {
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
                event.remove("json.perfdata");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.status_update_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.status_update_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("@timestamp")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("json.status_update_time", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.status_update_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSXXX"], None, None) {
                        Some(parsed) => event.set("json.status_update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.status_update_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_check") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.last_check", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_check".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.next_check") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.next_check", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.next_check".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_up") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.last_time_up", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_up".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_state_change") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.last_state_change", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_state_change".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_hard_state_change") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => {
                            event.set("nagios_xi.host.last_hard_state_change", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_hard_state_change".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_up") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.last_time_up", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_up".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_down") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.last_time_down", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_down".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_unreachable") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => {
                            event.set("nagios_xi.host.last_time_unreachable", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_unreachable".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_notification") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.last_notification", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_notification".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.next_notification") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.next_notification", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.next_notification".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.next_notification") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.host.next_notification", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.next_notification".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.pl") {
                    if let Some(val) = event.get("json.pl") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pl".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.performance_data.pl", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.rta") {
                    if let Some(val) = event.get("json.rta") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.rta".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.performance_data.rta", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.host.performance_data.time") {
                    if let Some(val) = event.get("nagios_xi.host.performance_data.time") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.host.performance_data.time".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.performance_data.time", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.host.performance_data.size") {
                    if let Some(val) = event.get("nagios_xi.host.performance_data.size") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.host.performance_data.size".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.performance_data.size", converted)?;
                    }
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.address") {
                    if let Some(val) = event.get("json.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.address".into(),
                                message,
                            }
                        })?;
                        event.set("json.address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.rename("json.address", "nagios_xi.host.address")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.normal_check_interval") {
                    if let Some(val) = event.get("json.normal_check_interval") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.normal_check_interval".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.normal_check_interval", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.retry_check_interval") {
                    if let Some(val) = event.get("json.retry_check_interval") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.retry_check_interval".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.retry_check_interval", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.latency") {
                    if let Some(val) = event.get("json.latency") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.latency".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.latency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.execution_time") {
                    if let Some(val) = event.get("json.execution_time") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.execution_time".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.host.execution_time", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.status_update_time") {
                    event.rename(
                        "json.status_update_time",
                        "nagios_xi.host.status_update_time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_command") {
                    event.rename("json.check_command", "nagios_xi.host.check_command")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_object_id") {
                    event.rename("json.host_object_id", "nagios_xi.host.host_object_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_name") {
                    event.rename("json.host_name", "nagios_xi.host.host_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_alias") {
                    event.rename("json.host_alias", "nagios_xi.host.host_alias")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.display_name") {
                    event.rename("json.display_name", "nagios_xi.host.display_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.icon_image") {
                    event.rename("json.icon_image", "nagios_xi.host.icon_image")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.icon_image_alt") {
                    event.rename("json.icon_image_alt", "nagios_xi.host.icon_image_alt")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.notes") {
                    event.rename("json.notes", "nagios_xi.host.notes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.notes_url") {
                    event.rename("json.notes_url", "nagios_xi.host.notes_url")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.action_url") {
                    event.rename("json.action_url", "nagios_xi.host.action_url")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.hoststatus_id") {
                    event.rename("json.hoststatus_id", "nagios_xi.host.hoststatus_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.instance_id") {
                    event.rename("json.instance_id", "nagios_xi.host.instance_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.output") {
                    event.rename("json.output", "nagios_xi.host.output")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.long_output") {
                    event.rename("json.long_output", "nagios_xi.host.long_output")?;
                }
                Ok(())
            })();

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("0")),
                    serde_json::Value::String(s) => s.contains("0"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Up/Pending");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.host.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("1")),
                    serde_json::Value::String(s) => s.contains("1"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Warning");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.host.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("2")),
                    serde_json::Value::String(s) => s.contains("2"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Critical");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.host.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("3")),
                    serde_json::Value::String(s) => s.contains("3"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Unknown");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.host.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.has_been_checked") {
                    event.rename("json.has_been_checked", "nagios_xi.host.has_been_checked")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.should_be_scheduled") {
                    event.rename(
                        "json.should_be_scheduled",
                        "nagios_xi.host.should_be_scheduled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.current_check_attempt") {
                    event.rename(
                        "json.current_check_attempt",
                        "nagios_xi.host.current_check_attempt",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.max_check_attempts") {
                    event.rename(
                        "json.max_check_attempts",
                        "nagios_xi.host.max_check_attempts",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_type") {
                    event.rename("json.check_type", "nagios_xi.host.check_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_options") {
                    event.rename("json.check_options", "nagios_xi.host.check_options")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.last_hard_state") {
                    event.rename("json.last_hard_state", "nagios_xi.host.last_hard_state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.state_type") {
                    event.rename("json.state_type", "nagios_xi.host.state_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.no_more_notifications") {
                    event.rename(
                        "json.no_more_notifications",
                        "nagios_xi.host.no_more_notifications",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.notifications_enabled") {
                    event.rename(
                        "json.notifications_enabled",
                        "nagios_xi.host.notifications_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.problem_has_been_acknowledged") {
                    event.rename(
                        "json.problem_has_been_acknowledged",
                        "nagios_xi.host.problem_has_been_acknowledged",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.acknowledgement_type") {
                    event.rename(
                        "json.acknowledgement_type",
                        "nagios_xi.host.acknowledgement_type",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.current_notification_number") {
                    event.rename(
                        "json.current_notification_number",
                        "nagios_xi.host.current_notification_number",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.passive_checks_enabled") {
                    event.rename(
                        "json.passive_checks_enabled",
                        "nagios_xi.host.passive_checks_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.active_checks_enabled") {
                    event.rename(
                        "json.active_checks_enabled",
                        "nagios_xi.host.active_checks_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event_handler_enabled") {
                    event.rename(
                        "json.event_handler_enabled",
                        "nagios_xi.host.event_handler_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.flap_detection_enabled") {
                    event.rename(
                        "json.flap_detection_enabled",
                        "nagios_xi.host.flap_detection_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.is_flapping") {
                    event.rename("json.is_flapping", "nagios_xi.host.is_flapping")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.percent_state_change") {
                    event.rename(
                        "json.percent_state_change",
                        "nagios_xi.host.percent_state_change",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.scheduled_downtime_depth") {
                    event.rename(
                        "json.scheduled_downtime_depth",
                        "nagios_xi.host.scheduled_downtime_depth",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.failure_prediction_enabled") {
                    event.rename(
                        "json.failure_prediction_enabled",
                        "nagios_xi.host.failure_prediction_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.process_performance_data") {
                    event.rename(
                        "json.process_performance_data",
                        "nagios_xi.host.process_performance_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.obsess_over_host") {
                    event.rename("json.obsess_over_host", "nagios_xi.host.obsess_over_host")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.modified_host_attributes") {
                    event.rename(
                        "json.modified_host_attributes",
                        "nagios_xi.host.modified_host_attributes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event_handler") {
                    event.rename("json.event_handler", "nagios_xi.host.event_handler")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_timeperiod_object_id") {
                    event.rename(
                        "json.check_timeperiod_object_id",
                        "nagios_xi.host.check_timeperiod_object_id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("nagios_xi.host");
                if !painless_is_empty_value(&v) {
                    event.set("event.dataset", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("nagios_xi");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json");
                Ok(())
            })();

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
