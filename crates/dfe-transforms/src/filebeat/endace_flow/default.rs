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
                event.has_value("_conf.map_to_ecs")
                    && event.get_bool("_conf.map_to_ecs") == Some(true)
            };
            if _cond {
                // Begin nested pipeline: "compatibility"
                if event.has_value("flow") {
                    event.rename("flow", "network_traffic.flow")?;
                }
                if event.has_value("status") {
                    event.rename("status", "network_traffic.status")?;
                }
                if event.has_value("process.ppid") {
                    event.rename("process.ppid", "process.parent.pid")?;
                }
                event.remove("type");
                event.remove("event.dataset");
                // End nested pipeline: "compatibility"
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "")?;
            }

            if event.has_value("host.mac") {
                gsub_field(
                    event,
                    "host.mac",
                    "host.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            let _cond = {
                event.has_value("observer.hostname")
                    && event.get_str("observer.hostname") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("observer.ip")
                    && event.get("observer.ip").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "observer.ip", |event| {
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

            let _cond = {
                event.has_value("host")
                    && event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("forwarded"))
                        }
                        serde_json::Value::String(s) => s.contains("forwarded"),
                        _ => false,
                    })
            };
            if _cond {
                if event.remove("host").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "host".into(),
                    });
                }
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

            let _cond = {
                event.has_value("_conf.geoip_enrich")
                    && event.get_bool("_conf.geoip_enrich") == Some(true)
            };
            if _cond {
                // Begin nested pipeline: "geoip"
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
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
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                if event.has_value("server.ip") {
                    if let Some(ip_str) = event.get_string("server.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("server.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("server.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("server.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("server.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("server.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("server.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("server.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("server.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("server.ip") {
                    if let Some(ip_str) = event.get_string("server.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("server.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("server.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("server.as.asn") {
                    event.rename("server.as.asn", "server.as.number")?;
                }
                if event.has_value("server.as.organization_name") {
                    event.rename("server.as.organization_name", "server.as.organization.name")?;
                }
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("client.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("client.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("client.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("client.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("client.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("client.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("client.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("client.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("client.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("client.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("client.as.asn") {
                    event.rename("client.as.asn", "client.as.number")?;
                }
                if event.has_value("client.as.organization_name") {
                    event.rename("client.as.organization_name", "client.as.organization.name")?;
                }
                // End nested pipeline: "geoip"
            }

            let _cond = {
                (event.has_value("source.ip") || event.has_value("destination.ip"))
                    && (event.get_str("source.ip") != Some("0.0.0.0")
                        && event.get_str("destination.ip") != Some("0.0.0.0"))
            };
            if _cond {
                // Begin nested pipeline: "endace"
                let _cond = {
                    (event.has_value("destination.ip")
                        && event.get_str("destination.ip") != Some(""))
                        && (event.has_value("source.ip") && event.get_str("source.ip") != Some(""))
                };
                if _cond {
                    event.set(
                        "_conf.ip_conv",
                        json!(format!(
                            "ip_conv={}%26{}",
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = {
                    (event.has_value("destination.ip")
                        && event.get_str("destination.ip") != Some(""))
                        && (!event.has_value("source.ip") || event.get_str("source.ip") == Some(""))
                };
                if _cond {
                    event.set(
                        "_conf.ip_conv",
                        json!(format!(
                            "ip={}",
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = {
                    (!event.has_value("destination.ip")
                        || event.get_str("destination.ip") == Some(""))
                        && (event.has_value("source.ip") && event.get_str("source.ip") != Some(""))
                };
                if _cond {
                    event.set(
                        "_conf.ip_conv",
                        json!(format!(
                            "ip={}",
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond =
                    { event.has_value("event.start") && event.get_str("event.start") != Some("") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("event.start") {
                        match parse_date_out(&date_str, &["ISO8601"], None, Some("epoch_millis")) {
                            Some(parsed) => event.set("_conf.event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    event.has_value("_conf.event.start")
                        && event.get_str("_conf.event.start") != Some("")
                };
                if _cond {
                    if let Some(val) = event.get("_conf.event.start") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "_conf.event.start".into(),
                                message,
                            }
                        })?;
                        event.set("_conf.event.start", converted)?;
                    }
                }
                let _cond =
                    { event.has_value("event.end") && event.get_str("event.end") != Some("") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("event.end") {
                        match parse_date_out(&date_str, &["ISO8601"], None, Some("epoch_millis")) {
                            Some(parsed) => event.set("_conf.event.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.end".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    event.has_value("_conf.event.end")
                        && event.get_str("_conf.event.end") != Some("")
                };
                if _cond {
                    if let Some(val) = event.get("_conf.event.end") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "_conf.event.end".into(),
                                message,
                            }
                        })?;
                        event.set("_conf.event.end", converted)?;
                    }
                }
                let _cond = {
                    event.has_value("_conf.endace_view_window")
                        && event.get_str("_conf.endace_view_window") != Some("")
                };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx._conf.timedelta = ctx._conf.endace_view_window * 60 * 1000
                    scale_field(
                        event,
                        &ScaleField::new(
                            "_conf.endace_view_window",
                            "_conf.timedelta",
                            Factor::Long(1000),
                        ),
                    );
                }
                let _cond = {
                    (event.has_value("_conf.event.end")
                        && event.get_str("_conf.event.end") != Some(""))
                        && (event.has_value("_conf.timedelta")
                            && event.get_str("_conf.timedelta") != Some(""))
                };
                if _cond {
                    // Painless script
                    // Source: ctx._conf.event.end = ctx._conf.event.end + ctx._conf.timedelta/2
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx._conf.event.end = ctx._conf.event.end + ctx._conf.timedelta/2"#
                        ),
                    )?;
                }
                let _cond = {
                    (event.has_value("_conf.event.start")
                        && event.get_str("_conf.event.start") != Some(""))
                        && (event.has_value("_conf.timedelta")
                            && event.get_str("_conf.timedelta") != Some(""))
                };
                if _cond {
                    // Painless script
                    // Source: ctx._conf.event.start = ctx._conf.event.start - ctx._conf.timedelta/2
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx._conf.event.start = ctx._conf.event.start - ctx._conf.timedelta/2"#
                        ),
                    )?;
                }
                let _cond = {
                    (event.has_value("destination.ip")
                        && event.get_str("destination.ip") != Some(""))
                        || (event.has_value("source.ip") && event.get_str("source.ip") != Some(""))
                };
                if _cond {
                    let v = json!(format!(
                        "{}/vision2/pivotintovision/?title=endace_pivot&datasources={}&start={}&end={}&tools={}&{}",
                        event
                            .get("_conf.endace_url")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_conf.endace_datasources")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_conf.event.start")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_conf.event.end")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_conf.endace_tools")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_conf.ip_conv")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("event.reference", v)?;
                    }
                }
                // End nested pipeline: "endace"
            }

            event.remove("_conf");

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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
