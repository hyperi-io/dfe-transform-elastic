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
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "json")?;

            // Community ID v1 hash
            if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                event.get_string("json.jsonPayload.connection.src_ip"),
                event.get_string("json.jsonPayload.connection.dest_ip"),
                event
                    .get_as_string("json.jsonPayload.connection.protocol")
                    .or_else(|| event.get_as_string("network.transport")),
            ) {
                let icmp = matches!(
                    protocol.to_ascii_lowercase().as_str(),
                    "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                );
                let (src_field, dst_field) = if icmp {
                    ("icmp.type", "icmp.code")
                } else {
                    (
                        "json.jsonPayload.connection.src_port",
                        "json.jsonPayload.connection.dest_port",
                    )
                };
                let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
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

            if let Some(date_str) = event.get_as_string("json.timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.insertId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            event.set("cloud.provider", json!("gcp"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.logName").cloned() {
                    event.set("log.logger", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.connection.dest_ip").cloned() {
                    event.set("destination.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.connection.dest_port").cloned() {
                    event.set("destination.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.connection.protocol").cloned() {
                    event.set("network.iana_number", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.connection.src_ip").cloned() {
                    event.set("source.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.connection.src_port").cloned() {
                    event.set("source.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_instance.vm_name").cloned() {
                    event.set("source.domain", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_instance.vm_name").cloned() {
                    event.set("destination.domain", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.bytes_sent").cloned() {
                    event.set("source.bytes", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.packets_sent").cloned() {
                    event.set("source.packets", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.start_time").cloned() {
                    event.set("event.start", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.end_time").cloned() {
                    event.set("event.end", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_location.asn").cloned() {
                    event.set("destination.as.number", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.jsonPayload.dest_location.continent")
                    .cloned()
                {
                    event.set("destination.geo.continent_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_location.country").cloned() {
                    event.set("destination.geo.country_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_location.region").cloned() {
                    event.set("destination.geo.region_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_location.city").cloned() {
                    event.set("destination.geo.city_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_location.asn").cloned() {
                    event.set("source.as.number", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.jsonPayload.src_location.continent")
                    .cloned()
                {
                    event.set("source.geo.continent_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_location.country").cloned() {
                    event.set("source.geo.country_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_location.region").cloned() {
                    event.set("source.geo.region_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_location.city").cloned() {
                    event.set("source.geo.city_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_instance").cloned() {
                    event.set("gcp.destination.instance", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.dest_vpc").cloned() {
                    event.set("gcp.destination.vpc", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_instance").cloned() {
                    event.set("gcp.source.instance", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.src_vpc").cloned() {
                    event.set("gcp.source.vpc", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.jsonPayload.rtt_msec") {
                if let Some(val) = event.get("json.jsonPayload.rtt_msec") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.jsonPayload.rtt_msec".into(),
                            message,
                        }
                    })?;
                    event.set("json.jsonPayload.rtt.ms", converted)?;
                }
            }

            if event.has("json.jsonPayload") {
                event.rename("json.jsonPayload", "gcp.vpcflow")?;
            }

            if event.has_value("source.bytes") {
                if let Some(val) = event.get("source.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("source.bytes", converted)?;
                }
            }

            if event.has_value("source.packets") {
                if let Some(val) = event.get("source.packets") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.packets".into(),
                            message,
                        }
                    })?;
                    event.set("source.packets", converted)?;
                }
            }

            if event.has_value("network.iana_number") {
                if let Some(val) = event.get("network.iana_number") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "network.iana_number".into(),
                            message,
                        }
                    })?;
                    event.set("network.iana_number", converted)?;
                }
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

            let _cond = { event.has_value("source.address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("source.address").cloned() {
                        event.set("source.ip", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("destination.address").cloned() {
                        event.set("destination.ip", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("DEST") };
            if _cond {
                if event.has_value("gcp.source.instance.project_id") {
                    if let Some(val) = event.get("gcp.source.instance.project_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.source.instance.project_id".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.project.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("DEST") };
            if _cond {
                if event.has_value("gcp.source.instance.vm_name") {
                    if let Some(val) = event.get("gcp.source.instance.vm_name") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.source.instance.vm_name".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.instance.name", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("DEST") };
            if _cond {
                if event.has_value("gcp.source.instance.region") {
                    if let Some(val) = event.get("gcp.source.instance.region") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.source.instance.region".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.region", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("DEST") };
            if _cond {
                if event.has_value("gcp.source.instance.zone") {
                    if let Some(val) = event.get("gcp.source.instance.zone") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.source.instance.zone".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.availability_zone", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("DEST") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.source.vpc.subnetwork_name") {
                        if let Some(val) = event.get("gcp.source.vpc.subnetwork_name") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "gcp.source.vpc.subnetwork_name".into(),
                                    message,
                                }
                            })?;
                            event.set("network.name", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("SRC") };
            if _cond {
                if event.has_value("gcp.destination.instance.project_id") {
                    if let Some(val) = event.get("gcp.destination.instance.project_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.destination.instance.project_id".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.project.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("SRC") };
            if _cond {
                if event.has_value("gcp.destination.instance.vm_name") {
                    if let Some(val) = event.get("gcp.destination.instance.vm_name") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.destination.instance.vm_name".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.instance.name", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("SRC") };
            if _cond {
                if event.has_value("gcp.destination.instance.region") {
                    if let Some(val) = event.get("gcp.destination.instance.region") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.destination.instance.region".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.region", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("SRC") };
            if _cond {
                if event.has_value("gcp.destination.instance.zone") {
                    if let Some(val) = event.get("gcp.destination.instance.zone") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gcp.destination.instance.zone".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.availability_zone", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("gcp.vpcflow.reporter") == Some("SRC") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.destination.vpc.subnetwork_name") {
                        if let Some(val) = event.get("gcp.destination.vpc.subnetwork_name") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "gcp.destination.vpc.subnetwork_name".into(),
                                    message,
                                }
                            })?;
                            event.set("network.name", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("source.bytes") {
                if let Some(val) = event.get("source.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("network.bytes", converted)?;
                }
            }

            if event.has_value("source.packets") {
                if let Some(val) = event.get("source.packets") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.packets".into(),
                            message,
                        }
                    })?;
                    event.set("network.packets", converted)?;
                }
            }

            let _cond = {
                event.has_value("gcp.source.instance")
                    && event.has_value("gcp.destination.instance")
            };
            if _cond {
                event.set("network.direction", json!("internal"))?;
            }

            let _cond = {
                event.has_value("gcp.source.instance")
                    && !event.has_value("gcp.destination.instance")
            };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = {
                !event.has_value("gcp.source.instance")
                    && event.has_value("gcp.destination.instance")
            };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { !event.has_value("network.direction") };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
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
                    && !(event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
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

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.keep_json") == Some(true) };
            if _cond {
                event.rename("json", "gcp.vpcflow.flattened")?;
            }

            event.remove("gcp.destination.instance.vm_name");
            event.remove("gcp.source.instance.vm_name");
            event.remove("gcp.vpcflow.bytes_sent");
            event.remove("gcp.vpcflow.connection");
            event.remove("gcp.vpcflow.connection.dest_ip");
            event.remove("gcp.vpcflow.connection.dest_port");
            event.remove("gcp.vpcflow.connection.protocol");
            event.remove("gcp.vpcflow.connection.src_ip");
            event.remove("gcp.vpcflow.connection.src_port");
            event.remove("gcp.vpcflow.dest_instance");
            event.remove("gcp.vpcflow.dest_instance.vm_name");
            event.remove("gcp.vpcflow.dest_location");
            event.remove("gcp.vpcflow.dest_location.asn");
            event.remove("gcp.vpcflow.dest_location.city");
            event.remove("gcp.vpcflow.dest_location.continent");
            event.remove("gcp.vpcflow.dest_location.country");
            event.remove("gcp.vpcflow.dest_location.region");
            event.remove("gcp.vpcflow.dest_vpc");
            event.remove("gcp.vpcflow.end_time");
            event.remove("gcp.vpcflow.packets_sent");
            event.remove("gcp.vpcflow.rtt_msec");
            event.remove("gcp.vpcflow.src_instance");
            event.remove("gcp.vpcflow.src_instance.vm_name");
            event.remove("gcp.vpcflow.src_location");
            event.remove("gcp.vpcflow.src_location.asn");
            event.remove("gcp.vpcflow.src_location.city");
            event.remove("gcp.vpcflow.src_location.continent");
            event.remove("gcp.vpcflow.src_location.country");
            event.remove("gcp.vpcflow.src_location.region");
            event.remove("gcp.vpcflow.src_vpc");
            event.remove("gcp.vpcflow.start_time");
            event.remove("_conf");
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
