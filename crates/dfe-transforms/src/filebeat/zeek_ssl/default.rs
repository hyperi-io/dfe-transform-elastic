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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.ssl")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            event.append("event.type", json!("protocol"))?;

            event.set("network.transport", json!("tcp"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.ssl", "id.orig_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.ssl", "id.orig_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.ssl", "id.resp_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.ssl", "id.resp_p")?;
                Ok(())
            })();

            if event.has_value("zeek.ssl.id.orig_h") {
                event.rename("zeek.ssl.id.orig_h", "source.address")?;
            }

            if event.has_value("zeek.ssl.id.orig_p") {
                event.rename("zeek.ssl.id.orig_p", "source.port")?;
            }

            if event.has_value("zeek.ssl.id.resp_h") {
                event.rename("zeek.ssl.id.resp_h", "destination.address")?;
            }

            if event.has_value("zeek.ssl.id.resp_p") {
                event.rename("zeek.ssl.id.resp_p", "destination.port")?;
            }

            if event.has_value("zeek.ssl.uid") {
                event.rename("zeek.ssl.uid", "zeek.session_id")?;
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("client.address", v)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("server.address", v)?;
                }
            }

            if event.has_value("zeek.ssl.server_name") {
                event.rename("zeek.ssl.server_name", "zeek.ssl.server.name")?;
            }

            if event.has_value("zeek.ssl.cert_chain") {
                event.rename("zeek.ssl.cert_chain", "zeek.ssl.server.cert_chain")?;
            }

            if event.has_value("zeek.ssl.cert_chain_fuids") {
                event.rename(
                    "zeek.ssl.cert_chain_fuids",
                    "zeek.ssl.server.cert_chain_fuids",
                )?;
            }

            if event.has_value("zeek.ssl.client_cert_chain") {
                event.rename("zeek.ssl.client_cert_chain", "zeek.ssl.client.cert_chain")?;
            }

            if event.has_value("zeek.ssl.client_cert_chain_fuids") {
                event.rename(
                    "zeek.ssl.client_cert_chain_fuids",
                    "zeek.ssl.client.cert_chain_fuids",
                )?;
            }

            if event.has_value("zeek.ssl.validation_status") {
                event.rename("zeek.ssl.validation_status", "zeek.ssl.validation.status")?;
            }

            if event.has_value("zeek.ssl.validation_code") {
                event.rename("zeek.ssl.validation_code", "zeek.ssl.validation.code")?;
            }

            if let Some(date_str) = event.get_as_string("zeek.ssl.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.ssl.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.ssl.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.ssl.ts".into(),
                });
            }

            if event.has_value("zeek.ssl.not_valid_after") {
                event.rename("zeek.ssl.not_valid_after", "tls.server.not_after")?;
            }

            if event.has_value("zeek.ssl.not_valid_before") {
                event.rename("zeek.ssl.not_valid_before", "tls.server.not_before")?;
            }

            let _cond = { event.has_value("tls.server.not_before") };
            if _cond {
                if let Some(date_str) = event.get_as_string("tls.server.not_before") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("tls.server.not_before", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tls.server.not_before".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("tls.server.not_after") };
            if _cond {
                if let Some(date_str) = event.get_as_string("tls.server.not_after") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("tls.server.not_after", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tls.server.not_after".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
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

            let _cond = {
                event.get("zeek.ssl.client.cert_chain_fuids").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                event.remove("zeek.ssl.client.cert_chain_fuids");
            }

            if event.has_value("zeek.ssl.issuer") {
                gsub_field(
                    event,
                    "zeek.ssl.issuer",
                    "zeek.ssl.issuer",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.ssl.issuer") {
                if let Some(kv_str) = event.get_string("zeek.ssl.issuer") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "zeek.ssl.issuer".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("zeek.ssl.server.issuer.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("zeek.ssl.issuer") {
                event.rename("zeek.ssl.issuer", "tls.server.issuer")?;
            }

            if event.has_value("zeek.ssl.resp_certificate_sha1") {
                event.rename("zeek.ssl.resp_certificate_sha1", "tls.server.hash.sha1")?;
            }

            if event.has_value("tls.server.hash.sha1") {
                map_strings(
                    event,
                    "tls.server.hash.sha1",
                    "tls.server.hash.sha1",
                    str::to_uppercase,
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.issuer.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.issuer.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.issuer.C") {
                event.rename("zeek.ssl.server.issuer.C", "zeek.ssl.server.issuer.country")?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.issuer.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.issuer.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.issuer.CN") {
                event.rename(
                    "zeek.ssl.server.issuer.CN",
                    "zeek.ssl.server.issuer.common_name",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.issuer.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.issuer.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.issuer.L") {
                event.rename(
                    "zeek.ssl.server.issuer.L",
                    "zeek.ssl.server.issuer.locality",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.issuer.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.issuer.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.issuer.O") {
                event.rename(
                    "zeek.ssl.server.issuer.O",
                    "zeek.ssl.server.issuer.organization",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.issuer.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.issuer.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.issuer.OU") {
                event.rename(
                    "zeek.ssl.server.issuer.OU",
                    "zeek.ssl.server.issuer.organizational_unit",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.issuer.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.issuer.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.issuer.ST") {
                event.rename("zeek.ssl.server.issuer.ST", "zeek.ssl.server.issuer.state")?;
            }

            if event.has_value("zeek.ssl.subject") {
                gsub_field(
                    event,
                    "zeek.ssl.subject",
                    "zeek.ssl.subject",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.ssl.subject") {
                if let Some(kv_str) = event.get_string("zeek.ssl.subject") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "zeek.ssl.subject".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("zeek.ssl.server.subject.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("zeek.ssl.subject") {
                event.rename("zeek.ssl.subject", "tls.server.subject")?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.subject.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.subject.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.subject.C") {
                event.rename(
                    "zeek.ssl.server.subject.C",
                    "zeek.ssl.server.subject.country",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.subject.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.subject.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.subject.CN") {
                event.rename(
                    "zeek.ssl.server.subject.CN",
                    "zeek.ssl.server.subject.common_name",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.subject.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.subject.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.subject.L") {
                event.rename(
                    "zeek.ssl.server.subject.L",
                    "zeek.ssl.server.subject.locality",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.subject.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.subject.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.subject.O") {
                event.rename(
                    "zeek.ssl.server.subject.O",
                    "zeek.ssl.server.subject.organization",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.subject.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.subject.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.subject.OU") {
                event.rename(
                    "zeek.ssl.server.subject.OU",
                    "zeek.ssl.server.subject.organizational_unit",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.server.subject.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.server.subject.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.server.subject.ST") {
                event.rename(
                    "zeek.ssl.server.subject.ST",
                    "zeek.ssl.server.subject.state",
                )?;
            }

            if event.has_value("zeek.ssl.client_issuer") {
                gsub_field(
                    event,
                    "zeek.ssl.client_issuer",
                    "zeek.ssl.client_issuer",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.ssl.client_issuer") {
                if let Some(kv_str) = event.get_string("zeek.ssl.client_issuer") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "zeek.ssl.client_issuer".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("zeek.ssl.client.issuer.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("zeek.ssl.client_issuer") {
                event.rename("zeek.ssl.client_issuer", "tls.client.issuer")?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.issuer.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.issuer.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.issuer.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.issuer.C") {
                event.rename("zeek.ssl.client.issuer.C", "zeek.ssl.client.issuer.country")?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.issuer.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.issuer.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.issuer.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.issuer.CN") {
                event.rename(
                    "zeek.ssl.client.issuer.CN",
                    "zeek.ssl.client.issuer.common_name",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.issuer.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.issuer.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.issuer.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.issuer.L") {
                event.rename(
                    "zeek.ssl.client.issuer.L",
                    "zeek.ssl.client.issuer.locality",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.issuer.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.issuer.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.issuer.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.issuer.O") {
                event.rename(
                    "zeek.ssl.client.issuer.O",
                    "zeek.ssl.client.issuer.organization",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.issuer.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.issuer.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.issuer.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.issuer.subject.OU") {
                event.rename(
                    "zeek.ssl.issuer.subject.OU",
                    "zeek.ssl.issuer.subject.organizational_unit",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.issuer.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.issuer.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.issuer.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.issuer.ST") {
                event.rename("zeek.ssl.client.issuer.ST", "zeek.ssl.client.issuer.state")?;
            }

            if event.has_value("zeek.ssl.client_subject") {
                gsub_field(
                    event,
                    "zeek.ssl.client_subject",
                    "zeek.ssl.client_subject",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.ssl.client_subject") {
                if let Some(kv_str) = event.get_string("zeek.ssl.client_subject") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "zeek.ssl.client_subject".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("zeek.ssl.client.subject.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            event.remove("zeek.ssl.client_subject");

            let _cond = {
                event
                    .get("zeek.ssl.client.subject.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.subject.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.subject.C") {
                event.rename(
                    "zeek.ssl.client.subject.C",
                    "zeek.ssl.client.subject.country",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.subject.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.subject.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.subject.CN") {
                event.rename(
                    "zeek.ssl.client.subject.CN",
                    "zeek.ssl.client.subject.common_name",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.subject.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.subject.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.subject.L") {
                event.rename(
                    "zeek.ssl.client.subject.L",
                    "zeek.ssl.client.subject.locality",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.subject.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.subject.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.subject.O") {
                event.rename(
                    "zeek.ssl.client.subject.O",
                    "zeek.ssl.client.subject.organization",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.subject.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.subject.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.subject.OU") {
                event.rename(
                    "zeek.ssl.client.subject.OU",
                    "zeek.ssl.client.subject.organizational_unit",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.ssl.client.subject.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.ssl.client.subject.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.ssl.client.subject.ST") {
                event.rename(
                    "zeek.ssl.client.subject.ST",
                    "zeek.ssl.client.subject.state",
                )?;
            }

            let _cond = { event.has_value("zeek.ssl.cipher") };
            if _cond {
                event.set(
                    "tls.cipher",
                    json!(
                        event
                            .get("zeek.ssl.cipher")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zeek.ssl.curve") };
            if _cond {
                event.set(
                    "tls.curve",
                    json!(
                        event
                            .get("zeek.ssl.curve")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("zeek.ssl.established") {
                if let Some(val) = event.get("zeek.ssl.established") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "zeek.ssl.established".into(),
                            message,
                        }
                    })?;
                    event.set("tls.established", converted)?;
                }
            }

            if event.has_value("zeek.ssl.resumed") {
                if let Some(val) = event.get("zeek.ssl.resumed") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "zeek.ssl.resumed".into(),
                            message,
                        }
                    })?;
                    event.set("tls.resumed", converted)?;
                }
            }

            let _cond = { event.has_value("zeek.ssl.version") };
            if _cond {
                // Painless script
                // Source: def parts = ctx.zeek.ssl.version.splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} if (parts[0] == \"SSL\") {\n  ctx.tls.version = parts[1] + \".0\";\n} else {\n  ctx.tls.version = parts[1].substring(0,1) + \".\" + parts[1].substring(1);\n} ctx.tls.version_protocol = parts[0].toLowerCase();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parts = ctx.zeek.ssl.version.splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} if (parts[0] == \"SSL\") {\n  ctx.tls.version = parts[1] + \".0\";\n} else {\n  ctx.tls.version = parts[1].substring(0,1) + \".\" + parts[1].substring(1);\n} ctx.tls.version_protocol = parts[0].toLowerCase();"#
                    ),
                )?;
            }

            if event.has_value("zeek.ssl.ja3") {
                event.rename("zeek.ssl.ja3", "tls.client.ja3")?;
            }

            if event.has_value("zeek.ssl.ja3s") {
                event.rename("zeek.ssl.ja3s", "tls.server.ja3s")?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.server.ja3s") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("tls.server.ja3s")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.client.ja3") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("tls.client.ja3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Community ID v1 hash
            if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                event.get_string("source.ip"),
                event.get_string("destination.ip"),
                event
                    .get_as_string("network.iana_number")
                    .or_else(|| event.get_as_string("network.transport")),
            ) {
                let icmp = matches!(
                    protocol.to_ascii_lowercase().as_str(),
                    "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                );
                let (src_field, dst_field) = if icmp {
                    ("icmp.type", "icmp.code")
                } else {
                    ("source.port", "destination.port")
                };
                let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                    Ok(cid) => event.set("network.community_id", cid)?,
                    Err(message) => {
                        return Err(TransformError::ParseError {
                            path: "network.community_id".into(),
                            message,
                        });
                    }
                }
            }

            let _cond = {
                !event.has_value("zeek.ssl.client")
                    || event.get("zeek.ssl.client").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                event.remove("zeek.ssl.client");
            }

            event.remove("zeek.ssl.id");

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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
