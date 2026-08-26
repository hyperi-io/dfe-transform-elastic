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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("network.iana_number") {
                if let Some(val) = event.get("network.iana_number") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "network.iana_number".into(),
                            message,
                        })?;
                    event.set("network.iana_number", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("observer.ip") {
                    event.rename("observer.ip", "_tmp_.observer.ip")?;
                }

            let _cond = { event.has_value("_tmp_.observer.ip") };
            if _cond {
                event.append("observer.ip", json!(event.get("_tmp_.observer.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("event.category") && event.get_str("event.category") == Some("network_session") };
            if _cond {
            event.set("event.category", Value::Array(vec![json!("network"), json!("session")]))?;
            }

            let _cond = { (event.has_value("_tmp_.observer.ip")) };
            if _cond {
                event.append_unique("related.ip", json!(event.get("_tmp_.observer.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("netflow").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: HashSet set = null;\nfor (key in ctx.netflow.keySet()) {\n    if (key.endsWith(\"_ipv4_address\") || key.endsWith(\"_ipv6_address\")) {\n        // Creating set on demand, avoiding extra garbage\n        if (set == null) {\n            set = new HashSet();\n        }\n        set.add(ctx.netflow[key]);\n    }\n}\n\nif (set != null) {\n    if (ctx.related == null) {\n        ctx.related = new HashMap();\n    }\n    if (ctx.related?.ip != null) {\n        for (ip in ctx.related.ip) {\n            set.add(ip);\n        }\n    }\n\n    ArrayList list = new ArrayList(set);\n    Collections.sort(list);\n    ctx.related.ip = list;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"HashSet set = null;\nfor (key in ctx.netflow.keySet()) {\n    if (key.endsWith(\"_ipv4_address\") || key.endsWith(\"_ipv6_address\")) {\n        // Creating set on demand, avoiding extra garbage\n        if (set == null) {\n            set = new HashSet();\n        }\n        set.add(ctx.netflow[key]);\n    }\n}\n\nif (set != null) {\n    if (ctx.related == null) {\n        ctx.related = new HashMap();\n    }\n    if (ctx.related?.ip != null) {\n        for (ip in ctx.related.ip) {\n            set.add(ip);\n        }\n    }\n\n    ArrayList list = new ArrayList(set);\n    Collections.sort(list);\n    ctx.related.ip = list;\n}\n"#))?;
            }

            let _cond = { event.get("netflow.tcp_control_bits").is_some_and(|v| v.is_number()) };
            if _cond {
                // Painless script
                // Source: String[] flags = new String[]{\"CWR\", \"ECE\", \"URG\", \"ACK\", \"PSH\", \"RST\", \"SYN\", \"FIN\"};\nArrayList flagsSeen = new ArrayList();\n\nint tcp_flags = ctx.netflow.tcp_control_bits;\nfor (int i = 8; i > 0; --i) {\n    int value = (tcp_flags & (1 << (i-1))) & 0x0ff;\n    if (value != 0) {\n        flagsSeen.add(flags[8-i]);\n    }\n}\n\nif (flagsSeen.length > 0) {\n    ctx.netflow.tcp_flags = flagsSeen;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"String[] flags = new String[]{\"CWR\", \"ECE\", \"URG\", \"ACK\", \"PSH\", \"RST\", \"SYN\", \"FIN\"};\nArrayList flagsSeen = new ArrayList();\n\nint tcp_flags = ctx.netflow.tcp_control_bits;\nfor (int i = 8; i > 0; --i) {\n    int value = (tcp_flags & (1 << (i-1))) & 0x0ff;\n    if (value != 0) {\n        flagsSeen.add(flags[8-i]);\n    }\n}\n\nif (flagsSeen.length > 0) {\n    ctx.netflow.tcp_flags = flagsSeen;\n}\n"#))?;
            }

            let _cond = { event.has_value("netflow.source_ipv4_address") || event.has_value("netflow.destination_ipv4_address") };
            if _cond {
            event.set("network.type", json!("ipv4"))?;
            }

            let _cond = { (event.has_value("netflow.source_ipv6_address") || event.has_value("netflow.destination_ipv6_address")) && !event.has_value("network.type") };
            if _cond {
            event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { (event.has_value("netflow.source_ipv6_address") || event.has_value("netflow.destination_ipv6_address")) && event.get_str("network.type") == Some("ipv4") };
            if _cond {
                event.append("network.type", json!("ipv6"))?;
            }

            let _cond = { event.get_str("source.locality") == Some("external") && event.get_str("destination.locality") == Some("internal") };
            if _cond {
            event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { event.get_str("source.locality") == Some("internal") && event.get_str("destination.locality") == Some("external") };
            if _cond {
            event.set("network.direction", json!("outbound"))?;
            }

            let _cond = { event.get_str("source.locality") == Some("internal") && event.get_str("destination.locality") == Some("internal") };
            if _cond {
            event.set("network.direction", json!("internal"))?;
            }

            let _cond = { event.get_str("source.locality") == Some("external") && event.get_str("destination.locality") == Some("external") };
            if _cond {
            event.set("network.direction", json!("external"))?;
            }

            let _cond = { !event.has_value("network.direction") };
            if _cond {
            event.set("network.direction", json!("unknown"))?;
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
                    event.rename("destination.as.organization_name", "destination.as.organization.name")?;
                }

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
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
