// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `event` pipeline.
pub struct FortinetEvent;

impl Transform for FortinetEvent {
    fn name(&self) -> &str {
        "fortinet_event"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("fortinet_event"))?;

        // TODO: conditional: ctx.fortinet?.firewall?.result == 'ERROR' || ctx.fortinet?.firewall?.status == 'negotiate_error'
        {
            event.set("event.outcome", json!("failure"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.result == 'OK' || ['FSSO-logon', 'auth-logon', 'FSSO-logoff', 'auth-logout'].contains(ctx.fortinet?.firewall?.action)
        {
            event.set("event.outcome", json!("success"))?;
        }

        // TODO: conditional: ['FSSO-logon', 'auth-logon'].contains(ctx.fortinet?.firewall?.action)
        {
            event.append("event.type", json!("start"))?;
        }

        // TODO: conditional: ['FSSO-logoff', 'auth-logout'].contains(ctx.fortinet?.firewall?.action)
        {
            event.append("event.type", json!("end"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.subtype == 'vpn'
        {
            event.append("event.type", json!("connection"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.subtype == 'vpn'
        {
            event.append("event.category", json!("network"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.action == 'perf-stats'
        {
            event.append("event.type", json!("info"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.action == 'perf-stats'
        {
            event.append("event.category", json!("host"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.subtype == 'update'
        {
            event.append("event.type", json!("info"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.subtype == 'update'
        {
            event.append("event.category", json!("host"))?;
            event.append("event.category", json!("malware"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.subtype == 'user'
        {
            event.append("event.category", json!("authentication"))?;
        }

        if event.has("fortinet.firewall.dstip") {
            event.rename("fortinet.firewall.dstip", "destination.ip")?;
        }

        // TODO: conditional: ctx.destination?.ip == null
        {
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
                            }
                            .into());
                        }
                    };
                    event.set("destination.port", converted)?;
                }
            }
            Ok(())
        })();

        // TODO: conditional: ctx.destination?.port == null
        {
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
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.remport".into(),
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
                                    path: "fortinet.firewall.remport".into(),
                                    message: "cannot convert to integer".into(),
                                }
                                .into());
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
                            }
                            .into());
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

        // TODO: conditional: ctx.destination?.address == null
        {
            if event.has("fortinet.firewall.dst_host") {
                event.rename("fortinet.firewall.dst_host", "destination.address")?;
            }
        }

        // TODO: conditional: ctx.destination?.address == null
        {
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
                            }
                            .into());
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

        // TODO: conditional: ctx.source?.ip == null
        {
            if event.has("fortinet.firewall.locip") {
                event.rename("fortinet.firewall.locip", "source.ip")?;
            }
        }

        if event.has("fortinet.firewall.srcmac") {
            event.rename("fortinet.firewall.srcmac", "source.mac")?;
        }

        // TODO: conditional: ctx.source?.mac == null
        {
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
                            }
                            .into());
                        }
                    };
                    event.set("source.port", converted)?;
                }
            }
            Ok(())
        })();

        // TODO: conditional: ctx.source?.port == null
        {
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
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.locport".into(),
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
                                    path: "fortinet.firewall.locport".into(),
                                    message: "cannot convert to integer".into(),
                                }
                                .into());
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.filesize".into(),
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
                                path: "fortinet.firewall.filesize".into(),
                                message: "cannot convert to integer".into(),
                            }
                            .into());
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

        // TODO: conditional: ctx.event?.code == null
        {
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
            if let Some(s) = event.get_str("network.protocol").map(String::from) {
                let s = s.as_str();
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

        // TODO: conditional: ctx.fortinet?.firewall?.addr != null && ctx.fortinet.firewall.addrgrp == null
        {
            if let Some(s) = event.get_str("fortinet.firewall.addr").map(String::from) {
                let s = s.as_str();
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "fortinet.firewall.addr".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    }
                    .into());
                }
                event.set("fortinet.firewall.addr", s)?;
            }
        }

        if event.has("fortinet.firewall.url") {
            event.rename("fortinet.firewall.url", "url.path")?;
        }

        // TODO: conditional: ctx.event?.duration == null
        {
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
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.sess_duration".into(),
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
                                    path: "fortinet.firewall.sess_duration".into(),
                                    message: "cannot convert to integer".into(),
                                }
                                .into());
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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.mem".into(),
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
                                path: "fortinet.firewall.mem".into(),
                                message: "cannot convert to integer".into(),
                            }
                            .into());
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
                        }
                        .into());
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
                        }
                        .into());
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
    }
}
