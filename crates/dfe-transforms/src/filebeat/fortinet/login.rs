// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `login` pipeline.
pub struct Login;

impl Transform for Login {
    fn name(&self) -> &str {
        "login"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            if !event.has("event.action") {
                event.set("event.action", json!("login"))?;
            }

            event.append("event.category", json!("authentication"))?;

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.set(
                    "user.name",
                    event
                        .get("source.user.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("fortinet.firewall.adminprof") };
            if _cond {
                event.append(
                    "user.roles",
                    event
                        .get("fortinet.firewall.adminprof")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("fortinet.firewall.adminprof") };
            if _cond {
                event.append(
                    "source.user.roles",
                    event
                        .get("fortinet.firewall.adminprof")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("fortinet.firewall.userfrom")
                    && event
                        .get_str("fortinet.firewall.userfrom")
                        .is_some_and(|s| s.starts_with("JSON("))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("fortinet.firewall.userfrom") {
                        let mut remaining: &str = &input;
                        if let Some(rest) = remaining.strip_prefix("JSON(") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(")") {
                            event.set("source.ip", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(")") {
                            remaining = rest;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "userfrom")?;
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
                }
            }

            let _cond = {
                event.has_value("message")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("user"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("fortinet.firewall.desc") {
                        let mut remaining: &str = &input;
                        if let Some(rest) = remaining.strip_prefix("User login/logout ") {
                            remaining = rest;
                        }
                        event.set("event.outcome", remaining)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "event outcome")?;
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
                }
            }

            let _cond = {
                event.has_value("message")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("Login from ssh:"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        if let Some(rest) = remaining.strip_prefix("Login from ssh: ") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(" for ") {
                            event.set("event.outcome", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" for ") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(" from ") {
                            event.set("user.name", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" from ") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(" port ") {
                            event.set("source.ip", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" port ") {
                            remaining = rest;
                        }
                        event.set("source.port", remaining)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "ssh login 1")?;
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
                }
            }

            let _cond = {
                event.has_value("message")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("Administrator"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" ") {
                            event.set("_tmp.user.roles", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" ") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(" login ") {
                            event.set("user.name", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" login ") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(" from ") {
                            event.set("event.outcome", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" from ") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find("(") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix("(") {
                            remaining = rest;
                        }
                        if let Some(pos) = remaining.find(") ") {
                            event.set("source.ip", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(") ") {
                            remaining = rest;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "ssh login 2")?;
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
                }
            }

            let _cond = {
                event.has_value("event.outcome")
                    && event
                        .get_str("event.outcome")
                        .is_some_and(|s| s.to_lowercase().starts_with("fail"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("event.outcome")
                    && event
                        .get_str("event.outcome")
                        .is_some_and(|s| s.to_lowercase().starts_with("success"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            if event.has("fortinet.firewall.log_id") {
                event.rename("fortinet.firewall.log_id", "event.id")?;
            }

            if event.has("fortinet.firewall.pri") {
                event.rename("fortinet.firewall.pri", "log.level")?;
            }

            if event.has("fortinet.firewall.device_id") {
                event.rename("fortinet.firewall.device_id", "observer.serial_number")?;
            }

            let _cond = { event.has_value("_tmp.user.roles") };
            if _cond {
                event.append(
                    "user.roles",
                    event.get("_tmp.user.roles").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("_tmp.user.roles") };
            if _cond {
                event.append(
                    "source.user.roles",
                    event.get("_tmp.user.roles").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("source.port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert source.port")?;
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
                }
            }

            let _cond = { event.has_value("fortinet.firewall.valid") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("fortinet.firewall.valid") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.valid".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.valid".into(),
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
                                    path: "fortinet.firewall.valid".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("fortinet.firewall.valid", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert fortinet.firewall.valid",
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
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("fortinet.firewall.adminprof");
                event.remove("fortinet.firewall.userfrom");
                event.remove("fortinet.firewall.remote_ip");
                event.remove("fortinet.firewall.remote_port");
                event.remove("fortinet.firewall.ui");
                event.remove("fortinet.firewall.status");
                event.remove("_tmp.user.roles");
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
                event.append(
                    "error.message",
                    event
                        .get("_ingest.on_failure_message")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
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
