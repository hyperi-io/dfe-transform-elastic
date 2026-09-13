// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_dns_response` pipeline.
pub struct PipelineDnsResponse;

impl Transform for PipelineDnsResponse {
    fn name(&self) -> &str {
        "pipeline_dns_response"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("info"))?;

                event.append("event.category", json!("network"))?;

                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename("cef.extensions.applicationProtocol", "infoblox_threat_defense.event.application_protocol")?;
                }

            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
            }

                if event.has_value("cef.extensions.destinationDnsDomain") {
                    event.rename("cef.extensions.destinationDnsDomain", "infoblox_threat_defense.event.destination.dns_domain")?;
                }

            let _cond = { event.get_str("cef.extensions.deviceAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.deviceAddress") {
                if let Some(val) = event.get("cef.extensions.deviceAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.deviceAddress".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.device.address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_deviceAddress_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
            if _cond {
                event.append_unique("host.ip", json!(event.get("infoblox_threat_defense.event.device.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.device.address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("infoblox_threat_defense.event.device.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("cef.extensions.deviceHostName") != Some("") };
            if _cond {
            // on_failure: 3 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.deviceHostName") {
                if let Some(val) = event.get("cef.extensions.deviceHostName") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.deviceHostName".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.device.host_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_deviceHostName_to_ip")?;
                        if event.has_value("cef.extensions.deviceHostName") {
                            event.rename("cef.extensions.deviceHostName", "infoblox_threat_defense.event.device.host_name")?;
                        }
                    if let Some(v) = event.get("infoblox_threat_defense.event.device.host_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                        event.set("host.hostname", v)?;
                    }
                    let _cond = { event.has_value("infoblox_threat_defense.event.device.host_name") };
                    if _cond {
                        event.append_unique("related.hosts", json!(event.get("infoblox_threat_defense.event.device.host_name").map_or_else(String::new, template_to_string)))?;
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.device.host_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("infoblox_threat_defense.event.device.host_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.device.host_ip") };
            if _cond {
                event.append_unique("host.ip", json!(event.get("infoblox_threat_defense.event.device.host_ip").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("cef.extensions.InfobloxAnCount") {
                    event.rename("cef.extensions.InfobloxAnCount", "infoblox_threat_defense.event.infoblox.an_count")?;
                }

                if event.has_value("cef.extensions.InfobloxArCount") {
                    event.rename("cef.extensions.InfobloxArCount", "infoblox_threat_defense.event.infoblox.ar_count")?;
                }

                if event.has_value("cef.extensions.InfobloxB1ConnectionType") {
                    event.rename("cef.extensions.InfobloxB1ConnectionType", "infoblox_threat_defense.event.infoblox.b1.connection_type")?;
                }

                if event.has_value("cef.extensions.InfobloxB1DHCPFingerprint") {
                    event.rename("cef.extensions.InfobloxB1DHCPFingerprint", "infoblox_threat_defense.event.infoblox.b1.dhcp_fingerprint")?;
                }

                if event.has_value("cef.extensions.InfobloxB1DNSTags") {
                    event.rename("cef.extensions.InfobloxB1DNSTags", "infoblox_threat_defense.event.infoblox.b1.dns_tags")?;
                }

                if event.has_value("cef.extensions.InfobloxB1Network") {
                    event.rename("cef.extensions.InfobloxB1Network", "infoblox_threat_defense.event.infoblox.b1.network")?;
                }

            let _cond = { event.get_str("cef.extensions.InfobloxB1OPHIPAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.InfobloxB1OPHIPAddress") {
                if let Some(val) = event.get("cef.extensions.InfobloxB1OPHIPAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.InfobloxB1OPHIPAddress".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.infoblox.b1.oph.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_InfobloxB1OPHIPAddress_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.b1.oph.ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("infoblox_threat_defense.event.infoblox.b1.oph.ip_address").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("cef.extensions.InfobloxB1OPHName") {
                    event.rename("cef.extensions.InfobloxB1OPHName", "infoblox_threat_defense.event.infoblox.b1.oph.name")?;
                }

                if event.has_value("cef.extensions.InfobloxB1Region") {
                    event.rename("cef.extensions.InfobloxB1Region", "infoblox_threat_defense.event.infoblox.b1.region")?;
                }

                if event.has_value("cef.extensions.InfobloxB1SrcOSVersion") {
                    event.rename("cef.extensions.InfobloxB1SrcOSVersion", "infoblox_threat_defense.event.infoblox.b1.src_os_version")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.b1.src_os_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.full", v)?;
            }

                if event.has_value("cef.extensions.InfobloxDNSQClass") {
                    event.rename("cef.extensions.InfobloxDNSQClass", "infoblox_threat_defense.event.infoblox.dns_q.class")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.dns_q.class").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.class", v)?;
            }

                if event.has_value("cef.extensions.InfobloxDNSQFlags") {
                    event.rename("cef.extensions.InfobloxDNSQFlags", "infoblox_threat_defense.event.infoblox.dns_q.flags")?;
                }

                if event.has_value("cef.extensions.InfobloxDNSQType") {
                    event.rename("cef.extensions.InfobloxDNSQType", "infoblox_threat_defense.event.infoblox.dns_q.type")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.dns_q.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.type", v)?;
            }

                if event.has_value("cef.extensions.InfobloxDNSRCode") {
                    event.rename("cef.extensions.InfobloxDNSRCode", "infoblox_threat_defense.event.infoblox.dns_r_code")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.dns_r_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.response_code", v)?;
            }

                if event.has_value("cef.extensions.InfobloxNsCount") {
                    event.rename("cef.extensions.InfobloxNsCount", "infoblox_threat_defense.event.infoblox.ns_count")?;
                }

                if event.has_value("cef.extensions.message") {
                    event.rename("cef.extensions.message", "infoblox_threat_defense.event.message")?;
                }

            let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.sourceAddress") {
                if let Some(val) = event.get("cef.extensions.sourceAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.sourceAddress".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.source.address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_sourceAddress_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.source.address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("infoblox_threat_defense.event.source.address").map_or_else(String::new, template_to_string)))?;
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

            let _cond = { event.get_str("cef.extensions.sourcePort") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.sourcePort") {
                if let Some(val) = event.get("cef.extensions.sourcePort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.sourcePort".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.source.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_sourcePort_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("cef.extensions.InfobloxDNSView") {
                    event.rename("cef.extensions.InfobloxDNSView", "infoblox_threat_defense.event.infoblox.dns_view")?;
                }

                if event.has_value("cef.extensions.sourceMacAddress") {
                    event.rename("cef.extensions.sourceMacAddress", "infoblox_threat_defense.event.source.mac_address")?;
                }

                if event.has_value("cef.extensions.sourceUserName") {
                    event.rename("cef.extensions.sourceUserName", "infoblox_threat_defense.event.source.user_name")?;
                }

            let _cond = { event.has_value("infoblox_threat_defense.event.source.user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("infoblox_threat_defense.event.source.user_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("cef.extensions.destinationAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.destinationAddress") {
                if let Some(val) = event.get("cef.extensions.destinationAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.destinationAddress".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.destination.address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_destinationAddress_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.destination.address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("infoblox_threat_defense.event.destination.address").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("cef.extensions.transportProtocol") {
                    event.rename("cef.extensions.transportProtocol", "infoblox_threat_defense.event.transport_protocol")?;
                }

            if event.has_value("network.transport") {
                map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
