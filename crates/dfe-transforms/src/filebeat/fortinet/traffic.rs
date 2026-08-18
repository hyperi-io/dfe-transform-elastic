// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `traffic` pipeline.
pub struct Traffic;

impl Transform for Traffic {
    fn name(&self) -> &str {
        "traffic"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("event"))?;

        event.set(
            "event.action",
            event
                .get("fortinet.firewall.action")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        // TODO: conditional: ctx.fortinet?.firewall?.action != null
        {
            event.set("event.outcome", json!("success"))?;
        }

        event.append("event.category", json!("network"))?;

        event.append("event.type", json!("connection"))?;

        // TODO: conditional: ctx.fortinet?.firewall?.action == 'start'
        {
            event.append("event.type", json!("start"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.action != null && ctx.fortinet?.firewall?.action !='start'
        {
            event.append("event.type", json!("end"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.app != null && ctx.fortinet?.firewall?.action != 'deny'
        {
            event.append("event.type", json!("protocol"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.utmaction == null && ctx.fortinet?.firewall?.action != 'deny'
        {
            event.append("event.type", json!("allowed"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.utmaction == 'block' || ctx.fortinet?.firewall?.action == 'deny'
        {
            event.append("event.type", json!("denied"))?;
        }

        if event.has("fortinet.firewall.dstip") {
            event.rename("fortinet.firewall.dstip", "destination.ip")?;
        }

        if event.has("fortinet.firewall.tranip") {
            if let Some(s) = event.get_str("fortinet.firewall.tranip").map(String::from) {
                let s = s.as_str();
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.dstport".into(),
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.tranport".into(),
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.rcvdbyte".into(),
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.rcvdpkt".into(),
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

        // TODO: conditional: ctx.fortinet?.firewall?.dstcollectedemail != null
        {
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.sentbyte".into(),
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.srcport".into(),
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

        // TODO: conditional: ctx.source?.user?.name == null
        {
            if event.has("fortinet.firewall.user") {
                event.rename("fortinet.firewall.user", "source.user.name")?;
            }
        }

        // TODO: conditional: ctx.fortinet?.firewall?.collectedemail != null
        {
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.sentpkt".into(),
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

        if event.has("fortinet.firewall.transip") {
            if let Some(s) = event.get_str("fortinet.firewall.transip").map(String::from) {
                let s = s.as_str();
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.transport".into(),
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

        // TODO: conditional: ctx.event?.code == null
        {
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

        // TODO: conditional: ctx.rule?.id == null
        {
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
            if let Some(s) = event.get_str("rule.category").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("\\.").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
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
            if let Some(s) = event.get_str("network.protocol").map(String::from) {
                let s = s.as_str();
                let lowered = s.to_lowercase();
                event.set("network.protocol", lowered)?;
            }
        }

        if event.has("fortinet.firewall.url") {
            event.rename("fortinet.firewall.url", "url.path")?;
        }

        event.remove("fortinet.firewall.dstport");
        event.remove("fortinet.firewall.tranport");
        event.remove("fortinet.firewall.rcvdbyte");
        event.remove("fortinet.firewall.rcvdpkt");
        event.remove("fortinet.firewall.sentbyte");
        event.remove("fortinet.firewall.srcport");
        event.remove("fortinet.firewall.sentpkt");
        event.remove("fortinet.firewall.transport");

        Ok(TransformResult::Continue)
    }
}
