// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `packetfilter` pipeline.
pub struct Packetfilter;

impl Transform for Packetfilter {
    fn name(&self) -> &str {
        "packetfilter"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("_tmp.raw_data") };
            if _cond {
            if event.has_value("_tmp.raw_data") {
                if let Some(kv_str) = event.get_string("_tmp.raw_data") {
                    for pair in cached_regex!(" (?=[a-z0-9\\_\\-]+=)").split(&kv_str).into_iter() {
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

                if event.has_value("sophos.utm.action") {
                    event.rename("sophos.utm.action", "event.action")?;
                }

            let _cond = { event.get_str("event.action") == Some("accept") };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("event.action") == Some("drop") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

                if event.has_value("sophos.utm.dstip") {
                    event.rename("sophos.utm.dstip", "destination.ip")?;
                }

                if event.has_value("sophos.utm.dstmac") {
                    event.rename("sophos.utm.dstmac", "destination.mac")?;
                }

            if event.has_value("sophos.utm.dstport") {
                if let Some(val) = event.get("sophos.utm.dstport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.dstport".into(),
                            message,
                        })?;
                    event.set("sophos.utm.dstport", converted)?;
                }
            }

                if event.has_value("sophos.utm.dstport") {
                    event.rename("sophos.utm.dstport", "destination.port")?;
                }

                if event.has_value("sophos.utm.id") {
                    event.rename("sophos.utm.id", "event.id")?;
                }

                if event.has_value("sophos.utm.srcip") {
                    event.rename("sophos.utm.srcip", "source.ip")?;
                }

                if event.has_value("sophos.utm.srcmac") {
                    event.rename("sophos.utm.srcmac", "source.mac")?;
                }

            if event.has_value("sophos.utm.srcport") {
                if let Some(val) = event.get("sophos.utm.srcport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.srcport".into(),
                            message,
                        })?;
                    event.set("sophos.utm.srcport", converted)?;
                }
            }

                if event.has_value("sophos.utm.srcport") {
                    event.rename("sophos.utm.srcport", "source.port")?;
                }

                if event.has_value("sophos.utm.fwrule") {
                    event.rename("sophos.utm.fwrule", "rule.id")?;
                }

                if event.has_value("sophos.utm.initf") {
                    event.rename("sophos.utm.initf", "observer.ingress.interface.name")?;
                }

                if event.has_value("sophos.utm.outitf") {
                    event.rename("sophos.utm.outitf", "observer.egress.interface.name")?;
                }

                if event.has_value("sophos.utm.proto") {
                    event.rename("sophos.utm.proto", "network.iana_number")?;
                }

                if event.has_value("sophos.utm.message") {
                    event.rename("sophos.utm.message", "message")?;
                }

                if event.has_value("sophos.utm.app") {
                    event.rename("sophos.utm.app", "sophos.utm.app_id")?;
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

            if event.has_value("sophos.utm.tcpflags") {
                if let Some(s) = event.get_string("sophos.utm.tcpflags") {
                    let parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("sophos.utm.tcpflags", Value::Array(parts))?;
                }
            }

            if event.has_value("sophos.utm.tcpflags") {
                map_strings(event, "sophos.utm.tcpflags", "sophos.utm.tcpflags", str::to_lowercase)?;
            }

            if event.has_value("source.mac") {
                gsub_field(event, "source.mac", "source.mac", cached_regex!("[:]"), "-")?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("destination.mac") {
                gsub_field(event, "destination.mac", "destination.mac", cached_regex!("[:]"), "-")?;
            }

            if event.has_value("destination.mac") {
                map_strings(event, "destination.mac", "destination.mac", str::to_uppercase)?;
            }

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

            if event.has_value("sophos.utm.length") {
                if let Some(val) = event.get("sophos.utm.length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.length".into(),
                            message,
                        })?;
                    event.set("sophos.utm.length", converted)?;
                }
            }

            if event.has_value("sophos.utm.ttl") {
                if let Some(val) = event.get("sophos.utm.ttl") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.utm.ttl".into(),
                            message,
                        })?;
                    event.set("sophos.utm.ttl", converted)?;
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
