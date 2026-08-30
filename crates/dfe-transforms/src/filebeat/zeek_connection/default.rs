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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("_temp_", "zeek.connection")?;
                Ok(())
            })();

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.connection", "id.orig_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.connection", "id.orig_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.connection", "id.resp_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.connection", "id.resp_p")?;
                Ok(())
            })();

            if event.has_value("zeek.connection.duration") {
                event.rename("zeek.connection.duration", "temp.duration")?;
            }

            if event.has_value("zeek.connection.id.orig_h") {
                event.rename("zeek.connection.id.orig_h", "source.address")?;
            }

            if event.has_value("zeek.connection.id.orig_p") {
                event.rename("zeek.connection.id.orig_p", "source.port")?;
            }

            if event.has_value("zeek.connection.id.resp_h") {
                event.rename("zeek.connection.id.resp_h", "destination.address")?;
            }

            if event.has_value("zeek.connection.id.resp_p") {
                event.rename("zeek.connection.id.resp_p", "destination.port")?;
            }

            if event.has_value("zeek.connection.proto") {
                event.rename("zeek.connection.proto", "network.transport")?;
            }

            if event.has_value("zeek.connection.service") {
                event.rename("zeek.connection.service", "network.protocol")?;
            }

            if event.has_value("zeek.connection.uid") {
                event.rename("zeek.connection.uid", "zeek.session_id")?;
            }

            if event.has_value("zeek.connection.orig_ip_bytes") {
                event.rename("zeek.connection.orig_ip_bytes", "source.bytes")?;
            }

            if event.has_value("zeek.connection.resp_ip_bytes") {
                event.rename("zeek.connection.resp_ip_bytes", "destination.bytes")?;
            }

            if event.has_value("zeek.connection.orig_pkts") {
                event.rename("zeek.connection.orig_pkts", "source.packets")?;
            }

            if event.has_value("zeek.connection.resp_pkts") {
                event.rename("zeek.connection.resp_pkts", "destination.packets")?;
            }

            if event.has_value("zeek.connection.conn_state") {
                event.rename("zeek.connection.conn_state", "zeek.connection.state")?;
            }

            if event.has_value("zeek.connection.orig_l2_addr") {
                event.rename("zeek.connection.orig_l2_addr", "source.mac")?;
            }

            if event.has_value("zeek.connection.resp_l2_addr") {
                event.rename("zeek.connection.resp_l2_addr", "destination.mac")?;
            }

            let _cond = { event.get_str("network.transport") == Some("icmp") };
            if _cond {
                if event.has_value("source.port") {
                    event.rename("source.port", "zeek.connection.icmp.type")?;
                }
            }

            let _cond = { event.get_str("network.transport") == Some("icmp") };
            if _cond {
                if event.has_value("destination.port") {
                    event.rename("destination.port", "zeek.connection.icmp.code")?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            let _cond = {
                event.get_str("network.transport") != Some("icmp")
                    && event.get_i64("source.port") != Some(0)
                    && event.get_i64("destination.port") != Some(0)
            };
            if _cond {
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

            let _cond = { event.get_str("network.transport") == Some("icmp") };
            if _cond {
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
                        ("zeek.connection.icmp.type", "zeek.connection.icmp.code")
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

            if let Some(date_str) = event.get_as_string("zeek.connection.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.connection.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.connection.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.connection.ts".into(),
                });
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("temp.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx.temp.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Math.round(ctx.temp.duration * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1000000000}"),
                )?;
            }

            let _cond = { event.has_value("zeek.connection.local_orig") };
            if _cond {
                event.append_unique("tags", json!("local_orig"))?;
            }

            let _cond = { event.has_value("zeek.connection.local_resp") };
            if _cond {
                event.append_unique("tags", json!("local_resp"))?;
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
                // Painless script
                // Source: ctx.network.packets = ctx.source.packets + ctx.destination.packets
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.network.packets = ctx.source.packets + ctx.destination.packets"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes"#
                    ),
                )?;
                Ok(())
            })();

            // Painless script
            // Source: if (ctx.zeek?.connection?.local_orig == null ||\n    ctx.zeek?.connection?.local_resp == null) {\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"internal\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"outbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"inbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"external\";\n  return;\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.zeek?.connection?.local_orig == null ||\n    ctx.zeek?.connection?.local_resp == null) {\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"internal\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"outbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"inbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"external\";\n  return;\n}"#
                ),
            )?;

            let _cond = { event.has_value("destination.ip") };
            if _cond {
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
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

            // Painless script
            // Source: if (ctx.zeek?.connection?.state == null) {\n  return;\n} if (params.containsKey(ctx.zeek.connection.state)) {\n  ctx.zeek.connection.state_message = params[ctx.zeek.connection.state][\"conn_str\"];\n  ctx.event.type = params[ctx.zeek.connection.state][\"types\"];\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.zeek?.connection?.state == null) {\n  return;\n} if (params.containsKey(ctx.zeek.connection.state)) {\n  ctx.zeek.connection.state_message = params[ctx.zeek.connection.state][\"conn_str\"];\n  ctx.event.type = params[ctx.zeek.connection.state][\"types\"];\n}"#
                ),
                cached_params!(
                    "{\"S0\":{\"conn_str\":\"Connection attempt seen, no reply.\",\"types\":[\"connection\",\"start\"]},\"S1\":{\"conn_str\":\"Connection established, not terminated.\",\"types\":[\"connection\",\"start\"]},\"SF\":{\"conn_str\":\"Normal establishment and termination.\",\"types\":[\"connection\",\"start\",\"end\"]},\"REJ\":{\"conn_str\":\"Connection attempt rejected.\",\"types\":[\"connection\",\"start\",\"denied\"]},\"S2\":{\"conn_str\":\"Connection established and close attempt by originator seen (but no reply from responder).\",\"types\":[\"connection\",\"info\"]},\"S3\":{\"conn_str\":\"Connection established and close attempt by responder seen (but no reply from originator).\",\"types\":[\"connection\",\"info\"]},\"RSTO\":{\"conn_str\":\"Connection established, originator aborted (sent a RST).\",\"types\":[\"connection\",\"info\"]},\"RSTR\":{\"conn_str\":\"Responder sent a RST.\",\"types\":[\"connection\",\"info\"]},\"RSTOS0\":{\"conn_str\":\"Originator sent a SYN followed by a RST, we never saw a SYN-ACK from the responder.\",\"types\":[\"connection\",\"info\"]},\"RSTRH\":{\"conn_str\":\"Responder sent a SYN ACK followed by a RST, we never saw a SYN from the (purported) originator.\",\"types\":[\"connection\",\"info\"]},\"SH\":{\"conn_str\":\"Originator sent a SYN followed by a FIN, we never saw a SYN ACK from the responder (hence the connection was 'half' open).\",\"types\":[\"connection\",\"info\"]},\"SHR\":{\"conn_str\":\"Responder sent a SYN ACK followed by a FIN, we never saw a SYN from the originator.\",\"types\":[\"connection\",\"info\"]},\"OTH\":{\"conn_str\":\"No SYN seen, just midstream traffic (a 'partial connection' that was not later closed).\",\"types\":[\"connection\",\"info\"]}}"
                ),
            )?;

            event.remove("zeek.connection.id");
            event.remove("zeek.connection.orig_bytes");
            event.remove("zeek.connection.resp_bytes");
            event.remove("zeek.connection.tunnel_parents");
            event.remove("message");
            event.remove("json");
            event.remove("temp");

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
