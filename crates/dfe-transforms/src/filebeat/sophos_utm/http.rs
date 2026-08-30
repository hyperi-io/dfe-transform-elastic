// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `http` pipeline.
pub struct Http;

impl Transform for Http {
    fn name(&self) -> &str {
        "http"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("network.protocol", json!("http"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            let _cond = { event.has_value("_tmp.raw_data") };
            if _cond {
            if event.has_value("_tmp.raw_data") {
                if let Some(kv_str) = event.get_string("_tmp.raw_data") {
                    for pair in cached_regex!(" (?=[a-z0-9\\_\\-]+=\")").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.raw_data".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| "\" ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("sophos.utm.{}", key), value)?;
                            }
                        }
                    }
                }
            }
            }

            let _cond = { event.has_value("sophos.utm.action") && !event.has_value("sophos.utm.reason") };
            if _cond {
                event.rename("sophos.utm.action", "event.action")?;
            }

            let _cond = { event.has_value("sophos.utm.action") && event.has_value("sophos.utm.reason") };
            if _cond {
            event.set("event.action", json!(format!("{}-{}", event.get("sophos.utm.action").map_or_else(String::new, template_to_string), event.get("sophos.utm.reason").map_or_else(String::new, template_to_string))))?;
            }

                if event.has_value("sophos.utm.application") {
                    event.rename("sophos.utm.application", "network.application")?;
                }

                if event.has_value("sophos.utm.dstip") {
                    event.rename("sophos.utm.dstip", "destination.ip")?;
                }

                if event.has_value("sophos.utm.device") {
                    event.rename("sophos.utm.device", "device.id")?;
                }

            let _cond = { event.has_value("sophos.utm.error") && event.get_str("sophos.utm.error") != Some("") };
            if _cond {
                event.rename("sophos.utm.error", "error.message")?;
            }

            let _cond = { event.has_value("sophos.utm.file") && event.get_str("sophos.utm.file") != Some("") };
            if _cond {
                event.rename("sophos.utm.file", "file.name")?;
            }

            let _cond = { event.has_value("sophos.utm.filename") && event.get_str("sophos.utm.filename") != Some("") };
            if _cond {
                event.rename("sophos.utm.filename", "file.name")?;
            }

                if event.has_value("sophos.utm.group") {
                    event.rename("sophos.utm.group", "group.name")?;
                }

                if event.has_value("sophos.utm.id") {
                    event.rename("sophos.utm.id", "event.id")?;
                }

                if event.has_value("sophos.utm.message") {
                    event.rename("sophos.utm.message", "message")?;
                }

                if event.has_value("sophos.utm.method") {
                    event.rename("sophos.utm.method", "http.request.method")?;
                }

                if event.has_value("sophos.utm.referer") {
                    event.rename("sophos.utm.referer", "http.request.referrer")?;
                }

                if event.has_value("sophos.utm.request") {
                    event.rename("sophos.utm.request", "http.request.id")?;
                }

            if event.has_value("sophos.utm.size") {
                if let Some(val) = event.get("sophos.utm.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.size".into(),
                            message,
                        })?;
                    event.set("sophos.utm.size", converted)?;
                }
            }

                if event.has_value("sophos.utm.size") {
                    event.rename("sophos.utm.size", "http.request.bytes")?;
                }

                if event.has_value("sophos.utm.srcip") {
                    event.rename("sophos.utm.srcip", "source.ip")?;
                }

            if event.has_value("sophos.utm.statuscode") {
                if let Some(val) = event.get("sophos.utm.statuscode") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.statuscode".into(),
                            message,
                        })?;
                    event.set("sophos.utm.statuscode", converted)?;
                }
            }

                if event.has_value("sophos.utm.statuscode") {
                    event.rename("sophos.utm.statuscode", "http.response.status_code")?;
                }

                if event.has_value("sophos.utm.ua") {
                    event.rename("sophos.utm.ua", "user_agent.original")?;
                }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

            let _cond = { event.has_value("sophos.utm.url") };
            if _cond {
                uri_parts(event, "sophos.utm.url", "url", true, true)?;
            }

                if event.has_value("sophos.utm.user") {
                    event.rename("sophos.utm.user", "user.name")?;
                }

            let _cond = { event.get_str("sophos.utm.severity") == Some("emergency") };
            if _cond {
            event.set("event.severity", json!(0))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("alert") };
            if _cond {
            event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("critical") };
            if _cond {
            event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("error") };
            if _cond {
            event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("warning") };
            if _cond {
            event.set("event.severity", json!(4))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("notice") };
            if _cond {
            event.set("event.severity", json!(5))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("info") };
            if _cond {
            event.set("event.severity", json!(6))?;
            }

            let _cond = { event.get_str("sophos.utm.severity") == Some("debug") };
            if _cond {
            event.set("event.severity", json!(7))?;
            }

            if event.has_value("sophos.utm.category") {
                if let Some(s) = event.get_string("sophos.utm.category") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("sophos.utm.category", Value::Array(parts))?;
                }
            }

            if event.has_value("sophos.utm.categoryname") {
                if let Some(s) = event.get_string("sophos.utm.categoryname") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("sophos.utm.categoryname", Value::Array(parts))?;
                }
            }

            if event.has_value("sophos.utm.exceptions") {
                if let Some(s) = event.get_string("sophos.utm.exceptions") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("sophos.utm.exceptions", Value::Array(parts))?;
                }
            }

                foreach_array(event, "sophos.utm", |event| {
                    gsub_field(event, "_ingest._key", "_ingest._key", cached_regex!("-"), "_")?;
                    Ok(())
                })?;

            let _cond = { event.get_str("source.ip") != Some("") };
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

            let _cond = { event.get_str("destination.ip") != Some("") };
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

            let _cond = { event.get_str("source.ip") != Some("") };
            if _cond {
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
            }

            let _cond = { event.get_str("destination.ip") != Some("") };
            if _cond {
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

            if event.has_value("sophos.utm.aptptime") {
                if let Some(val) = event.get("sophos.utm.aptptime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.aptptime".into(),
                            message,
                        })?;
                    event.set("sophos.utm.aptptime", converted)?;
                }
            }

            if event.has_value("sophos.utm.authtime") {
                if let Some(val) = event.get("sophos.utm.authtime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.authtime".into(),
                            message,
                        })?;
                    event.set("sophos.utm.authtime", converted)?;
                }
            }

            if event.has_value("sophos.utm.avscantime") {
                if let Some(val) = event.get("sophos.utm.avscantime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.avscantime".into(),
                            message,
                        })?;
                    event.set("sophos.utm.avscantime", converted)?;
                }
            }

            if event.has_value("sophos.utm.cattime") {
                if let Some(val) = event.get("sophos.utm.cattime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.cattime".into(),
                            message,
                        })?;
                    event.set("sophos.utm.cattime", converted)?;
                }
            }

            if event.has_value("sophos.utm.dnstime") {
                if let Some(val) = event.get("sophos.utm.dnstime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.dnstime".into(),
                            message,
                        })?;
                    event.set("sophos.utm.dnstime", converted)?;
                }
            }

            if event.has_value("sophos.utm.fullreqtime") {
                if let Some(val) = event.get("sophos.utm.fullreqtime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.fullreqtime".into(),
                            message,
                        })?;
                    event.set("sophos.utm.fullreqtime", converted)?;
                }
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.ip") && event.get_str("destination.ip") != Some("") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
