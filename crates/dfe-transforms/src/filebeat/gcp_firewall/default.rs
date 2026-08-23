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
                if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    event.set("@timestamp", parsed)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.set("event.action", json!("firewall-rule"))?;

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
                if let Some(v) = event.get("json.resource.labels.subnetwork_name").cloned() {
                    event.set("network.name", v)?;
                }
                Ok(())
            })();

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

            let _cond = { event.has_value("json.jsonPayload.disposition") };
            if _cond {
                if let Some(s) = event.get_string("json.jsonPayload.disposition") {
                    let lowered = s.to_lowercase();
                    event.set("json.jsonPayload.disposition", lowered)?;
                }
            }

            let _cond = { event.has_value("json.jsonPayload.disposition") };
            if _cond {
                event.append(
                    "event.type",
                    json!(
                        event
                            .get("json.jsonPayload.disposition")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.append("event.type", json!("connection"))?;

            let _cond =
                { event.get_str("json.jsonPayload.rule_details.direction") == Some("INGRESS") };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond =
                { event.get_str("json.jsonPayload.rule_details.direction") == Some("EGRESS") };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = { !event.has_value("network.direction") };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.vpc").cloned() {
                        event.set("_jsonPayload.src_vpc", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.instance").cloned() {
                        event.set("_jsonPayload.src_instance", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.location").cloned() {
                        event.set("_jsonPayload.src_location", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.remote_vpc").cloned() {
                        event.set("_jsonPayload.dest_vpc", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.remote_instance").cloned() {
                        event.set("_jsonPayload.dest_instance", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.remote_location").cloned() {
                        event.set("_jsonPayload.dest_location", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.vpc").cloned() {
                        event.set("_jsonPayload.dest_vpc", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.instance").cloned() {
                        event.set("_jsonPayload.dest_instance", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.location").cloned() {
                        event.set("_jsonPayload.dest_location", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.remote_vpc").cloned() {
                        event.set("_jsonPayload.src_vpc", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.remote_instance").cloned() {
                        event.set("_jsonPayload.src_instance", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.jsonPayload.remote_location").cloned() {
                        event.set("_jsonPayload.src_location", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.connection.protocol").cloned() {
                    event.set("network.iana_number", v)?;
                }
                Ok(())
            })();

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

            if event.has("_jsonPayload.src_instance.vm_name") {
                event.rename("_jsonPayload.src_instance.vm_name", "source.domain")?;
            }

            if event.has("_jsonPayload.dest_instance.vm_name") {
                event.rename("_jsonPayload.dest_instance.vm_name", "destination.domain")?;
            }

            if event.has("_jsonPayload.dest_location.asn") {
                event.rename("_jsonPayload.dest_location.asn", "destination.as.number")?;
            }

            if event.has("_jsonPayload.dest_location.continent") {
                event.rename(
                    "_jsonPayload.dest_location.continent",
                    "destination.geo.continent_name",
                )?;
            }

            if event.has("_jsonPayload.dest_location.country") {
                event.rename(
                    "_jsonPayload.dest_location.country",
                    "destination.geo.country_name",
                )?;
            }

            if event.has("_jsonPayload.dest_location.region") {
                event.rename(
                    "_jsonPayload.dest_location.region",
                    "destination.geo.region_name",
                )?;
            }

            if event.has("_jsonPayload.dest_location.city") {
                event.rename(
                    "_jsonPayload.dest_location.city",
                    "destination.geo.city_name",
                )?;
            }

            if event.has("_jsonPayload.src_location.asn") {
                event.rename("_jsonPayload.src_location.asn", "source.as.number")?;
            }

            if event.has("_jsonPayload.src_location.continent") {
                event.rename(
                    "_jsonPayload.src_location.continent",
                    "source.geo.continent_name",
                )?;
            }

            if event.has("_jsonPayload.src_location.country") {
                event.rename(
                    "_jsonPayload.src_location.country",
                    "source.geo.country_name",
                )?;
            }

            if event.has("_jsonPayload.src_location.region") {
                event.rename("_jsonPayload.src_location.region", "source.geo.region_name")?;
            }

            if event.has("_jsonPayload.src_location.city") {
                event.rename("_jsonPayload.src_location.city", "source.geo.city_name")?;
            }

            if event.has("_jsonPayload.dest_instance") {
                event.rename("_jsonPayload.dest_instance", "gcp.destination.instance")?;
            }

            if event.has("_jsonPayload.dest_vpc") {
                event.rename("_jsonPayload.dest_vpc", "gcp.destination.vpc")?;
            }

            if event.has("_jsonPayload.src_instance") {
                event.rename("_jsonPayload.src_instance", "gcp.source.instance")?;
            }

            if event.has("_jsonPayload.src_vpc") {
                event.rename("_jsonPayload.src_vpc", "gcp.source.vpc")?;
            }

            if event.has("json.jsonPayload.rule_details.reference") {
                event.rename("json.jsonPayload.rule_details.reference", "rule.name")?;
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "source.ip",
                        json!(
                            event
                                .get("source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "destination.ip",
                        json!(
                            event
                                .get("destination.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
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

            let _cond = { event.get_str("network.direction") == Some("outbound") };
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

            let _cond = { event.get_str("network.direction") == Some("outbound") };
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

            let _cond = { event.get_str("network.direction") == Some("outbound") };
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

            let _cond = { event.get_str("network.direction") == Some("outbound") };
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

            let _cond = { event.get_str("network.direction") == Some("inbound") };
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

            let _cond = { event.get_str("network.direction") == Some("inbound") };
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

            let _cond = { event.get_str("network.direction") == Some("inbound") };
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

            let _cond = { event.get_str("network.direction") == Some("inbound") };
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

            let _cond = { event.get_str("network.direction") == Some("inbound") };
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

            let _cond = {
                event.get("gcp.source.instance").filter(|v| !v.is_null())
                    == event
                        .get("gcp.destination.instance")
                        .filter(|v| !v.is_null())
            };
            if _cond {
                event.set("network.direction", json!("internal"))?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.rule_details").cloned() {
                    event.set("gcp.firewall.rule_details", v)?;
                }
                Ok(())
            })();

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

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.keep_json") == Some(true) };
            if _cond {
                event.rename("json", "gcp.firewall.flattened")?;
            }

            event.remove("gcp.firewall.connection");
            event.remove("gcp.firewall.dest_location");
            event.remove("gcp.firewall.disposition");
            event.remove("gcp.firewall.src_location");
            event.remove("_conf");
            event.remove("json");
            event.remove("_jsonPayload");

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
