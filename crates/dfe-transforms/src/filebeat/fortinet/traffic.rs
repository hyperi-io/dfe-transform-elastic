// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `traffic` pipeline.
pub struct Traffic;

impl Transform for Traffic {
    fn name(&self) -> &str {
        "traffic"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let v = event
                .get("fortinet.firewall.action")
                .cloned()
                .unwrap_or(Value::Null);
            if !painless_is_empty_value(&v) {
                event.set("event.action", v)?;
            }

            let _cond = { event.has_value("fortinet.firewall.action") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            let _cond = { event.get_str("fortinet.firewall.action") == Some("start") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.has_value("fortinet.firewall.action")
                    && event.get_str("fortinet.firewall.action") != Some("start")
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = {
                event.has_value("fortinet.firewall.app")
                    && event.get_str("fortinet.firewall.action") != Some("deny")
            };
            if _cond {
                event.append("event.type", json!("protocol"))?;
            }

            let _cond = {
                !event.has_value("fortinet.firewall.utmaction")
                    && event.get_str("fortinet.firewall.action") != Some("deny")
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.get_str("fortinet.firewall.utmaction") == Some("block")
                    || event.get_str("fortinet.firewall.action") == Some("deny")
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            if event.has("fortinet.firewall.dstip") {
                event.rename("fortinet.firewall.dstip", "destination.ip")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("fortinet.firewall.tranip") {
                    if let Some(s) = event.get_string("fortinet.firewall.tranip") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "fortinet.firewall.tranip".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("destination.nat.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_fortinet_firewall_tranip_to_destination_nat_ip_7b6fb54b",
                )?;
                if event.remove("fortinet.firewall.tranip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "fortinet.firewall.tranip".into(),
                    });
                }
                event.append(
                    "error.message",
                    event
                        .get("_ingest.on_failure_message")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.tranport") {
                    if let Some(val) = event.get("fortinet.firewall.tranport") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.tranport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.tranport".into(),
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
                                    path: "fortinet.firewall.tranport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.nat.port", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.rcvddelta") {
                    if let Some(val) = event.get("fortinet.firewall.rcvddelta") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.rcvddelta".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.rcvddelta".into(),
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
                                    path: "fortinet.firewall.rcvddelta".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("fortinet.firewall.rcvddelta", converted)?;
                    }
                }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.rcvdpkt") {
                    if let Some(val) = event.get("fortinet.firewall.rcvdpkt") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.rcvdpkt".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.rcvdpkt".into(),
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
                                    path: "fortinet.firewall.rcvdpkt".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.packets", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("fortinet.firewall.dstcollectedemail") };
            if _cond {
                event.append(
                    "email.to.address",
                    event
                        .get("fortinet.firewall.dstcollectedemail")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            if event.has("fortinet.firewall.dstname") {
                event.rename("fortinet.firewall.dstname", "destination.address")?;
            }

            if event.has("fortinet.firewall.dstunauthuser") {
                event.rename("fortinet.firewall.dstunauthuser", "destination.user.name")?;
            }

            if event.has("fortinet.firewall.group") {
                event.rename("fortinet.firewall.group", "source.user.group.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.sentdelta") {
                    if let Some(val) = event.get("fortinet.firewall.sentdelta") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sentdelta".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sentdelta".into(),
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
                                    path: "fortinet.firewall.sentdelta".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("fortinet.firewall.sentdelta", converted)?;
                    }
                }
                Ok(())
            })();

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

            if event.has("fortinet.firewall.srcdomain") {
                event.rename("fortinet.firewall.srcdomain", "source.domain")?;
            }

            if event.has("fortinet.firewall.srcip") {
                event.rename("fortinet.firewall.srcip", "source.ip")?;
            }

            if event.has("fortinet.firewall.srcmac") {
                event.rename("fortinet.firewall.srcmac", "source.mac")?;
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

            if event.has("fortinet.firewall.unauthuser") {
                event.rename("fortinet.firewall.unauthuser", "source.user.name")?;
            }

            let _cond = { !event.has_value("source.user.name") };
            if _cond {
                if event.has("fortinet.firewall.user") {
                    event.rename("fortinet.firewall.user", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("fortinet.firewall.collectedemail") };
            if _cond {
                event.append(
                    "email.from.address",
                    event
                        .get("fortinet.firewall.collectedemail")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.sentpkt") {
                    if let Some(val) = event.get("fortinet.firewall.sentpkt") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sentpkt".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sentpkt".into(),
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
                                    path: "fortinet.firewall.sentpkt".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.packets", converted)?;
                    }
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("fortinet.firewall.transip") {
                    if let Some(s) = event.get_string("fortinet.firewall.transip") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "fortinet.firewall.transip".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.nat.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_fortinet_firewall_transip_to_source_nat_ip_c36fcafa",
                )?;
                if event.remove("fortinet.firewall.transip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "fortinet.firewall.transip".into(),
                    });
                }
                event.append(
                    "error.message",
                    event
                        .get("_ingest.on_failure_message")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.transport") {
                    if let Some(val) = event.get("fortinet.firewall.transport") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.transport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.transport".into(),
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
                                    path: "fortinet.firewall.transport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.nat.port", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.app") {
                event.rename("fortinet.firewall.app", "network.application")?;
            }

            if event.has("fortinet.firewall.filename") {
                event.rename("fortinet.firewall.filename", "file.name")?;
            }

            let _cond = { !event.has_value("event.code") };
            if _cond {
                if event.has("fortinet.firewall.logid") {
                    event.rename("fortinet.firewall.logid", "event.code")?;
                }
            }

            if event.has("fortinet.firewall.msg") {
                event.rename("fortinet.firewall.msg", "message")?;
            }

            if event.has("fortinet.firewall.comment") {
                event.rename("fortinet.firewall.comment", "rule.description")?;
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has("fortinet.firewall.policyid") {
                    event.rename("fortinet.firewall.policyid", "rule.id")?;
                }
            }

            if event.has("fortinet.firewall.poluuid") {
                event.rename("fortinet.firewall.poluuid", "rule.uuid")?;
            }

            if event.has("fortinet.firewall.policytype") {
                event.rename("fortinet.firewall.policytype", "rule.ruleset")?;
            }

            if event.has("fortinet.firewall.policyname") {
                event.rename("fortinet.firewall.policyname", "rule.name")?;
            }

            if event.has("fortinet.firewall.appcat") {
                event.rename("fortinet.firewall.appcat", "rule.category")?;
            }

            if event.has("rule.category") {
                if let Some(s) = event.get_string("rule.category") {
                    let re = cached_regex!("\\.");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("rule.category", replaced)?;
                }
            }

            if event.has("fortinet.firewall.proto") {
                event.rename("fortinet.firewall.proto", "network.iana_number")?;
            }

            if event.has("fortinet.firewall.service") {
                event.rename("fortinet.firewall.service", "network.protocol")?;
            }

            if event.has("fortinet.firewall.srcthreatfeed") {
                event.rename("fortinet.firewall.srcthreatfeed", "threat.feed.name")?;
            }

            if event.has("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            let _cond = { event.has_value("fortinet.firewall.url") };
            if _cond {
                uri_parts(event, "fortinet.firewall.url", "url", false, false)?;
            }

            let _cond = {
                event
                    .get("fortinet.firewall.rcvddelta")
                    .is_some_and(|v| v.is_number())
                    && event
                        .get("fortinet.firewall.sentdelta")
                        .is_some_and(|v| v.is_number())
            };
            if _cond {
                // Painless script
                // Source: ctx.fortinet.firewall.deltabytes = ctx.fortinet.firewall.rcvddelta + ctx.fortinet.firewall.sentdelta
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx.fortinet.firewall.deltabytes = ctx.fortinet.firewall.rcvddelta + ctx.fortinet.firewall.sentdelta"#
                    ),
                )?;
            }

            event.remove("fortinet.firewall.url");

            event.remove("fortinet.firewall.dstport");
            event.remove("fortinet.firewall.tranport");
            event.remove("fortinet.firewall.rcvdbyte");
            event.remove("fortinet.firewall.rcvdpkt");
            event.remove("fortinet.firewall.sentbyte");
            event.remove("fortinet.firewall.srcport");
            event.remove("fortinet.firewall.sentpkt");
            event.remove("fortinet.firewall.transport");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.append("tags", json!("preserve_original_event"))?;
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
