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

            event.set("event.kind", json!("event"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = {
                event.has_value("event.original")
                    && !(event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| {
                            x.as_str() == Some("device_product=\"Application Security Module\"")
                        }),
                        serde_json::Value::String(s) => {
                            s.contains("device_product=\"Application Security Module\"")
                        }
                        _ => false,
                    }) || event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("device_product=ASM"))
                        }
                        serde_json::Value::String(s) => s.contains("device_product=ASM"),
                        _ => false,
                    }) || event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("device_product=\"ASM\"")),
                        serde_json::Value::String(s) => s.contains("device_product=\"ASM\""),
                        _ => false,
                    }))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "json")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.set("observer.vendor", json!("F5"))?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == 'N/A' || object == 'NA') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into(), "NA".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "f5_bigip.log.hostname")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("f5_bigip.log.hostname").cloned() {
                    event.set("host.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.application") {
                event.rename("json.application", "f5_bigip.log.application.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("f5_bigip.log.application.name").cloned() {
                    event.set("network.application", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(
                    event,
                    "network.application",
                    "network.application",
                    str::to_lowercase,
                )?;
                Ok(())
            })();

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("LTM") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipltm"
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("info"))?;
                event.set("observer.product", json!("Local Traffic Manager"))?;
                let _cond = {
                    event.has_value("json.event_timestamp")
                        && event.get_str("json.event_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.event_timestamp") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "yyyy-MM-dd:HH:mm.SSSz",
                                    "yyyy-MM-dd:HH:mm:ss.SSSz",
                                    "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.event.timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.event_timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_event_timestamp")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.event.timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                let _cond = { event.get_str("json.client_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.client_ip") {
                            if let Some(val) = event.get("json.client_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.client_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.http_method") {
                    event.rename("json.http_method", "f5_bigip.log.http.method")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.http.method")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.method", v)?;
                }
                if event.has_value("json.http_referrer") {
                    event.rename("json.http_referrer", "f5_bigip.log.http.referrer")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.http.referrer")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.referrer", v)?;
                }
                if event.has_value("json.http_status") {
                    event.rename("json.http_status", "f5_bigip.log.http.status")?;
                }
                if event.has_value("json.http_version") {
                    event.rename("json.http_version", "f5_bigip.log.http.version")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.http.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.version", v)?;
                }
                let _cond = { event.get_str("json.server_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.server_ip") {
                            if let Some(val) = event.get("json.server_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.server_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.server.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.server.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.ip", v)?;
                }
                if let Some(v) = event
                    .get("server.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
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
                if let Some(v) = event
                    .get("destination.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.geo", v)?;
                }
                if let Some(v) = event
                    .get("destination.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("server.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.src_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.src_ip") {
                            if let Some(val) = event.get("json.src_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.src_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.src.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_src_ip_to_ip")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.src.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if let Some(v) = event
                    .get("source.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo", v)?;
                }
                if let Some(v) = event
                    .get("source.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.http_user_agent") {
                    event.rename("json.http_user_agent", "f5_bigip.log.http.user_agent")?;
                }
                if event.has_value("f5_bigip.log.http.user_agent") {
                    gsub_field(
                        event,
                        "f5_bigip.log.http.user_agent",
                        "f5_bigip.log.http.user_agent",
                        cached_regex!("(\\([^)]*)\\+(https?://)"),
                        "$1%2b$2",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.http.user_agent") {
                        if let Some(s) = event.get_string("f5_bigip.log.http.user_agent") {
                            match url_decode(&s) {
                                Some(decoded) => {
                                    event.set("f5_bigip.log.http.user_agent", json!(decoded))?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.http.user_agent".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if let Some(ua_str) = event.get_string("f5_bigip.log.http.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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
                if event.has_value("json.event_source") {
                    event.rename("json.event_source", "f5_bigip.log.event.source")?;
                }
                if event.has_value("json.http_uri") {
                    event.rename("json.http_uri", "f5_bigip.log.http.uri")?;
                }
                if event.has_value("json.telemetryEventCategory") {
                    event.rename(
                        "json.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                if event.has_value("json.tenant") {
                    event.rename("json.tenant", "f5_bigip.log.tenant")?;
                }
                if event.has_value("json.virtual_name") {
                    event.rename("json.virtual_name", "f5_bigip.log.virtual.name")?;
                }
                let _cond = { event.get_str("json.bytes_in") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.bytes_in") {
                            if let Some(val) = event.get("json.bytes_in") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.bytes_in".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bytes.in", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bytes_in_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.bytes_out") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.bytes_out") {
                            if let Some(val) = event.get("json.bytes_out") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.bytes_out".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bytes.out", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bytes_out_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bytes.in")
                        && event.has_value("f5_bigip.log.bytes.out")
                };
                if _cond {
                    // Painless script
                    // Source: ctx.network = new HashMap();\nctx.network.bytes = ctx.f5_bigip.log.bytes.in + ctx.f5_bigip.log.bytes.out;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network = new HashMap();\nctx.network.bytes = ctx.f5_bigip.log.bytes.in + ctx.f5_bigip.log.bytes.out;\n"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bytes.in")
                        && event.get_i64("f5_bigip.log.bytes.in") != Some(0)
                        && event.get_i64("f5_bigip.log.bytes.out") == Some(0)
                };
                if _cond {
                    let v = json!("ingress");
                    if !painless_is_empty_value(&v) {
                        event.set("network.direction", v)?;
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bytes.out")
                        && event.get_i64("f5_bigip.log.bytes.out") != Some(0)
                        && event.get_i64("f5_bigip.log.bytes.in") == Some(0)
                };
                if _cond {
                    let v = json!("egress");
                    if !painless_is_empty_value(&v) {
                        event.set("network.direction", v)?;
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.http.method")
                        && event.get_str("f5_bigip.log.http.method") != Some("")
                };
                if _cond {
                    let v = json!("http");
                    if !painless_is_empty_value(&v) {
                        event.set("network.protocol", v)?;
                    }
                }
                if event.has_value("json.cookie") {
                    event.rename("json.cookie", "f5_bigip.log.cookie")?;
                }
                if event.has_value("json.http_content_type") {
                    event.rename("json.http_content_type", "f5_bigip.log.http.content_type")?;
                }
                if event.has_value("json.http_host") {
                    event.rename("json.http_host", "f5_bigip.log.http.host")?;
                }
                if event.has_value("json.http_url") {
                    event.rename("json.http_url", "f5_bigip.log.http.url")?;
                }
                let _cond = { event.has_value("f5_bigip.log.http.url") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if !uri_parts(event, "f5_bigip.log.http.url", "url", true, false)?
                            && event
                                .get_str("f5_bigip.log.http.url")
                                .is_some_and(|value| !value.is_empty())
                        {
                            return Err(TransformError::ParseError {
                                path: "f5_bigip.log.http.url".into(),
                                message: "uri_parts: not a parseable URI".into(),
                            });
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.node") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.node") {
                            if let Some(val) = event.get("json.node") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.node".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.node", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_node_to_ip")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.node")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.node_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.node_port") {
                            if let Some(val) = event.get("json.node_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.node_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.node_port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_node_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.req_elapsed_time") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.req_elapsed_time") {
                            if let Some(val) = event.get("json.req_elapsed_time") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.req_elapsed_time".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.req.elapsed_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_req_elapsed_time_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.req_start_time")
                        && event.get_str("json.req_start_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.req_start_time") {
                            match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss"], None, None) {
                                Some(parsed) => event.set("f5_bigip.log.req.start_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.req_start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_req_start_time")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.res_start_time")
                        && event.get_str("json.res_start_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.res_start_time") {
                            match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss"], None, None) {
                                Some(parsed) => event.set("f5_bigip.log.res.start_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.res_start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_res_start_time")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.user") {
                    event.rename("json.user", "f5_bigip.log.user.name")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.user.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.vip") {
                    event.rename("json.vip", "f5_bigip.log.vip")?;
                }
                if event.has_value("json.virtual_server") {
                    event.rename("json.virtual_server", "f5_bigip.log.virtual.server")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    Ok(())
                })();
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("f5_bigip.log.event.timestamp");
                        event.remove("f5_bigip.log.client.ip");
                        event.remove("f5_bigip.log.hostname");
                        event.remove("f5_bigip.log.http.method");
                        event.remove("f5_bigip.log.http.referrer");
                        event.remove("f5_bigip.log.http.version");
                        event.remove("f5_bigip.log.server.ip");
                        event.remove("f5_bigip.log.src.ip");
                        event.remove("f5_bigip.log.user.name");
                        event.remove("f5_bigip.log.application.name");
                        event.remove("f5_bigip.log.http.user_agent");
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_bigipltm"
            }

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("AFM") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipafm"
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("info"))?;
                event.set("observer.product", json!("Advanced Firewall Module"))?;
                let _cond = {
                    event.has_value("json.date_time") && event.get_str("json.date_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.date_time") {
                            match parse_date_out(
                                &date_str,
                                &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("f5_bigip.log.date_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_time_conversion")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.date_time").cloned() {
                        event.set("@timestamp", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("json.dest_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.dest_ip") {
                            if let Some(val) = event.get("json.dest_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.dest_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.dest.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_dest_ip_to_ip")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.dest.ip").cloned() {
                        event.set("destination.ip", v)?;
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.ip", v)?;
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
                if let Some(v) = event
                    .get("destination.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.geo", v)?;
                }
                if let Some(v) = event
                    .get("destination.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.dest_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.dest_port") {
                            if let Some(val) = event.get("json.dest_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.dest_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.dest.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dest_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.dest.port").cloned() {
                        event.set("destination.port", v)?;
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.port", v)?;
                }
                if event.has_value("json.action") {
                    event.rename("json.action", "f5_bigip.log.action")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.action").cloned() {
                        event.set("event.action", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("json.severity") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.severity") {
                            if let Some(val) = event.get("json.severity") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.severity".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.severity.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_severity_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.severity.code").cloned() {
                        event.set("event.severity", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.ip_protocol") {
                    event.rename("json.ip_protocol", "f5_bigip.log.ip_protocol")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.ip_protocol").cloned() {
                        event.set("network.transport", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("network.transport") {
                    map_strings(
                        event,
                        "network.transport",
                        "network.transport",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.get_str("json.source_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.source_ip") {
                            if let Some(val) = event.get("json.source_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.source_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                if let Some(v) = event
                    .get("source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if let Some(v) = event
                    .get("source.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo", v)?;
                }
                if let Some(v) = event
                    .get("source.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.source_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.source_port") {
                            if let Some(val) = event.get("json.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.source.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                if let Some(v) = event
                    .get("source.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.port", v)?;
                }
                if event.has_value("json.source_user_group") {
                    event.rename("json.source_user_group", "f5_bigip.log.source.user_group")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.source.user_group")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.group.name", v)?;
                }
                if event.has_value("json.source_user") {
                    event.rename("json.source_user", "f5_bigip.log.source.user")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.source.user")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.name", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.acl_policy_name") {
                    event.rename("json.acl_policy_name", "f5_bigip.log.acl.policy.name")?;
                }
                if event.has_value("json.acl_policy_type") {
                    event.rename("json.acl_policy_type", "f5_bigip.log.acl.policy.type")?;
                }
                if event.has_value("json.acl_rule_name") {
                    event.rename("json.acl_rule_name", "f5_bigip.log.acl.rule.name")?;
                }
                let _cond = { event.get_str("json.bigip_mgmt_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.bigip_mgmt_ip") {
                            if let Some(val) = event.get("json.bigip_mgmt_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.bigip_mgmt_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.mgmt_ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bigip_mgmt_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.mgmt_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.context_name") {
                    event.rename("json.context_name", "f5_bigip.log.context.name")?;
                }
                if event.has_value("json.context_type") {
                    event.rename("json.context_type", "f5_bigip.log.context.type")?;
                }
                if event.has_value("json.dest_fqdn") {
                    event.rename("json.dest_fqdn", "f5_bigip.log.dest.fqdn")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.dest.fqdn")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.domain", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.dest.fqdn")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.device_product") {
                    event.rename("json.device_product", "f5_bigip.log.device.product")?;
                }
                if event.has_value("json.device_vendor") {
                    event.rename("json.device_vendor", "f5_bigip.log.device.vendor")?;
                }
                if event.has_value("json.device_version") {
                    event.rename("json.device_version", "f5_bigip.log.device.version")?;
                }
                if event.has_value("json.drop_reason") {
                    event.rename("json.drop_reason", "f5_bigip.log.drop_reason")?;
                }
                if event.has_value("json.dst_geo") {
                    event.rename("json.dst_geo", "f5_bigip.log.dst.geo")?;
                }
                if event.has_value("json.errdefs_msg_name") {
                    event.rename("json.errdefs_msg_name", "f5_bigip.log.errdefs.msg_name")?;
                }
                if event.has_value("json.errdefs_msgno") {
                    event.rename("json.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
                }
                if event.has_value("json.flow_id") {
                    event.rename("json.flow_id", "f5_bigip.log.flow.id")?;
                }
                if event.has_value("json.partition_name") {
                    event.rename("json.partition_name", "f5_bigip.log.partition_name")?;
                }
                if event.has_value("json.route_domain") {
                    event.rename("json.route_domain", "f5_bigip.log.route_domain")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.sa_translation_pool") {
                    event.rename(
                        "json.sa_translation_pool",
                        "f5_bigip.log.sa_translation.pool",
                    )?;
                }
                if event.has_value("json.sa_translation_type") {
                    event.rename(
                        "json.sa_translation_type",
                        "f5_bigip.log.sa_translation.type",
                    )?;
                }
                if event.has_value("json.send_to_vs") {
                    event.rename("json.send_to_vs", "f5_bigip.log.send_to_vs")?;
                }
                if event.has_value("json.source_fqdn") {
                    event.rename("json.source_fqdn", "f5_bigip.log.source.fqdn")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.source.fqdn")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.domain", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.source.fqdn")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.src_geo") {
                    event.rename("json.src_geo", "f5_bigip.log.src.geo")?;
                }
                if event.has_value("json.telemetryEventCategory") {
                    event.rename(
                        "json.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                if event.has_value("json.tenant") {
                    event.rename("json.tenant", "f5_bigip.log.tenant")?;
                }
                let _cond = { event.get_str("json.translated_dest_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.translated_dest_ip") {
                            if let Some(val) = event.get("json.translated_dest_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.translated_dest_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.translated.dest.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_translated_dest_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.translated.dest.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.translated_dest_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.translated_dest_port") {
                            if let Some(val) = event.get("json.translated_dest_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.translated_dest_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.translated.dest.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_translated_dest_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.translated_ip_protocol") {
                    event.rename(
                        "json.translated_ip_protocol",
                        "f5_bigip.log.translated.ip_protocol",
                    )?;
                }
                if event.has_value("json.translated_route_domain") {
                    event.rename(
                        "json.translated_route_domain",
                        "f5_bigip.log.translated.route_domain",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.translated.route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.translated_source_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.translated_source_ip") {
                            if let Some(val) = event.get("json.translated_source_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.translated_source_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.translated.source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_translated_source_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.translated.source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.translated_source_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.translated_source_port") {
                            if let Some(val) = event.get("json.translated_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.translated_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.translated.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_translated_source_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.translated_vlan") {
                    event.rename("json.translated_vlan", "f5_bigip.log.translated.vlan")?;
                }
                if event.has_value("json.acl_rule_uuid") {
                    event.rename("json.acl_rule_uuid", "f5_bigip.log.acl.rule.uuid")?;
                }
                if event.has_value("json.src_zone") {
                    event.rename("json.src_zone", "f5_bigip.log.src.zone")?;
                }
                if event.has_value("json.dest_ipint_categories") {
                    event.rename(
                        "json.dest_ipint_categories",
                        "f5_bigip.log.dest.ipint_categories",
                    )?;
                }
                if event.has_value("json.dest_vlan") {
                    event.rename("json.dest_vlan", "f5_bigip.log.dest.vlan")?;
                }
                if event.has_value("json.dest_zone") {
                    event.rename("json.dest_zone", "f5_bigip.log.dest.zone")?;
                }
                if event.has_value("json.source_ipint_categories") {
                    event.rename(
                        "json.source_ipint_categories",
                        "f5_bigip.log.source.ipint_categories",
                    )?;
                }
                if event.has_value("json.vlan") {
                    event.rename("json.vlan", "f5_bigip.log.vlan")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    Ok(())
                })();
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("f5_bigip.log.date_time");
                        event.remove("f5_bigip.log.dest.ip");
                        event.remove("f5_bigip.log.dest.port");
                        event.remove("f5_bigip.log.action");
                        event.remove("f5_bigip.log.hostname");
                        event.remove("f5_bigip.log.severity.code");
                        event.remove("f5_bigip.log.dest.fqdn");
                        event.remove("f5_bigip.log.source.fqdn");
                        event.remove("f5_bigip.log.application.name");
                        event.remove("f5_bigip.log.ip_protocol");
                        event.remove("f5_bigip.log.source.ip");
                        event.remove("f5_bigip.log.source.port");
                        event.remove("f5_bigip.log.source.user_group");
                        event.remove("f5_bigip.log.source.user");
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_bigipafm"
            }

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("APM") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipapm"
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("info"))?;
                event.set(
                    "observer.product",
                    json!("Application Performance Monitoring"),
                )?;
                let _cond = {
                    event.has_value("json.f5telemetry_timestamp")
                        && event.get_str("json.f5telemetry_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.f5telemetry_timestamp") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.telemetry.timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.f5telemetry_timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_f5telemetry_timestamp",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.telemetry.timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                let _cond = { event.get_str("json.Bytes_In") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.Bytes_In") {
                            if let Some(val) = event.get("json.Bytes_In") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.Bytes_In".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bytes.in", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bytes_in_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.Bytes_Out") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.Bytes_Out") {
                            if let Some(val) = event.get("json.Bytes_Out") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.Bytes_Out".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bytes.out", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bytes_out_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bytes.in")
                        && event.has_value("f5_bigip.log.bytes.out")
                };
                if _cond {
                    // Painless script
                    // Source: ctx.network = new HashMap();\nctx.network.bytes = ctx.f5_bigip.log.bytes.in + ctx.f5_bigip.log.bytes.out;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network = new HashMap();\nctx.network.bytes = ctx.f5_bigip.log.bytes.in + ctx.f5_bigip.log.bytes.out;\n"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bytes.in")
                        && event.get_i64("f5_bigip.log.bytes.in") != Some(0)
                        && event.get_i64("f5_bigip.log.bytes.out") == Some(0)
                };
                if _cond {
                    let v = json!("ingress");
                    if !painless_is_empty_value(&v) {
                        event.set("network.direction", v)?;
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bytes.out")
                        && event.get_i64("f5_bigip.log.bytes.out") != Some(0)
                        && event.get_i64("f5_bigip.log.bytes.in") == Some(0)
                };
                if _cond {
                    let v = json!("egress");
                    if !painless_is_empty_value(&v) {
                        event.set("network.direction", v)?;
                    }
                }
                if event.has_value("json.User_Agent") {
                    event.rename("json.User_Agent", "f5_bigip.log.user.agent")?;
                }
                if event.has_value("f5_bigip.log.user.agent") {
                    gsub_field(
                        event,
                        "f5_bigip.log.user.agent",
                        "f5_bigip.log.user.agent",
                        cached_regex!("(\\([^)]*)\\+(https?://)"),
                        "$1%2b$2",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.user.agent") {
                        if let Some(s) = event.get_string("f5_bigip.log.user.agent") {
                            match url_decode(&s) {
                                Some(decoded) => {
                                    event.set("f5_bigip.log.user.agent", json!(decoded))?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.user.agent".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if let Some(ua_str) = event.get_string("f5_bigip.log.user.agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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
                let _cond = { event.get_str("json.Client_IP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.Client_IP") {
                            if let Some(val) = event.get("json.Client_IP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.Client_IP".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
                }
                if let Some(v) = event
                    .get("client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if let Some(v) = event
                    .get("source.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo", v)?;
                }
                if let Some(v) = event
                    .get("source.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.Continent") {
                    event.rename("json.Continent", "f5_bigip.log.continent")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.continent")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.geo.continent_name", v)?;
                }
                if event.has_value("json.Country") {
                    event.rename("json.Country", "f5_bigip.log.country")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.country").cloned() {
                        event.set("host.geo.country_name", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.Listener") {
                    event.rename("json.Listener", "f5_bigip.log.listener")?;
                }
                if event.has_value("json.Reputation") {
                    event.rename("json.Reputation", "f5_bigip.log.reputation")?;
                }
                if event.has_value("json.State") {
                    event.rename("json.State", "f5_bigip.log.state")?;
                }
                let _cond = { event.get_str("json.Virtual_IP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.Virtual_IP") {
                            if let Some(val) = event.get("json.Virtual_IP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.Virtual_IP".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.virtual.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_virtual_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.virtual.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.Access_Policy_Result") {
                    event.rename(
                        "json.Access_Policy_Result",
                        "f5_bigip.log.access.policy_result",
                    )?;
                }
                if event.has_value("json.Access_Profile") {
                    event.rename("json.Access_Profile", "f5_bigip.log.access.profile")?;
                }
                if event.has_value("json.errdefs_msgno") {
                    event.rename("json.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
                }
                if event.has_value("json.Partition") {
                    event.rename("json.Partition", "f5_bigip.log.partition")?;
                }
                if event.has_value("json.partition_name") {
                    event.rename("json.partition_name", "f5_bigip.log.partition_name")?;
                }
                if event.has_value("json.session_id") {
                    event.rename("json.session_id", "f5_bigip.log.session.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "f5_bigip.log.session.id",
                        json!(
                            event
                                .get("json.session_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.append_unique(
                        "f5_bigip.log.session.id",
                        json!(
                            event
                                .get("json.Session_Id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.append_unique(
                        "f5_bigip.log.session.id",
                        json!(
                            event
                                .get("json.Session_ID")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.telemetryEventCategory") {
                    event.rename(
                        "json.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                if event.has_value("json.tenant") {
                    event.rename("json.tenant", "f5_bigip.log.tenant")?;
                }
                let _cond = { event.get_str("json.Max_concurrent_Users") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.Max_concurrent_Users") {
                            if let Some(val) = event.get("json.Max_concurrent_Users") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.Max_concurrent_Users".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.concurrent.users.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_concurrent_users_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    Ok(())
                })();
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("f5_bigip.log.telemetry.timestamp");
                        event.remove("f5_bigip.log.hostname");
                        event.remove("f5_bigip.log.application.name");
                        event.remove("f5_bigip.log.user.agent");
                        event.remove("f5_bigip.log.client.ip");
                        event.remove("f5_bigip.log.continent");
                        event.remove("f5_bigip.log.country");
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.set("event.kind", json!("pipeline_error"))?;
                }
                // End nested pipeline: "pipeline_bigipapm"
            }

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("ASM") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipasm"
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("info"))?;
                let _cond = {
                    event.has_value("json.severity")
                        && event.get_str("json.severity").is_some_and(|s| {
                            ["emergency", "critical", "alert", "warning", "error"]
                                .contains(&s.to_lowercase().as_str())
                        })
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                event.set("observer.product", json!("Application Security Module"))?;
                let _cond = {
                    event.has_value("json.date_time") && event.get_str("json.date_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.date_time") {
                            match parse_date_out(
                                &date_str,
                                &["yyyy-MM-dd HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("f5_bigip.log.date_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_time_conversion")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.date_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                let _cond = { event.get_str("json.ip_client") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ip_client") {
                            if let Some(val) = event.get("json.ip_client") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ip_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
                }
                if let Some(v) = event
                    .get("client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if let Some(v) = event
                    .get("source.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo", v)?;
                }
                if let Some(v) = event
                    .get("source.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.dest_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.dest_ip") {
                            if let Some(val) = event.get("json.dest_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.dest_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.dest.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_dest_ip_to_ip")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.dest.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
                }
                if let Some(v) = event
                    .get("destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.ip", v)?;
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
                if let Some(v) = event
                    .get("destination.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.geo", v)?;
                }
                if let Some(v) = event
                    .get("destination.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.dest_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.dest_port") {
                            if let Some(val) = event.get("json.dest_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.dest_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.dest.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dest_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.dest.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.port", v)?;
                }
                if let Some(v) = event
                    .get("destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.port", v)?;
                }
                if event.has_value("json.geo_location") {
                    event.rename("json.geo_location", "f5_bigip.log.geo.location")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.geo.location")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.geo.country_iso_code", v)?;
                }
                if event.has_value("json.device_id") {
                    event.rename("json.device_id", "f5_bigip.log.device.id")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.device.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.method") {
                    event.rename("json.method", "f5_bigip.log.method")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.method")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.method", v)?;
                }
                if event.has_value("json.severity") {
                    event.rename("json.severity", "f5_bigip.log.severity.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.severity.name").cloned() {
                        event.set("log.level", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("log.level") {
                    map_strings(event, "log.level", "log.level", str::to_lowercase)?;
                }
                if event.has_value("json.protocol") {
                    event.rename("json.protocol", "f5_bigip.log.protocol")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.protocol")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.protocol", v)?;
                }
                let _cond = { event.get_str("json.src_port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.src_port") {
                            if let Some(val) = event.get("json.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.src.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_src_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.src.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                if let Some(v) = event
                    .get("source.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.port", v)?;
                }
                if event.has_value("json.username") {
                    event.rename("json.username", "f5_bigip.log.username")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.username")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.attack_type") {
                    event.rename("json.attack_type", "f5_bigip.log.attack.type")?;
                }
                if event.has_value("json.blocking_exception_reason") {
                    event.rename(
                        "json.blocking_exception_reason",
                        "f5_bigip.log.blocking_exception_reason",
                    )?;
                }
                if event.has_value("json.captcha_result") {
                    event.rename("json.captcha_result", "f5_bigip.log.captcha_result")?;
                }
                if event.has_value("json.fragment") {
                    event.rename("json.fragment", "f5_bigip.log.fragment")?;
                }
                if event.has_value("json.http_class_name") {
                    event.rename("json.http_class_name", "f5_bigip.log.http.class_name")?;
                }
                if event.has_value("json.ip_address_intelligence") {
                    event.rename(
                        "json.ip_address_intelligence",
                        "f5_bigip.log.ip_address_intelligence",
                    )?;
                }
                let _cond = { event.get_str("json.management_ip_address") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.management_ip_address") {
                            if let Some(val) = event.get("json.management_ip_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.management_ip_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.management.ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_management_ip_address_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.management.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.management_ip_address_2") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.management_ip_address_2") {
                            if let Some(val) = event.get("json.management_ip_address_2") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.management_ip_address_2".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.management.ip_address_2", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_management_ip_address_2_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.management.ip_address_2")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("json.policy_apply_date")
                        && event.get_str("json.policy_apply_date") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.policy_apply_date") {
                            match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.policy.apply_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.policy_apply_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_policy_apply_date")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.policy_name") {
                    event.rename("json.policy_name", "f5_bigip.log.policy.name")?;
                }
                // Painless script
                // Source: def log; if (ctx.event?.original != null) {\n  log = ctx.event.original;\n} else if (ctx.json?.originalRawData != null) {\n  log = ctx.json.originalRawData;\n} if (log != null) {\n  def sAMAccountNameMatch = /sAMAccountName=([^\\\\)]+)/.matcher(log);\n  if (sAMAccountNameMatch.find()) {\n    ctx.f5_bigip.log.sam_account_name = sAMAccountNameMatch.group(1);\n  }\n\n  def userPrincipleNameMatch = /UserPrincipleName=([^\\\\)]+)/.matcher(log);\n  if (userPrincipleNameMatch.find()) {\n    ctx.f5_bigip.log.user_principle_name = userPrincipleNameMatch.group(1);\n  }\n\n  def userNameMatch = /User_Name=([^\\\\)]+)/.matcher(log);\n  if (userNameMatch.find()) {\n    ctx.f5_bigip.log.user_name = userNameMatch.group(1);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def log; if (ctx.event?.original != null) {\n  log = ctx.event.original;\n} else if (ctx.json?.originalRawData != null) {\n  log = ctx.json.originalRawData;\n} if (log != null) {\n  def sAMAccountNameMatch = /sAMAccountName=([^\\\\)]+)/.matcher(log);\n  if (sAMAccountNameMatch.find()) {\n    ctx.f5_bigip.log.sam_account_name = sAMAccountNameMatch.group(1);\n  }\n\n  def userPrincipleNameMatch = /UserPrincipleName=([^\\\\)]+)/.matcher(log);\n  if (userPrincipleNameMatch.find()) {\n    ctx.f5_bigip.log.user_principle_name = userPrincipleNameMatch.group(1);\n  }\n\n  def userNameMatch = /User_Name=([^\\\\)]+)/.matcher(log);\n  if (userNameMatch.find()) {\n    ctx.f5_bigip.log.user_name = userNameMatch.group(1);\n  }\n}\n"#
                    ),
                )?;
                if event.has_value("json.query_string") {
                    event.rename("json.query_string", "f5_bigip.log.query.string")?;
                }
                if event.has_value("f5_bigip.log.sam_account_name") {
                    event.rename(
                        "f5_bigip.log.sam_account_name",
                        "f5_bigip.log.query.sam_account_name",
                    )?;
                }
                let _cond = { event.has_value("f5_bigip.log.query.sam_account_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("f5_bigip.log.query.sam_account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("f5_bigip.log.user_principle_name") {
                    event.rename(
                        "f5_bigip.log.user_principle_name",
                        "f5_bigip.log.query.user_principle_name",
                    )?;
                }
                if event.has_value("f5_bigip.log.user_name") {
                    event.rename("f5_bigip.log.user_name", "f5_bigip.log.query.user_name")?;
                }
                if event.has_value("json.request") {
                    event.rename("json.request", "f5_bigip.log.request.detail")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("f5_bigip.log.request.detail") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.method", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.path", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nHost: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.protocol", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nHost: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nConnection: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.host", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nConnection: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nCache-Control: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.connection", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nCache-Control: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("f5_bigip.log.request.cache_control", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("f5_bigip.log.request.detail") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.method", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.path", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\r\nHost: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.protocol", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\r\nHost: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\r\nUser-Agent: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.host", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\r\nUser-Agent: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\r\nAccept: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.user_agent", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\r\nAccept: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\r\nX-Forwarded-For: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.request.accept", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\r\nX-Forwarded-For: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\r\n\r\n") else {
                                break 'dissect false;
                            };
                            captured
                                .push(("f5_bigip.log.request.x_forwarded_for", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\r\n\r\n") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.request.host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("related.hosts") {
                    map_strings(event, "related.hosts", "related.hosts", str::to_lowercase)?;
                }
                if event.has_value("f5_bigip.log.request.user_agent") {
                    gsub_field(
                        event,
                        "f5_bigip.log.request.user_agent",
                        "f5_bigip.log.request.user_agent",
                        cached_regex!("(\\([^)]*)\\+(https?://)"),
                        "$1%2b$2",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.request.user_agent") {
                        if let Some(s) = event.get_string("f5_bigip.log.request.user_agent") {
                            match url_decode(&s) {
                                Some(decoded) => {
                                    event.set("f5_bigip.log.request.user_agent", json!(decoded))?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.request.user_agent".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("f5_bigip.log.request.user_agent") {
                    if let Some(ua_str) = event.get_string("f5_bigip.log.request.user_agent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.remove("user_agent");
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("f5_bigip.log.request.protocol") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("network.protocol", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
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
                        }
                    }
                    Ok(())
                })();
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                let _cond = {
                    event.has_value("network.protocol")
                        && event.has_value("f5_bigip.log.request.host")
                        && event.has_value("f5_bigip.log.request.protocol")
                };
                if _cond {
                    // Painless script
                    // Source: if (ctx.url == null) {\n    ctx.url = new HashMap();\n}\nctx.url.original = ctx.network.protocol + '://' +ctx.f5_bigip.log.request.host + ctx.f5_bigip.log.request.path\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.url == null) {\n    ctx.url = new HashMap();\n}\nctx.url.original = ctx.network.protocol + '://' +ctx.f5_bigip.log.request.host + ctx.f5_bigip.log.request.path\n"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("url.original") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if !uri_parts(event, "url.original", "url", true, false)?
                            && event
                                .get_str("url.original")
                                .is_some_and(|value| !value.is_empty())
                        {
                            return Err(TransformError::ParseError {
                                path: "url.original".into(),
                                message: "uri_parts: not a parseable URI".into(),
                            });
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("f5_bigip.log.request.x_forwarded_for") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("f5_bigip.log.request.x_forwarded_for") {
                            if let Some(val) = event.get("f5_bigip.log.request.x_forwarded_for") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "f5_bigip.log.request.x_forwarded_for".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.request.x_forwarded_for", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_x_forwarded_for_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.request.x_forwarded_for")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.request_status") {
                    event.rename("json.request_status", "f5_bigip.log.request.status")?;
                }
                let _cond = { event.get_str("json.response_code") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.response_code") {
                            if let Some(val) = event.get("json.response_code") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.response_code".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.response.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_response_code_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.route_domain") {
                    event.rename("json.route_domain", "f5_bigip.log.route_domain")?;
                }
                if event.has_value("json.session_id") {
                    event.rename("json.session_id", "f5_bigip.log.session.id")?;
                }
                if event.has_value("json.sig_ids") {
                    event.rename("json.sig_ids", "f5_bigip.log.sig.ids")?;
                }
                if event.has_value("json.sig_names") {
                    event.rename("json.sig_names", "f5_bigip.log.sig.names")?;
                }
                if event.has_value("json.staged_sig_ids") {
                    event.rename("json.staged_sig_ids", "f5_bigip.log.staged.sig.ids")?;
                }
                if event.has_value("json.staged_sig_names") {
                    event.rename("json.staged_sig_names", "f5_bigip.log.staged.sig.names")?;
                }
                if event.has_value("json.staged_threat_campaign_names") {
                    event.rename(
                        "json.staged_threat_campaign_names",
                        "f5_bigip.log.staged.threat_campaign_names",
                    )?;
                }
                if event.has_value("json.sub_violations") {
                    event.rename("json.sub_violations", "f5_bigip.log.sub_violations")?;
                }
                if event.has_value("json.support_id") {
                    event.rename("json.support_id", "f5_bigip.log.support.id")?;
                }
                if event.has_value("json.telemetryEventCategory") {
                    event.rename(
                        "json.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                if event.has_value("json.tenant") {
                    event.rename("json.tenant", "f5_bigip.log.tenant")?;
                }
                if event.has_value("json.threat_campaign_names") {
                    event.rename(
                        "json.threat_campaign_names",
                        "f5_bigip.log.threat_campaign_names",
                    )?;
                }
                if event.has_value("json.uri") {
                    event.rename("json.uri", "f5_bigip.log.uri")?;
                }
                let _cond = { event.get_str("json.violation_rating") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.violation_rating") {
                            if let Some(val) = event.get("json.violation_rating") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.violation_rating".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.violation.rating", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_violation_rating_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.violations") {
                    event.rename("json.violations", "f5_bigip.log.violations")?;
                }
                if event.has_value("json.virus_name") {
                    event.rename("json.virus_name", "f5_bigip.log.virus_name")?;
                }
                if event.has_value("json.web_application_name") {
                    event.rename(
                        "json.web_application_name",
                        "f5_bigip.log.web_application_name",
                    )?;
                }
                if event.has_value("json.websocket_direction") {
                    event.rename(
                        "json.websocket_direction",
                        "f5_bigip.log.websocket.direction",
                    )?;
                }
                if event.has_value("json.websocket_message_type") {
                    event.rename(
                        "json.websocket_message_type",
                        "f5_bigip.log.websocket.message_type",
                    )?;
                }
                if event.has_value("json.x_forwarded_for_header_value") {
                    if let Some(s) = event.get_string("json.x_forwarded_for_header_value") {
                        let mut parts: Vec<Value> = cached_regex!(",\\s*")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("json.x_forwarded_for_header_value", Value::Array(parts))?;
                    }
                }
                let _cond = { event.get_str("json.x_forwarded_for_header_value") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.x_forwarded_for_header_value") {
                            if let Some(val) = event.get("json.x_forwarded_for_header_value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.x_forwarded_for_header_value".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("f5_bigip.log.x_forwarded_for_header_value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_x_forwarded_for_header_value_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event
                        .get("f5_bigip.log.x_forwarded_for_header_value")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "f5_bigip.log.x_forwarded_for_header_value",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                }
                if event.has_value("json.geo_info") {
                    event.rename("json.geo_info", "f5_bigip.log.geo.info")?;
                }
                if event.has_value("json.headers") {
                    event.rename("json.headers", "f5_bigip.log.headers")?;
                }
                if event.has_value("json.ip_route_domain") {
                    event.rename("json.ip_route_domain", "f5_bigip.log.ip_route_domain")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.ip_route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.is_trunct") {
                    event.rename("json.is_trunct", "f5_bigip.log.is_trunct")?;
                }
                if event.has_value("json.resp") {
                    event.rename("json.resp", "f5_bigip.log.resp")?;
                }
                if event.has_value("json.unit_host") {
                    event.rename("json.unit_host", "f5_bigip.log.unit_host")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.unit_host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.violate_details") {
                    event.rename("json.violate_details", "f5_bigip.log.violate_details")?;
                }
                if event.has_value("json.microservice") {
                    event.rename("json.microservice", "f5_bigip.log.microservice")?;
                }
                if event.has_value("json.response") {
                    event.rename("json.response", "f5_bigip.log.response.value")?;
                }
                if event.has_value("json.sig_cves") {
                    event.rename("json.sig_cves", "f5_bigip.log.sig.cves")?;
                }
                if event.has_value("json.staged_sig_cves") {
                    event.rename("json.staged_sig_cves", "f5_bigip.log.staged.sig.cves")?;
                }
                if event.has_value("json.tap_event_id") {
                    event.rename("json.tap_event_id", "f5_bigip.log.tap.event_id")?;
                }
                if event.has_value("json.tap_vid") {
                    event.rename("json.tap_vid", "f5_bigip.log.tap.vid")?;
                }
                if event.has_value("json.vs_name") {
                    event.rename("json.vs_name", "f5_bigip.log.vs_name")?;
                }
                if event.has_value("json.compression_method") {
                    event.rename("json.compression_method", "f5_bigip.log.compression_method")?;
                }
                if event.has_value("json.client_type") {
                    event.rename("json.client_type", "f5_bigip.log.client.type")?;
                }
                if event.has_value("json.conviction_traps") {
                    event.rename("json.conviction_traps", "f5_bigip.log.conviction_traps")?;
                }
                if event.has_value("json.credential_stuffing_lookup_result") {
                    event.rename(
                        "json.credential_stuffing_lookup_result",
                        "f5_bigip.log.credential_stuffing_lookup_result",
                    )?;
                }
                if event.has_value("json.enforced_by") {
                    event.rename("json.enforced_by", "f5_bigip.log.enforced_by")?;
                }
                if event.has_value("json.enforcement_action") {
                    event.rename("json.enforcement_action", "f5_bigip.log.enforcement_action")?;
                }
                let _cond = {
                    event.has_value("json.epoch_time")
                        && event.get_str("json.epoch_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.epoch_time") {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set("f5_bigip.log.epoch_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.epoch_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_epoch_time")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.ip_with_route_domain") {
                    event.rename(
                        "json.ip_with_route_domain",
                        "f5_bigip.log.ip_with_route_domain",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.ip_with_route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.is_truncated") {
                    event.rename("json.is_truncated", "f5_bigip.log.is_truncated")?;
                }
                if event.has_value("json.likely_false_positive_sig_ids") {
                    event.rename(
                        "json.likely_false_positive_sig_ids",
                        "f5_bigip.log.likely_false_positive_sig_ids",
                    )?;
                }
                if event.has_value("json.login_result") {
                    event.rename("json.login_result", "f5_bigip.log.login_result")?;
                }
                if event.has_value("json.mobile_application_name") {
                    event.rename(
                        "json.mobile_application_name",
                        "f5_bigip.log.mobile_application.name",
                    )?;
                }
                if event.has_value("json.mobile_application_version") {
                    event.rename(
                        "json.mobile_application_version",
                        "f5_bigip.log.mobile_application.version",
                    )?;
                }
                if event.has_value("json.operation_id") {
                    event.rename("json.operation_id", "f5_bigip.log.operation.id")?;
                }
                if event.has_value("json.password_hash_prefix") {
                    event.rename(
                        "json.password_hash_prefix",
                        "f5_bigip.log.password_hash_prefix",
                    )?;
                }
                if event.has_value("json.protocol_info") {
                    event.rename("json.protocol_info", "f5_bigip.log.protocol_info")?;
                }
                if event.has_value("json.sig_set_names") {
                    event.rename("json.sig_set_names", "f5_bigip.log.sig.set_names")?;
                }
                let _cond = { event.get_str("json.slot_number") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.slot_number") {
                            if let Some(val) = event.get("json.slot_number") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.slot_number".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.slot.number", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_slot_number")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.staged_sig_set_names") {
                    event.rename(
                        "json.staged_sig_set_names",
                        "f5_bigip.log.staged.sig.set_names",
                    )?;
                }
                if event.has_value("json.tap_requested_actions") {
                    event.rename(
                        "json.tap_requested_actions",
                        "f5_bigip.log.tap.requested_actions",
                    )?;
                }
                let _cond = { event.get_str("json.tap_sent_token") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.tap_sent_token") {
                            if let Some(val) = event.get("json.tap_sent_token") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.tap_sent_token".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.tap.sent_token", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_tap_sent_token_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.tap_transaction_id") {
                    event.rename("json.tap_transaction_id", "f5_bigip.log.tap.transaction_id")?;
                }
                if event.has_value("json.unit_hostname") {
                    event.rename("json.unit_hostname", "f5_bigip.log.unit_hostname")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.unit_hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.violation_details") {
                    event.rename("json.violation_details", "f5_bigip.log.violation.details")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    Ok(())
                })();
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("f5_bigip.log.date_time");
                        event.remove("f5_bigip.log.client.ip");
                        event.remove("f5_bigip.log.dest.ip");
                        event.remove("f5_bigip.log.dest.port");
                        event.remove("f5_bigip.log.geo.location");
                        event.remove("f5_bigip.log.device.id");
                        event.remove("f5_bigip.log.hostname");
                        event.remove("f5_bigip.log.method");
                        event.remove("f5_bigip.log.protocol");
                        event.remove("f5_bigip.log.src.port");
                        event.remove("f5_bigip.log.severity.name");
                        event.remove("f5_bigip.log.username");
                        event.remove("f5_bigip.log.application.name");
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_bigipasm"
            }

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("AVR") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipavr"
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("info"))?;
                event.set(
                    "observer.product",
                    json!("Application Visibility and Reporting"),
                )?;
                let _cond = {
                    event.has_value("json.EOCTimestamp")
                        && event.get_str("json.EOCTimestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.EOCTimestamp") {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set("f5_bigip.log.eoc.timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.EOCTimestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_eoc_timestamp")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.eoc.timestamp").cloned() {
                        event.set("@timestamp", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("json.ClientIP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientIP") {
                            if let Some(val) = event.get("json.ClientIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientIP".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.client.ip").cloned() {
                        event.set("client.ip", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.POOLIP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.POOLIP") {
                            if let Some(val) = event.get("json.POOLIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.POOLIP".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.pool.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_pool_ip_to_ip")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.pool.ip").cloned() {
                        event.set("destination.ip", v)?;
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.ip", v)?;
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
                if let Some(v) = event
                    .get("destination.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.geo", v)?;
                }
                if let Some(v) = event
                    .get("destination.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.POOLPort") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.POOLPort") {
                            if let Some(val) = event.get("json.POOLPort") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.POOLPort".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.pool.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_pool_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.pool.port").cloned() {
                        event.set("destination.port", v)?;
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.port", v)?;
                }
                if event.has_value("json.QueryName") {
                    event.rename("json.QueryName", "f5_bigip.log.query.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.query.name").cloned() {
                        event.set("dns.question.name", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.QueryType") {
                    event.rename("json.QueryType", "f5_bigip.log.query.type")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.query.type").cloned() {
                        event.set("dns.question.type", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.Action") {
                    event.rename("json.Action", "f5_bigip.log.action")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.action").cloned() {
                        event.set("event.action", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.CountryCode") {
                    event.rename("json.CountryCode", "f5_bigip.log.country_code")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.country_code").cloned() {
                        event.set("host.geo.country_iso_code", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.GeoCountry") {
                    event.rename("json.GeoCountry", "f5_bigip.log.geo.country")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.geo.country").cloned() {
                        event.set("host.geo.country_name", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.DeviceId") {
                    event.rename("json.DeviceId", "f5_bigip.log.device.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.device.id").cloned() {
                        event.set("host.id", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.OsName") {
                    event.rename("json.OsName", "f5_bigip.log.osname")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.osname").cloned() {
                        event.set("host.os.name", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.Method") {
                    event.rename("json.Method", "f5_bigip.log.method")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.method").cloned() {
                        event.set("http.request.method", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("json.ResponseCode") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ResponseCode") {
                            if let Some(val) = event.get("json.ResponseCode") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ResponseCode".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.response.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_response_code_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.response.code").cloned() {
                        event.set("http.response.status_code", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("json.Severity") {
                    event.rename("json.Severity", "f5_bigip.log.severity.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.severity.name").cloned() {
                        event.set("log.level", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("log.level") {
                    map_strings(event, "log.level", "log.level", str::to_lowercase)?;
                }
                if event.has_value("json.NetworkProtocol") {
                    event.rename("json.NetworkProtocol", "f5_bigip.log.network.protocol")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.network.protocol")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.protocol", v)?;
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.get_str("json.SourceIP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SourceIP") {
                            if let Some(val) = event.get("json.SourceIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SourceIP".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if let Some(v) = event
                    .get("source.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo", v)?;
                }
                if let Some(v) = event
                    .get("source.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.as", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.URL") {
                    event.rename("json.URL", "f5_bigip.log.url")?;
                }
                let _cond = { event.has_value("f5_bigip.log.url") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if !uri_parts(event, "f5_bigip.log.url", "url", true, false)?
                            && event
                                .get_str("f5_bigip.log.url")
                                .is_some_and(|value| !value.is_empty())
                        {
                            return Err(TransformError::ParseError {
                                path: "f5_bigip.log.url".into(),
                                message: "uri_parts: not a parseable URI".into(),
                            });
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(v) = event.get("f5_bigip.log.url").cloned() {
                                event.set("url.original", v)?;
                            }
                            Ok(())
                        })();
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.UserAgent") {
                    event.rename("json.UserAgent", "f5_bigip.log.user.agent")?;
                }
                if event.has_value("f5_bigip.log.user.agent") {
                    gsub_field(
                        event,
                        "f5_bigip.log.user.agent",
                        "f5_bigip.log.user.agent",
                        cached_regex!("(\\([^)]*)\\+(https?://)"),
                        "$1%2b$2",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.user.agent") {
                        if let Some(s) = event.get_string("f5_bigip.log.user.agent") {
                            match url_decode(&s) {
                                Some(decoded) => {
                                    event.set("f5_bigip.log.user.agent", json!(decoded))?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.user.agent".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if let Some(ua_str) = event.get_string("f5_bigip.log.user.agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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
                if event.has_value("json.UserName") {
                    event.rename("json.UserName", "f5_bigip.log.user.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("f5_bigip.log.user.name").cloned() {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.AggrInterval") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AggrInterval") {
                            if let Some(val) = event.get("json.AggrInterval") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AggrInterval".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.aggr_interval", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_aggr_interval_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.ApplicationName") {
                    event.rename("json.ApplicationName", "f5_bigip.log.application_name")?;
                }
                let _cond = { event.get_str("json.ApplicationResponseTime") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ApplicationResponseTime") {
                            if let Some(val) = event.get("json.ApplicationResponseTime") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ApplicationResponseTime".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.application.response.time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_application_response_time_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.AVRProfileName") {
                    event.rename("json.AVRProfileName", "f5_bigip.log.profile.name")?;
                }
                if event.has_value("json.BrowserName") {
                    event.rename("json.BrowserName", "f5_bigip.log.browser_name")?;
                }
                if event.has_value("json.ClientIPRouteDomain") {
                    event.rename(
                        "json.ClientIPRouteDomain",
                        "f5_bigip.log.client.ip_route_domain",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.client.ip_route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.ClientLatencyHitCount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientLatencyHitCount") {
                            if let Some(val) = event.get("json.ClientLatencyHitCount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientLatencyHitCount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.latency.hit_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_latency_hit_count_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ClientLatencyMax") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientLatencyMax") {
                            if let Some(val) = event.get("json.ClientLatencyMax") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientLatencyMax".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.latency.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_latency_max_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ClientLatencyTotal") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientLatencyTotal") {
                            if let Some(val) = event.get("json.ClientLatencyTotal") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientLatencyTotal".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.latency.total", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_latency_total_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ClientSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientSideNetworkLatency") {
                            if let Some(val) = event.get("json.ClientSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client_side.network.latency", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ClientTtfb") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientTtfb") {
                            if let Some(val) = event.get("json.ClientTtfb") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientTtfb".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client_ttfb.value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ttfb_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ClientTtfbHitcount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ClientTtfbHitcount") {
                            if let Some(val) = event.get("json.ClientTtfbHitcount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ClientTtfbHitcount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client_ttfb.hit_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ttfb_hit_count_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.Entity") {
                    event.rename("json.Entity", "f5_bigip.log.entity")?;
                }
                if event.has_value("json.errdefs_msgno") {
                    event.rename("json.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
                }
                if event.has_value("json.GeoCode") {
                    event.rename("json.GeoCode", "f5_bigip.log.geo.code")?;
                }
                let _cond = { event.get_str("json.HitCount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.HitCount") {
                            if let Some(val) = event.get("json.HitCount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.HitCount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.hit_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_hit_count_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.LatencyHistogram") {
                    event.rename("json.LatencyHistogram", "f5_bigip.log.latency_histogram")?;
                }
                let _cond = { event.get_str("json.MaxApplicationResponseTime") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxApplicationResponseTime") {
                            if let Some(val) = event.get("json.MaxApplicationResponseTime") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxApplicationResponseTime".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("f5_bigip.log.application.response.max_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_application_response_time_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxClientSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxClientSideNetworkLatency") {
                            if let Some(val) = event.get("json.MaxClientSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxClientSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.client_side.network.max_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_client_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxClientTtfb") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxClientTtfb") {
                            if let Some(val) = event.get("json.MaxClientTtfb") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxClientTtfb".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client_ttfb.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_client_ttfb_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxRequestDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxRequestDuration") {
                            if let Some(val) = event.get("json.MaxRequestDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxRequestDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.request.max_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_request_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxResponseDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxResponseDuration") {
                            if let Some(val) = event.get("json.MaxResponseDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxResponseDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.response.max_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_response_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxServerSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxServerSideNetworkLatency") {
                            if let Some(val) = event.get("json.MaxServerSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxServerSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.server_side.network.max_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_server_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinApplicationResponseTime") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinApplicationResponseTime") {
                            if let Some(val) = event.get("json.MinApplicationResponseTime") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinApplicationResponseTime".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("f5_bigip.log.application.response.min_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_application_response_time_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinClientSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinClientSideNetworkLatency") {
                            if let Some(val) = event.get("json.MinClientSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinClientSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.client_side.network.min_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_client_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinClientTtfb") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinClientTtfb") {
                            if let Some(val) = event.get("json.MinClientTtfb") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinClientTtfb".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client_ttfb.min", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_client_ttfb_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinRequestDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinRequestDuration") {
                            if let Some(val) = event.get("json.MinRequestDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinRequestDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.request.min_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_request_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinResponseDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinResponseDuration") {
                            if let Some(val) = event.get("json.MinResponseDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinResponseDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.response.min_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_response_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinServerSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinServerSideNetworkLatency") {
                            if let Some(val) = event.get("json.MinServerSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinServerSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.server_side.network.min_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_server_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.Module") {
                    event.rename("json.Module", "f5_bigip.log.module")?;
                }
                if event.has_value("json.POOLIPRouteDomain") {
                    event.rename(
                        "json.POOLIPRouteDomain",
                        "f5_bigip.log.pool.ip_route_domain",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.pool.ip_route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                let _cond = { event.get_str("json.RequestDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.RequestDuration") {
                            if let Some(val) = event.get("json.RequestDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.RequestDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.request.duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_request_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.RequestDurationHitcount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.RequestDurationHitcount") {
                            if let Some(val) = event.get("json.RequestDurationHitcount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.RequestDurationHitcount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.request.duration_hit_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_request_duration_hitcount_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ResponseDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ResponseDuration") {
                            if let Some(val) = event.get("json.ResponseDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ResponseDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.response.duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_response_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ResponseDurationHitcount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ResponseDurationHitcount") {
                            if let Some(val) = event.get("json.ResponseDurationHitcount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ResponseDurationHitcount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.response.duration_hit_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_response_duration_hitcount_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ServerHitcount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ServerHitcount") {
                            if let Some(val) = event.get("json.ServerHitcount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ServerHitcount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.server.hit_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_hitcount_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ServerLatencyMax") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ServerLatencyMax") {
                            if let Some(val) = event.get("json.ServerLatencyMax") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ServerLatencyMax".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.server.latency.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_latency_max_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ServerLatencyMin") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ServerLatencyMin") {
                            if let Some(val) = event.get("json.ServerLatencyMin") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ServerLatencyMin".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.server.latency.min", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_latency_min_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ServerLatencyTotal") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ServerLatencyTotal") {
                            if let Some(val) = event.get("json.ServerLatencyTotal") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ServerLatencyTotal".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.server.latency.total", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_latency_total_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ServerSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ServerSideNetworkLatency") {
                            if let Some(val) = event.get("json.ServerSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ServerSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.server_side.network.latency", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.SlotId") {
                    event.rename("json.SlotId", "f5_bigip.log.slot.id")?;
                }
                let _cond = { event.get_str("json.SosApplicationResponseTime") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SosApplicationResponseTime") {
                            if let Some(val) = event.get("json.SosApplicationResponseTime") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SosApplicationResponseTime".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("f5_bigip.log.sos.application_response_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sos_application_response_time_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SosClientSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SosClientSideNetworkLatency") {
                            if let Some(val) = event.get("json.SosClientSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SosClientSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.sos.client_side_network_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sos_client_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SosClientTtfb") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SosClientTtfb") {
                            if let Some(val) = event.get("json.SosClientTtfb") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SosClientTtfb".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.sos.client_ttfb", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sos_client_ttfb_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SosRequestDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SosRequestDuration") {
                            if let Some(val) = event.get("json.SosRequestDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SosRequestDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.sos.request_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sos_request_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SosResponseDuration") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SosResponseDuration") {
                            if let Some(val) = event.get("json.SosResponseDuration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SosResponseDuration".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.sos.response_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sos_response_duration_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SosServerSideNetworkLatency") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SosServerSideNetworkLatency") {
                            if let Some(val) = event.get("json.SosServerSideNetworkLatency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SosServerSideNetworkLatency".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.sos.server_side_network_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sos_server_side_network_latency_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SubnetIP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SubnetIP") {
                            if let Some(val) = event.get("json.SubnetIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SubnetIP".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.subnet.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_subnet_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.subnet.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.SubnetName") {
                    event.rename("json.SubnetName", "f5_bigip.log.subnet.name")?;
                }
                if event.has_value("json.SubnetRouteDomain") {
                    event.rename("json.SubnetRouteDomain", "f5_bigip.log.subnet.route_domain")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.subnet.route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.telemetryEventCategory") {
                    event.rename(
                        "json.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                let _cond = { event.get_str("json.ThroughputReqMaxPerSec") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ThroughputReqMaxPerSec") {
                            if let Some(val) = event.get("json.ThroughputReqMaxPerSec") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ThroughputReqMaxPerSec".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.throughput.req_per_sec.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_throughput_req_max_per_sec_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ThroughputReqTotalPerInterval") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ThroughputReqTotalPerInterval") {
                            if let Some(val) = event.get("json.ThroughputReqTotalPerInterval") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ThroughputReqTotalPerInterval".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.throughput.req_per_interval.total",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_throughput_req_total_per_interval_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ThroughputRespMaxPerSec") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ThroughputRespMaxPerSec") {
                            if let Some(val) = event.get("json.ThroughputRespMaxPerSec") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ThroughputRespMaxPerSec".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.throughput.resp_per_sec.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_throughput_resp_max_per_sec_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ThroughputRespTotalPerInterval") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ThroughputRespTotalPerInterval") {
                            if let Some(val) = event.get("json.ThroughputRespTotalPerInterval") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ThroughputRespTotalPerInterval".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "f5_bigip.log.throughput.resp_per_interval.total",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_throughput_resp_total_per_interval_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.TPSMax") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.TPSMax") {
                            if let Some(val) = event.get("json.TPSMax") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.TPSMax".into(),
                                            message,
                                        }
                                    })?;
                                event.set("f5_bigip.log.tps.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_tps_max_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.UserSessionsNewTotal") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.UserSessionsNewTotal") {
                            if let Some(val) = event.get("json.UserSessionsNewTotal") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.UserSessionsNewTotal".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.user.sessions.new_total", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_user_sessions_new_total_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.VSName") {
                    event.rename("json.VSName", "f5_bigip.log.vs_name")?;
                }
                if event.has_value("json.ObjectTagsList") {
                    event.rename("json.ObjectTagsList", "f5_bigip.log.object_tags_list")?;
                }
                if event.has_value("json.DosProfileName") {
                    event.rename("json.DosProfileName", "f5_bigip.log.dos.profile_name")?;
                }
                if event.has_value("json.AttackId") {
                    event.rename("json.AttackId", "f5_bigip.log.attack.id")?;
                }
                if event.has_value("json.SourceIpRouteDomain") {
                    event.rename(
                        "json.SourceIpRouteDomain",
                        "f5_bigip.log.source.ip_route_domain",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.source.ip_route_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.TransactionOutcome") {
                    event.rename(
                        "json.TransactionOutcome",
                        "f5_bigip.log.transaction_outcome",
                    )?;
                }
                if event.has_value("json.AttackVectorName") {
                    event.rename("json.AttackVectorName", "f5_bigip.log.attack.vector_name")?;
                }
                if event.has_value("json.AttackTriggerName") {
                    event.rename("json.AttackTriggerName", "f5_bigip.log.attack.trigger_name")?;
                }
                if event.has_value("json.AttackMitigationName") {
                    event.rename(
                        "json.AttackMitigationName",
                        "f5_bigip.log.attack.mitigation_name",
                    )?;
                }
                let _cond = { event.get_str("json.IsInternalActivity") == Some("0") };
                if _cond {
                    let v = json!(false);
                    if !painless_is_empty_value(&v) {
                        event.set("f5_bigip.log.is_internal_activity", v)?;
                    }
                }
                let _cond = { event.get_str("json.IsInternalActivity") == Some("1") };
                if _cond {
                    let v = json!(true);
                    if !painless_is_empty_value(&v) {
                        event.set("f5_bigip.log.is_internal_activity", v)?;
                    }
                }
                let _cond = { event.get_str("json.IsAttackingIp") == Some("0") };
                if _cond {
                    let v = json!(false);
                    if !painless_is_empty_value(&v) {
                        event.set("f5_bigip.log.is_attacking_ip", v)?;
                    }
                }
                let _cond = { event.get_str("json.IsAttackingIp") == Some("1") };
                if _cond {
                    let v = json!(true);
                    if !painless_is_empty_value(&v) {
                        event.set("f5_bigip.log.is_attacking_ip", v)?;
                    }
                }
                if event.has_value("json.globalBigiqConf") {
                    event.rename("json.globalBigiqConf", "f5_bigip.log.global_bigiq_conf")?;
                }
                if event.has_value("json.Policy") {
                    event.rename("json.Policy", "f5_bigip.log.policy.name")?;
                }
                let _cond = { event.get_str("json.ViolationRating") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ViolationRating") {
                            if let Some(val) = event.get("json.ViolationRating") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ViolationRating".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.violation.rating", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_violation_rating_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.IPReputation") {
                    event.rename("json.IPReputation", "f5_bigip.log.ip_reputation")?;
                }
                if event.has_value("json.SessionID") {
                    event.rename("json.SessionID", "f5_bigip.log.session.id")?;
                }
                let _cond = { event.get_str("json.IsMobileDevice") == Some("0") };
                if _cond {
                    event.set("f5_bigip.log.is_mobile_device", json!(false))?;
                }
                let _cond = { event.get_str("json.IsMobileDevice") == Some("1") };
                if _cond {
                    event.set("f5_bigip.log.is_mobile_device", json!(true))?;
                }
                if event.has_value("json.DosMobileAppClientType") {
                    event.rename(
                        "json.DosMobileAppClientType",
                        "f5_bigip.log.dos.mobile_app.client_type",
                    )?;
                }
                if event.has_value("json.DosMobileAppVersion") {
                    event.rename(
                        "json.DosMobileAppVersion",
                        "f5_bigip.log.dos.mobile_app.version",
                    )?;
                }
                if event.has_value("json.DosMobileAppDisplayName") {
                    event.rename(
                        "json.DosMobileAppDisplayName",
                        "f5_bigip.log.dos.mobile_app.display_name",
                    )?;
                }
                if event.has_value("json.STAT_SRC") {
                    event.rename("json.STAT_SRC", "f5_bigip.log.stat_src")?;
                }
                if event.has_value("json.AttackType") {
                    event.rename("json.AttackType", "f5_bigip.log.attack.type")?;
                }
                let _cond = { event.get_str("json.AttackCount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AttackCount") {
                            if let Some(val) = event.get("json.AttackCount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AttackCount".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.attack.count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_attack_count_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.TotalEvents") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.TotalEvents") {
                            if let Some(val) = event.get("json.TotalEvents") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.TotalEvents".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.events.total", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_events_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.SoftwareDrops") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.SoftwareDrops") {
                            if let Some(val) = event.get("json.SoftwareDrops") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.SoftwareDrops".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.software_drops", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_software_drops_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.HardwareDrops") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.HardwareDrops") {
                            if let Some(val) = event.get("json.HardwareDrops") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.HardwareDrops".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.hardware_drops", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_hardware_drops_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.BadActorEvents") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.BadActorEvents") {
                            if let Some(val) = event.get("json.BadActorEvents") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.BadActorEvents".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bad_actor.events", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bad_actor_events_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.BadActorDrops") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.BadActorDrops") {
                            if let Some(val) = event.get("json.BadActorDrops") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.BadActorDrops".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bad_actor.drops", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bad_actor_drops_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.WLEvents") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.WLEvents") {
                            if let Some(val) = event.get("json.WLEvents") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.WLEvents".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.wl_events", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_wl_events_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgDetection") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgDetection") {
                            if let Some(val) = event.get("json.AvgDetection") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgDetection".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.detection.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_detection_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinMitigation") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinMitigation") {
                            if let Some(val) = event.get("json.MinMitigation") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinMitigation".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.mitigation.min", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_mitigation_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxMitigation") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxMitigation") {
                            if let Some(val) = event.get("json.MaxMitigation") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxMitigation".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.mitigation.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_mitigation_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgBadActorDetection") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgBadActorDetection") {
                            if let Some(val) = event.get("json.AvgBadActorDetection") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgBadActorDetection".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bad_actor.detection.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_bad_actor_detection_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MinBadActorMitigation") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MinBadActorMitigation") {
                            if let Some(val) = event.get("json.MinBadActorMitigation") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MinBadActorMitigation".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bad_actor.mitigation.min", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_bad_actor_mitigation_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxBadActorMitigation") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxBadActorMitigation") {
                            if let Some(val) = event.get("json.MaxBadActorMitigation") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxBadActorMitigation".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bad_actor.mitigation.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_bad_actor_mitigation_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.abandoned_conns") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.abandoned_conns") {
                            if let Some(val) = event.get("json.abandoned_conns") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.abandoned_conns".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.abandoned_conns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_abandoned_conns_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.expired_conns") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.expired_conns") {
                            if let Some(val) = event.get("json.expired_conns") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.expired_conns".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.expired_conns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_expired_conns_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.failed_conns") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.failed_conns") {
                            if let Some(val) = event.get("json.failed_conns") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.failed_conns".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.failed_conns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_failed_conns_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.accept_fails") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.accept_fails") {
                            if let Some(val) = event.get("json.accept_fails") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.accept_fails".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.accept_fails", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_accept_fails_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.accepts") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.accepts") {
                            if let Some(val) = event.get("json.accepts") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.accepts".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.accepts", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_accepts_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.active_conns") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.active_conns") {
                            if let Some(val) = event.get("json.active_conns") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.active_conns".into(),
                                            message,
                                        }
                                    })?;
                                event.set("f5_bigip.log.active_conns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_active_conns_to_double",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.hw_cookie_valid") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.hw_cookie_valid") {
                            if let Some(val) = event.get("json.hw_cookie_valid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.hw_cookie_valid".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.hw.cookie_valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_hw_cookie_valid_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.max_active_conns") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.max_active_conns") {
                            if let Some(val) = event.get("json.max_active_conns") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.max_active_conns".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.max_active_conns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_active_conns_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.new_conns") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.new_conns") {
                            if let Some(val) = event.get("json.new_conns") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.new_conns".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.new_conns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_new_conns_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.rxbad_cookie") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.rxbad_cookie") {
                            if let Some(val) = event.get("json.rxbad_cookie") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.rxbad_cookie".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.rxbad_cookie", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rxbad_cookie_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.rxbadseg") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.rxbadseg") {
                            if let Some(val) = event.get("json.rxbadseg") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.rxbadseg".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.rxbadseg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rxbadseg_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.rxbadsum") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.rxbadsum") {
                            if let Some(val) = event.get("json.rxbadsum") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.rxbadsum".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.rxbadsum", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rxbadsum_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.rxcookie") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.rxcookie") {
                            if let Some(val) = event.get("json.rxcookie") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.rxcookie".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.rxcookie", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rxcookie_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.rxooseg") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.rxooseg") {
                            if let Some(val) = event.get("json.rxooseg") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.rxooseg".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.rxooseg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rxooseg_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.rxrst") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.rxrst") {
                            if let Some(val) = event.get("json.rxrst") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.rxrst".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.rxrst", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_rxrst_to_long")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.sndpack") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.sndpack") {
                            if let Some(val) = event.get("json.sndpack") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.sndpack".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.sndpack", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sndpack_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.syncacheover") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.syncacheover") {
                            if let Some(val) = event.get("json.syncacheover") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.syncacheover".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.syncacheover", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_syncacheover_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.tcp_prof") {
                    event.rename("json.tcp_prof", "f5_bigip.log.tcp_prof")?;
                }
                let _cond = { event.get_str("json.txrexmits") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.txrexmits") {
                            if let Some(val) = event.get("json.txrexmits") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.txrexmits".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.txrexmits", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_txrexmits_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.vip") {
                    event.rename("json.vip", "f5_bigip.log.vip")?;
                }
                if event.has_value("json.tenant") {
                    event.rename("json.tenant", "f5_bigip.log.tenant")?;
                }
                let _cond = { event.get_str("json.AvgConcurrentConnections") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgConcurrentConnections") {
                            if let Some(val) = event.get("json.AvgConcurrentConnections") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgConcurrentConnections".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.concurrent.connections.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_concurrent_connections_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgCpu") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgCpu") {
                            if let Some(val) = event.get("json.AvgCpu") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgCpu".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.cpu.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_cpu_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgCpuAnalysisPlane") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgCpuAnalysisPlane") {
                            if let Some(val) = event.get("json.AvgCpuAnalysisPlane") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgCpuAnalysisPlane".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.cpu.analysis_plane.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_cpu_analysis_plane_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgCpuDataPlane") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgCpuDataPlane") {
                            if let Some(val) = event.get("json.AvgCpuDataPlane") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgCpuDataPlane".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.cpu.data_plane.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_cpu_data_plane_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgMemory") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgMemory") {
                            if let Some(val) = event.get("json.AvgMemory") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgMemory".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.memory.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_memory_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgThroughput") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgThroughput") {
                            if let Some(val) = event.get("json.AvgThroughput") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgThroughput".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.throughput.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_throughput_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ConcurrentConnectionsHealth") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ConcurrentConnectionsHealth") {
                            if let Some(val) = event.get("json.ConcurrentConnectionsHealth") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ConcurrentConnectionsHealth".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("f5_bigip.log.concurrent.connections.health", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_concurrent_connections_health_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.CpuHealth") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.CpuHealth") {
                            if let Some(val) = event.get("json.CpuHealth") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.CpuHealth".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.cpu.health", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_cpu_health_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MaxConcurrentConnections") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MaxConcurrentConnections") {
                            if let Some(val) = event.get("json.MaxConcurrentConnections") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MaxConcurrentConnections".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.concurrent.connections.max", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_concurrent_connections_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.MemoryHealth") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.MemoryHealth") {
                            if let Some(val) = event.get("json.MemoryHealth") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.MemoryHealth".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.memory.health", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_memory_health_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.ThroughputHealth") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.ThroughputHealth") {
                            if let Some(val) = event.get("json.ThroughputHealth") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.ThroughputHealth".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.throughput.health", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_throughput_health_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.TotalBytes") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.TotalBytes") {
                            if let Some(val) = event.get("json.TotalBytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.TotalBytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bytes.total", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("json.AvgCpuControlPlane") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.AvgCpuControlPlane") {
                            if let Some(val) = event.get("json.AvgCpuControlPlane") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.AvgCpuControlPlane".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.cpu.control_plane.avg", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_avg_cpu_control_plane_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    Ok(())
                })();
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("f5_bigip.log.eoc.timestamp");
                        event.remove("f5_bigip.log.client.ip");
                        event.remove("f5_bigip.log.pool.ip");
                        event.remove("f5_bigip.log.pool.port");
                        event.remove("f5_bigip.log.query.name");
                        event.remove("f5_bigip.log.query.type");
                        event.remove("f5_bigip.log.action");
                        event.remove("f5_bigip.log.country_code");
                        event.remove("f5_bigip.log.geo.country");
                        event.remove("f5_bigip.log.device.id");
                        event.remove("f5_bigip.log.hostname");
                        event.remove("f5_bigip.log.osname");
                        event.remove("f5_bigip.log.method");
                        event.remove("f5_bigip.log.response.code");
                        event.remove("f5_bigip.log.severity.name");
                        event.remove("f5_bigip.log.network.protocol");
                        event.remove("f5_bigip.log.source.ip");
                        event.remove("f5_bigip.log.url");
                        event.remove("f5_bigip.log.user.agent");
                        event.remove("f5_bigip.log.user.name");
                        event.remove("f5_bigip.log.application.name");
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_bigipavr"
            }

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("systemInfo") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipsystem"
                event.set("observer.product", json!("System Information"))?;
                event.append("event.category", json!("host"))?;
                event.append("event.type", json!("info"))?;
                if event.has_value("json.system.afmState") {
                    event.rename("json.system.afmState", "f5_bigip.log.afm_state")?;
                }
                if event.has_value("json.system.apmState") {
                    event.rename("json.system.apmState", "f5_bigip.log.apm_state")?;
                }
                if event.has_value("json.system.asmAttackSignatures") {
                    event.rename(
                        "json.system.asmAttackSignatures",
                        "f5_bigip.log.asm_attack_signatures",
                    )?;
                }
                let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("f5_bigip.log.asm_attack_signatures").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.createDateTime")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.create_date_time", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.createDateTime".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "convert_createDateTime",
                                    )?;
                                    event.remove("_ingest._value.createDateTime");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "f5_bigip.log.asm_attack_signatures",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
                let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.asm_attack_signatures", |event| {
                        event.remove("_ingest._value.createDateTime");
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.asm_attack_signatures", |event| {
                        event.append_unique(
                            "file.name",
                            json!(
                                event
                                    .get("_ingest._value.filename")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.asm_attack_signatures", |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.filename");
                        }
                        Ok(())
                    })?;
                }
                if event.has_value("json.system.asmState") {
                    event.rename("json.system.asmState", "f5_bigip.log.asm_state")?;
                }
                if event.has_value("json.system.callBackUrl") {
                    event.rename("json.system.callBackUrl", "f5_bigip.log.callback_url")?;
                }
                if event.has_value("json.system.chassisId") {
                    event.rename("json.system.chassisId", "f5_bigip.log.chassis_id")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.chassis_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.serial_number", v)?;
                }
                if event.has_value("json.system.configReady") {
                    event.rename("json.system.configReady", "f5_bigip.log.config_ready")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.configSyncSucceeded") {
                        if let Some(val) = event.get("json.system.configSyncSucceeded") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.system.configSyncSucceeded".into(),
                                    message,
                                }
                            })?;
                            event.set("f5_bigip.log.config_sync_succeeded", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_configSyncSucceeded_to_boolean",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.system.connectionsPerformance") {
                    event.rename(
                        "json.system.connectionsPerformance",
                        "f5_bigip.log.connections_performance",
                    )?;
                }
                let _cond = { event.has_value("f5_bigip.log.connections_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.average") {
                                if let Some(val) = event.get("_ingest._value.average") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.average".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.average", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_average")?;
                            event.remove("_ingest._value.average");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.connections_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.current") {
                                if let Some(val) = event.get("_ingest._value.current") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.current".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.current", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_current")?;
                            event.remove("_ingest._value.current");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.connections_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.max") {
                                if let Some(val) = event.get("_ingest._value.max") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.max".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.max", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_max")?;
                            event.remove("_ingest._value.max");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.connections_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.cpu") {
                        if let Some(val) = event.get("json.system.cpu") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.system.cpu".into(),
                                    message,
                                }
                            })?;
                            event.set("f5_bigip.log.cpu_value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_cpu_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.system.description") {
                    event.rename("json.system.description", "f5_bigip.log.description")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                let _cond = { event.has_value("f5_bigip.log.description") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.description")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("json.system.diskStorage") {
                    event.rename("json.system.diskStorage", "f5_bigip.log.disk_storage")?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_storage") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_storage", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.Capacity_Float") {
                                if let Some(val) = event.get("_ingest._value.Capacity_Float") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.Capacity_Float".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.capacity_float", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_capacity_float",
                            )?;
                            event.remove("_ingest._value.Capacity_Float");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_storage") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_storage", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.1024-blocks") {
                                if let Some(val) = event.get("_ingest._value.1024-blocks") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.1024-blocks".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.1024_blocks", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_1024-blocks")?;
                            event.remove("_ingest._value.1024-blocks");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_storage") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_storage", |event| {
                        event.remove("_ingest._value.1024-blocks");
                        event.remove("_ingest._value.Capacity_Float");
                        event.remove("_ingest._value.Capacity");
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                if event.has_value("json.system.diskLatency") {
                    event.rename("json.system.diskLatency", "f5_bigip.log.disk_latency")?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_latency") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.%util") {
                                if let Some(val) = event.get("_ingest._value.%util") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.%util".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.per_util", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_%util")?;
                            event.remove("_ingest._value.%util");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_latency") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                        event.remove("_ingest._value.%util");
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_latency") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.r/s") {
                                if let Some(val) = event.get("_ingest._value.r/s") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.r/s".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.r/s", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_r/s")?;
                            event.remove("_ingest._value.r/s");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.disk_latency") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.w/s") {
                                if let Some(val) = event.get("_ingest._value.w/s") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.w/s".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.w/s", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_w/s")?;
                            event.remove("_ingest._value.w/s");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                if event.has_value("json.system.failoverColor") {
                    event.rename("json.system.failoverColor", "f5_bigip.log.failover_color")?;
                }
                if event.has_value("json.system.failoverStatus") {
                    event.rename("json.system.failoverStatus", "f5_bigip.log.failover_status")?;
                }
                let _cond = {
                    event.has_value("json.system.gtmConfigTime")
                        && event.get_str("json.system.gtmConfigTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.system.gtmConfigTime") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.gtm_config_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.system.gtmConfigTime".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_system_gtmConfigTime",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.system.hostname") {
                    event.rename("json.system.hostname", "f5_bigip.log.hostname")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                let _cond = { event.has_value("host.hostname") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("json.system.lastAfmDeploy")
                        && event.get_str("json.system.lastAfmDeploy") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.system.lastAfmDeploy") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.last_afm_deploy", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.system.lastAfmDeploy".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_system_lastAfmDeploy",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.system.lastAsmChange")
                        && event.get_str("json.system.lastAsmChange") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.system.lastAsmChange") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.last_asm_change", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.system.lastAsmChange".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_system_lastAsmChange",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.telemetryServiceInfo.cycleEnd")
                        && event.get_str("json.telemetryServiceInfo.cycleEnd") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.telemetryServiceInfo.cycleEnd")
                        {
                            match parse_date_out(
                                &date_str,
                                &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event
                                    .set("f5_bigip.log.telemetry_service_info.cycle_end", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.telemetryServiceInfo.cycleEnd".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_cycle_end_conversion",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.telemetryServiceInfo.cycleStart")
                        && event.get_str("json.telemetryServiceInfo.cycleStart") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.telemetryServiceInfo.cycleStart")
                        {
                            match parse_date_out(
                                &date_str,
                                &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set(
                                    "f5_bigip.log.telemetry_service_info.cycle_start",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.telemetryServiceInfo.cycleStart".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_cycle_start_conversion",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.telemetryServiceInfo.pollingInterval") {
                        if let Some(val) = event.get("json.telemetryServiceInfo.pollingInterval") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.telemetryServiceInfo.pollingInterval".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "f5_bigip.log.telemetry_service_info.polling_interval",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_telemetryservice_polling_interval",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.system.licenseReady") {
                    event.rename("json.system.licenseReady", "f5_bigip.log.license_ready")?;
                }
                if event.has_value("json.system.location") {
                    event.rename("json.system.location", "f5_bigip.log.location")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.location")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.geo.name", v)?;
                }
                let _cond = {
                    event.has_value("json.system.ltmConfigTime")
                        && event.get_str("json.system.ltmConfigTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.system.ltmConfigTime") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.ltm_config_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.system.ltmConfigTime".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_system_ltmConfigTime",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.baseMac") {
                        gsub_field(
                            event,
                            "json.system.baseMac",
                            "f5_bigip.log.base_mac",
                            cached_regex!("[:.]"),
                            "-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_system_baseMac")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("f5_bigip.log.base_mac") {
                    map_strings(
                        event,
                        "f5_bigip.log.base_mac",
                        "f5_bigip.log.base_mac",
                        str::to_uppercase,
                    )?;
                }
                let _cond = { event.has_value("f5_bigip.log.base_mac") };
                if _cond {
                    event.append_unique(
                        "host.mac",
                        json!(
                            event
                                .get("f5_bigip.log.base_mac")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("host.mac") };
                if _cond {
                    foreach_array(event, "host.mac", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                if event.has_value("json.system.machineId") {
                    event.rename("json.system.machineId", "f5_bigip.log.machine_id")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.machine_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
                let _cond = { event.has_value("host.id") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("json.system.marketingName") {
                    event.rename("json.system.marketingName", "f5_bigip.log.marketing_name")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.marketing_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.vendor", v)?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.memory") {
                        if let Some(val) = event.get("json.system.memory") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.system.memory".into(),
                                    message,
                                }
                            })?;
                            event.set("f5_bigip.log.memory_value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_memory_value_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.system.networkInterfaces") {
                    event.rename(
                        "json.system.networkInterfaces",
                        "f5_bigip.log.network_interfaces",
                    )?;
                }
                let _cond = { event.has_value("f5_bigip.log.network_interfaces") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.network_interfaces", |event| {
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                if event.has_value("json.system.platformId") {
                    event.rename("json.system.platformId", "f5_bigip.log.platform_id")?;
                }
                if event.has_value("json.system.provisionReady") {
                    event.rename("json.system.provisionReady", "f5_bigip.log.provision_ready")?;
                }
                if event.has_value("json.system.provisioning") {
                    event.rename("json.system.provisioning", "f5_bigip.log.provisioning")?;
                }
                let _cond = { event.has_value("f5_bigip.log.provisioning") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.provisioning", |event| {
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.swap") {
                        if let Some(val) = event.get("json.system.swap") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.system.swap".into(),
                                    message,
                                }
                            })?;
                            event.set("f5_bigip.log.swap", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_swap_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.system.syncColor") {
                    event.rename("json.system.syncColor", "f5_bigip.log.sync_color")?;
                }
                if event.has_value("json.system.syncMode") {
                    event.rename("json.system.syncMode", "f5_bigip.log.sync_mode")?;
                }
                if event.has_value("json.system.syncStatus") {
                    event.rename("json.system.syncStatus", "f5_bigip.log.sync_status")?;
                }
                if event.has_value("json.system.syncSummary") {
                    event.rename("json.system.syncSummary", "f5_bigip.log.sync_summary")?;
                }
                let _cond = {
                    event.has_value("json.system.systemTimestamp")
                        && event.get_str("json.system.systemTimestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.system.systemTimestamp") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.system_timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.system.systemTimestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_system_systemTimestamp",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.system_timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                if event.has_value("json.telemetryEventCategory") {
                    event.rename(
                        "json.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                if event.has_value("json.system.throughputPerformance") {
                    event.rename(
                        "json.system.throughputPerformance",
                        "f5_bigip.log.throughput_performance",
                    )?;
                }
                let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.average") {
                                if let Some(val) = event.get("_ingest._value.average") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.average".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.average", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_average")?;
                            event.remove("_ingest._value.average");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.current") {
                                if let Some(val) = event.get("_ingest._value.current") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.current".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.current", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_current")?;
                            event.remove("_ingest._value.current");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.max") {
                                if let Some(val) = event.get("_ingest._value.max") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.max".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.max", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_max")?;
                            event.remove("_ingest._value.max");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
                if _cond {
                    foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                        event.remove("_ingest._value.name");
                        Ok(())
                    })?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.tmmCpu") {
                        if let Some(val) = event.get("json.system.tmmCpu") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.system.tmmCpu".into(),
                                    message,
                                }
                            })?;
                            event.set("f5_bigip.log.tmm_cpu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_tmmCpu_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.system.tmmMemory") {
                        if let Some(val) = event.get("json.system.tmmMemory") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.system.tmmMemory".into(),
                                    message,
                                }
                            })?;
                            event.set("f5_bigip.log.tmm_memory", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_tmmMemory_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event.has_value("json.system.tmmTraffic")
                        && event
                            .get("json.system.tmmTraffic")
                            .and_then(|v| v.get("clientSideTraffic.bitsIn"))
                            .is_some_and(|v| !v.is_null())
                };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: def client_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsIn'); client_side_traffic.put('bits_in', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n} else{\n  ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_in', obj);\n}
                    move_map_entry(
                        event,
                        &MoveMapEntry::new(
                            "json.system.tmmTraffic".into(),
                            "clientSideTraffic.bitsIn".into(),
                            "f5_bigip.log.tmm_traffic.client_side_traffic.bits_in".into(),
                        ),
                    );
                }
                let _cond = {
                    event.has_value("json.system.tmmTraffic")
                        && event
                            .get("json.system.tmmTraffic")
                            .and_then(|v| v.get("clientSideTraffic.bitsOut"))
                            .is_some_and(|v| !v.is_null())
                };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: def client_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsOut'); client_side_traffic.put('bits_out', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n} else{\n  ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_out', obj);\n}
                    move_map_entry(
                        event,
                        &MoveMapEntry::new(
                            "json.system.tmmTraffic".into(),
                            "clientSideTraffic.bitsOut".into(),
                            "f5_bigip.log.tmm_traffic.client_side_traffic.bits_out".into(),
                        ),
                    );
                }
                let _cond = {
                    event.has_value("json.system.tmmTraffic")
                        && event
                            .get("json.system.tmmTraffic")
                            .and_then(|v| v.get("serverSideTraffic.bitsIn"))
                            .is_some_and(|v| !v.is_null())
                };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: def server_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('serverSideTraffic.bitsIn'); server_side_traffic.put('bits_in', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('server_side_traffic', server_side_traffic);\n} else{\n    if (ctx.f5_bigip?.log?.tmm_traffic?.server_side_traffic == null) {\n        ctx.f5_bigip.log.tmm_traffic.server_side_traffic = new HashMap();\n    }\n    ctx.f5_bigip.log.tmm_traffic.server_side_traffic.put('bits_in', obj);\n}
                    move_map_entry(
                        event,
                        &MoveMapEntry::new(
                            "json.system.tmmTraffic".into(),
                            "serverSideTraffic.bitsIn".into(),
                            "f5_bigip.log.tmm_traffic.server_side_traffic.bits_in".into(),
                        ),
                    );
                }
                let _cond = {
                    event.has_value("json.system.tmmTraffic")
                        && event
                            .get("json.system.tmmTraffic")
                            .and_then(|v| v.get("serverSideTraffic.bitsOut"))
                            .is_some_and(|v| !v.is_null())
                };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: def server_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('serverSideTraffic.bitsOut'); server_side_traffic.put('bits_out', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('server_side_traffic', server_side_traffic);\n} else{\n    if (ctx.f5_bigip?.log?.tmm_traffic?.server_side_traffic == null) {\n        ctx.f5_bigip.log.tmm_traffic.server_side_traffic = new HashMap();\n    }\n    ctx.f5_bigip.log.tmm_traffic.server_side_traffic.put('bits_out', obj);\n}
                    move_map_entry(
                        event,
                        &MoveMapEntry::new(
                            "json.system.tmmTraffic".into(),
                            "serverSideTraffic.bitsOut".into(),
                            "f5_bigip.log.tmm_traffic.server_side_traffic.bits_out".into(),
                        ),
                    );
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.tmm_traffic.client_side_traffic.bits_in") {
                        if let Some(val) =
                            event.get("f5_bigip.log.tmm_traffic.client_side_traffic.bits_in")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "f5_bigip.log.tmm_traffic.client_side_traffic.bits_in"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "f5_bigip.log.tmm_traffic.client_side_traffic.bits_in",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_tmm_traffic_client_side_traffic_bits_in_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.tmm_traffic.client_side_traffic.bits_out") {
                        if let Some(val) =
                            event.get("f5_bigip.log.tmm_traffic.client_side_traffic.bits_out")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "f5_bigip.log.tmm_traffic.client_side_traffic.bits_out"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "f5_bigip.log.tmm_traffic.client_side_traffic.bits_out",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_tmm_traffic_client_side_traffic_bits_out_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.tmm_traffic.server_side_traffic.bits_in") {
                        if let Some(val) =
                            event.get("f5_bigip.log.tmm_traffic.server_side_traffic.bits_in")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "f5_bigip.log.tmm_traffic.server_side_traffic.bits_in"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "f5_bigip.log.tmm_traffic.server_side_traffic.bits_in",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_tmm_traffic_server_side_traffic_bits_in_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.tmm_traffic.server_side_traffic.bits_out") {
                        if let Some(val) =
                            event.get("f5_bigip.log.tmm_traffic.server_side_traffic.bits_out")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "f5_bigip.log.tmm_traffic.server_side_traffic.bits_out"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "f5_bigip.log.tmm_traffic.server_side_traffic.bits_out",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_system_tmm_traffic_server_side_traffic_bits_out_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.system.version") {
                    event.rename("json.system.version", "f5_bigip.log.version")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
                if event.has_value("json.system.versionBuild") {
                    event.rename("json.system.versionBuild", "f5_bigip.log.version_build")?;
                }
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    event.remove("f5_bigip.log.chassis_id");
                    event.remove("f5_bigip.log.description");
                    event.remove("f5_bigip.log.hostname");
                    event.remove("f5_bigip.log.location");
                    event.remove("f5_bigip.log.base_mac");
                    event.remove("f5_bigip.log.machine_id");
                    event.remove("f5_bigip.log.marketing_name");
                    event.remove("f5_bigip.log.system_timestamp");
                    event.remove("f5_bigip.log.version");
                }
                event.remove("json");
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append_unique("event.kind", json!("pipeline_error"))?;
                }
                // End nested pipeline: "pipeline_bigipsystem"
            }

            let _cond = { event.get_str("json.telemetryEventCategory") == Some("ihealthInfo") };
            if _cond {
                // Begin nested pipeline: "pipeline_bigipihealthinfo"
                event.append("event.category", json!("vulnerability"))?;
                event.append("event.type", json!("info"))?;
                event.set("observer.product", json!("iHealth Information"))?;
                // Painless script
                // Source: def convertKeysToString(Map versionMap, String[] keysToConvert) {\n  def convertedMap = [:];\n  versionMap.entrySet().forEach(entry -> {\n    def key = entry.getKey().toString();\n    def value = entry.getValue();\n    if (Arrays.asList(keysToConvert).contains(key)) {\n      convertedMap[key] = value.toString();\n    } else {\n      convertedMap[key] = value;\n    }\n  });\n  return convertedMap;\n}\ndef diagnosticArray = ctx.json.diagnostics;\ndef keysToConvert = new String[] {\n  'minor',\n  'major',\n  'maintenance',\n  'fix',\n  'point'\n};\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def versionArray = diagnosticArray[i].version;\n    if (versionArray instanceof List) {\n      for (int j = 0; j < versionArray.size(); j++) {\n        versionArray[j] = convertKeysToString(versionArray[j], keysToConvert);\n      }\n      ctx.json.diagnostics[i].put('version', versionArray);\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def convertKeysToString(Map versionMap, String[] keysToConvert) {\n  def convertedMap = [:];\n  versionMap.entrySet().forEach(entry -> {\n    def key = entry.getKey().toString();\n    def value = entry.getValue();\n    if (Arrays.asList(keysToConvert).contains(key)) {\n      convertedMap[key] = value.toString();\n    } else {\n      convertedMap[key] = value;\n    }\n  });\n  return convertedMap;\n}\ndef diagnosticArray = ctx.json.diagnostics;\ndef keysToConvert = new String[] {\n  'minor',\n  'major',\n  'maintenance',\n  'fix',\n  'point'\n};\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def versionArray = diagnosticArray[i].version;\n    if (versionArray instanceof List) {\n      for (int j = 0; j < versionArray.size(); j++) {\n        versionArray[j] = convertKeysToString(versionArray[j], keysToConvert);\n      }\n      ctx.json.diagnostics[i].put('version', versionArray);\n    }\n  }\n}\n"#
                    ),
                )?;
                // Painless script
                // Source: def diagnosticArray = ctx.json.diagnostics;\ndef cveIds = new HashSet();\ndef importance = new HashSet();\ndef header = new HashSet();\ndef summary = new HashSet();\ndef ruleName = new HashSet();\ndef ruleRef = new HashSet();\ndef threatRef = new HashSet();\nif (ctx.vulnerability == null) {\n  ctx.vulnerability = new HashMap()\n}\nif (ctx.rule == null) {\n  ctx.rule = new HashMap()\n}\nif (ctx.threat == null) {\n  ctx.threat = new HashMap()\n}\nif (ctx.threat.enrichments == null) {\n  ctx.threat.enrichments = new HashMap()\n}\nif (ctx.threat.enrichments.indicator == null) {\n  ctx.threat.enrichments.indicator = new HashMap()\n}\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def cveIdsArray = diagnosticArray[i].cveIds;\n    if (cveIdsArray instanceof List) {\n      for (int j=0; j < cveIdsArray.size(); j++) {\n        cveIds.add(cveIdsArray[j]);\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    def solutionArray = diagnosticArray[i].solution;\n    if (solutionArray instanceof List) {\n      for (int j=0; j < solutionArray.size(); j++) {\n        if (solutionArray[j].id != null) {\n          ruleRef.add(solutionArray[j].id);\n        }\n        if (solutionArray[j].value != null) {\n          threatRef.add(solutionArray[j].value);\n        }\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    if (diagnosticArray[i].importance != null){\n      importance.add(diagnosticArray[i].importance);\n    }\n    if (diagnosticArray[i].header != null){\n      header.add(diagnosticArray[i].header);\n    }\n    if (diagnosticArray[i].summary != null){\n      summary.add(diagnosticArray[i].summary);\n    }\n    if (diagnosticArray[i].name != null){\n      ruleName.add(diagnosticArray[i].name);\n    }\n  }\n}\nctx.vulnerability.put('id', cveIds);\nctx.vulnerability.put('severity', importance);\nctx.vulnerability.put('description', header);\nctx.vulnerability.put('description', summary);\nctx.rule.put('name', ruleName);\nctx.rule.put('reference', ruleRef);\nctx.threat.enrichments.indicator.put('reference', threatRef);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def diagnosticArray = ctx.json.diagnostics;\ndef cveIds = new HashSet();\ndef importance = new HashSet();\ndef header = new HashSet();\ndef summary = new HashSet();\ndef ruleName = new HashSet();\ndef ruleRef = new HashSet();\ndef threatRef = new HashSet();\nif (ctx.vulnerability == null) {\n  ctx.vulnerability = new HashMap()\n}\nif (ctx.rule == null) {\n  ctx.rule = new HashMap()\n}\nif (ctx.threat == null) {\n  ctx.threat = new HashMap()\n}\nif (ctx.threat.enrichments == null) {\n  ctx.threat.enrichments = new HashMap()\n}\nif (ctx.threat.enrichments.indicator == null) {\n  ctx.threat.enrichments.indicator = new HashMap()\n}\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def cveIdsArray = diagnosticArray[i].cveIds;\n    if (cveIdsArray instanceof List) {\n      for (int j=0; j < cveIdsArray.size(); j++) {\n        cveIds.add(cveIdsArray[j]);\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    def solutionArray = diagnosticArray[i].solution;\n    if (solutionArray instanceof List) {\n      for (int j=0; j < solutionArray.size(); j++) {\n        if (solutionArray[j].id != null) {\n          ruleRef.add(solutionArray[j].id);\n        }\n        if (solutionArray[j].value != null) {\n          threatRef.add(solutionArray[j].value);\n        }\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    if (diagnosticArray[i].importance != null){\n      importance.add(diagnosticArray[i].importance);\n    }\n    if (diagnosticArray[i].header != null){\n      header.add(diagnosticArray[i].header);\n    }\n    if (diagnosticArray[i].summary != null){\n      summary.add(diagnosticArray[i].summary);\n    }\n    if (diagnosticArray[i].name != null){\n      ruleName.add(diagnosticArray[i].name);\n    }\n  }\n}\nctx.vulnerability.put('id', cveIds);\nctx.vulnerability.put('severity', importance);\nctx.vulnerability.put('description', header);\nctx.vulnerability.put('description', summary);\nctx.rule.put('name', ruleName);\nctx.rule.put('reference', ruleRef);\nctx.threat.enrichments.indicator.put('reference', threatRef);\n"#
                    ),
                )?;
                if event.has_value("json") {
                    event.rename("json", "f5_bigip.log")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.system.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                if event.has_value("f5_bigip.log.system.ihealthLink") {
                    event.rename(
                        "f5_bigip.log.system.ihealthLink",
                        "f5_bigip.log.system.ihealth_link",
                    )?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.system.ihealth_link")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.full", v)?;
                }
                if event.has_value("f5_bigip.log.system.qkviewNumber") {
                    event.rename(
                        "f5_bigip.log.system.qkviewNumber",
                        "f5_bigip.log.system.qkview_number",
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.telemetryServiceInfo.cycleEnd")
                        && event.get_str("f5_bigip.log.telemetryServiceInfo.cycleEnd") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("f5_bigip.log.telemetryServiceInfo.cycleEnd")
                        {
                            match parse_date_out(
                                &date_str,
                                &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event
                                    .set("f5_bigip.log.telemetry_service_info.cycle_end", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.telemetryServiceInfo.cycleEnd".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_cycle_end_conversion",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.telemetry_service_info.cycle_end")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.end", v)?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.telemetryServiceInfo.cycleStart")
                        && event.get_str("f5_bigip.log.telemetryServiceInfo.cycleStart") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("f5_bigip.log.telemetryServiceInfo.cycleStart")
                        {
                            match parse_date_out(
                                &date_str,
                                &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set(
                                    "f5_bigip.log.telemetry_service_info.cycle_start",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.telemetryServiceInfo.cycleStart".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_cycle_start_conversion",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.telemetry_service_info.cycle_start")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.start", v)?;
                }
                if event.has_value("f5_bigip.log.telemetryEventCategory") {
                    event.rename(
                        "f5_bigip.log.telemetryEventCategory",
                        "f5_bigip.log.telemetry.event.category",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("f5_bigip.log.telemetryServiceInfo.cycleEnd");
                    event.remove("f5_bigip.log.telemetryServiceInfo.cycleStart");
                    Ok(())
                })();
                let _cond = {
                    event
                        .get("f5_bigip.log.diagnostics")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "f5_bigip.log.diagnostics", |event| {
                            let _cond = {
                                !event.has_value("tags")
                                    || !(event.get("tags").is_some_and(|v| match v {
                                        serde_json::Value::Array(a) => a.iter().any(|x| {
                                            x.as_str() == Some("preserve_duplicate_custom_fields")
                                        }),
                                        serde_json::Value::String(s) => {
                                            s.contains("preserve_duplicate_custom_fields")
                                        }
                                        _ => false,
                                    }))
                            };
                            if _cond {
                                event.remove("_ingest._value.cve_ids");
                                event.remove("_ingest._value.header");
                                event.remove("_ingest._value.importance");
                                event.remove("_ingest._value.name");
                                event.remove("_ingest._value.solution");
                                event.remove("_ingest._value.summary");
                            }
                            Ok(())
                        })?;
                        Ok(())
                    })();
                }
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("f5_bigip.log.system.hostname");
                        event.remove("f5_bigip.log.system.ihealth_link");
                        event.remove("f5_bigip.log.telemetry_service_info.cycle_end");
                        event.remove("f5_bigip.log.telemetry_service_info.cycle_start");
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_bigipihealthinfo"
            }

            let _cond = {
                event.has_value("event.original")
                    && (event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| {
                            x.as_str() == Some("device_product=\"Application Security Module\"")
                        }),
                        serde_json::Value::String(s) => {
                            s.contains("device_product=\"Application Security Module\"")
                        }
                        _ => false,
                    }) || event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("device_product=\"ASM\"")),
                        serde_json::Value::String(s) => s.contains("device_product=\"ASM\""),
                        _ => false,
                    }) || event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("device_product=ASM"))
                        }
                        serde_json::Value::String(s) => s.contains("device_product=ASM"),
                        _ => false,
                    }))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_bigip_bot_and_dos"
                let _cond = {
                    event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| {
                            x.as_str() == Some("device_product=\"Application Security Module\"")
                        }),
                        serde_json::Value::String(s) => {
                            s.contains("device_product=\"Application Security Module\"")
                        }
                        _ => false,
                    })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("event.original") {
                            for pair in kv_str.split(",") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "event.original".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let value = match (value.chars().next(), value.chars().last()) {
                                        (Some('('), Some(')'))
                                        | (Some('['), Some(']'))
                                        | (Some('<'), Some('>'))
                                        | (Some('"'), Some('"'))
                                        | (Some('\''), Some('\''))
                                            if value.chars().count() > 1 =>
                                        {
                                            &value[1..value.len() - 1]
                                        }
                                        _ => value,
                                    };
                                    if !key.is_empty() {
                                        kv_put(event, &format!("kv.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("device_product=ASM"))
                        }
                        serde_json::Value::String(s) => s.contains("device_product=ASM"),
                        _ => false,
                    }) || event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("device_product=\"ASM\"")),
                        serde_json::Value::String(s) => s.contains("device_product=\"ASM\""),
                        _ => false,
                    })
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("event.original") {
                            for pair in kv_str.split(";") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "event.original".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let value = match (value.chars().next(), value.chars().last()) {
                                        (Some('('), Some(')'))
                                        | (Some('['), Some(']'))
                                        | (Some('<'), Some('>'))
                                        | (Some('"'), Some('"'))
                                        | (Some('\''), Some('\''))
                                            if value.chars().count() > 1 =>
                                        {
                                            &value[1..value.len() - 1]
                                        }
                                        _ => value,
                                    };
                                    if !key.is_empty() {
                                        kv_put(event, &format!("kv.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv_event_original_for_dos",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // Painless script, resolved to its runners at generation time
                // Source: boolean dropEmptyFields(Object object) {\n  if (object == 'N/A' || object == 'NA') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        empty_collections: true,
                        prune_lists: true,
                        sentinels: vec!["N/A".into(), "NA".into()],
                        ..DropPolicy::none()
                    },
                    None,
                );
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("info"))?;
                event.set("event.kind", json!("alert"))?;
                if event.has_value("kv.action") {
                    event.rename("kv.action", "f5_bigip.log.action")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = { event.get_str("event.action") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("event.action") {
                            gsub_field(
                                event,
                                "event.action",
                                "event.action",
                                cached_regex!(" "),
                                "-",
                            )?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "gsub")?;
                        event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("kv.actual_mitigation_action_reason") {
                    event.rename(
                        "kv.actual_mitigation_action_reason",
                        "f5_bigip.log.actual_mitigation_action.reason",
                    )?;
                }
                if event.has_value("kv.actual_mitigation_action") {
                    event.rename(
                        "kv.actual_mitigation_action",
                        "f5_bigip.log.actual_mitigation_action.value",
                    )?;
                }
                if event.has_value("kv.additional_bot_signatures") {
                    event.rename(
                        "kv.additional_bot_signatures",
                        "f5_bigip.log.additional_bot_signatures",
                    )?;
                }
                if event.has_value("kv.anomalies") {
                    event.rename("kv.anomalies", "f5_bigip.log.anomalies")?;
                }
                if event.has_value("kv.anomaly_categories") {
                    event.rename("kv.anomaly_categories", "f5_bigip.log.anomaly_categories")?;
                }
                if event.has_value("kv.application_display_name") {
                    event.rename(
                        "kv.application_display_name",
                        "f5_bigip.log.application.display_name",
                    )?;
                }
                if event.has_value("kv.application_version") {
                    event.rename("kv.application_version", "f5_bigip.log.application.version")?;
                }
                let _cond = {
                    event.get_str("kv.bigip_mgmt_ip") != Some("null")
                        && event.get_str("kv.bigip_mgmt_ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.bigip_mgmt_ip") {
                            if let Some(val) = event.get("kv.bigip_mgmt_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.bigip_mgmt_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bigip_management.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bigip_mgmt_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bigip_management.ip")
                        && event.get_str("f5_bigip.log.bigip_management.ip") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("f5_bigip.log.bigip_management.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bigip_management.ip")
                        && event.get_str("f5_bigip.log.bigip_management.ip") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.bigip_management.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.get_str("kv.bigip_mgmt_ip2") != Some("null")
                        && event.get_str("kv.bigip_mgmt_ip2") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.bigip_mgmt_ip2") {
                            if let Some(val) = event.get("kv.bigip_mgmt_ip2") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.bigip_mgmt_ip2".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bigip_management.ip2", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bigip_mgmt_ip2_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bigip_management.ip2")
                        && event.get_str("f5_bigip.log.bigip_management.ip2") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("f5_bigip.log.bigip_management.ip2")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bigip_management.ip2")
                        && event.get_str("f5_bigip.log.bigip_management.ip2") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.bigip_management.ip2")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.get_str("kv.bigip_mgmt_ip_2") != Some("null")
                        && event.get_str("kv.bigip_mgmt_ip_2") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.bigip_mgmt_ip_2") {
                            if let Some(val) = event.get("kv.bigip_mgmt_ip_2") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.bigip_mgmt_ip_2".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.bigip_management.ip_2", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bigip_mgmt_ip_2_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bigip_management.ip_2")
                        && event.get_str("f5_bigip.log.bigip_management.ip_2") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("f5_bigip.log.bigip_management.ip_2")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.bigip_management.ip_2")
                        && event.get_str("f5_bigip.log.bigip_management.ip_2") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.bigip_management.ip_2")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("kv.bot_name") {
                    event.rename("kv.bot_name", "f5_bigip.log.bot_name")?;
                }
                if event.has_value("kv.bot_signature_category") {
                    event.rename(
                        "kv.bot_signature_category",
                        "f5_bigip.log.bot_signature.category",
                    )?;
                }
                if event.has_value("kv.bot_signature") {
                    event.rename("kv.bot_signature", "f5_bigip.log.bot_signature.value")?;
                }
                if event.has_value("kv.browser_actual_verification_action_reason") {
                    event.rename(
                        "kv.browser_actual_verification_action_reason",
                        "f5_bigip.log.browser_actual_verification_action.reason",
                    )?;
                }
                if event.has_value("kv.browser_actual_verification_action") {
                    event.rename(
                        "kv.browser_actual_verification_action",
                        "f5_bigip.log.browser_actual_verification_action.value",
                    )?;
                }
                if event.has_value("kv.browser_configured_verification_action") {
                    event.rename(
                        "kv.browser_configured_verification_action",
                        "f5_bigip.log.browser_configured_verification_action",
                    )?;
                }
                if event.has_value("kv.browser_verification_status") {
                    event.rename(
                        "kv.browser_verification_status",
                        "f5_bigip.log.browser_verification_status",
                    )?;
                }
                if event.has_value("kv.captcha_status") {
                    event.rename("kv.captcha_status", "f5_bigip.log.captcha_status")?;
                }
                if event.has_value("kv.class") {
                    event.rename("kv.class", "f5_bigip.log.class")?;
                }
                if event.has_value("kv.classification_reason") {
                    event.rename(
                        "kv.classification_reason",
                        "f5_bigip.log.classification_reason",
                    )?;
                }
                if event.has_value("kv.client_ip_geo_location") {
                    event.rename(
                        "kv.client_ip_geo_location",
                        "f5_bigip.log.client.ip_geo_location",
                    )?;
                }
                let _cond = {
                    event.get_str("kv.client_ip") != Some("null")
                        && event.get_str("kv.client_ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.client_ip") {
                            if let Some(val) = event.get("kv.client_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.client_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.client.ip")
                        && event.get_str("f5_bigip.log.client.ip") != Some("null")
                };
                if _cond {
                    if let Some(v) = event
                        .get("f5_bigip.log.client.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if let Some(v) = event
                    .get("source.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.geo", v)?;
                }
                if let Some(v) = event
                    .get("source.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.as", v)?;
                }
                if let Some(v) = event
                    .get("source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.client.ip")
                        && event.get_str("f5_bigip.log.client.ip") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.get_str("kv.client_port") != Some("null")
                        && event.get_str("kv.client_port") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.client_port") {
                            if let Some(val) = event.get("kv.client_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.client_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.client.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.client.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                if let Some(v) = event
                    .get("source.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.port", v)?;
                }
                if event.has_value("kv.client_request_uri") {
                    event.rename("kv.client_request_uri", "f5_bigip.log.client.request_uri")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.client.request_uri")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.path", v)?;
                }
                if event.has_value("kv.client_type") {
                    event.rename("kv.client_type", "f5_bigip.log.client.type")?;
                }
                let _cond = {
                    event.has_value("kv.configuration_date_time")
                        && event.get_str("kv.configuration_date_time") != Some("null")
                        && event.get_str("kv.configuration_date_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("kv.configuration_date_time") {
                            match parse_date_out(
                                &date_str,
                                &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.configuration_date_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "kv.configuration_date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_configuration_date_time",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.configuration_date_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                if event.has_value("kv.configured_mitigation_action_reason") {
                    event.rename(
                        "kv.configured_mitigation_action_reason",
                        "f5_bigip.log.configured_mitigation_action.reason",
                    )?;
                }
                if event.has_value("kv.configured_mitigation_action") {
                    event.rename(
                        "kv.configured_mitigation_action",
                        "f5_bigip.log.configured_mitigation_action.value",
                    )?;
                }
                if event.has_value("kv.context_name") {
                    event.rename("kv.context_name", "f5_bigip.log.context.name")?;
                }
                if event.has_value("kv.context_type") {
                    event.rename("kv.context_type", "f5_bigip.log.context.type")?;
                }
                let _cond = {
                    event.has_value("kv.date_time")
                        && event.get_str("kv.date_time") != Some("null")
                        && event.get_str("kv.date_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("kv.date_time") {
                            match parse_date_out(
                                &date_str,
                                &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("f5_bigip.log.date_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "kv.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_date_time")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.date_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.start", v)?;
                }
                let _cond = {
                    event.get_str("kv.dest_ip") != Some("null")
                        && event.get_str("kv.dest_ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.dest_ip") {
                            if let Some(val) = event.get("kv.dest_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.dest_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.destination.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_dest_ip_to_ip")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
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
                if let Some(v) = event
                    .get("destination.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.geo", v)?;
                }
                if let Some(v) = event
                    .get("destination.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.as", v)?;
                }
                if let Some(v) = event
                    .get("destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.ip", v)?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.destination.ip")
                        && event.get_str("f5_bigip.log.destination.ip") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.get_str("kv.dest_port") != Some("null")
                        && event.get_str("kv.dest_port") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.dest_port") {
                            if let Some(val) = event.get("kv.dest_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.dest_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dest_port_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.port", v)?;
                }
                if let Some(v) = event
                    .get("destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("server.port", v)?;
                }
                if event.has_value("kv.device_blade") {
                    event.rename("kv.device_blade", "f5_bigip.log.device.blade")?;
                }
                if event.has_value("kv.device_id") {
                    event.rename("kv.device_id", "f5_bigip.log.device.id")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.device.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("device.id", v)?;
                }
                if event.has_value("kv.device_product") {
                    event.rename("kv.device_product", "f5_bigip.log.device.product")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.device.product")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.product", v)?;
                }
                if event.has_value("kv.device_vendor") {
                    event.rename("kv.device_vendor", "f5_bigip.log.device.vendor")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.device.vendor")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.vendor", v)?;
                }
                if event.has_value("kv.device_version") {
                    event.rename("kv.device_version", "f5_bigip.log.device.version")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.device.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.version", v)?;
                }
                if event.has_value("kv.device_id_action") {
                    event.rename("kv.device_id_action", "f5_bigip.log.device_id.action")?;
                }
                if event.has_value("kv.device_id_status") {
                    event.rename("kv.device_id_status", "f5_bigip.log.device_id.status")?;
                }
                if event.has_value("kv.dos_attack_detection_mode") {
                    event.rename(
                        "kv.dos_attack_detection_mode",
                        "f5_bigip.log.dos.attack.detection_mode",
                    )?;
                }
                if event.has_value("kv.dos_attack_event") {
                    event.rename("kv.dos_attack_event", "f5_bigip.log.dos.attack.event")?;
                }
                if event.has_value("kv.dos_attack_id") {
                    event.rename("kv.dos_attack_id", "f5_bigip.log.dos.attack.id")?;
                }
                if event.has_value("kv.dos_attack_latency") {
                    event.rename("kv.dos_attack_latency", "f5_bigip.log.dos.attack.latency")?;
                }
                if event.has_value("kv.dos_attack_name") {
                    event.rename("kv.dos_attack_name", "f5_bigip.log.dos.attack.name")?;
                }
                if event.has_value("kv.dos_attack_tps") {
                    event.rename("kv.dos_attack_tps", "f5_bigip.log.dos.attack.tps")?;
                }
                if event.has_value("kv.dos_baseline_latency") {
                    event.rename(
                        "kv.dos_baseline_latency",
                        "f5_bigip.log.dos.baseline.latency",
                    )?;
                }
                if event.has_value("kv.dos_baseline_tps") {
                    event.rename("kv.dos_baseline_tps", "f5_bigip.log.dos.baseline.tps")?;
                }
                if event.has_value("kv.dos_baseline_traffic_percent") {
                    event.rename(
                        "kv.dos_baseline_traffic_percent",
                        "f5_bigip.log.dos.baseline.traffic_percent",
                    )?;
                }
                if event.has_value("kv.dos_current_traffic_percent") {
                    event.rename(
                        "kv.dos_current_traffic_percent",
                        "f5_bigip.log.dos.current_traffic_percent",
                    )?;
                }
                let _cond = {
                    event.get_str("kv.dos_dropped_requests_count") != Some("null")
                        && event.get_str("kv.dos_dropped_requests_count") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.dos_dropped_requests_count") {
                            if let Some(val) = event.get("kv.dos_dropped_requests_count") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.dos_dropped_requests_count".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.dos.dropped_requests_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dos_dropped_requests_count_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.get_str("kv.dos_incoming_requests_count") != Some("null")
                        && event.get_str("kv.dos_incoming_requests_count") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.dos_incoming_requests_count") {
                            if let Some(val) = event.get("kv.dos_incoming_requests_count") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.dos_incoming_requests_count".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.dos.incoming_requests_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dos_incoming_requests_count_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("kv.dos_detection_condition") {
                    event.rename(
                        "kv.dos_detection_condition",
                        "f5_bigip.log.dos_detection.condition",
                    )?;
                }
                if event.has_value("kv.dos_detection_threshold") {
                    event.rename(
                        "kv.dos_detection_threshold",
                        "f5_bigip.log.dos_detection.threshold",
                    )?;
                }
                if event.has_value("kv.dos_mitigate_to_threshold") {
                    event.rename(
                        "kv.dos_mitigate_to_threshold",
                        "f5_bigip.log.dos_mitigate_to_threshold",
                    )?;
                }
                if event.has_value("kv.dos_mitigation_action") {
                    event.rename(
                        "kv.dos_mitigation_action",
                        "f5_bigip.log.dos_mitigation.action",
                    )?;
                }
                if event.has_value("kv.dos_mitigation_reason") {
                    event.rename(
                        "kv.dos_mitigation_reason",
                        "f5_bigip.log.dos_mitigation.reason",
                    )?;
                }
                if event.has_value("kv.enforced_by") {
                    event.rename("kv.enforced_by", "f5_bigip.log.enforced_by")?;
                }
                if event.has_value("kv.errdefs_msg_name") {
                    event.rename("kv.errdefs_msg_name", "f5_bigip.log.errdefs.msg_name")?;
                }
                if event.has_value("kv.errdefs_msgno") {
                    event.rename("kv.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.errdefs.msgno")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("error.id", v)?;
                }
                if event.has_value("kv.event_id") {
                    event.rename("kv.event_id", "f5_bigip.log.event.id")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.event.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if event.has_value("kv.hostname") {
                    event.rename("kv.hostname", "f5_bigip.log.hostname")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.hostname")
                        && event.get_str("f5_bigip.log.hostname") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("f5_bigip.log.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("kv.http_method") {
                    event.rename("kv.http_method", "f5_bigip.log.http.method")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.http.method")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.method", v)?;
                }
                if event.has_value("kv.http_protocol_indication") {
                    event.rename(
                        "kv.http_protocol_indication",
                        "f5_bigip.log.http.protocol_indication",
                    )?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.http.protocol_indication")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.protocol", v)?;
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                // Painless script
                // Source: String message = ctx.event.original;\nint startIndex = message.indexOf('http_request=\"') + 'http_request=\"'.length();\nint endIndex = message.indexOf('\"', startIndex);\nif (startIndex >= 0 && endIndex >= 0) {\n  ctx.kv.http_request = message.substring(startIndex, endIndex);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String message = ctx.event.original;\nint startIndex = message.indexOf('http_request=\"') + 'http_request=\"'.length();\nint endIndex = message.indexOf('\"', startIndex);\nif (startIndex >= 0 && endIndex >= 0) {\n  ctx.kv.http_request = message.substring(startIndex, endIndex);\n}\n"#
                    ),
                )?;
                if event.has_value("kv.http_request") {
                    event.rename("kv.http_request", "f5_bigip.log.http.request")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("f5_bigip.log.http.request") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.method", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.path", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nHost: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.version", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nHost: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nConnection: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.host", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nConnection: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nPragma: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.connection", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nPragma: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nCache-Control: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.pragma", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nCache-Control: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\nUser-Agent: ") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.cache_control", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\nUser-Agent: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\n") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.user_agent", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\n") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\\r\\n") else {
                                break 'dissect false;
                            };
                            captured.push(("f5_bigip.log.http.other_headers", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\r\\n") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("f5_bigip.log.http.user_agent") {
                    gsub_field(
                        event,
                        "f5_bigip.log.http.user_agent",
                        "f5_bigip.log.http.user_agent",
                        cached_regex!("(\\([^)]*)\\+(https?://)"),
                        "$1%2b$2",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("f5_bigip.log.http.user_agent") {
                        if let Some(s) = event.get_string("f5_bigip.log.http.user_agent") {
                            match url_decode(&s) {
                                Some(decoded) => {
                                    event.set("f5_bigip.log.http.user_agent", json!(decoded))?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "f5_bigip.log.http.user_agent".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("f5_bigip.log.http.user_agent") {
                    if let Some(ua_str) = event.get_string("f5_bigip.log.http.user_agent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.remove("user_agent");
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("f5_bigip.log.http.version") {
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
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.get_str("f5_bigip.log.http.host") != Some("null")
                        && event.get_str("f5_bigip.log.http.host") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("f5_bigip.log.http.host") {
                            if let Some(val) = event.get("f5_bigip.log.http.host") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "f5_bigip.log.http.host".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.http.request_host", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_f5_bigip_log_http_host_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.http.request_host")
                        && event.get_str("f5_bigip.log.http.request_host") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.http.request_host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("kv.human_behaviour") {
                    event.rename("kv.human_behaviour", "f5_bigip.log.human_behaviour")?;
                }
                if event.has_value("kv.imei") {
                    event.rename("kv.imei", "f5_bigip.log.imei")?;
                }
                if event.has_value("kv.jailbroken_or_rooted_device") {
                    event.rename(
                        "kv.jailbroken_or_rooted_device",
                        "f5_bigip.log.jailbroken_or_rooted_device",
                    )?;
                }
                if event.has_value("kv.micro_service_hostname") {
                    event.rename(
                        "kv.micro_service_hostname",
                        "f5_bigip.log.micro_service.hostname",
                    )?;
                }
                if event.has_value("kv.micro_service_matched_wildcard_url") {
                    event.rename(
                        "kv.micro_service_matched_wildcard_url",
                        "f5_bigip.log.micro_service.matched_wildcard_url",
                    )?;
                }
                if event.has_value("kv.micro_service_name") {
                    event.rename("kv.micro_service_name", "f5_bigip.log.micro_service.name")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.micro_service.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("service.name", v)?;
                }
                if event.has_value("kv.micro_service_type") {
                    event.rename("kv.micro_service_type", "f5_bigip.log.micro_service.type")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.micro_service.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("service.type", v)?;
                }
                if event.has_value("kv.mobile_in_emulation_mode") {
                    event.rename(
                        "kv.mobile_in_emulation_mode",
                        "f5_bigip.log.mobile_in_emulation_mode",
                    )?;
                }
                if event.has_value("kv.mobile_is_app") {
                    event.rename("kv.mobile_is_app", "f5_bigip.log.mobile_is_app")?;
                }
                if event.has_value("kv.new_request_status") {
                    event.rename("kv.new_request_status", "f5_bigip.log.new_request_status")?;
                }
                if event.has_value("kv.os_name") {
                    event.rename("kv.os_name", "f5_bigip.log.osname")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.osname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.name", v)?;
                }
                if event.has_value("kv.partition_name") {
                    event.rename("kv.partition_name", "f5_bigip.log.partition_name")?;
                }
                if event.has_value("kv.previous_action") {
                    event.rename("kv.previous_action", "f5_bigip.log.previous.action")?;
                }
                if event.has_value("kv.previous_initiated_action_status") {
                    event.rename(
                        "kv.previous_initiated_action_status",
                        "f5_bigip.log.previous.initiated_action.status",
                    )?;
                }
                if event.has_value("kv.previous_initiated_action") {
                    event.rename(
                        "kv.previous_initiated_action",
                        "f5_bigip.log.previous.initiated_action.value",
                    )?;
                }
                let _cond = {
                    event.has_value("kv.previous_request_date_time")
                        && event.get_str("kv.previous_request_date_time") != Some("null")
                        && event.get_str("kv.previous_request_date_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("kv.previous_request_date_time")
                        {
                            match parse_date_out(
                                &date_str,
                                &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.previous.request_date_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "kv.previous_request_date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_previous_request_date_time",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("kv.previous_support_id") {
                    event.rename("kv.previous_support_id", "f5_bigip.log.previous.support_id")?;
                }
                if event.has_value("kv.profile_name") {
                    event.rename("kv.profile_name", "f5_bigip.log.profile_name")?;
                }
                if event.has_value("kv.reason") {
                    event.rename("kv.reason", "f5_bigip.log.reason")?;
                }
                if let Some(v) = event
                    .get("f5_bigip.log.reason")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reason", v)?;
                }
                if event.has_value("kv.reported_entity_type") {
                    event.rename(
                        "kv.reported_entity_type",
                        "f5_bigip.log.reported_entity_type",
                    )?;
                }
                let _cond = {
                    event.has_value("kv.request_date_time")
                        && event.get_str("kv.request_date_time") != Some("null")
                        && event.get_str("kv.request_date_time") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("kv.request_date_time") {
                            match parse_date_out(
                                &date_str,
                                &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("f5_bigip.log.request.date_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "kv.request_date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_request_date_time")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.request.date_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.start", v)?;
                }
                if event.has_value("kv.request_status") {
                    event.rename("kv.request_status", "f5_bigip.log.request.status")?;
                }
                if event.has_value("kv.route_domain") {
                    event.rename("kv.route_domain", "f5_bigip.log.route_domain")?;
                }
                if event.has_value("kv.session_id") {
                    event.rename("kv.session_id", "f5_bigip.log.session.id")?;
                }
                let _cond = {
                    event.get_str("kv.severity") != Some("null")
                        && event.get_str("kv.severity") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.severity") {
                            if let Some(val) = event.get("kv.severity") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.severity".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.severity.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_severity_to_long",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.severity.code")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.severity", v)?;
                }
                let _cond = {
                    event.get_str("kv.source_ip") != Some("null")
                        && event.get_str("kv.source_ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("kv.source_ip") {
                            if let Some(val) = event.get("kv.source_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "kv.source_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("f5_bigip.log.source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_ip_to_ip",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("f5_bigip.log.source.ip")
                        && event.get_str("f5_bigip.log.source.ip") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("f5_bigip.log.source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("f5_bigip.log.source.ip")
                        && event.get_str("f5_bigip.log.source.ip") != Some("null")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("f5_bigip.log.source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("kv.support_id") {
                    event.rename("kv.support_id", "f5_bigip.log.support.id")?;
                }
                let _cond = {
                    event.has_value("kv.timestamp")
                        && event.get_str("kv.timestamp") != Some("null")
                        && event.get_str("kv.timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("kv.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("f5_bigip.log.timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "kv.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("f5_bigip.log.timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                if event.has_value("kv.virtual_server_name") {
                    event.rename("kv.virtual_server_name", "f5_bigip.log.virtual_server_name")?;
                }
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    event.remove("f5_bigip.log.action");
                    event.remove("f5_bigip.log.bigip_management.ip");
                    event.remove("f5_bigip.log.bigip_management.ip_2");
                    event.remove("f5_bigip.log.bigip_management.ip2");
                    event.remove("f5_bigip.log.client.ip");
                    event.remove("f5_bigip.log.client.port");
                    event.remove("f5_bigip.log.client.request_uri");
                    event.remove("f5_bigip.log.configuration_date_time");
                    event.remove("f5_bigip.log.date_time");
                    event.remove("f5_bigip.log.destination.ip");
                    event.remove("f5_bigip.log.destination.port");
                    event.remove("f5_bigip.log.device.id");
                    event.remove("f5_bigip.log.device.product");
                    event.remove("f5_bigip.log.device.vendor");
                    event.remove("f5_bigip.log.device.version");
                    event.remove("f5_bigip.log.errdefs.msgno");
                    event.remove("f5_bigip.log.event.id");
                    event.remove("f5_bigip.log.hostname");
                    event.remove("f5_bigip.log.http.method");
                    event.remove("f5_bigip.log.http.protocol_indication");
                    event.remove("f5_bigip.log.micro_service.name");
                    event.remove("f5_bigip.log.micro_service.type");
                    event.remove("f5_bigip.log.osname");
                    event.remove("f5_bigip.log.reason");
                    event.remove("f5_bigip.log.request.date_time");
                    event.remove("f5_bigip.log.severity");
                    event.remove("f5_bigip.log.source.ip");
                    event.remove("f5_bigip.log.timestamp");
                }
                event.remove("kv.bigip_mgmt_ip");
                event.remove("kv.bigip_mgmt_ip_2");
                event.remove("kv.bigip_mgmt_ip2");
                event.remove("kv.client_ip");
                event.remove("kv.client_port");
                event.remove("kv.configuration_date_time");
                event.remove("kv.date_time");
                event.remove("kv.date_time");
                event.remove("kv.dest_ip");
                event.remove("kv.dest_port");
                event.remove("kv.dos_dropped_requests_count");
                event.remove("kv.dos_incoming_requests_count");
                event.remove("kv.previous_request_date_time");
                event.remove("kv.request_date_time");
                event.remove("kv.severity");
                event.remove("kv.source_ip");
                event.remove("kv.timestamp");
                if event.has_value("kv") {
                    event.rename("kv", "f5_bigip.log.extra_fields")?;
                }
                // End nested pipeline: "pipeline_bigip_bot_and_dos"
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'null') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["null".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
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
