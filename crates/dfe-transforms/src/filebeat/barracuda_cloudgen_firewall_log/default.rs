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

            if event.has_value("source.address") {
                event.rename("source.address", "labels.origin_address")?;
            }

            if event.has_value("tls.client.subject") {
                event.rename("tls.client.subject", "labels.origin_client_subject")?;
            }

            event.remove("source");
            event.remove("tls");

            event.set("observer.vendor", json!("Barracuda"))?;

            event.set("observer.type", json!("firewall"))?;

            if event.has_value("lumberjack.beat.hostname") {
                event.rename("lumberjack.beat.hostname", "observer.hostname")?;
            }

            let _cond = { !event.has_value("observer.hostname") };
            if _cond {
                if event.has_value("lumberjack.agent.hostname") {
                    event.rename("lumberjack.agent.hostname", "observer.hostname")?;
                }
            }

            if event.has_value("lumberjack.product") {
                event.rename("lumberjack.product", "observer.product")?;
            }

            if event.has_value("lumberjack.sn") {
                event.rename("lumberjack.sn", "observer.serial_number")?;
            }

            if event.has_value("lumberjack.message") {
                event.rename("lumberjack.message", "event.original")?;
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.remove("message");

            parse_json_field(event, "event.original", "json")?;

            let _cond = { event.get_str("lumberjack.type") == Some("ngfw-act") };
            if _cond {
                // Begin nested pipeline: "firewall"
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                let _cond = { event.get_str("json.src_ip") != Some("-") };
                if _cond {
                    if event.has_value("json.src_ip") {
                        event.rename("json.src_ip", "source.address")?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.address") {
                        if let Some(val) = event.get("source.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "source.address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("json.src_port") {
                    if let Some(val) = event.get("json.src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.src_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                if event.has_value("json.src_iface") {
                    event.rename("json.src_iface", "observer.ingress.interface.name")?;
                }
                let _cond = { event.get_str("json.src_mac") != Some("00:00:00:00:00:00") };
                if _cond {
                    if event.has_value("json.src_mac") {
                        gsub_field(
                            event,
                            "json.src_mac",
                            "source.mac",
                            cached_regex!("[-:.]"),
                            "-",
                        )?;
                    }
                }
                let _cond = { event.get_str("json.src_ip_nat") != Some("0.0.0.0") };
                if _cond {
                    if event.has_value("json.src_ip_nat") {
                        if let Some(val) = event.get("json.src_ip_nat") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src_ip_nat".into(),
                                    message,
                                }
                            })?;
                            event.set("source.nat.ip", converted)?;
                        }
                    }
                }
                if event.has_value("json.fwd_bytes") {
                    if let Some(val) = event.get("json.fwd_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.fwd_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("source.bytes", converted)?;
                    }
                }
                if event.has_value("json.fwd_packets") {
                    if let Some(val) = event.get("json.fwd_packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.fwd_packets".into(),
                                message,
                            }
                        })?;
                        event.set("source.packets", converted)?;
                    }
                }
                let _cond = { event.get_str("json.dst_ip") != Some("-") };
                if _cond {
                    if event.has_value("json.dst_ip") {
                        event.rename("json.dst_ip", "destination.address")?;
                    }
                }
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                if event.has_value("json.dst_port") {
                    if let Some(val) = event.get("json.dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                if event.has_value("json.dst_iface") {
                    event.rename("json.dst_iface", "observer.egress.interface.name")?;
                }
                let _cond = { event.get_str("json.dst_mac") != Some("00:00:00:00:00:00") };
                if _cond {
                    if event.has_value("json.dst_mac") {
                        gsub_field(
                            event,
                            "json.dst_mac",
                            "destination.mac",
                            cached_regex!("[-:.]"),
                            "-",
                        )?;
                    }
                }
                let _cond = { event.get_str("json.dst_ip_nat") != Some("0.0.0.0") };
                if _cond {
                    if event.has_value("json.dst_ip_nat") {
                        if let Some(val) = event.get("json.dst_ip_nat") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dst_ip_nat".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.nat.ip", converted)?;
                        }
                    }
                }
                if event.has_value("destination.mac") {
                    map_strings(
                        event,
                        "destination.mac",
                        "destination.mac",
                        str::to_uppercase,
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
                        cached_regex!("[.:]"),
                        "-",
                    )?;
                }
                if event.has_value("source.mac") {
                    gsub_field(
                        event,
                        "source.mac",
                        "source.mac",
                        cached_regex!("[.:]"),
                        "-",
                    )?;
                }
                if event.has_value("json.rev_bytes") {
                    if let Some(val) = event.get("json.rev_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.rev_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("destination.bytes", converted)?;
                    }
                }
                if event.has_value("json.rev_packets") {
                    if let Some(val) = event.get("json.rev_packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.rev_packets".into(),
                                message,
                            }
                        })?;
                        event.set("destination.packets", converted)?;
                    }
                }
                if event.has_value("json.fw_rule") {
                    event.rename("json.fw_rule", "rule.name")?;
                }
                if event.has_value("json.app_rule") {
                    event.rename("json.app_rule", "barracuda_cloudgen_firewall.log.app_rule")?;
                }
                if event.has_value("json.apps") {
                    event.rename("json.apps", "barracuda_cloudgen_firewall.log.apps")?;
                }
                if event.has_value("json.protos") {
                    event.rename("json.protos", "barracuda_cloudgen_firewall.log.protos")?;
                }
                let _cond = { event.get_str("json.fw_info") != Some("-") };
                if _cond {
                    if event.has_value("json.fw_info") {
                        event.rename("json.fw_info", "barracuda_cloudgen_firewall.log.fw_info")?;
                    }
                }
                if event.has_value("json.action") {
                    event.rename("json.action", "event.action")?;
                }
                let _cond = { event.has_value("json.duration") };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.event.duration = (long)ctx.json.duration * 1000000;
                    scale_field(
                        event,
                        &ScaleField::new("json.duration", "event.duration", Factor::Long(1000000)),
                    );
                }
                let _cond = {
                    event.has_value("source.bytes")
                        && event.has_value("destination.bytes")
                        && !event.has_value("network.bytes")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script, resolved to its runners at generation time
                        // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                        sum_directions_into_existing(event, &["bytes"]);
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("source.packets")
                        && event.has_value("destination.packets")
                        && !event.has_value("network.packets")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script, resolved to its runners at generation time
                        // Source: ctx.network.packets = ctx.source.packets + ctx.destination.packets
                        sum_directions_into_existing(event, &["packets"]);
                        Ok(())
                    })();
                }
                if event.has_value("json.ip_proto") {
                    if let Some(val) = event.get("json.ip_proto") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ip_proto".into(),
                                message,
                            }
                        })?;
                        event.set("network.iana_number", converted)?;
                    }
                }
                if event.has_value("json.user") {
                    event.rename("json.user", "user.name")?;
                }
                let _cond = { event.has_value("network.iana_number") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script
                        // Source: def iana_number = ctx.network.iana_number;\nif (iana_number == '0') {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n}\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def iana_number = ctx.network.iana_number;\nif (iana_number == '0') {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n}\n"#
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.set("event.kind", json!("event"))?;
                event.append_unique("event.category", json!("network"))?;
                let _cond = { event.get_str("event.action") == Some("AppBlock") };
                if _cond {
                    event.append_unique("event.type", json!("denied"))?;
                }
                let _cond = { event.get_str("event.action") == Some("End") };
                if _cond {
                    event.append_unique("event.type", json!("end"))?;
                }
                // End nested pipeline: "firewall"
            }

            let _cond = { event.get_str("lumberjack.type") == Some("ngfw-wf") };
            if _cond {
                // Begin nested pipeline: "web"
                let _cond = { event.get_str("json.timestamp") != Some("-") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { event.get_str("json.source_ip") != Some("-") };
                if _cond {
                    if event.has_value("json.source_ip") {
                        event.rename("json.source_ip", "source.address")?;
                    }
                }
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                let _cond = { event.get_str("json.source_port") != Some("-") };
                if _cond {
                    if event.has_value("json.source_port") {
                        if let Some(val) = event.get("json.source_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.source_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                }
                let _cond = { event.get_str("json.destination_ip") != Some("-") };
                if _cond {
                    if event.has_value("json.destination_ip") {
                        event.rename("json.destination_ip", "destination.address")?;
                    }
                }
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                let _cond = { event.get_str("json.destination_port") != Some("-") };
                if _cond {
                    if event.has_value("json.destination_port") {
                        if let Some(val) = event.get("json.destination_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.destination_port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                }
                if event.has_value("json.method") {
                    event.rename("json.method", "http.request.method")?;
                }
                let _cond = { event.get_str("json.status_code") != Some("0") };
                if _cond {
                    if event.has_value("json.status_code") {
                        if let Some(val) = event.get("json.status_code") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.status_code".into(),
                                    message,
                                }
                            })?;
                            event.set("http.response.status_code", converted)?;
                        }
                    }
                }
                if event.has_value("json.user_agent") {
                    if let Some(ua_str) = event.get_string("json.user_agent") {
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
                if event.has_value("json.content_type") {
                    event.rename("json.content_type", "http.request.mime_type")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "json.name", "url", true, false)?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("json.domain")
                        && event.get_str("json.domain") != Some("")
                        && event
                            .get_str("json.domain")
                            .is_some_and(|s| cached_regex!(r"^(?:^https?:\/\/.*$)$").is_match(s))
                };
                if _cond {
                    if event.has_value("json.domain") {
                        event.rename("json.domain", "http.request.referrer")?;
                    }
                }
                let _cond =
                    { !event.has_value("url.domain") && event.has_value("destination.domain") };
                if _cond {
                    event.set(
                        "url.domain",
                        json!(
                            event
                                .get("destination.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("json.size") != Some("0") };
                if _cond {
                    if event.has_value("json.size") {
                        if let Some(val) = event.get("json.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.size".into(),
                                    message,
                                }
                            })?;
                            event.set("http.response.body.bytes", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.user_type")
                        && event.get_str("json.user_type") != Some("-")
                };
                if _cond {
                    if event.has_value("json.user_type") {
                        if let Some(val) = event.get("json.user_type") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.user_type".into(),
                                    message,
                                }
                            })?;
                            event.set("barracuda_cloudgen_firewall.log.user_type", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.user")
                        && event.get_str("json.user") != Some("-")
                        && event.get_i64("barracuda_cloudgen_firewall.log.user_type") == Some(1)
                };
                if _cond {
                    if event.has_value("json.user") {
                        event.rename("json.user", "user.name")?;
                    }
                }
                let _cond = {
                    event.has_value("json.traffic_type")
                        && event.get_str("json.traffic_type") != Some("-")
                };
                if _cond {
                    if event.has_value("json.traffic_type") {
                        event.rename(
                            "json.traffic_type",
                            "barracuda_cloudgen_firewall.log.traffic_type",
                        )?;
                    }
                }
                if event.has_value("json.fw_rule") {
                    event.rename("json.fw_rule", "rule.name")?;
                }
                if event.has_value("json.app_rule") {
                    event.rename("json.app_rule", "barracuda_cloudgen_firewall.log.app_rule")?;
                }
                if event.has_value("json.action") {
                    if let Some(val) = event.get("json.action") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action".into(),
                                message,
                            }
                        })?;
                        event.set("event.action", converted)?;
                    }
                }
                event.set("event.kind", json!("event"))?;
                event.append_unique("event.category", json!("network"))?;
                let _cond = { event.get_str("event.action") == Some("1") };
                if _cond {
                    event.append_unique("event.type", json!("denied"))?;
                }
                let _cond = { event.get_str("event.action") == Some("0") };
                if _cond {
                    event.append_unique("event.type", json!("allowed"))?;
                }
                // End nested pipeline: "web"
            }

            let _cond = { event.get_str("lumberjack.type") == Some("ngfw-threat") };
            if _cond {
                // Begin nested pipeline: "threat"
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get_str("json.timestamp") != Some("")
                        && event.get_str("json.timestamp") != Some("-")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("_tmp.timestamp") };
                if _cond {
                    event.set(
                        "_tmp.syslog_timestamp",
                        json!(format!(
                            "{} {}",
                            event
                                .get("json.date")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.time")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { !event.has_value("_tmp.timestamp") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("_tmp.syslog_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy MM dd HH:mm:ss"],
                            event.get_str("json.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.syslog_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { event.get_str("json.src_ip") != Some("-") };
                if _cond {
                    if event.has_value("json.src_ip") {
                        event.rename("json.src_ip", "source.address")?;
                    }
                }
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                let _cond = { event.get_str("json.dst_ip") != Some("-") };
                if _cond {
                    if event.has_value("json.dst_ip") {
                        event.rename("json.dst_ip", "destination.address")?;
                    }
                }
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                let _cond = { event.get_str("json.port") != Some("-") };
                if _cond {
                    if event.has_value("json.port") {
                        if let Some(val) = event.get("json.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                }
                let _cond = { event.get_str("json.severity") != Some("-") };
                if _cond {
                    if event.has_value("json.severity") {
                        event.rename("json.severity", "log.level")?;
                    }
                }
                if event.has_value("json.fw_rule") {
                    event.rename("json.fw_rule", "rule.name")?;
                }
                let _cond = { event.get_str("json.trans_proto") != Some("-") };
                if _cond {
                    if event.has_value("json.trans_proto") {
                        event.rename("json.trans_proto", "network.transport")?;
                    }
                }
                if event.has_value("network.transport") {
                    map_strings(
                        event,
                        "network.transport",
                        "network.transport",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.get_str("network.transport") == Some("tcp") };
                if _cond {
                    event.set("network.iana_number", json!("6"))?;
                }
                let _cond = { event.get_str("network.transport") == Some("udp") };
                if _cond {
                    event.set("network.iana_number", json!("17"))?;
                }
                if event.has_value("json.description") {
                    event.rename("json.description", "rule.description")?;
                }
                let _cond = { event.get_str("json.threat_severity") != Some("-") };
                if _cond {
                    if event.has_value("json.threat_severity") {
                        if let Some(val) = event.get("json.threat_severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.threat_severity".into(),
                                    message,
                                }
                            })?;
                            event.set("event.severity", converted)?;
                        }
                    }
                }
                if event.has_value("json.ips_category") {
                    event.rename("json.ips_category", "rule.category")?;
                }
                let _cond = { event.get_str("json.type") != Some("-") };
                if _cond {
                    if event.has_value("json.type") {
                        event.rename("json.type", "rule.ruleset")?;
                    }
                }
                if event.has_value("json.app_proto") {
                    event.rename(
                        "json.app_proto",
                        "barracuda_cloudgen_firewall.log.app_proto",
                    )?;
                }
                if event.has_value("json.user") {
                    event.rename("json.user", "user.name")?;
                }
                let _cond = { event.get_str("json.operation") != Some("-") };
                if _cond {
                    if event.has_value("json.operation") {
                        map_strings(event, "json.operation", "event.action", str::to_lowercase)?;
                    }
                }
                event.set("event.kind", json!("event"))?;
                event.append_unique("event.category", json!("network"))?;
                let _cond = { event.get_str("event.action") == Some("block") };
                if _cond {
                    event.append_unique("event.type", json!("denied"))?;
                }
                let _cond = { event.get_str("event.action") == Some("allow") };
                if _cond {
                    event.append_unique("event.type", json!("allowed"))?;
                }
                // End nested pipeline: "threat"
            }

            if let Some(v) = event
                .get("_tmp.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("source.ip")
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
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
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

            event.remove("json");
            event.remove("lumberjack");
            event.remove("_tmp");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n    for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n    }\n    map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n    for (def x : list) {\n        if (x instanceof Map) {\n            handleMap(x);\n        } else if (x instanceof List) {\n            handleList(x);\n        }\n    }\n    list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
