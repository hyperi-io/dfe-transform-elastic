// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `utm` pipeline.
pub struct Utm;

impl Transform for Utm {
    fn name(&self) -> &str {
        "utm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = {
                ["block", "blocked"]
                    .contains(&event.get_str("fortinet.firewall.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("fortinet.firewall.subtype") == Some("dns") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                ["pass", "passthrough"]
                    .contains(&event.get_str("fortinet.firewall.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.has_value("fortinet.firewall.action") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            event.append("event.category", json!("network"))?;

            if event.has("fortinet.firewall.dstip") {
                event.rename("fortinet.firewall.dstip", "destination.ip")?;
            }

            let _cond = { !event.has_value("destination.ip") };
            if _cond {
                if event.has("fortinet.firewall.remip") {
                    event.rename("fortinet.firewall.remip", "destination.ip")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("fortinet.firewall.dst_port") {
                    if let Some(val) = event.get("fortinet.firewall.dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("destination.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("fortinet.firewall.remport") {
                        if let Some(val) = event.get("fortinet.firewall.remport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "fortinet.firewall.remport".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("destination.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("fortinet.firewall.dstport") {
                        if let Some(val) = event.get("fortinet.firewall.dstport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "fortinet.firewall.dstport".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("fortinet.firewall.rcvdbyte") {
                    if let Some(val) = event.get("fortinet.firewall.rcvdbyte") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.rcvdbyte".into(),
                                message,
                            }
                        })?;
                        event.set("destination.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.recipient") {
                event.rename("fortinet.firewall.recipient", "email.to.address")?;
            }

            let _cond = { event.has_value("fortinet.firewall.recipient") };
            if _cond {
                event.append(
                    "email.to.address",
                    json!(
                        event
                            .get("fortinet.firewall.recipient")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("fortinet.firewall.locport") {
                    if let Some(val) = event.get("fortinet.firewall.locport") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.locport".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("source.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("fortinet.firewall.src_port") {
                        if let Some(val) = event.get("fortinet.firewall.src_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "fortinet.firewall.src_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("source.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("fortinet.firewall.srcport") {
                        if let Some(val) = event.get("fortinet.firewall.srcport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "fortinet.firewall.srcport".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("fortinet.firewall.sentbyte") {
                    if let Some(val) = event.get("fortinet.firewall.sentbyte") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.sentbyte".into(),
                                message,
                            }
                        })?;
                        event.set("source.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.srcdomain") {
                event.rename("fortinet.firewall.srcdomain", "source.domain")?;
            }

            let _cond = { !event.has_value("source.ip") };
            if _cond {
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

            let _cond = { !event.has_value("source.user.name") };
            if _cond {
                if event.has("fortinet.firewall.user") {
                    event.rename("fortinet.firewall.user", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("fortinet.firewall.sender") };
            if _cond {
                event.append(
                    "email.sender.address",
                    json!(
                        event
                            .get("fortinet.firewall.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("fortinet.firewall.from") };
            if _cond {
                event.append(
                    "email.from.address",
                    json!(
                        event
                            .get("fortinet.firewall.from")
                            .map_or_else(String::new, template_to_string)
                    ),
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

            let _cond = { !event.has_value("rule.category") };
            if _cond {
                if event.has("fortinet.firewall.catdesc") {
                    event.rename("fortinet.firewall.catdesc", "rule.category")?;
                }
            }

            let _cond = { event.has_value("rule.category") };
            if _cond {
                if event.has_value("rule.category") {
                    gsub_field(
                        event,
                        "rule.category",
                        "rule.category",
                        cached_regex!("\\."),
                        "-",
                    )?;
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

            let _cond = { !event.has_value("event.id") };
            if _cond {
                if event.has("fortinet.firewall.eventid") {
                    event.rename("fortinet.firewall.eventid", "event.id")?;
                }
            }

            if event.has("fortinet.firewall.filename") {
                event.rename("fortinet.firewall.filename", "file.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("fortinet.firewall.filesize") {
                    if let Some(val) = event.get("fortinet.firewall.filesize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "fortinet.firewall.filesize".into(),
                                message,
                            }
                        })?;
                        event.set("file.size", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("fortinet.firewall.filetype") {
                event.rename("fortinet.firewall.filetype", "file.extension")?;
            }

            let _cond = { !event.has_value("file.name") };
            if _cond {
                if event.has("fortinet.firewall.infectedfilename") {
                    event.rename("fortinet.firewall.infectedfilename", "file.name")?;
                }
            }

            let _cond = { !event.has_value("file.size") };
            if _cond {
                if event.has("fortinet.firewall.infectedfilesize") {
                    event.rename("fortinet.firewall.infectedfilesize", "file.size")?;
                }
            }

            let _cond = { !event.has_value("file.extension") };
            if _cond {
                if event.has("fortinet.firewall.infectedfiletype") {
                    event.rename("fortinet.firewall.infectedfiletype", "file.extension")?;
                }
            }

            let _cond = { !event.has_value("file.name") };
            if _cond {
                if event.has("fortinet.firewall.matchedfilename") {
                    event.rename("fortinet.firewall.matchedfilename", "file.name")?;
                }
            }

            let _cond = { !event.has_value("file.extension") };
            if _cond {
                if event.has("fortinet.firewall.matchedfiletype") {
                    event.rename("fortinet.firewall.matchedfiletype", "file.extension")?;
                }
            }

            if event.has("fortinet.firewall.ipaddr") {
                event.rename("fortinet.firewall.ipaddr", "dns.resolved_ip")?;
            }

            if event.has_value("dns.resolved_ip") {
                if let Some(s) = event.get_string("dns.resolved_ip") {
                    let parts: Vec<Value> = s.split(", ").map(|p| json!(p)).collect();
                    event.set("dns.resolved_ip", Value::Array(parts))?;
                }
            }

            if event.has("fortinet.firewall.level") {
                event.rename("fortinet.firewall.level", "log.level")?;
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

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has("fortinet.firewall.policy_id") {
                    event.rename("fortinet.firewall.policy_id", "rule.id")?;
                }
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has("fortinet.firewall.policyid") {
                    event.rename("fortinet.firewall.policyid", "rule.id")?;
                }
            }

            let _cond = { !event.has_value("rule.ruleset") };
            if _cond {
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

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("fortinet.firewall.url") };
            if _cond {
                uri_parts(event, "fortinet.firewall.url", "url", false, false)?;
            }

            if let Some(v) = event
                .get("fortinet.firewall.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.domain", v)?;
            }

            let _cond = {
                !event.has_value("url.scheme")
                    && ["http", "https"].contains(&event.get_str("network.protocol").unwrap_or(""))
                    && (event.has_value("url.domain") || event.has_value("fortinet.firewall.url"))
            };
            if _cond {
                if let Some(v) = event
                    .get("network.protocol")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.scheme", v)?;
                }
            }

            let _cond = {
                !event.has_value("url.full")
                    && event
                        .get("fortinet.firewall.url")
                        .is_some_and(|v| v.is_string())
                    && (event
                        .get_str("fortinet.firewall.url")
                        .is_some_and(|s| s.starts_with("http://"))
                        || event
                            .get_str("fortinet.firewall.url")
                            .is_some_and(|s| s.starts_with("https://")))
            };
            if _cond {
                if let Some(v) = event
                    .get("fortinet.firewall.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.full", v)?;
                }
            }

            let _cond = {
                event.has_value("url.scheme")
                    && event.has_value("url.domain")
                    && event.has_value("url.path")
                    && !event.has_value("url.full")
                    && event.has_value("url.query")
            };
            if _cond {
                event.set(
                    "url.full",
                    json!(format!(
                        "{}://{}{}?{}",
                        event
                            .get("url.scheme")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.path")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.query")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("url.scheme")
                    && event.has_value("url.domain")
                    && event.has_value("url.path")
                    && !event.has_value("url.full")
                    && !event.has_value("url.query")
            };
            if _cond {
                event.set(
                    "url.full",
                    json!(format!(
                        "{}://{}{}",
                        event
                            .get("url.scheme")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.path")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            event.remove("fortinet.firewall.hostname");

            event.remove("fortinet.firewall.url");

            if event.has("fortinet.firewall.xid") {
                event.rename("fortinet.firewall.xid", "dns.id")?;
            }

            let _cond = { event.has_value("fortinet.firewall.scertcname") };
            if _cond {
                event.append(
                    "tls.server.x509.subject.common_name",
                    json!(
                        event
                            .get("fortinet.firewall.scertcname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("fortinet.firewall.scertissuer") {
                event.rename("fortinet.firewall.scertissuer", "tls.server.issuer")?;
            }

            let _cond = { event.has_value("tls.server.issuer") };
            if _cond {
                event.append(
                    "tls.server.x509.issuer.common_name",
                    json!(
                        event
                            .get("tls.server.issuer")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("fortinet.firewall.ccertissuer") {
                event.rename("fortinet.firewall.ccertissuer", "tls.client.issuer")?;
            }

            let _cond = { event.has_value("tls.client.issuer") };
            if _cond {
                event.append(
                    "tls.client.x509.issuer.common_name",
                    json!(
                        event
                            .get("tls.client.issuer")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("fortinet.firewall.sender") {
                event.rename("fortinet.firewall.sender", "tls.server.issuer")?;
            }

            let _cond = { !event.has_value("tls.server.issuer") };
            if _cond {
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

            let _cond = { !event.has_value("tls.server.x509.public_key_algorithm") };
            if _cond {
                if event.has("fortinet.firewall.keyalgo") {
                    event.rename(
                        "fortinet.firewall.keyalgo",
                        "tls.server.x509.public_key_algorithm",
                    )?;
                }
            }

            let _cond = { !event.has_value("tls.server.not_before") };
            if _cond {
                if event.has("fortinet.firewall.notbefore") {
                    event.rename("fortinet.firewall.notbefore", "tls.server.not_before")?;
                }
            }

            let _cond = { !event.has_value("tls.server.not_after") };
            if _cond {
                if event.has("fortinet.firewall.notafter") {
                    event.rename("fortinet.firewall.notafter", "tls.server.not_after")?;
                }
            }

            let _cond = { !event.has_value("tls.server.x509.public_key_size") };
            if _cond {
                if event.has("fortinet.firewall.keysize") {
                    event.rename(
                        "fortinet.firewall.keysize",
                        "tls.server.x509.public_key_size",
                    )?;
                }
            }

            if event.has_value("tls.server.x509.public_key_size") {
                if let Some(val) = event.get("tls.server.x509.public_key_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "tls.server.x509.public_key_size".into(),
                            message,
                        }
                    })?;
                    event.set("tls.server.x509.public_key_size", converted)?;
                }
            }

            let _cond = { !event.has_value("tls.server.x509.serial_number") };
            if _cond {
                if event.has("fortinet.firewall.sn") {
                    event.rename("fortinet.firewall.sn", "tls.server.x509.serial_number")?;
                }
            }

            let _cond = { !event.has_value("tls.server.hash.sha1") };
            if _cond {
                if event.has("fortinet.firewall.certhash") {
                    event.rename("fortinet.firewall.certhash", "tls.server.hash.sha1")?;
                }
            }

            let _cond = { event.has_value("tls.server.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("tls.server.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("tls.server.not_after")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.x509.not_after", v)?;
            }

            if let Some(v) = event
                .get("tls.server.not_before")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.x509.not_before", v)?;
            }

            if event.has_value("fortinet.firewall.san") {
                if let Some(s) = event.get_string("fortinet.firewall.san") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("tls.server.x509.alternative_names", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("fortinet.firewall.cn") };
            if _cond {
                event.append_unique(
                    "tls.server.x509.alternative_names",
                    json!(
                        event
                            .get("fortinet.firewall.cn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("tls.server.x509.public_key_algorithm")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.x509.public_key_algorithm", v)?;
            }

            if event.has("fortinet.firewall.kxcurve") {
                event.rename("fortinet.firewall.kxcurve", "tls.curve")?;
            }

            if event.has("fortinet.firewall.cipher") {
                event.rename("fortinet.firewall.cipher", "tls.cipher")?;
            }

            if event.has("fortinet.firewall.sni") {
                event.rename("fortinet.firewall.sni", "tls.client.server_name")?;
            }

            let _cond = { !event.has_value("destination.domain") };
            if _cond {
                if let Some(v) = event
                    .get("tls.client.server_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.domain", v)?;
                }
            }

            let _cond = { event.get_str("fortinet.firewall.handshake") == Some("full") };
            if _cond {
                event.set("tls.established", json!(true))?;
            }

            let _cond = {
                event
                    .get("fortinet.firewall.tlsver")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // Painless script
                // Source: def pat = /\\d+/; def tlsver = ctx.fortinet.firewall.tlsver.toLowerCase(); def matcher = pat.matcher(tlsver); if (!matcher.find()) {\n    return;\n} if (ctx.tls == null) {\n    ctx.tls = new HashMap();\n} ctx.tls.version_protocol = tlsver.substring(0, matcher.start()); ctx.tls.version = tlsver.substring(matcher.start(), tlsver.length()); if (!ctx.tls.version.contains(\".\")) {\n    ctx.tls.version += \".0\";\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def pat = /\\d+/; def tlsver = ctx.fortinet.firewall.tlsver.toLowerCase(); def matcher = pat.matcher(tlsver); if (!matcher.find()) {\n    return;\n} if (ctx.tls == null) {\n    ctx.tls = new HashMap();\n} ctx.tls.version_protocol = tlsver.substring(0, matcher.start()); ctx.tls.version = tlsver.substring(matcher.start(), tlsver.length()); if (!ctx.tls.version.contains(\".\")) {\n    ctx.tls.version += \".0\";\n}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("fortinet.firewall.dtype")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.append_unique(
                    "vulnerability.category",
                    json!(
                        event
                            .get("fortinet.firewall.dtype")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("fortinet.firewall.ref") {
                event.rename("fortinet.firewall.ref", "event.reference")?;
            }

            if event.has("fortinet.firewall.filehash") {
                event.rename("fortinet.firewall.filehash", "fortinet.file.hash.crc32")?;
            }

            let _cond = { event.has_value("fortinet.file.hash.crc32") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("fortinet.file.hash.crc32")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("dns.question.name") {
                    if let Some(domain_str) = event.get_string("dns.question.name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("dns.question.registered_domain", json!(registered))?;
                            }
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
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
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
