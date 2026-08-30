// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dns` pipeline.
pub struct Dns;

impl Transform for Dns {
    fn name(&self) -> &str {
        "dns"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;

            event.set("network.protocol", json!("dns"))?;

            event.set("network.transport", json!("udp"))?;

            let _cond = { event.has_value("_tmp.raw_data") && event.get("_tmp.raw_data").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("listening")), serde_json::Value::String(s) => s.contains("listening"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} on %{WORD:network.type} interface %{WORD:observer.ingress.interface.name}, %{IP:server.ip}#%{NUMBER:server.port:long}$
                    // Grok pattern: ^no longer %{WORD:event.action} on %{IP:server.ip}#%{NUMBER:server.port:long}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{WORD:network.type} interface %{WORD:observer.ingress.interface.name}, %{IP:server.ip}#%{NUMBER:server.port:long}$"),
                            cached_grok!("^no longer %{WORD:event.action} on %{IP:server.ip}#%{NUMBER:server.port:long}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    let _ = cached_grok!("^%{GREEDYDATA:message}$").extract_into(&input, event)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("network.type") {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
            }
                Ok(())
            })();

            let _cond = { event.has_value("event.action") };
            if _cond {
            event.set("event.action", json!(format!("{}-{}", event.get("process.name").map_or_else(String::new, template_to_string), event.get("event.action").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { event.get_str("server.ip") != Some("") };
            if _cond {
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
            }

            let _cond = { event.get_str("server.ip") != Some("") };
            if _cond {
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
            }

                if event.has_value("server.as.asn") {
                    event.rename("server.as.asn", "server.as.number")?;
                }

                if event.has_value("server.as.organization_name") {
                    event.rename("server.as.organization_name", "server.as.organization.name")?;
                }

            let _cond = { event.has_value("server.ip") && event.get_str("server.ip") != Some("") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
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
