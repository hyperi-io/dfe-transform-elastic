// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `event` pipeline.
pub struct Event;

impl Transform for Event {
    fn name(&self) -> &str {
        "event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_str("fortinet.firewall.result") == Some("ERROR")
                    || event.get_str("fortinet.firewall.status") == Some("negotiate_error")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("fortinet.firewall.result") == Some("OK")
                    || ["FSSO-logon", "auth-logon", "FSSO-logoff", "auth-logout"]
                        .contains(&event.get_str("fortinet.firewall.action").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["FSSO-logon", "auth-logon"]
                    .contains(&event.get_str("fortinet.firewall.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                ["FSSO-logoff", "auth-logout"]
                    .contains(&event.get_str("fortinet.firewall.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.subtype") == Some("vpn") };
            if _cond {
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.subtype") == Some("vpn") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.action") == Some("perf-stats") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.action") == Some("perf-stats") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.subtype") == Some("update") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.subtype") == Some("update") };
            if _cond {
                event.append("event.category", json!("host"))?;
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.subtype") == Some("user") };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            if event.has("fortinet.firewall.dstip") {
                event.rename("fortinet.firewall.dstip", "destination.ip")?;
            }

            let _cond = { !event.has("destination.ip") };
            if _cond {
                if event.has("fortinet.firewall.remip") {
                    event.rename("fortinet.firewall.remip", "destination.ip")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.dstport") {
                    if let Some(val) = event.get("fortinet.firewall.dstport") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.dstport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.dstport".into(),
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
                                    path: "fortinet.firewall.dstport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has("destination.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("fortinet.firewall.remport") {
                        if let Some(val) = event.get("fortinet.firewall.remport") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "fortinet.firewall.remport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "fortinet.firewall.remport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
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
                                        path: "fortinet.firewall.remport".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.rcvdbyte") {
                    if let Some(val) = event.get("fortinet.firewall.rcvdbyte") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.rcvdbyte".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.rcvdbyte".into(),
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
                                    path: "fortinet.firewall.rcvdbyte".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.daddr") {
                event.rename("fortinet.firewall.daddr", "destination.address")?;
            }

            let _cond = { !event.has("destination.address") };
            if _cond {
                if event.has("fortinet.firewall.dst_host") {
                    event.rename("fortinet.firewall.dst_host", "destination.address")?;
                }
            }

            let _cond = { !event.has("destination.address") };
            if _cond {
                if event.has("fortinet.firewall.dst_host") {
                    event.rename("fortinet.firewall.dst_host", "destination.domain")?;
                }
            }

            if event.has("fortinet.firewall.group") {
                event.rename("fortinet.firewall.group", "source.user.group.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.sentbyte") {
                    if let Some(val) = event.get("fortinet.firewall.sentbyte") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sentbyte".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sentbyte".into(),
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
                                    path: "fortinet.firewall.sentbyte".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.srcip") {
                event.rename("fortinet.firewall.srcip", "source.ip")?;
            }

            let _cond = { !event.has("source.ip") };
            if _cond {
                if event.has("fortinet.firewall.locip") {
                    event.rename("fortinet.firewall.locip", "source.ip")?;
                }
            }

            if event.has("fortinet.firewall.srcmac") {
                event.rename("fortinet.firewall.srcmac", "source.mac")?;
            }

            let _cond = { !event.has("source.mac") };
            if _cond {
                if event.has("fortinet.firewall.source_mac") {
                    event.rename("fortinet.firewall.source_mac", "source.mac")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.srcport") {
                    if let Some(val) = event.get("fortinet.firewall.srcport") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.srcport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.srcport".into(),
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
                                    path: "fortinet.firewall.srcport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has("source.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("fortinet.firewall.locport") {
                        if let Some(val) = event.get("fortinet.firewall.locport") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "fortinet.firewall.locport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "fortinet.firewall.locport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
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
                                        path: "fortinet.firewall.locport".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            if event.has("fortinet.firewall.user") {
                event.rename("fortinet.firewall.user", "source.user.name")?;
            }

            if event.has("fortinet.firewall.saddr") {
                event.rename("fortinet.firewall.saddr", "source.address")?;
            }

            if event.has("fortinet.firewall.agent") {
                event.rename("fortinet.firewall.agent", "user_agent.original")?;
            }

            if event.has("fortinet.firewall.file") {
                event.rename("fortinet.firewall.file", "file.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.filesize") {
                    if let Some(val) = event.get("fortinet.firewall.filesize") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.filesize".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.filesize".into(),
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
                                    path: "fortinet.firewall.filesize".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("file.size", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.level") {
                event.rename("fortinet.firewall.level", "log.level")?;
            }

            let _cond = { !event.has("event.code") };
            if _cond {
                if event.has("fortinet.firewall.logid") {
                    event.rename("fortinet.firewall.logid", "event.code")?;
                }
            }

            if event.has("fortinet.firewall.msg") {
                event.rename("fortinet.firewall.msg", "message")?;
            }

            if event.has("fortinet.firewall.policyid") {
                event.rename("fortinet.firewall.policyid", "rule.id")?;
            }

            if event.has("fortinet.firewall.proto") {
                event.rename("fortinet.firewall.proto", "network.iana_number")?;
            }

            if event.has("fortinet.firewall.service") {
                event.rename("fortinet.firewall.service", "network.protocol")?;
            }

            if event.has("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            if event.has("fortinet.firewall.error_num") {
                event.rename("fortinet.firewall.error_num", "error.code")?;
            }

            if event.has("fortinet.firewall.hostname") {
                event.rename("fortinet.firewall.hostname", "url.domain")?;
            }

            if event.has("fortinet.firewall.logdesc") {
                event.rename("fortinet.firewall.logdesc", "rule.description")?;
            }

            let _cond =
                { event.has("fortinet.firewall.addr") && !event.has("fortinet.firewall.addrgrp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("fortinet.firewall.addr") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "fortinet.firewall.addr".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("fortinet.firewall.addr", s)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.rename("fortinet.firewall.addr", "fortinet.firewall.addrgrp")?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                }
            }

            if event.has("fortinet.firewall.url") {
                event.rename("fortinet.firewall.url", "url.path")?;
            }

            let _cond = { !event.has("event.duration") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("fortinet.firewall.sess_duration") {
                        if let Some(val) = event.get("fortinet.firewall.sess_duration") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "fortinet.firewall.sess_duration".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "fortinet.firewall.sess_duration".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
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
                                        path: "fortinet.firewall.sess_duration".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("event.duration", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.mem") {
                    if let Some(val) = event.get("fortinet.firewall.mem") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.mem".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.mem".into(),
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
                                    path: "fortinet.firewall.mem".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("fortinet.firewall.mem", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.jitter") {
                if let Some(val) = event.get("fortinet.firewall.jitter") {
                    let converted = match val {
                        Value::String(s) => json!(s.trim().parse::<f64>().map_err(|_| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.jitter".into(),
                                message: format!("cannot convert '{}' to float", s),
                            }
                        })?),
                        Value::Number(n) => json!(n.as_f64().unwrap_or(0.0)),
                        Value::Bool(b) => json!(if *b { 1.0 } else { 0.0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "fortinet.firewall.jitter".into(),
                                message: "cannot convert to float".into(),
                            });
                        }
                    };
                    event.set("fortinet.firewall.jitter", converted)?;
                }
            }

            if event.has("fortinet.firewall.latency") {
                if let Some(val) = event.get("fortinet.firewall.latency") {
                    let converted = match val {
                        Value::String(s) => json!(s.trim().parse::<f64>().map_err(|_| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.latency".into(),
                                message: format!("cannot convert '{}' to float", s),
                            }
                        })?),
                        Value::Number(n) => json!(n.as_f64().unwrap_or(0.0)),
                        Value::Bool(b) => json!(if *b { 1.0 } else { 0.0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "fortinet.firewall.latency".into(),
                                message: "cannot convert to float".into(),
                            });
                        }
                    };
                    event.set("fortinet.firewall.latency", converted)?;
                }
            }

            event.remove("fortinet.firewall.dstport");
            event.remove("fortinet.firewall.remport");
            event.remove("fortinet.firewall.rcvdbyte");
            event.remove("fortinet.firewall.sentbyte");
            event.remove("fortinet.firewall.srcport");
            event.remove("fortinet.firewall.locport");
            event.remove("fortinet.firewall.filesize");
            event.remove("fortinet.firewall.sess_duration");

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
