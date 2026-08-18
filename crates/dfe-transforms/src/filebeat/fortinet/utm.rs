// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `utm` pipeline.
pub struct Utm;

impl Transform for Utm {
    fn name(&self) -> &str {
        "utm"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("event"))?;

        // TODO: conditional: ['block', 'blocked'].contains(ctx.fortinet?.firewall?.action)
        {
            event.append("event.type", json!("denied"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.subtype == 'dns'
        {
            event.append("event.type", json!("info"))?;
        }

        // TODO: conditional: ['pass', 'passthrough'].contains(ctx.fortinet?.firewall?.action)
        {
            event.append("event.type", json!("allowed"))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.action != null
        {
            event.set("event.outcome", json!("success"))?;
        }

        event.append("event.category", json!("network"))?;

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
            if event.has("fortinet.firewall.dst_port") {
                if let Some(val) = event.get("fortinet.firewall.dst_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "fortinet.firewall.dst_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.dst_port".into(),
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
                                path: "fortinet.firewall.dst_port".into(),
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

        // TODO: conditional: ctx.destination?.port == null
        {
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

        if event.has("fortinet.firewall.recipient") {
            event.rename("fortinet.firewall.recipient", "email.to.address")?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.recipient != null
        {
            event.append(
                "email.to.address",
                event
                    .get("fortinet.firewall.recipient")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("fortinet.firewall.group") {
            event.rename("fortinet.firewall.group", "source.user.group.name")?;
        }

        if event.has("fortinet.firewall.locip") {
            event.rename("fortinet.firewall.locip", "source.ip")?;
        }

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
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "fortinet.firewall.locport".into(),
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

        // TODO: conditional: ctx.source?.port == null
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("fortinet.firewall.src_port") {
                    if let Some(val) = event.get("fortinet.firewall.src_port") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.src_port".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "fortinet.firewall.src_port".into(),
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
                                    path: "fortinet.firewall.src_port".into(),
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

        // TODO: conditional: ctx.source?.port == null
        {
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

        if event.has("fortinet.firewall.srcdomain") {
            event.rename("fortinet.firewall.srcdomain", "source.domain")?;
        }

        // TODO: conditional: ctx.source?.ip == null
        {
            if event.has("fortinet.firewall.srcip") {
                event.rename("fortinet.firewall.srcip", "source.ip")?;
            }
        }

        if event.has("fortinet.firewall.httpmethod") {
            event.rename("fortinet.firewall.httpmethod", "http.request.method")?;
        }

        if event.has("fortinet.firewall.referralurl") {
            event.rename("fortinet.firewall.referralurl", "http.request.referrer")?;
        }

        if event.has("fortinet.firewall.srcmac") {
            event.rename("fortinet.firewall.srcmac", "source.mac")?;
        }

        if event.has("fortinet.firewall.unauthuser") {
            event.rename("fortinet.firewall.unauthuser", "source.user.name")?;
        }

        // TODO: conditional: ctx.source?.user?.name == null
        {
            if event.has("fortinet.firewall.user") {
                event.rename("fortinet.firewall.user", "source.user.name")?;
            }
        }

        // TODO: conditional: ctx.fortinet?.firewall?.sender != null
        {
            event.append(
                "email.sender.address",
                event
                    .get("fortinet.firewall.sender")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.from != null
        {
            event.append(
                "email.from.address",
                event
                    .get("fortinet.firewall.from")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("fortinet.firewall.agent") {
            event.rename("fortinet.firewall.agent", "user_agent.original")?;
        }

        if event.has("fortinet.firewall.app") {
            event.rename("fortinet.firewall.app", "network.application")?;
        }

        if event.has("fortinet.firewall.appcat") {
            event.rename("fortinet.firewall.appcat", "rule.category")?;
        }

        if event.has("fortinet.firewall.applist") {
            event.rename("fortinet.firewall.applist", "rule.ruleset")?;
        }

        // TODO: conditional: ctx.rule?.category == null
        {
            if event.has("fortinet.firewall.catdesc") {
                event.rename("fortinet.firewall.catdesc", "rule.category")?;
            }
        }

        // TODO: conditional: ctx.rule?.category != null
        {
            if event.has("rule.category") {
                if let Some(s) = event.get_str("rule.category").map(String::from) {
                    let s = s.as_str();
                    let re = regex::Regex::new("\\.").unwrap();
                    let replaced = re.replace_all(s, "-").into_owned();
                    event.set("rule.category", replaced)?;
                }
            }
        }

        if event.has("fortinet.firewall.error") {
            event.rename("fortinet.firewall.error", "event.message")?;
        }

        if event.has("fortinet.firewall.errorcode") {
            event.rename("fortinet.firewall.errorcode", "event.code")?;
        }

        if event.has("fortinet.firewall.event_id") {
            event.rename("fortinet.firewall.event_id", "event.id")?;
        }

        // TODO: conditional: ctx.event?.id == null
        {
            if event.has("fortinet.firewall.eventid") {
                event.rename("fortinet.firewall.eventid", "event.id")?;
            }
        }

        if event.has("fortinet.firewall.filename") {
            event.rename("fortinet.firewall.filename", "file.name")?;
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

        if event.has("fortinet.firewall.filetype") {
            event.rename("fortinet.firewall.filetype", "file.extension")?;
        }

        // TODO: conditional: ctx.file?.name == null
        {
            if event.has("fortinet.firewall.infectedfilename") {
                event.rename("fortinet.firewall.infectedfilename", "file.name")?;
            }
        }

        // TODO: conditional: ctx.file?.size == null
        {
            if event.has("fortinet.firewall.infectedfilesize") {
                event.rename("fortinet.firewall.infectedfilesize", "file.size")?;
            }
        }

        // TODO: conditional: ctx.file?.extension == null
        {
            if event.has("fortinet.firewall.infectedfiletype") {
                event.rename("fortinet.firewall.infectedfiletype", "file.extension")?;
            }
        }

        // TODO: conditional: ctx.file?.name == null
        {
            if event.has("fortinet.firewall.matchedfilename") {
                event.rename("fortinet.firewall.matchedfilename", "file.name")?;
            }
        }

        // TODO: conditional: ctx.file?.extension == null
        {
            if event.has("fortinet.firewall.matchedfiletype") {
                event.rename("fortinet.firewall.matchedfiletype", "file.extension")?;
            }
        }

        if event.has("fortinet.firewall.hostname") {
            event.rename("fortinet.firewall.hostname", "url.domain")?;
        }

        if event.has("fortinet.firewall.ipaddr") {
            event.rename("fortinet.firewall.ipaddr", "dns.resolved_ip")?;
        }

        if event.has("dns.resolved_ip") {
            if let Some(s) = event.get_str("dns.resolved_ip").map(String::from) {
                let s = s.as_str();
                let parts: Vec<Value> = s.split(", ").map(|p| json!(p)).collect();
                event.set("dns.resolved_ip", Value::Array(parts))?;
            }
        }

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

        // TODO: conditional: ctx.rule?.id == null
        {
            if event.has("fortinet.firewall.policy_id") {
                event.rename("fortinet.firewall.policy_id", "rule.id")?;
            }
        }

        // TODO: conditional: ctx.rule?.id == null
        {
            if event.has("fortinet.firewall.policyid") {
                event.rename("fortinet.firewall.policyid", "rule.id")?;
            }
        }

        // TODO: conditional: ctx.rule?.ruleset == null
        {
            if event.has("fortinet.firewall.profile") {
                event.rename("fortinet.firewall.profile", "rule.ruleset")?;
            }
        }

        if event.has("fortinet.firewall.proto") {
            event.rename("fortinet.firewall.proto", "network.iana_number")?;
        }

        if event.has("fortinet.firewall.qclass") {
            event.rename("fortinet.firewall.qclass", "dns.question.class")?;
        }

        if event.has("fortinet.firewall.qname") {
            event.rename("fortinet.firewall.qname", "dns.question.name")?;
        }

        if event.has("fortinet.firewall.qtype") {
            event.rename("fortinet.firewall.qtype", "dns.question.type")?;
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

        if event.has("fortinet.firewall.url") {
            event.rename("fortinet.firewall.url", "url.path")?;
        }

        if event.has("fortinet.firewall.xid") {
            event.rename("fortinet.firewall.xid", "dns.id")?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.scertcname != null
        {
            event.append(
                "tls.server.x509.subject.common_name",
                event
                    .get("fortinet.firewall.scertcname")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("fortinet.firewall.scertissuer") {
            event.rename("fortinet.firewall.scertissuer", "tls.server.issuer")?;
        }

        // TODO: conditional: ctx.tls?.server?.issuer != null
        {
            event.append(
                "tls.server.x509.issuer.common_name",
                event
                    .get("tls.server.issuer")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("fortinet.firewall.ccertissuer") {
            event.rename("fortinet.firewall.ccertissuer", "tls.client.issuer")?;
        }

        // TODO: conditional: ctx.tls?.client?.issuer != null
        {
            event.append(
                "tls.client.x509.issuer.common_name",
                event
                    .get("tls.client.issuer")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("fortinet.firewall.sender") {
            event.rename("fortinet.firewall.sender", "tls.server.issuer")?;
        }

        // TODO: conditional: ctx.tls?.server?.issuer == null
        {
            if event.has("fortinet.firewall.issuer") {
                event.rename("fortinet.firewall.issuer", "tls.server.issuer")?;
            }
        }

        if event.has("fortinet.firewall.authalgo") {
            event.rename(
                "fortinet.firewall.authalgo",
                "tls.server.x509.public_key_algorithm",
            )?;
        }

        // TODO: conditional: ctx.tls?.server?.x509?.public_key_algorithm == null
        {
            if event.has("fortinet.firewall.keyalgo") {
                event.rename(
                    "fortinet.firewall.keyalgo",
                    "tls.server.x509.public_key_algorithm",
                )?;
            }
        }

        // TODO: conditional: ctx.tls?.server?.not_before == null
        {
            if event.has("fortinet.firewall.notbefore") {
                event.rename("fortinet.firewall.notbefore", "tls.server.not_before")?;
            }
        }

        // TODO: conditional: ctx.tls?.server?.not_after == null
        {
            if event.has("fortinet.firewall.notafter") {
                event.rename("fortinet.firewall.notafter", "tls.server.not_after")?;
            }
        }

        // TODO: conditional: ctx.tls?.server?.x509?.public_key_size == null
        {
            if event.has("fortinet.firewall.keysize") {
                event.rename(
                    "fortinet.firewall.keysize",
                    "tls.server.x509.public_key_size",
                )?;
            }
        }

        if event.has("tls.server.x509.public_key_size") {
            if let Some(val) = event.get("tls.server.x509.public_key_size") {
                let converted = match val {
                    Value::String(s) => {
                        let s = s.trim();
                        if let Some(hex) = s.strip_prefix("0x") {
                            json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                TransformError::ParseError {
                                    path: "tls.server.x509.public_key_size".into(),
                                    message: format!("cannot convert '{}' to integer", s),
                                }
                            })?)
                        } else {
                            json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                path: "tls.server.x509.public_key_size".into(),
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
                            path: "tls.server.x509.public_key_size".into(),
                            message: "cannot convert to integer".into(),
                        }
                        .into());
                    }
                };
                event.set("tls.server.x509.public_key_size", converted)?;
            }
        }

        // TODO: conditional: ctx.tls?.server?.x509?.serial_number == null
        {
            if event.has("fortinet.firewall.sn") {
                event.rename("fortinet.firewall.sn", "tls.server.x509.serial_number")?;
            }
        }

        // TODO: conditional: ctx.tls?.server?.hash?.sha1 == null
        {
            if event.has("fortinet.firewall.certhash") {
                event.rename("fortinet.firewall.certhash", "tls.server.hash.sha1")?;
            }
        }

        // TODO: conditional: ctx.tls?.server?.hash?.sha1 != null
        {
            event.append(
                "related.hash",
                event
                    .get("tls.server.hash.sha1")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        event.set(
            "tls.server.x509.not_after",
            event
                .get("tls.server.not_after")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        event.set(
            "tls.server.x509.not_before",
            event
                .get("tls.server.not_before")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        if event.has("fortinet.firewall.san") {
            if let Some(s) = event.get_str("fortinet.firewall.san").map(String::from) {
                let s = s.as_str();
                let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                event.set("tls.server.x509.alternative_names", Value::Array(parts))?;
            }
        }

        // TODO: conditional: ctx.fortinet?.firewall?.cn != null
        {
            event.append(
                "tls.server.x509.alternative_names",
                event
                    .get("fortinet.firewall.cn")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        event.set(
            "tls.client.x509.public_key_algorithm",
            event
                .get("tls.server.x509.public_key_algorithm")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        if event.has("fortinet.firewall.kxcurve") {
            event.rename("fortinet.firewall.kxcurve", "tls.curve")?;
        }

        if event.has("fortinet.firewall.cipher") {
            event.rename("fortinet.firewall.cipher", "tls.cipher")?;
        }

        if event.has("fortinet.firewall.sni") {
            event.rename("fortinet.firewall.sni", "tls.client.server_name")?;
        }

        // TODO: conditional: ctx.destination?.domain == null
        {
            event.set(
                "destination.domain",
                event
                    .get("tls.client.server_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.handshake == "full"
        {
            event.set("tls.established", json!(true))?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.tlsver instanceof String
        {
            // Painless script
            // Source: def pat = /\\d+/; def tlsver = ctx.fortinet.firewall.tlsver.toLowerCase(); def matcher = pat.matcher(tlsver); if (!matcher.find()) {\n    return;\n} ctx.tls.version_protocol = tlsver.substring(0, matcher.start()); ctx.tls.version = tlsver.substring(matcher.start(), tlsver.length()); if (!ctx.tls.version.contains(\".\")) {\n  ctx.tls.version += \".0\";\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def pat = /\\d+/; def tlsver = ctx.fortinet.firewall.tlsver.toLowerCase(); def matcher = pat.matcher(tlsver); if (!matcher.find()) {\n    return;\n} ctx.tls.version_protocol = tlsver.substring(0, matcher.start()); ctx.tls.version = tlsver.substring(matcher.start(), tlsver.length()); if (!ctx.tls.version.contains(\".\")) {\n  ctx.tls.version += \".0\";\n}"#,
            )?;
        }

        // TODO: conditional: ctx.fortinet?.firewall?.dtype instanceof String
        {
            event.append(
                "vulnerability.category",
                event
                    .get("fortinet.firewall.dtype")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("fortinet.firewall.ref") {
            event.rename("fortinet.firewall.ref", "event.reference")?;
        }

        if event.has("fortinet.firewall.filehash") {
            event.rename("fortinet.firewall.filehash", "fortinet.file.hash.crc32")?;
        }

        // TODO: conditional: ctx.fortinet?.file?.hash?.crc32 != null
        {
            event.append(
                "related.hash",
                event
                    .get("fortinet.file.hash.crc32")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("dns.question.name") {
                if let Some(domain_str) = event.get_str("dns.question.name").map(String::from) {
                    let domain_str = domain_str.as_str();
                    let domain = domain_str.to_string();
                    event.set("dns.question.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set(
                            "dns.question.registered_domain",
                            json!(rd.registered_domain),
                        )?;
                        event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("dns.question.subdomain", json!(sub))?;
                        }
                    }
                }
            }
            Ok(())
        })();

        event.remove("dns.question.domain");

        event.remove("fortinet.firewall.cn");
        event.remove("fortinet.firewall.san");
        event.remove("fortinet.firewall.dst_port");
        event.remove("fortinet.firewall.remport");
        event.remove("fortinet.firewall.dstport");
        event.remove("fortinet.firewall.rcvdbyte");
        event.remove("fortinet.firewall.locport");
        event.remove("fortinet.firewall.scertcname");
        event.remove("fortinet.firewall.src_port");
        event.remove("fortinet.firewall.srcport");
        event.remove("fortinet.firewall.sentbyte");
        event.remove("fortinet.firewall.filesize");
        event.remove("fortinet.firewall.dtype");
        event.remove("fortinet.firewall.tlsver");

        Ok(TransformResult::Continue)
    }
}
