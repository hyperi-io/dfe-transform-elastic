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

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("event.kind", json!("pipeline_error"))?;
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

            let _cond = {
                event.get("json.response").is_some_and(|v| v.is_array()) && event.get("json.response").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.txid") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = {
                event.has_value("json.isotimestamp")
                    && event.get_str("json.isotimestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.isotimestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("cisco_duo.auth.isotimestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.isotimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_isotimestamp")?;
                    event.remove("json.isotimestamp");
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

            event.set(
                "event.category",
                Value::Array(vec![json!("authentication")]),
            )?;

            event.set("event.kind", json!("event"))?;

            event.set("event.outcome", json!("failure"))?;

            let _cond = { event.get_str("json.result") == Some("success") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.reason").cloned() {
                    event.set("event.reason", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.access_device.ip") {
                    if let Some(input) = event.get_string("json.access_device.ip") {
                        // Grok pattern: ^%{IPV4:json.access_device.ip}:(?P<json_access_device_port>(?:[0-9]+))$
                        // Grok pattern: ^\\[%{IPV6:json.access_device.ip}\\]:(?P<json_access_device_port>(?:[0-9]+))$
                        // Grok pattern: ^(?P<json_access_device_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<json_access_device_port>(?:[0-9]+))$
                        // Grok pattern: ^%{IPV6:json.access_device.ip}(?:(?: port |[p#.]))(?P<json_access_device_port>(?:[0-9]+))$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^%{IPV4:json.access_device.ip}:(?P<json_access_device_port>(?:[0-9]+))$",
                                    [("json_access_device_port", "json.access_device.port")]
                                ),
                                cached_grok_mapped!(
                                    "^\\[%{IPV6:json.access_device.ip}\\]:(?P<json_access_device_port>(?:[0-9]+))$",
                                    [("json_access_device_port", "json.access_device.port")]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<json_access_device_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<json_access_device_port>(?:[0-9]+))$",
                                    [
                                        ("json_access_device_ip", "json.access_device.ip"),
                                        ("json_access_device_port", "json.access_device.port")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^%{IPV6:json.access_device.ip}(?:(?: port |[p#.]))(?P<json_access_device_port>(?:[0-9]+))$",
                                    [("json_access_device_port", "json.access_device.port")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.access_device.ip") {
                    if let Some(val) = event.get("json.access_device.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.access_device.ip".into(),
                                message,
                            }
                        })?;
                        event.set("json.access_device.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.access_device.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.access_device.ip".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.access_device.port") {
                    if let Some(val) = event.get("json.access_device.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.access_device.port".into(),
                                message,
                            }
                        })?;
                        event.set("json.access_device.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.access_device.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.access_device.port".into(),
                    });
                }
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

            if event.has_value("json.access_device.ip") {
                event.rename("json.access_device.ip", "cisco_duo.auth.access_device.ip")?;
            }

            if event.has_value("json.access_device.port") {
                event.rename(
                    "json.access_device.port",
                    "cisco_duo.auth.access_device.port",
                )?;
            }

            if event.has_value("cisco_duo.auth.access_device.ip") {
                if let Some(ip_str) = event.get_string("cisco_duo.auth.access_device.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set(
                                "cisco_duo.auth.access_device.geo.country_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event
                                .set("cisco_duo.auth.access_device.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set(
                                "cisco_duo.auth.access_device.geo.continent_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set(
                                "cisco_duo.auth.access_device.geo.region_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("cisco_duo.auth.access_device.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("cisco_duo.auth.access_device.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("cisco_duo.auth.access_device.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("cisco_duo.auth.access_device.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("cisco_duo.auth.access_device.ip") {
                if let Some(ip_str) = event.get_string("cisco_duo.auth.access_device.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("cisco_duo.auth.access_device.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set(
                                "cisco_duo.auth.access_device.as.organization_name",
                                v.clone(),
                            )?;
                        }
                    }
                }
            }

            if event.has_value("cisco_duo.auth.access_device.as.asn") {
                event.rename(
                    "cisco_duo.auth.access_device.as.asn",
                    "cisco_duo.auth.access_device.as.number",
                )?;
            }

            if event.has_value("cisco_duo.auth.access_device.as.organization_name") {
                event.rename(
                    "cisco_duo.auth.access_device.as.organization_name",
                    "cisco_duo.auth.access_device.as.organization.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("cisco_duo.auth.access_device.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("cisco_duo.auth.access_device.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("cisco_duo.auth.access_device.as")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.as", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("cisco_duo.auth.access_device.geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.geo", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.auth_device.ip") {
                    if let Some(input) = event.get_string("json.auth_device.ip") {
                        // Grok pattern: ^%{IPV4:json.auth_device.ip}:(?P<json_auth_device_port>(?:[0-9]+))$
                        // Grok pattern: ^\\[%{IPV6:json.auth_device.ip}\\]:(?P<json_auth_device_port>(?:[0-9]+))$
                        // Grok pattern: ^(?P<json_auth_device_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<json_auth_device_port>(?:[0-9]+))$
                        // Grok pattern: ^%{IPV6:json.auth_device.ip}(?:(?: port |[p#.]))(?P<json_auth_device_port>(?:[0-9]+))$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^%{IPV4:json.auth_device.ip}:(?P<json_auth_device_port>(?:[0-9]+))$",
                                    [("json_auth_device_port", "json.auth_device.port")]
                                ),
                                cached_grok_mapped!(
                                    "^\\[%{IPV6:json.auth_device.ip}\\]:(?P<json_auth_device_port>(?:[0-9]+))$",
                                    [("json_auth_device_port", "json.auth_device.port")]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<json_auth_device_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<json_auth_device_port>(?:[0-9]+))$",
                                    [
                                        ("json_auth_device_ip", "json.auth_device.ip"),
                                        ("json_auth_device_port", "json.auth_device.port")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^%{IPV6:json.auth_device.ip}(?:(?: port |[p#.]))(?P<json_auth_device_port>(?:[0-9]+))$",
                                    [("json_auth_device_port", "json.auth_device.port")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.auth_device.ip") {
                    if let Some(val) = event.get("json.auth_device.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.auth_device.ip".into(),
                                message,
                            }
                        })?;
                        event.set("json.auth_device.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.auth_device.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.auth_device.ip".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.auth_device.port") {
                    if let Some(val) = event.get("json.auth_device.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.auth_device.port".into(),
                                message,
                            }
                        })?;
                        event.set("json.auth_device.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.auth_device.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.auth_device.port".into(),
                    });
                }
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

            let _cond = { event.has_value("json.access_device.hostname") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.access_device.hostname").cloned() {
                        event.set("source.address", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.email").cloned() {
                    event.set("source.user.email", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.user.key").cloned() {
                    event.set("source.user.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.user.name").cloned() {
                    event.set("source.user.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.user.groups") {
                event.rename("json.user.groups", "source.user.group.name")?;
            }

            if event.has_value("json.auth_device.ip") {
                if let Some(ip_str) = event.get_string("json.auth_device.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set(
                                "cisco_duo.auth.auth_device.geo.country_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("cisco_duo.auth.auth_device.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event
                                .set("cisco_duo.auth.auth_device.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event
                                .set("cisco_duo.auth.auth_device.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("cisco_duo.auth.auth_device.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("cisco_duo.auth.auth_device.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("cisco_duo.auth.auth_device.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("cisco_duo.auth.auth_device.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("json.auth_device.ip") {
                if let Some(ip_str) = event.get_string("json.auth_device.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("cisco_duo.auth.auth_device.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set(
                                "cisco_duo.auth.auth_device.as.organization_name",
                                v.clone(),
                            )?;
                        }
                    }
                }
            }

            if event.has_value("cisco_duo.auth.auth_device.as.asn") {
                event.rename(
                    "cisco_duo.auth.auth_device.as.asn",
                    "cisco_duo.auth.auth_device.as.number",
                )?;
            }

            if event.has_value("cisco_duo.auth.auth_device.as.organization_name") {
                event.rename(
                    "cisco_duo.auth.auth_device.as.organization_name",
                    "cisco_duo.auth.auth_device.as.organization.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.email").cloned() {
                    event.set("user.email", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.user.name").cloned() {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.user.key").cloned() {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.access_device.browser").cloned() {
                    event.set("user_agent.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.access_device.browser_version").cloned() {
                    event.set("user_agent.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.access_device.os").cloned() {
                    event.set("user_agent.os.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.access_device.os_version").cloned() {
                    event.set("user_agent.os.version", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.email") {
                event.rename("json.email", "cisco_duo.auth.email")?;
            }

            if event.has_value("json.event_type") {
                event.rename("json.event_type", "cisco_duo.auth.event_type")?;
            }

            if event.has_value("json.factor") {
                event.rename("json.factor", "cisco_duo.auth.factor")?;
            }

            if event.has_value("json.ood_software") {
                event.rename("json.ood_software", "cisco_duo.auth.ood_software")?;
            }

            if event.has_value("json.reason") {
                event.rename("json.reason", "cisco_duo.auth.reason")?;
            }

            if event.has_value("json.result") {
                event.rename("json.result", "cisco_duo.auth.result")?;
            }

            if event.has_value("json.txid") {
                event.rename("json.txid", "cisco_duo.auth.txid")?;
            }

            if event.has_value("json.alias") {
                event.rename("json.alias", "cisco_duo.auth.alias")?;
            }

            if event.has_value("json.access_device.epkey") {
                event.rename(
                    "json.access_device.epkey",
                    "cisco_duo.auth.access_device.epkey",
                )?;
            }

            if event.has_value("json.access_device.flash_version") {
                event.rename(
                    "json.access_device.flash_version",
                    "cisco_duo.auth.access_device.flash_version",
                )?;
            }

            let _cond = { event.has_value("json.access_device.hostname") };
            if _cond {
                event.rename(
                    "json.access_device.hostname",
                    "cisco_duo.auth.access_device.hostname",
                )?;
            }

            if event.has_value("json.access_device.is_encryption_enabled") {
                event.rename(
                    "json.access_device.is_encryption_enabled",
                    "cisco_duo.auth.access_device.is_encryption_enabled",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_duo.auth.access_device.is_encryption_enabled") {
                    if let Some(val) =
                        event.get("cisco_duo.auth.access_device.is_encryption_enabled")
                    {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_duo.auth.access_device.is_encryption_enabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cisco_duo.auth.access_device.is_encryption_enabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.access_device.is_firewall_enabled") {
                event.rename(
                    "json.access_device.is_firewall_enabled",
                    "cisco_duo.auth.access_device.is_firewall_enabled",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_duo.auth.access_device.is_firewall_enabled") {
                    if let Some(val) = event.get("cisco_duo.auth.access_device.is_firewall_enabled")
                    {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_duo.auth.access_device.is_firewall_enabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cisco_duo.auth.access_device.is_firewall_enabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.access_device.is_password_set") {
                event.rename(
                    "json.access_device.is_password_set",
                    "cisco_duo.auth.access_device.is_password_set",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_duo.auth.access_device.is_password_set") {
                    if let Some(val) = event.get("cisco_duo.auth.access_device.is_password_set") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_duo.auth.access_device.is_password_set".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_duo.auth.access_device.is_password_set", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.access_device.java_version") {
                event.rename(
                    "json.access_device.java_version",
                    "cisco_duo.auth.access_device.java_version",
                )?;
            }

            if event.has_value("json.access_device.location.city") {
                event.rename(
                    "json.access_device.location.city",
                    "cisco_duo.auth.access_device.location.city",
                )?;
            }

            if event.has_value("json.access_device.location.country") {
                event.rename(
                    "json.access_device.location.country",
                    "cisco_duo.auth.access_device.location.country",
                )?;
            }

            if event.has_value("json.access_device.location.state") {
                event.rename(
                    "json.access_device.location.state",
                    "cisco_duo.auth.access_device.location.state",
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.json?.access_device?.security_agents != null && ( !(ctx.json.access_device.security_agents instanceof List) || ctx.json.access_device.security_agents.length == 0 || !(ctx.json.access_device.securi ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.remove("json.access_device.security_agents").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.access_device.security_agents".into(),
                    });
                }
            }

            if event.has_value("json.access_device.security_agents") {
                event.rename(
                    "json.access_device.security_agents",
                    "cisco_duo.auth.access_device.security_agents",
                )?;
            }

            if event.has_value("json.application.destination_name") {
                event.rename(
                    "json.application.destination_name",
                    "cisco_duo.auth.application.destination_name",
                )?;
            }

            if event.has_value("json.application.key") {
                event.rename("json.application.key", "cisco_duo.auth.application.key")?;
            }

            if event.has_value("json.application.name") {
                event.rename("json.application.name", "cisco_duo.auth.application.name")?;
            }

            if event.has_value("json.auth_device.ip") {
                event.rename("json.auth_device.ip", "cisco_duo.auth.auth_device.ip")?;
            }

            if event.has_value("json.auth_device.key") {
                event.rename("json.auth_device.key", "cisco_duo.auth.auth_device.key")?;
            }

            if event.has_value("json.auth_device.port") {
                event.rename("json.auth_device.port", "cisco_duo.auth.auth_device.port")?;
            }

            if event.has_value("json.auth_device.location.city") {
                event.rename(
                    "json.auth_device.location.city",
                    "cisco_duo.auth.auth_device.location.city",
                )?;
            }

            if event.has_value("json.auth_device.location.country") {
                event.rename(
                    "json.auth_device.location.country",
                    "cisco_duo.auth.auth_device.location.country",
                )?;
            }

            if event.has_value("json.auth_device.location.state") {
                event.rename(
                    "json.auth_device.location.state",
                    "cisco_duo.auth.auth_device.location.state",
                )?;
            }

            if event.has_value("json.auth_device.name") {
                event.rename("json.auth_device.name", "cisco_duo.auth.auth_device.name")?;
            }

            if event.has_value("json.trusted_endpoint_status") {
                event.rename(
                    "json.trusted_endpoint_status",
                    "cisco_duo.auth.trusted_endpoint_status",
                )?;
            }

            if event.has_value("json.trusted_session_info") {
                event.rename(
                    "json.trusted_session_info",
                    "cisco_duo.auth.trusted_session_info",
                )?;
            }

            if event.has_value("json.adaptive_trust_assessments") {
                event.rename(
                    "json.adaptive_trust_assessments",
                    "cisco_duo.auth.adaptive_trust_assessments",
                )?;
            }

            if event.has_value(
                "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.features_version",
            ) {
                if let Some(val) = event.get(
                    "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.features_version",
                ) {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.features_version".into(),
                            message,
                        })?;
                    event.set("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.features_version", converted)?;
                }
            }

            if event
                .has_value("cisco_duo.auth.adaptive_trust_assessments.remember_me.features_version")
            {
                if let Some(val) = event
                    .get("cisco_duo.auth.adaptive_trust_assessments.remember_me.features_version")
                {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.remember_me.features_version".into(),
                            message,
                        })?;
                    event.set(
                        "cisco_duo.auth.adaptive_trust_assessments.remember_me.features_version",
                        converted,
                    )?;
                }
            }

            if event.has_value(
                "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.model_version",
            ) {
                if let Some(val) = event
                    .get("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.model_version")
                {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.model_version".into(),
                            message,
                        })?;
                    event.set(
                        "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.model_version",
                        converted,
                    )?;
                }
            }

            if event
                .has_value("cisco_duo.auth.adaptive_trust_assessments.remember_me.model_version")
            {
                if let Some(val) =
                    event.get("cisco_duo.auth.adaptive_trust_assessments.remember_me.model_version")
                {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.remember_me.model_version".into(),
                            message,
                        })?;
                    event.set(
                        "cisco_duo.auth.adaptive_trust_assessments.remember_me.model_version",
                        converted,
                    )?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.policy_enabled",
                ) {
                    if let Some(val) = event.get(
                        "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.policy_enabled",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.policy_enabled".into(),
                            message,
                        })?;
                        event.set("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.policy_enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_adaptive_trust_assessments_more_secure_auth_policy_enabled_to_boolean",
                )?;
                if event
                    .remove(
                        "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.policy_enabled",
                    )
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound { path: "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.policy_enabled".into() });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cisco_duo.auth.adaptive_trust_assessments.remember_me.policy_enabled",
                ) {
                    if let Some(val) = event
                        .get("cisco_duo.auth.adaptive_trust_assessments.remember_me.policy_enabled")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.remember_me.policy_enabled".into(),
                            message,
                        })?;
                        event.set(
                            "cisco_duo.auth.adaptive_trust_assessments.remember_me.policy_enabled",
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
                    "convert_adaptive_trust_assessments_remember_me_policy_enabled_to_boolean",
                )?;
                if event
                    .remove("cisco_duo.auth.adaptive_trust_assessments.remember_me.policy_enabled")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path:
                            "cisco_duo.auth.adaptive_trust_assessments.remember_me.policy_enabled"
                                .into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.preview_mode_enabled") {
                if let Some(val) = event.get("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.preview_mode_enabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.preview_mode_enabled".into(),
                            message,
                        })?;
                    event.set("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.preview_mode_enabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_adaptive_trust_assessments_more_secure_auth_preview_mode_enabled_to_boolean")?;
                if event.remove("cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.preview_mode_enabled").is_none() {
                            return Err(TransformError::FieldNotFound { path: "cisco_duo.auth.adaptive_trust_assessments.more_secure_auth.preview_mode_enabled".into() });
                        }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cisco_duo.auth.adaptive_trust_assessments.remember_me.preview_mode_enabled",
                ) {
                    if let Some(val) = event.get("cisco_duo.auth.adaptive_trust_assessments.remember_me.preview_mode_enabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_duo.auth.adaptive_trust_assessments.remember_me.preview_mode_enabled".into(),
                            message,
                        })?;
                    event.set("cisco_duo.auth.adaptive_trust_assessments.remember_me.preview_mode_enabled", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_adaptive_trust_assessments_remember_me_preview_mode_enabled_to_boolean")?;
                if event.remove("cisco_duo.auth.adaptive_trust_assessments.remember_me.preview_mode_enabled").is_none() {
                            return Err(TransformError::FieldNotFound { path: "cisco_duo.auth.adaptive_trust_assessments.remember_me.preview_mode_enabled".into() });
                        }
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

            if event.has_value("json.passport_assessment") {
                event.rename(
                    "json.passport_assessment",
                    "cisco_duo.auth.passport_assessment",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_duo.auth.passport_assessment.is_supported") {
                    if let Some(val) = event.get("cisco_duo.auth.passport_assessment.is_supported")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_duo.auth.passport_assessment.is_supported".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_duo.auth.passport_assessment.is_supported", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_passport_assessment_is_supported_to_boolean",
                )?;
                if event
                    .remove("cisco_duo.auth.passport_assessment.is_supported")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "cisco_duo.auth.passport_assessment.is_supported".into(),
                    });
                }
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

            if event.has_value("json.rbfs_triggered_attacks") {
                event.rename(
                    "json.rbfs_triggered_attacks",
                    "cisco_duo.auth.rbfs_triggered_attacks",
                )?;
            }

            let _cond = {
                event
                    .get("cisco_duo.auth.rbfs_triggered_attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cisco_duo.auth.rbfs_triggered_attacks").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                if event.has_value("_ingest._value.detected_occurrences") {
                                    if let Some(val) =
                                        event.get("_ingest._value.detected_occurrences")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.detected_occurrences"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.detected_occurrences",
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
                                    "convert_rbfs_triggered_attacks_detected_occurrences_to_long",
                                )?;
                                if event
                                    .remove("_ingest._value.detected_occurrences")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.detected_occurrences".into(),
                                    });
                                }
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
                            "cisco_duo.auth.rbfs_triggered_attacks",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cisco_duo.auth.rbfs_triggered_attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cisco_duo.auth.rbfs_triggered_attacks").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                if event.has_value("_ingest._value.shadow_mode") {
                                    if let Some(val) = event.get("_ingest._value.shadow_mode") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.shadow_mode".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.shadow_mode", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_rbfs_triggered_attacks_shadow_mode_to_boolean",
                                )?;
                                if event.remove("_ingest._value.shadow_mode").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.shadow_mode".into(),
                                    });
                                }
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
                            "cisco_duo.auth.rbfs_triggered_attacks",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cisco_duo.auth.rbfs_triggered_attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cisco_duo.auth.rbfs_triggered_attacks").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                if event.has_value("_ingest._value.threshold_occurrences") {
                                    if let Some(val) =
                                        event.get("_ingest._value.threshold_occurrences")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.threshold_occurrences"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.threshold_occurrences",
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
                                    "convert_rbfs_triggered_attacks_threshold_occurrences_to_long",
                                )?;
                                if event
                                    .remove("_ingest._value.threshold_occurrences")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.threshold_occurrences".into(),
                                    });
                                }
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
                            "cisco_duo.auth.rbfs_triggered_attacks",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cisco_duo.auth.rbfs_triggered_attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cisco_duo.auth.rbfs_triggered_attacks").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                if event.has_value("_ingest._value.threshold_time_frame") {
                                    if let Some(val) =
                                        event.get("_ingest._value.threshold_time_frame")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.threshold_time_frame"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.threshold_time_frame",
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
                                    "convert_rbfs_triggered_attacks_threshold_time_frame_to_long",
                                )?;
                                if event
                                    .remove("_ingest._value.threshold_time_frame")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.threshold_time_frame".into(),
                                    });
                                }
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
                            "cisco_duo.auth.rbfs_triggered_attacks",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cisco_duo.auth.rbfs_triggered_attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cisco_duo.auth.rbfs_triggered_attacks").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                if event.has_value("_ingest._value.unrealistic_travel_velocity") {
                                    if let Some(val) =
                                        event.get("_ingest._value.unrealistic_travel_velocity")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path:
                                                        "_ingest._value.unrealistic_travel_velocity"
                                                            .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.unrealistic_travel_velocity",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_rbfs_triggered_attacks_unrealistic_travel_velocity_to_long")?;
                                if event
                                    .remove("_ingest._value.unrealistic_travel_velocity")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.unrealistic_travel_velocity".into(),
                                    });
                                }
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
                            "cisco_duo.auth.rbfs_triggered_attacks",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            let _cond = { event.has_value("cisco_duo.auth.auth_device.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("cisco_duo.auth.auth_device.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
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
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("cisco_duo.auth.access_device.hostname") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("cisco_duo.auth.access_device.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
