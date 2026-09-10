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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("forwarded"))
                        }
                        serde_json::Value::String(s) => s.contains("forwarded"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.product", json!("Suricata"))?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("forwarded"))
                        }
                        serde_json::Value::String(s) => s.contains("forwarded"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.type", json!("ids"))?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("forwarded"))
                        }
                        serde_json::Value::String(s) => s.contains("forwarded"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.vendor", json!("OISF"))?;
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    if !event.has("event.created") {
                        event.set("event.created", v)?;
                    }
                }
                Ok(())
            })();

            parse_json_field(event, "event.original", "suricata.eve")?;

            if event.has_value("suricata.eve.ether.dest_mac") {
                event.rename("suricata.eve.ether.dest_mac", "destination.mac")?;
            }

            if event.has_value("suricata.eve.ether.src_mac") {
                event.rename("suricata.eve.ether.src_mac", "source.mac")?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            let _cond = { event.has_value("host.mac") };
            if _cond {
                // Painless script
                // Source: def fixup(ArrayList macs) {\n  for (def i = 0; i < macs.length; i++) {\n    macs[i] = macs[i].replace(':','-').toUpperCase();\n  }\n  return macs;\n}\nctx.host['mac'] = fixup(ctx.host?.mac);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def fixup(ArrayList macs) {\n  for (def i = 0; i < macs.length; i++) {\n    macs[i] = macs[i].replace(':','-').toUpperCase();\n  }\n  return macs;\n}\nctx.host['mac'] = fixup(ctx.host?.mac);\n"#
                    ),
                )?;
            }

            if event.has_value("suricata.eve.src_ip") {
                event.rename("suricata.eve.src_ip", "source.address")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("suricata.eve.src_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "suricata.eve.src_port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
                Ok(())
            })();

            if event.has_value("suricata.eve.dest_ip") {
                event.rename("suricata.eve.dest_ip", "destination.address")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("destination.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("suricata.eve.dest_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "suricata.eve.dest_port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
                Ok(())
            })();

            if event.has_value("suricata.eve.proto") {
                event.rename("suricata.eve.proto", "network.transport")?;
            }

            if event.has_value("suricata.eve.flow_id") {
                if let Some(val) = event.get("suricata.eve.flow_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "suricata.eve.flow_id".into(),
                            message,
                        }
                    })?;
                    event.set("suricata.eve.flow_id", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("@timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "@timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if let Some(date_str) = event.get_as_string("suricata.eve.timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "suricata.eve.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("suricata.eve.dns.rrname") {
                if let Some(domain) = event.get_string("suricata.eve.dns.rrname") {
                    // Public suffix list lookup for registered domain extraction.
                    // A failed lookup writes NO target field, which is what
                    // Elasticsearch does.
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set("dns.question.domain", json!(domain))?;
                        if let Some(registered) = rd.registered_domain {
                            event.set("dns.question.registered_domain", json!(registered))?;
                        }
                        event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("dns.question.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            if event.has_value("suricata.eve.event_type") {
                map_strings(
                    event,
                    "suricata.eve.event_type",
                    "suricata.eve.event_type",
                    str::to_lowercase,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.event.kind = 'event';\nctx.event.category = ['network'];\ndef type_params = params.get(ctx?.suricata?.eve?.event_type);\nif (type_params == null) {\n    return;\n}\ntype_params.forEach((k, v) -> {\n    if ('network_protocol' == k) {\n        if (ctx.network == null) {\n            ctx.network = ['protocol': v];\n        } else {\n            ctx.network.protocol = v;\n        }\n    } else if (v instanceof List) {\n        ctx.event[k] = new ArrayList(v);\n    } else {\n        ctx.event[k] = v;\n    }\n});\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.kind = 'event';\nctx.event.category = ['network'];\ndef type_params = params.get(ctx?.suricata?.eve?.event_type);\nif (type_params == null) {\n    return;\n}\ntype_params.forEach((k, v) -> {\n    if ('network_protocol' == k) {\n        if (ctx.network == null) {\n            ctx.network = ['protocol': v];\n        } else {\n            ctx.network.protocol = v;\n        }\n    } else if (v instanceof List) {\n        ctx.event[k] = new ArrayList(v);\n    } else {\n        ctx.event[k] = v;\n    }\n});\n"#
                    ),
                    cached_params!(
                        "{\"alert\":{\"kind\":\"alert\",\"category\":[\"network\",\"intrusion_detection\"]},\"dns\":{\"type\":[\"protocol\"],\"network_protocol\":\"dns\"},\"flow\":{\"type\":[\"connection\"]},\"ftp\":{\"type\":[\"protocol\"],\"network_protocol\":\"ftp\"},\"ftp_data\":{\"type\":[\"protocol\"],\"network_protocol\":\"ftp\"},\"http\":{\"category\":[\"network\",\"web\"],\"type\":[\"access\",\"protocol\"],\"network_protocol\":\"http\"},\"http2\":{\"category\":[\"network\",\"web\"],\"type\":[\"access\",\"protocol\"],\"network_protocol\":\"http\"},\"ikev2\":{\"type\":[\"protocol\"],\"network_protocol\":\"ikev2\"},\"krb5\":{\"type\":[\"protocol\"],\"network_protocol\":\"krb5\"},\"mqtt\":{\"type\":[\"protocol\"],\"network_protocol\":\"mqtt\"},\"smb\":{\"type\":[\"protocol\"],\"network_protocol\":\"smb\"},\"smtp\":{\"type\":[\"protocol\"],\"network_protocol\":\"smtp\"},\"snmp\":{\"type\":[\"protocol\"],\"network_protocol\":\"snmp\"},\"ssh\":{\"type\":[\"protocol\"],\"network_protocol\":\"ssh\"},\"stats\":{\"kind\":\"metric\"},\"tftp\":{\"type\":[\"protocol\"],\"network_protocol\":\"tftp\"},\"tls\":{\"type\":[\"protocol\"],\"network_protocol\":\"tls\"},\"rdp\":{\"type\":[\"protocol\"],\"network_protocol\":\"rdp\"},\"rfb\":{\"type\":[\"protocol\"],\"network_protocol\":\"rdp\"}}"
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("suricata.eve.app_proto") {
                map_strings(
                    event,
                    "suricata.eve.app_proto",
                    "suricata.eve.app_proto",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("suricata.eve.app_proto") == Some("ftp-data") };
            if _cond {
                event.set("network.protocol", json!("ftp"))?;
            }

            let _cond = {
                event.get_str("suricata.eve.app_proto") != Some("failed")
                    && event.get_str("suricata.eve.app_proto") != Some("template")
                    && event.get_str("suricata.eve.app_proto") != Some("template-rust")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("suricata.eve.app_proto").cloned() {
                        event.set("network.protocol", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("suricata.eve.event_type") == Some("http")
                    && event.has_value("suricata.eve.http.status")
                    && event
                        .get_i64("suricata.eve.http.status")
                        .is_some_and(|n| n < 400)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("suricata.eve.event_type") == Some("http")
                    && event.has_value("suricata.eve.http.status")
                    && event
                        .get_i64("suricata.eve.http.status")
                        .is_some_and(|n| n >= 400)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.has_value("suricata.eve.http.http_port") };
            if _cond {
                if let Some(val) = event.get("suricata.eve.http.http_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "suricata.eve.http.http_port".into(),
                            message,
                        }
                    })?;
                    event.set("suricata.eve.http.http_port", converted)?;
                }
            }

            let _cond = { event.get_str("network.protocol") == Some("dns") };
            if _cond {
                // Begin nested pipeline: "dns"
                let v = json!(
                    event
                        .get("suricata.eve.dns.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("dns.id", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.dns.rcode")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("dns.response_code", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.dns.type")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("dns.type", v)?;
                }
                let _cond = {
                    event.get_str("dns.type") == Some("query")
                        || event.get_i64("suricata.eve.dns.version") == Some(2)
                };
                if _cond {
                    let v = json!(
                        event
                            .get("suricata.eve.dns.rrname")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("dns.question.name", v)?;
                    }
                }
                let _cond = {
                    event.get_str("dns.type") == Some("query")
                        || event.get_i64("suricata.eve.dns.version") == Some(2)
                };
                if _cond {
                    let v = json!(
                        event
                            .get("suricata.eve.dns.rrtype")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("dns.question.type", v)?;
                    }
                }
                let _cond = {
                    event.get_str("dns.type") == Some("answer")
                        && !event.has_value("suricata.eve.dns.version")
                };
                if _cond {
                    // Begin nested pipeline: "dns-answer-v1"
                    // Painless script
                    // Source: def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n"#
                        ),
                    )?;
                    // End nested pipeline: "dns-answer-v1"
                }
                let _cond = {
                    event.get_str("dns.type") == Some("answer")
                        && event.get_i64("suricata.eve.dns.version") == Some(2)
                };
                if _cond {
                    // Begin nested pipeline: "dns-answer-v2"
                    if event.has_value("suricata.eve.dns.answers") {
                        event.rename("suricata.eve.dns.answers", "dns.answers")?;
                    }
                    let _cond = { event.has_value("dns.answers") };
                    if _cond {
                        // Painless script
                        // Source: def resolvedIps = new ArrayList();\nfor (def answer : ctx?.dns?.answers) {\n    // Normalize field names to match ECS.\n    def name = answer.remove(\"rrname\");\n    if (name != null) {\n        answer[\"name\"] = name;\n    }\n    def type = answer.remove(\"rrtype\");\n    if (type != null) {\n        answer[\"type\"] = type;\n    }\n    def data = answer.remove(\"rdata\");\n    if (data != null) {\n        answer[\"data\"] = data;\n    }\n\n    if (type == \"A\" || type == \"AAAA\") {\n        resolvedIps.add(data);\n    }\n}\n\nif (resolvedIps.size() > 0) {\n    ctx.dns.resolved_ip = resolvedIps;\n}\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def resolvedIps = new ArrayList();\nfor (def answer : ctx?.dns?.answers) {\n    // Normalize field names to match ECS.\n    def name = answer.remove(\"rrname\");\n    if (name != null) {\n        answer[\"name\"] = name;\n    }\n    def type = answer.remove(\"rrtype\");\n    if (type != null) {\n        answer[\"type\"] = type;\n    }\n    def data = answer.remove(\"rdata\");\n    if (data != null) {\n        answer[\"data\"] = data;\n    }\n\n    if (type == \"A\" || type == \"AAAA\") {\n        resolvedIps.add(data);\n    }\n}\n\nif (resolvedIps.size() > 0) {\n    ctx.dns.resolved_ip = resolvedIps;\n}\n"#
                            ),
                        )?;
                    }
                    // End nested pipeline: "dns-answer-v2"
                }
                if event.has_value("dns.resolved_ip") {
                    foreach_array(event, "dns.resolved_ip", |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("dns.question.registered_domain") };
                if _cond {
                    // Painless script
                    // Source: def rd = ctx.dns.question.registered_domain;\ndef firstDot = rd.indexOf(\".\");\nif (firstDot == -1) {\n    return;\n}\nctx.dns.question.top_level_domain = rd.substring(firstDot + 1);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def rd = ctx.dns.question.registered_domain;\ndef firstDot = rd.indexOf(\".\");\nif (firstDot == -1) {\n    return;\n}\nctx.dns.question.top_level_domain = rd.substring(firstDot + 1);\n"#
                        ),
                    )?;
                }
                let _cond = { event.get_bool("suricata.eve.dns.aa") == Some(true) };
                if _cond {
                    event.append("dns.header_flags", json!("AA"))?;
                }
                let _cond = { event.get_bool("suricata.eve.dns.tc") == Some(true) };
                if _cond {
                    event.append("dns.header_flags", json!("TC"))?;
                }
                let _cond = { event.get_bool("suricata.eve.dns.rd") == Some(true) };
                if _cond {
                    event.append("dns.header_flags", json!("RD"))?;
                }
                let _cond = { event.get_bool("suricata.eve.dns.ra") == Some(true) };
                if _cond {
                    event.append("dns.header_flags", json!("RA"))?;
                }
                event.remove("suricata.eve.dns.aa");
                event.remove("suricata.eve.dns.tc");
                event.remove("suricata.eve.dns.rd");
                event.remove("suricata.eve.dns.ra");
                event.remove("suricata.eve.dns.qr");
                event.remove("suricata.eve.dns.version");
                event.remove("suricata.eve.dns.flags");
                event.remove("suricata.eve.dns.grouped");
                // End nested pipeline: "dns"
            }

            let _cond = {
                event.get_str("network.protocol") == Some("tls")
                    && event.has_value("suricata.eve.tls")
            };
            if _cond {
                // Begin nested pipeline: "tls"
                let _cond = { event.get_str("suricata.eve.tls.version") != Some("UNDETERMINED") };
                if _cond {
                    if let Some(input) = event.get_string("suricata.eve.tls.version") {
                        // Grok pattern: %{DATA:tls.version_protocol} %{GREEDYDATA:tls.version}
                        // Grok pattern: %{DATA:tls.version_protocol}v%{GREEDYDATA:tls.version}
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:tls.version_protocol} %{GREEDYDATA:tls.version}"
                                ),
                                cached_grok!(
                                    "%{DATA:tls.version_protocol}v%{GREEDYDATA:tls.version}"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                if event.has_value("tls.version_protocol") {
                    map_strings(
                        event,
                        "tls.version_protocol",
                        "tls.version_protocol",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.has_value("suricata.eve.tls.sni") };
                if _cond {
                    // Painless script
                    // Source: def sni = ctx.suricata.eve.tls.sni;\nif (!sni.endsWith(\".\")) {\n    return;\n}\nctx.suricata.eve.tls.sni = sni.substring(0, sni.length() - 1);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def sni = ctx.suricata.eve.tls.sni;\nif (!sni.endsWith(\".\")) {\n    return;\n}\nctx.suricata.eve.tls.sni = sni.substring(0, sni.length() - 1);\n"#
                        ),
                    )?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.subject")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.subject", v)?;
                }
                if event.has_value("suricata.eve.tls.subject") {
                    if let Some(kv_str) = event.get_string("suricata.eve.tls.subject") {
                        for pair in cached_regex!(", (?=[a-zA-Z]+=)").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "suricata.eve.tls.subject".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("suricata.eve.tls.kv_subject.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("suricata.eve.tls.kv_subject.C") {
                    event.rename(
                        "suricata.eve.tls.kv_subject.C",
                        "tls.server.x509.subject.country",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.subject.country")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.subject.country",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.subject.country")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_subject.CN") {
                    event.rename(
                        "suricata.eve.tls.kv_subject.CN",
                        "tls.server.x509.subject.common_name",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.subject.common_name")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.subject.common_name",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.subject.common_name")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_subject.L") {
                    event.rename(
                        "suricata.eve.tls.kv_subject.L",
                        "tls.server.x509.subject.locality",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.subject.locality")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.subject.locality",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.subject.locality")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_subject.O") {
                    event.rename(
                        "suricata.eve.tls.kv_subject.O",
                        "tls.server.x509.subject.organization",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.subject.organization")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.subject.organization",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.subject.organization")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_subject.OU") {
                    event.rename(
                        "suricata.eve.tls.kv_subject.OU",
                        "tls.server.x509.subject.organizational_unit",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.subject.organizational_unit")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.subject.organizational_unit",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.subject.organizational_unit")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_subject.ST") {
                    event.rename(
                        "suricata.eve.tls.kv_subject.ST",
                        "tls.server.x509.subject.state_or_province",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.subject.state_or_province")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.subject.state_or_province",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.subject.state_or_province")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.issuerdn")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.issuer", v)?;
                }
                if event.has_value("suricata.eve.tls.issuerdn") {
                    gsub_field(
                        event,
                        "suricata.eve.tls.issuerdn",
                        "suricata.eve.tls.issuerdn",
                        cached_regex!("\\\\,"),
                        "",
                    )?;
                }
                if event.has_value("suricata.eve.tls.issuerdn") {
                    if let Some(kv_str) = event.get_string("suricata.eve.tls.issuerdn") {
                        for pair in cached_regex!(", (?=[a-zA-Z]+=)").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "suricata.eve.tls.issuerdn".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("suricata.eve.tls.kv_issuerdn.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("suricata.eve.tls.kv_issuerdn.C") {
                    event.rename(
                        "suricata.eve.tls.kv_issuerdn.C",
                        "tls.server.x509.issuer.country",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.issuer.country")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.issuer.country",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.issuer.country")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_issuerdn.CN") {
                    event.rename(
                        "suricata.eve.tls.kv_issuerdn.CN",
                        "tls.server.x509.issuer.common_name",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.issuer.common_name")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.issuer.common_name",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.issuer.common_name")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_issuerdn.L") {
                    event.rename(
                        "suricata.eve.tls.kv_issuerdn.L",
                        "tls.server.x509.issuer.locality",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.issuer.locality")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.issuer.locality",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.issuer.locality")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_issuerdn.O") {
                    event.rename(
                        "suricata.eve.tls.kv_issuerdn.O",
                        "tls.server.x509.issuer.organization",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.issuer.organization")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.issuer.organization",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.issuer.organization")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_issuerdn.OU") {
                    event.rename(
                        "suricata.eve.tls.kv_issuerdn.OU",
                        "tls.server.x509.issuer.organizational_unit",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.issuer.organizational_unit")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.issuer.organizational_unit",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.issuer.organizational_unit")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.kv_issuerdn.ST") {
                    event.rename(
                        "suricata.eve.tls.kv_issuerdn.ST",
                        "tls.server.x509.issuer.state_or_province",
                    )?;
                }
                let _cond = {
                    event
                        .get("tls.server.x509.issuer.state_or_province")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.server.x509.issuer.state_or_province",
                        Value::Array(vec![json!(
                            event
                                .get("tls.server.x509.issuer.state_or_province")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("suricata.eve.tls.session_resumed") {
                    if let Some(val) = event.get("suricata.eve.tls.session_resumed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "suricata.eve.tls.session_resumed".into(),
                                message,
                            }
                        })?;
                        event.set("tls.resumed", converted)?;
                    }
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.fingerprint")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.hash.sha1", v)?;
                }
                if event.has_value("tls.server.hash.sha1") {
                    map_strings(
                        event,
                        "tls.server.hash.sha1",
                        "tls.server.hash.sha1",
                        str::to_uppercase,
                    )?;
                }
                if event.has_value("tls.server.hash.sha1") {
                    if let Some(s) = event.get_string("tls.server.hash.sha1") {
                        let mut parts: Vec<Value> = s.split(":").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("tls.server.hash.sha1", Value::Array(parts))?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let joined = event
                        .get("tls.server.hash.sha1")
                        .and_then(|v| join_values(v, ""));
                    if let Some(joined) = joined {
                        event.set("tls.server.hash.sha1", json!(joined))?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("tls.server.hash.sha1") };
                if _cond {
                    event.append(
                        "related.hash",
                        json!(
                            event
                                .get("tls.server.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.sni")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.client.server_name", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.sni")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.domain", v)?;
                }
                let _cond = {
                    event.has_value("suricata.eve.tls.sni")
                        && event.get_str("suricata.eve.tls.sni") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("suricata.eve.tls.sni")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.ja3s.hash")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.ja3s", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.ja3.hash")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.client.ja3", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.certificate")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.certificate", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.chain")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.certificate_chain", v)?;
                }
                let v = json!(
                    event
                        .get("suricata.eve.tls.serial")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.x509.serial_number", v)?;
                }
                if event.has_value("tls.server.x509.serial_number") {
                    gsub_field(
                        event,
                        "tls.server.x509.serial_number",
                        "tls.server.x509.serial_number",
                        cached_regex!(":"),
                        "",
                    )?;
                }
                let _cond = { event.has_value("suricata.eve.tls.notafter") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("suricata.eve.tls.notafter") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tls.server.not_after", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "suricata.eve.tls.notafter".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { event.has_value("suricata.eve.tls.notbefore") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("suricata.eve.tls.notbefore") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tls.server.not_before", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "suricata.eve.tls.notbefore".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let v = json!(
                    event
                        .get("tls.server.not_after")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.x509.not_after", v)?;
                }
                let v = json!(
                    event
                        .get("tls.server.not_before")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("tls.server.x509.not_before", v)?;
                }
                event.remove("suricata.eve.tls.kv_issuerdn");
                event.remove("suricata.eve.tls.kv_subject");
                // End nested pipeline: "tls"
            }

            let _cond = { event.get_str("suricata.eve.flow.state") == Some("new") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("suricata.eve.flow.state") == Some("closed") };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("suricata.eve.http.http_method").cloned() {
                    event.set("http.request.method", v)?;
                }
                Ok(())
            })();

            if event.has_value("suricata.eve.http.status") {
                event.rename("suricata.eve.http.status", "http.response.status_code")?;
            }

            let _cond = { event.has_value("suricata.eve.http.hostname") };
            if _cond {
                event.append_unique(
                    "destination.domain",
                    json!(
                        event
                            .get("suricata.eve.http.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("suricata.eve.http.hostname").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "suricata.eve.http.hostname".into(),
                    });
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def domain = ctx.destination?.domain; if (domain instanceof Collection) {\n\n\n  domain = domain.stream().distinct().collect(Collectors.toList());\n  if (domain.length == 1) {\n    domain = domain[0];\n  }\n  ctx.destination.domain = domain;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def domain = ctx.destination?.domain; if (domain instanceof Collection) {\n\n\n  domain = domain.stream().distinct().collect(Collectors.toList());\n  if (domain.length == 1) {\n    domain = domain[0];\n  }\n  ctx.destination.domain = domain;\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.get_str("network.protocol") == Some("http") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("destination.domain").cloned() {
                        event.set("url.domain", v)?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("suricata.eve.http.url") {
                if let Some(input) = event.get_string("suricata.eve.http.url") {
                    // Grok pattern: (?P<url_path>(?:[^?#]*))(?:\\?(?P<url_query>(?:[^#]*)))?(?:#(?P<url_fragment>(?:.*)))?
                    if !cached_grok_mapped!("(?P<url_path>(?:[^?#]*))(?:\\?(?P<url_query>(?:[^#]*)))?(?:#(?P<url_fragment>(?:.*)))?", [("url_path", "url.path"), ("url_query", "url.query"), ("url_fragment", "url.fragment")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("suricata.eve.http.url") {
                event.rename("suricata.eve.http.url", "url.original")?;
            }

            if event.has_value("suricata.eve.http.http_refer") {
                event.rename("suricata.eve.http.http_refer", "http.request.referrer")?;
            }

            if event.has_value("suricata.eve.http.length") {
                event.rename("suricata.eve.http.length", "http.response.body.bytes")?;
            }

            if event.has_value("suricata.eve.fileinfo.filename") {
                event.rename("suricata.eve.fileinfo.filename", "file.path")?;
            }

            if event.has_value("suricata.eve.fileinfo.size") {
                event.rename("suricata.eve.fileinfo.size", "file.size")?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("suricata.eve.alert.category") {
                if let Some(val) = event.get("suricata.eve.alert.category") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "suricata.eve.alert.category".into(),
                            message,
                        }
                    })?;
                    event.set("message", converted)?;
                }
            }

            let v = json!(
                event
                    .get("suricata.eve.alert.category")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.category", v)?;
            }

            let v = json!(
                event
                    .get("suricata.eve.alert.signature_id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.id", v)?;
            }

            let v = json!(
                event
                    .get("suricata.eve.alert.signature")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.name", v)?;
            }

            let _cond = { event.get_str("suricata.eve.alert.action") == Some("blocked") };
            if _cond {
                event.set("suricata.eve.alert.action", json!("denied"))?;
            }

            let _cond = { event.has_value("suricata.eve.alert.action") };
            if _cond {
                event.append(
                    "event.type",
                    json!(
                        event
                            .get("suricata.eve.alert.action")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("suricata.eve.alert.action").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "suricata.eve.alert.action".into(),
                    });
                }
                Ok(())
            })();

            if event.has_value("suricata.eve.alert.severity") {
                event.rename("suricata.eve.alert.severity", "event.severity")?;
            }

            if event.has_value("suricata.eve.alert.metadata.protocols") {
                event.rename(
                    "suricata.eve.alert.metadata.protocols",
                    "suricata.eve.alert.protocols",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.attack_target") {
                event.rename(
                    "suricata.eve.alert.metadata.attack_target",
                    "suricata.eve.alert.attack_target",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.capec_id") {
                event.rename(
                    "suricata.eve.alert.metadata.capec_id",
                    "suricata.eve.alert.capec_id",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.cwe_id") {
                event.rename(
                    "suricata.eve.alert.metadata.cwe_id",
                    "suricata.eve.alert.cwe_id",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.malware") {
                event.rename(
                    "suricata.eve.alert.metadata.malware",
                    "suricata.eve.alert.malware",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.cve") {
                event.rename("suricata.eve.alert.metadata.cve", "suricata.eve.alert.cve")?;
            }

            if event.has_value("suricata.eve.alert.metadata.cvss_v2_base") {
                event.rename(
                    "suricata.eve.alert.metadata.cvss_v2_base",
                    "suricata.eve.alert.cvss_v2_base",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.cvss_v2_temporal") {
                event.rename(
                    "suricata.eve.alert.metadata.cvss_v2_temporal",
                    "suricata.eve.alert.cvss_v2_temporal",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.cvss_v3_base") {
                event.rename(
                    "suricata.eve.alert.metadata.cvss_v3_base",
                    "suricata.eve.alert.cvss_v3_base",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.cvss_v3_temporal") {
                event.rename(
                    "suricata.eve.alert.metadata.cvss_v3_temporal",
                    "suricata.eve.alert.cvss_v3_temporal",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.priority") {
                event.rename(
                    "suricata.eve.alert.metadata.priority",
                    "suricata.eve.alert.priority",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.hostile") {
                event.rename(
                    "suricata.eve.alert.metadata.hostile",
                    "suricata.eve.alert.hostile",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.infected") {
                event.rename(
                    "suricata.eve.alert.metadata.infected",
                    "suricata.eve.alert.infected",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.created_at") {
                event.rename("suricata.eve.alert.metadata.created_at", "_tmp_.created_at")?;
            }

            let _cond = { event.has_value("_tmp_.created_at") };
            if _cond {
                let joined = event
                    .get("_tmp_.created_at")
                    .and_then(|v| join_values(v, ","));
                if let Some(joined) = joined {
                    event.set("_tmp_.created_at", json!(joined))?;
                }
            }

            let _cond = { event.has_value("_tmp_.created_at") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp_.created_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd", "yyyy_MM_dd"], None, None) {
                            Some(parsed) => event.set("suricata.eve.alert.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp_.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("suricata.eve.alert.metadata.updated_at") {
                event.rename("suricata.eve.alert.metadata.updated_at", "_tmp_.updated_at")?;
            }

            let _cond = { event.has_value("_tmp_.updated_at") };
            if _cond {
                let joined = event
                    .get("_tmp_.updated_at")
                    .and_then(|v| join_values(v, ","));
                if let Some(joined) = joined {
                    event.set("_tmp_.updated_at", json!(joined))?;
                }
            }

            let _cond = { event.has_value("_tmp_.updated_at") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp_.updated_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd", "yyyy_MM_dd"], None, None) {
                            Some(parsed) => event.set("suricata.eve.alert.updated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp_.updated_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("suricata.eve.alert.metadata.filename") {
                event.rename("suricata.eve.alert.metadata.filename", "file.name")?;
            }

            if event.has_value("suricata.eve.alert.metadata.classtype") {
                event.rename(
                    "suricata.eve.alert.metadata.classtype",
                    "suricata.eve.alert.classtype",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.rule_source") {
                event.rename(
                    "suricata.eve.alert.metadata.rule_source",
                    "suricata.eve.alert.rule_source",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.sid") {
                event.rename("suricata.eve.alert.metadata.sid", "suricata.eve.alert.sid")?;
            }

            if event.has_value("suricata.eve.alert.metadata.mitre_attack") {
                event.rename(
                    "suricata.eve.alert.metadata.mitre_attack",
                    "threat.tactic.id",
                )?;
            }

            let _cond = { !event.has_value("threat.tactic.id") };
            if _cond {
                if event.has_value("suricata.eve.alert.metadata.mitre_tactic_id") {
                    event.rename(
                        "suricata.eve.alert.metadata.mitre_tactic_id",
                        "threat.tactic.id",
                    )?;
                }
            }

            if event.has_value("suricata.eve.alert.metadata.mitre_tactic_name") {
                event.rename(
                    "suricata.eve.alert.metadata.mitre_tactic_name",
                    "threat.tactic.name",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.mitre_technique_id") {
                event.rename(
                    "suricata.eve.alert.metadata.mitre_technique_id",
                    "threat.technique.id",
                )?;
            }

            if event.has_value("suricata.eve.alert.metadata.mitre_technique_id") {
                event.rename(
                    "suricata.eve.alert.metadata.mitre_technique_id",
                    "threat.technique.name",
                )?;
            }

            if event.has_value("suricata.eve.flow.pkts_toclient") {
                event.rename("suricata.eve.flow.pkts_toclient", "destination.packets")?;
            }

            if event.has_value("suricata.eve.flow.pkts_toserver") {
                event.rename("suricata.eve.flow.pkts_toserver", "source.packets")?;
            }

            if event.has_value("suricata.eve.flow.bytes_toclient") {
                event.rename("suricata.eve.flow.bytes_toclient", "destination.bytes")?;
            }

            if event.has_value("suricata.eve.flow.bytes_toserver") {
                event.rename("suricata.eve.flow.bytes_toserver", "source.bytes")?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: long getOrZero(def map, def key) {\n  if (map!=null && map[key]!=null) {\n    return map[key];\n  }\n  return 0;\n} def network=ctx['network'], source=ctx['source'], dest=ctx['destination']; def sp=getOrZero(source,'packets'), sb=getOrZero(source,'bytes'), dp=getOrZero(dest,'packets'), db=getOrZero(dest,'bytes'); if (sb+db+sp+dp > 0) {\n  if (network == null) {\n    network=new HashMap();\n    ctx['network']=network;\n  }\n  if (sb+db > 0) {\n    network['bytes'] = sb+db;\n  }\n  if(sp+dp>0) {\n    network['packets'] = sp+dp;\n  }\n}\n
            sum_totals(
                event,
                &SumTotals::new(vec![
                    Total::new("network.bytes", "source.bytes", "destination.bytes"),
                    Total::new("network.packets", "source.packets", "destination.packets"),
                ]),
            );

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("suricata.eve.flow.start") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "suricata.eve.flow.start".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("suricata.eve.flow.end") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "suricata.eve.flow.end".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // Painless script
            // Source: Instant ins(def d) {\n  try {\n    return Instant.parse(d);\n  } catch(Exception e) {\n    return null;\n  }\n} def ev = ctx['event']; if (ev != null) {\n  def start = ins(ev['start']);\n  def end = ins(ev['end']);\n  if (start != null && end != null && !start.isAfter(end)) {\n    ev['duration'] = Duration.between(start,end).toNanos();\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Instant ins(def d) {\n  try {\n    return Instant.parse(d);\n  } catch(Exception e) {\n    return null;\n  }\n} def ev = ctx['event']; if (ev != null) {\n  def start = ins(ev['start']);\n  def end = ins(ev['end']);\n  if (start != null && end != null && !start.isAfter(end)) {\n    ev['duration'] = Duration.between(start,end).toNanos();\n  }\n}\n"#
                ),
            )?;

            if event.has_value("suricata.eve.proto") {
                map_strings(
                    event,
                    "suricata.eve.proto",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("suricata.eve.http.http_user_agent") {
                if let Some(ua_str) = event.get_string("suricata.eve.http.http_user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            let _cond = { !event.has_value("source.geo") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
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
            }

            let _cond = { !event.has_value("destination.geo") };
            if _cond {
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
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
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
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

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond =
                { event.has_value("url.domain") && event.get_str("url.domain") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.server.ja3s") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("tls.server.ja3s")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.client.ja3") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("tls.client.ja3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("suricata.eve.alert.metadata")
                    || event
                        .get("suricata.eve.alert.metadata")
                        .is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("suricata.eve.alert.metadata");
                    Ok(())
                })();
            }

            event.remove("suricata.eve.app_proto");
            event.remove("suricata.eve.flow.end");
            event.remove("suricata.eve.flow.start");
            event.remove("suricata.eve.http.http_method");
            event.remove("suricata.eve.http.http_user_agent");
            event.remove("suricata.eve.timestamp");
            event.remove("suricata.eve.src_port");
            event.remove("suricata.eve.dest_port");
            event.remove("dns.question.domain");
            event.remove("_tmp_");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
