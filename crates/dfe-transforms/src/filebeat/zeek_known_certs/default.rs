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

            parse_json_field(event, "event.original", "json")?;

            let _cond = { !event.has_value("json.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set(
                "event.category",
                Value::Array(vec![json!("network"), json!("file")]),
            )?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if let Some(date_str) = event.get_as_string("json.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("json.host") {
                event.rename("json.host", "host.ip")?;
            }

            let _cond = {
                event.get("host.ip").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                    serde_json::Value::String(s) => s.contains("."),
                    _ => false,
                })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                event.get("host.ip").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                    serde_json::Value::String(s) => s.contains(":"),
                    _ => false,
                })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("host.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "host.ip",
                    Value::Array(vec![json!(
                        event
                            .get("host.ip")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if let Some(v) = event
                .get("host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server", v)?;
            }

            if event.has_value("json.port_num") {
                event.rename("json.port_num", "server.port")?;
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

            if event.has_value("json.subject") {
                event.rename("json.subject", "tls.server.x509.subject.distinguished_name")?;
            }

            if event.has_value("json.issuer_subject") {
                event.rename(
                    "json.issuer_subject",
                    "tls.server.x509.issuer.distinguished_name",
                )?;
            }

            if event.has_value("json.serial") {
                event.rename("json.serial", "tls.server.x509.serial_number")?;
            }

            let _cond = {
                event.has_value("tls.server.x509.subject.distinguished_name")
                    && event
                        .get("tls.server.x509.subject.distinguished_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("CN="))
                            }
                            serde_json::Value::String(s) => s.contains("CN="),
                            _ => false,
                        })
            };
            if _cond {
                if event.has_value("tls.server.x509.subject.distinguished_name") {
                    if let Some(input) =
                        event.get_string("tls.server.x509.subject.distinguished_name")
                    {
                        // Grok pattern: CN=(?P<tls_server_x509_subject_common_name>(?:[^,]+))
                        let _ = cached_grok_mapped!(
                            "CN=(?P<tls_server_x509_subject_common_name>(?:[^,]+))",
                            [(
                                "tls_server_x509_subject_common_name",
                                "tls.server.x509.subject.common_name"
                            )]
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event
                    .get("tls.server.x509.subject.common_name")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("tls.server.x509.subject.common_name")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event.has_value("tls.server.x509.issuer.distinguished_name")
                    && event
                        .get("tls.server.x509.issuer.distinguished_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("CN="))
                            }
                            serde_json::Value::String(s) => s.contains("CN="),
                            _ => false,
                        })
            };
            if _cond {
                if event.has_value("tls.server.x509.issuer.distinguished_name") {
                    if let Some(input) =
                        event.get_string("tls.server.x509.issuer.distinguished_name")
                    {
                        // Grok pattern: CN=(?P<tls_server_x509_issuer_common_name>(?:[^,]+))
                        let _ = cached_grok_mapped!(
                            "CN=(?P<tls_server_x509_issuer_common_name>(?:[^,]+))",
                            [(
                                "tls_server_x509_issuer_common_name",
                                "tls.server.x509.issuer.common_name"
                            )]
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event
                    .get("tls.server.x509.issuer.common_name")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.issuer.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("tls.server.x509.issuer.common_name")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if let Some(v) = event
                .get("tls.server.x509.issuer.distinguished_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.issuer", v)?;
            }

            if let Some(v) = event
                .get("tls.server.x509.subject.distinguished_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.subject", v)?;
            }

            event.remove("json");

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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
