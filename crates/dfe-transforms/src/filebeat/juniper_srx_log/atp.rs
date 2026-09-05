// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `atp` pipeline.
pub struct Atp;

impl Transform for Atp {
    fn name(&self) -> &str {
        "atp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("juniper.srx.tag") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            event.append("event.category", json!("network"))?;

            let _cond = {
                [
                    "SRX_AAMW_ACTION_LOG",
                    "AAMW_MALWARE_EVENT_LOG",
                    "AAMW_HOST_INFECTED_EVENT_LOG",
                    "AAMW_ACTION_LOG",
                ]
                .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                    && event.get_str("juniper.srx.action") != Some("PERMIT")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                [
                    "SRX_AAMW_ACTION_LOG",
                    "AAMW_MALWARE_EVENT_LOG",
                    "AAMW_HOST_INFECTED_EVENT_LOG",
                    "AAMW_ACTION_LOG",
                ]
                .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                    && event.get_str("juniper.srx.action") != Some("PERMIT")
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.get_str("juniper.srx.action") == Some("BLOCK")
                    || event.get_str("juniper.srx.tag") == Some("AAMW_MALWARE_EVENT_LOG")
            };
            if _cond {
                event.append("event.type", json!("info"))?;
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = {
                event.get_str("juniper.srx.action") != Some("BLOCK")
                    && event.get_str("juniper.srx.tag") != Some("AAMW_MALWARE_EVENT_LOG")
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = {
                event.get_str("juniper.srx.action") == Some("BLOCK")
                    || event.get_str("juniper.srx.tag") == Some("AAMW_MALWARE_EVENT_LOG")
            };
            if _cond {
                event.set("event.action", json!("malware_detected"))?;
            }

            let _cond = { event.has_value("juniper.srx.destination_address") };
            if _cond {
                if event.has_value("juniper.srx.destination_address") {
                    event.rename("juniper.srx.destination_address", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.set(
                    "server.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("juniper.srx.nat_destination_address") };
            if _cond {
                if event.has_value("juniper.srx.nat_destination_address") {
                    event.rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.destination_port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.destination_port") {
                        if let Some(val) = event.get("juniper.srx.destination_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.destination_port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.port") };
            if _cond {
                event.set(
                    "server.port",
                    json!(
                        event
                            .get("destination.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("server.port") {
                        if let Some(val) = event.get("server.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.port".into(),
                                    message,
                                }
                            })?;
                            event.set("server.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.nat_destination_port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.nat_destination_port") {
                        if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.nat_destination_port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.nat.port") };
            if _cond {
                event.set(
                    "server.nat.port",
                    json!(
                        event
                            .get("destination.nat.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.nat.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("server.nat.port") {
                        if let Some(val) = event.get("server.nat.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.nat.port".into(),
                                    message,
                                }
                            })?;
                            event.set("server.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.bytes_from_server") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.bytes_from_server") {
                        if let Some(val) = event.get("juniper.srx.bytes_from_server") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.bytes_from_server".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.bytes") };
            if _cond {
                event.set(
                    "server.bytes",
                    json!(
                        event
                            .get("destination.bytes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("server.bytes") {
                        if let Some(val) = event.get("server.bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("server.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.packets_from_server") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.packets_from_server") {
                        if let Some(val) = event.get("juniper.srx.packets_from_server") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.packets_from_server".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.packets", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.packets") };
            if _cond {
                event.set(
                    "server.packets",
                    json!(
                        event
                            .get("destination.packets")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.packets") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("server.packets") {
                        if let Some(val) = event.get("server.packets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.packets".into(),
                                    message,
                                }
                            })?;
                            event.set("server.packets", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.source_address") };
            if _cond {
                if event.has_value("juniper.srx.source_address") {
                    event.rename("juniper.srx.source_address", "source.ip")?;
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.set(
                    "client.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("juniper.srx.nat_source_address") };
            if _cond {
                if event.has_value("juniper.srx.nat_source_address") {
                    event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.sourceip") };
            if _cond {
                if event.has_value("juniper.srx.sourceip") {
                    event.rename("juniper.srx.sourceip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.source_port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.source_port") {
                        if let Some(val) = event.get("juniper.srx.source_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.source_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.port") };
            if _cond {
                event.set(
                    "client.port",
                    json!(
                        event
                            .get("source.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("client.port") {
                        if let Some(val) = event.get("client.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.port".into(),
                                    message,
                                }
                            })?;
                            event.set("client.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.nat_source_port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.nat_source_port") {
                        if let Some(val) = event.get("juniper.srx.nat_source_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.nat_source_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.nat.port") };
            if _cond {
                event.set(
                    "client.nat.port",
                    json!(
                        event
                            .get("source.nat.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.nat.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("client.nat.port") {
                        if let Some(val) = event.get("client.nat.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.nat.port".into(),
                                    message,
                                }
                            })?;
                            event.set("client.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.bytes_from_client") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.bytes_from_client") {
                        if let Some(val) = event.get("juniper.srx.bytes_from_client") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.bytes_from_client".into(),
                                    message,
                                }
                            })?;
                            event.set("source.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.bytes") };
            if _cond {
                event.set(
                    "client.bytes",
                    json!(
                        event
                            .get("source.bytes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("client.bytes") {
                        if let Some(val) = event.get("client.bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("client.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.packets_from_client") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.packets_from_client") {
                        if let Some(val) = event.get("juniper.srx.packets_from_client") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.packets_from_client".into(),
                                    message,
                                }
                            })?;
                            event.set("source.packets", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.packets") };
            if _cond {
                event.set(
                    "client.packets",
                    json!(
                        event
                            .get("source.packets")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.packets") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("client.packets") {
                        if let Some(val) = event.get("client.packets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.packets".into(),
                                    message,
                                }
                            })?;
                            event.set("client.packets", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.username") };
            if _cond {
                if event.has_value("juniper.srx.username") {
                    event.rename("juniper.srx.username", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.hostname") };
            if _cond {
                if event.has_value("juniper.srx.hostname") {
                    event.rename("juniper.srx.hostname", "source.domain")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.client_ip") };
            if _cond {
                if event.has_value("juniper.srx.client_ip") {
                    event.rename("juniper.srx.client_ip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.http_host") };
            if _cond {
                if event.has_value("juniper.srx.http_host") {
                    event.rename("juniper.srx.http_host", "url.domain")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.protocol_id") };
            if _cond {
                if event.has_value("juniper.srx.protocol_id") {
                    event.rename("juniper.srx.protocol_id", "network.iana_number")?;
                }
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

            let _cond = { !event.has_value("source.geo") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
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
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
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

            let _cond = { !event.has_value("source.as") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
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

            let _cond = { !event.has_value("destination.as") };
            if _cond {
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
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
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { event.has_value("juniper.srx.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("juniper.srx.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["EEE MMM dd HH:mm:ss yyyy", "EEE MMM  d HH:mm:ss yyyy"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("juniper.srx.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "juniper.srx.timestamp".into(),
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
                        "date_juniper_srx_timestamp_to_juniper_srx_timestamp_bc1c29bf",
                    )?;
                    if event.remove("juniper.srx.timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "juniper.srx.timestamp".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("juniper.srx.destination_port");
            event.remove("juniper.srx.nat_destination_port");
            event.remove("juniper.srx.bytes_from_client");
            event.remove("juniper.srx.packets_from_client");
            event.remove("juniper.srx.source_port");
            event.remove("juniper.srx.nat_source_port");
            event.remove("juniper.srx.bytes_from_server");
            event.remove("juniper.srx.packets_from_server");

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
