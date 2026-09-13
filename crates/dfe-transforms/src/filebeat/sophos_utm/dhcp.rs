// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dhcp` pipeline.
pub struct Dhcp;

impl Transform for Dhcp {
    fn name(&self) -> &str {
        "dhcp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;

            event.set("network.protocol", json!("dhcp"))?;

            event.set("network.transport", json!("udp"))?;

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("DHCPDISCOVER")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$"),
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("DHCPOFFER")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("DHCPREQUEST")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip}( \\(%{IP:sophos.utm.router.ip}\\))? from %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}(: %{GREEDYDATA:message})?$
                    // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip}( \\(%{IP:sophos.utm.router.ip}\\))? from %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}(: %{GREEDYDATA:message})?$"),
                            cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("DHCPACK")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("DHCPNACK")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("DHCPINFORM")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$
                    // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$"),
                            cached_grok!("^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("Listening")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$
                    // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$"),
                            cached_grok!("^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.raw_data") && event.get_str("_tmp.raw_data").is_some_and(|s| s.starts_with("Sending")) };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$
                    // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$"),
                            cached_grok!("^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$"),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(input) = event.get_string("_tmp.raw_data") {
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !cached_grok!("^%{GREEDYDATA:message}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }
                Ok(())
            })();

            let _cond = { event.has_value("event.action") && (event.get_str("event.action") == Some("sending") || event.get_str("event.action") == Some("listening")) };
            if _cond {
            event.set("event.action", json!(format!("{}-{}", event.get("process.name").map_or_else(String::new, template_to_string), event.get("event.action").map_or_else(String::new, template_to_string))))?;
            }

            if event.has_value("client.mac") {
                gsub_field(event, "client.mac", "client.mac", cached_regex!("[:]"), "-")?;
            }

            if event.has_value("client.mac") {
                map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
            }

            let _cond = { event.get_str("client.ip") != Some("") };
            if _cond {
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
            }

            let _cond = { event.get_str("client.ip") != Some("") };
            if _cond {
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
            }

                if event.has_value("client.as.asn") {
                    event.rename("client.as.asn", "client.as.number")?;
                }

                if event.has_value("client.as.organization_name") {
                    event.rename("client.as.organization_name", "client.as.organization.name")?;
                }

            let _cond = { event.has_value("sophos.utm.client.hostname") && event.get_str("sophos.utm.client.hostname") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("sophos.utm.client.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("client.ip") && event.get_str("client.ip") != Some("") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
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
