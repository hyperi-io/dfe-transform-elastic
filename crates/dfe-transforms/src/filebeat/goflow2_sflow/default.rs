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
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            parse_json_field(event, "message", "goflow2")?;

            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            if event.remove("message").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "message".into(),
                });
            }

            // Painless script
            // Source: ctx.goflow2.time_flow_start_ns = (ctx.goflow2?.time_flow_start_ns / 1000000);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.goflow2.time_flow_start_ns = (ctx.goflow2?.time_flow_start_ns / 1000000);"#
                ),
            )?;

            let _cond = {
                event.has_value("goflow2.bytes")
                    && event.get_str("goflow2.bytes") != Some("")
                    && event.has_value("goflow2.sampling_rate")
                    && event.get_str("goflow2.sampling_rate") != Some("")
            };
            if _cond {
                // Painless script
                // Source: ctx.goflow2.flow_size = ctx.goflow2?.bytes * ctx.goflow2?.sampling_rate;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.goflow2.flow_size = ctx.goflow2?.bytes * ctx.goflow2?.sampling_rate;"#
                    ),
                )?;
            }

            if let Some(date_str) = event.get_as_string("goflow2.time_flow_start_ns") {
                match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "goflow2.time_flow_start_ns".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond =
                { event.has_value("goflow2.type") && event.get_str("goflow2.type") != Some("") };
            if _cond {
                event.rename("goflow2.type", "event.action")?;
            }

            let _cond = {
                event.has_value("goflow2.sampler_address")
                    && event.get_str("goflow2.sampler_address") != Some("")
            };
            if _cond {
                event.append(
                    "observer.ip",
                    json!(
                        event
                            .get("goflow2.sampler_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("goflow2.sequence_num")
                    && event.get_str("goflow2.sequence_num") != Some("")
            };
            if _cond {
                event.rename("goflow2.sequence_num", "sflow.sequence_num")?;
            }

            let _cond =
                { event.has_value("goflow2.in_if") && event.get_str("goflow2.in_if") != Some("") };
            if _cond {
                event.rename("goflow2.in_if", "observer.ingress.interface.id")?;
            }

            if event.has_value("observer.ingress.interface.id") {
                if let Some(val) = event.get("observer.ingress.interface.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "observer.ingress.interface.id".into(),
                            message,
                        }
                    })?;
                    event.set("observer.ingress.interface.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("goflow2.out_if") && event.get_str("goflow2.out_if") != Some("")
            };
            if _cond {
                event.rename("goflow2.out_if", "observer.egress.interface.id")?;
            }

            if event.has_value("observer.egress.interface.id") {
                if let Some(val) = event.get("observer.egress.interface.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "observer.egress.interface.id".into(),
                            message,
                        }
                    })?;
                    event.set("observer.egress.interface.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("goflow2.src_addr") && event.get_str("goflow2.src_addr") != Some("")
            };
            if _cond {
                if let Some(val) = event.get("goflow2.src_addr") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "goflow2.src_addr".into(),
                            message,
                        })?;
                    event.set("goflow2.src_addr", converted)?;
                }
            }

            let _cond = {
                event.has_value("goflow2.src_addr") && event.get_str("goflow2.src_addr") != Some("")
            };
            if _cond {
                event.rename("goflow2.src_addr", "source.ip")?;
            }

            let _cond = {
                event.has_value("goflow2.dst_addr") && event.get_str("goflow2.dst_addr") != Some("")
            };
            if _cond {
                if let Some(val) = event.get("goflow2.dst_addr") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "goflow2.dst_addr".into(),
                            message,
                        })?;
                    event.set("goflow2.dst_addr", converted)?;
                }
            }

            let _cond = {
                event.has_value("goflow2.dst_addr") && event.get_str("goflow2.dst_addr") != Some("")
            };
            if _cond {
                event.rename("goflow2.dst_addr", "destination.ip")?;
            }

            let _cond =
                { event.has_value("goflow2.etype") && event.get_str("goflow2.etype") != Some("") };
            if _cond {
                event.rename("goflow2.etype", "network.type")?;
            }

            let _cond =
                { event.has_value("goflow2.proto") && event.get_str("goflow2.proto") != Some("") };
            if _cond {
                event.rename("goflow2.proto", "network.transport")?;
            }

            let _cond = {
                event.has_value("goflow2.src_port") && event.get_str("goflow2.src_port") != Some("")
            };
            if _cond {
                event.rename("goflow2.src_port", "source.port")?;
            }

            let _cond = {
                event.has_value("goflow2.dst_port") && event.get_str("goflow2.dst_port") != Some("")
            };
            if _cond {
                event.rename("goflow2.dst_port", "destination.port")?;
            }

            let _cond = {
                event.has_value("goflow2.src_vlan") && event.get_str("goflow2.src_vlan") != Some("")
            };
            if _cond {
                event.rename("goflow2.src_vlan", "observer.ingress.vlan.id")?;
            }

            if event.has_value("observer.ingress.vlan.id") {
                if let Some(val) = event.get("observer.ingress.vlan.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "observer.ingress.vlan.id".into(),
                            message,
                        }
                    })?;
                    event.set("observer.ingress.vlan.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("goflow2.dst_vlan") && event.get_str("goflow2.dst_vlan") != Some("")
            };
            if _cond {
                event.rename("goflow2.dst_vlan", "observer.egress.vlan.id")?;
            }

            if event.has_value("observer.egress.vlan.id") {
                if let Some(val) = event.get("observer.egress.vlan.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "observer.egress.vlan.id".into(),
                            message,
                        }
                    })?;
                    event.set("observer.egress.vlan.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("goflow2.sampling_rate")
                    && event.get_str("goflow2.sampling_rate") != Some("")
            };
            if _cond {
                event.rename("goflow2.sampling_rate", "network.packets")?;
            }

            let _cond =
                { event.has_value("goflow2.bytes") && event.get_str("goflow2.bytes") != Some("") };
            if _cond {
                event.rename("goflow2.bytes", "sflow.bytes")?;
            }

            if event.has_value("goflow2.flow_size") {
                event.rename("goflow2.flow_size", "network.bytes")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("goflow2.time_flow_start_ns");
                event.remove("goflow2.proto");
                event.remove("goflow2.etype");
                event.remove("goflow2.dst_addr");
                event.remove("goflow2.src_addr");
                event.remove("goflow2.sampler_address");
                event.remove("goflow2.bytes");
                Ok(())
            })();

            let _cond = {
                event.get("goflow2").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("goflow2");
                    Ok(())
                })();
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.type") {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
            }

            if let Some(v) = event
                .get("network.packets")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sflow.sample_rate", v)?;
            }

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                event.append(
                    "destination.address",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append(
                    "source.address",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append(
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
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

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
                event.set(
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
