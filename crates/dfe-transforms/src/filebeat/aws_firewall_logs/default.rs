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
                event.rename("message", "event.original")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has("json.availability_zone") {
                event.rename("json.availability_zone", "cloud.availability_zone")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cloud.availability_zone") {
                    if let Some(input) = event.get_string("cloud.availability_zone") {
                        // Grok pattern: ^%{DATA:cloud.region}(?:[a-z]+)$
                        let _ = cached_grok!("^%{DATA:cloud.region}(?:[a-z]+)$")
                            .extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            if event.has("json.firewall_name") {
                event.rename("json.firewall_name", "observer.name")?;
            }

            event.set("observer.type", json!("firewall"))?;

            event.set("observer.vendor", json!("AWS"))?;

            event.set("observer.product", json!("Network Firewall"))?;

            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("connection"))?;

            let _cond = { event.get_str("json.event.event_type") == Some("netflow") };
            if _cond {
                event.set("json.event.event_type", json!("event"))?;
            }

            let _cond = { event.has_value("json.event.event_type") };
            if _cond {
                event.set(
                    "event.kind",
                    json!(
                        event
                            .get("json.event.event_type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.event.alert.action") == Some("blocked") };
            if _cond {
                event.set("json.event.alert.action", json!("denied"))?;
            }

            let _cond = { event.has_value("json.event.alert.action") };
            if _cond {
                event.append(
                    "event.type",
                    json!(
                        event
                            .get("json.event.alert.action")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.src_ip") {
                if let Some(val) = event.get("json.event.src_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.event.src_ip".into(),
                            message,
                        })?;
                    event.set("source.address", converted)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("json.event.src_port") };
            if _cond {
                if let Some(val) = event.get("json.event.src_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.src_port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = {
                !event.has_value("network.type")
                    && event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                !event.has_value("network.type")
                    && event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            if event.has_value("json.event.dest_ip") {
                if let Some(val) = event.get("json.event.dest_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.event.dest_ip".into(),
                            message,
                        })?;
                    event.set("destination.address", converted)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            let _cond = { event.has_value("json.event.dest_port") };
            if _cond {
                if let Some(val) = event.get("json.event.dest_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.dest_port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

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

            if event.has("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            if event.has("json.event.proto") {
                event.rename("json.event.proto", "network.transport")?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.event.alert.category") {
                if let Some(val) = event.get("json.event.alert.category") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.alert.category".into(),
                            message,
                        }
                    })?;
                    event.set("message", converted)?;
                }
            }

            let v = json!(
                event
                    .get("json.event.alert.category")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.category", v)?;
            }

            let v = json!(
                event
                    .get("json.event.alert.signature_id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.id", v)?;
            }

            let v = json!(
                event
                    .get("json.event.alert.signature")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.name", v)?;
            }

            let _cond = { !event.has_value("rule.name") && event.has_value("rule.id") };
            if _cond {
                event.set("rule.name", json!("rule.id"))?;
            }

            if event.has("json.event.alert.rev_id") {
                event.rename("json.event.alert.rev_id", "rule.version")?;
            }

            if event.has("json.event.alert.severity") {
                event.rename("json.event.alert.severity", "event.severity")?;
            }

            if event.has("json.event.app_proto") {
                event.rename("json.event.app_proto", "network.protocol")?;
            }

            let _cond = {
                !event.has_value("network.protocol")
                    || event.get_str("network.protocol") == Some("failed")
            };
            if _cond {
                event.set("network.protocol", json!("unknown"))?;
            }

            if event.has("json.event.http.hostname") {
                event.rename("json.event.http.hostname", "destination.domain")?;
            }

            let _cond = { event.has_value("json.event.http.url") };
            if _cond {
                uri_parts(event, "json.event.http.url", "url", true, false)?;
            }

            if event.has("json.event.http.http_method") {
                event.rename("json.event.http.http_method", "http.request.method")?;
            }

            if event.has_value("json.event.http.http_user_agent") {
                if let Some(ua_str) = event.get_string("json.event.http.http_user_agent") {
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

            if event.has_value("json.event.http.protocol") {
                if let Some(input) = event.get_string("json.event.http.protocol") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("HTTP/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("http.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "json.event.http.protocol".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            if event.has("json.event.tls.sni") {
                event.rename("json.event.tls.sni", "tls.client.server_name")?;
            }

            let _cond = { event.has_value("tls.client.server_name") };
            if _cond {
                if let Some(v) = event.get("tls.client.server_name").cloned() {
                    event.set("destination.domain", v)?;
                }
            }

            let _cond = { event.get_str("json.event.tls.version") != Some("UNDETERMINED") };
            if _cond {
                if event.has_value("json.event.tls.version") {
                    if let Some(input) = event.get_string("json.event.tls.version") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("tls.version_protocol", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("tls.version", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "json.event.tls.version".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
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

            if event.has("json.event.tls.ja3s.hash") {
                event.rename("json.event.tls.ja3s.hash", "tls.server.ja3s")?;
            }

            if event.has("json.event.tls.ja3.hash") {
                event.rename("json.event.tls.ja3.hash", "tls.server.ja3")?;
            }

            if event.has("json.event.tls.certificate") {
                event.rename("json.event.tls.certificate", "tls.server.certificate")?;
            }

            if event.has("tls.server.certificate_chain") {
                event.rename("tls.server.certificate_chain", "json.event.tls.chain")?;
            }

            if event.has("tls.server.x509.serial_number") {
                event.rename("tls.server.x509.serial_number", "json.event.tls.serial")?;
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

            let _cond = { event.has_value("json.event.tls.notafter") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.event.tls.notafter") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tls.server.not_after", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event.tls.notafter".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.event.tls.notbefore") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.event.tls.notbefore") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tls.server.not_before", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event.tls.notbefore".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has("tls.server.not_after") {
                event.rename("tls.server.not_after", "tls.server.x509.not_after")?;
            }

            if event.has("tls.server.not_before") {
                event.rename("tls.server.not_before", "tls.server.x509.not_before")?;
            }

            if event.has("json.event.tcp.tcp_flags") {
                event.rename("json.event.tcp.tcp_flags", "aws.firewall.tcp_flags")?;
            }

            let _cond = { event.has_value("aws.firewall.tcp_flags") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.aws.firewall.tcp_flags_array == null) {\n  ArrayList al = new ArrayList();\n  ctx.aws.firewall.put(\"tcp_flags_array\", al);\n}\n\ndef flags = Integer.parseUnsignedInt(ctx.aws.firewall.tcp_flags);\n\nif ((flags & 0x01) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"fin\");\n}\nif ((flags & 0x02) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"syn\");\n}\nif ((flags & 0x04) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"rst\");\n}\nif ((flags & 0x08) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"psh\");\n}\nif ((flags & 0x10) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"ack\");\n}\nif ((flags & 0x20) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"urg\");\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.aws.firewall.tcp_flags_array == null) {\n  ArrayList al = new ArrayList();\n  ctx.aws.firewall.put(\"tcp_flags_array\", al);\n}\n\ndef flags = Integer.parseUnsignedInt(ctx.aws.firewall.tcp_flags);\n\nif ((flags & 0x01) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"fin\");\n}\nif ((flags & 0x02) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"syn\");\n}\nif ((flags & 0x04) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"rst\");\n}\nif ((flags & 0x08) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"psh\");\n}\nif ((flags & 0x10) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"ack\");\n}\nif ((flags & 0x20) != 0) {\n  ctx.aws.firewall.tcp_flags_array.add(\"urg\");\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.event.netflow") {
                event.rename("json.event.netflow", "aws.firewall.flow")?;
            }

            if event.has("json.event.flow_id") {
                event.rename("json.event.flow_id", "aws.firewall.flow.id")?;
            }

            if event.has_value("aws.firewall.flow.id") {
                if let Some(val) = event.get("aws.firewall.flow.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "aws.firewall.flow.id".into(),
                            message,
                        }
                    })?;
                    event.set("aws.firewall.flow.id", converted)?;
                }
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.ip") {
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
                }
                Ok(())
            })();

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            event.remove("json");

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
