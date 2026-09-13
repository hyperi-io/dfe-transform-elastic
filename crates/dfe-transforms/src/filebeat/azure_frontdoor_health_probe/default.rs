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

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("connection")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("cloud.provider", json!("azure"))?;

            let _cond = {
                event.has_value("event.original")
                    && event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("\"records\""))
                        }
                        serde_json::Value::String(s) => s.contains("\"records\""),
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            parse_json_field(event, "event.original", "azure.frontdoor.health_probe")?;

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || v == 'n/a' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || v == 'n/a' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into(), "n/a".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            if event.has_value("azure.frontdoor.health_probe.resourceId") {
                event.rename(
                    "azure.frontdoor.health_probe.resourceId",
                    "azure.frontdoor.resource_id",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.operationName") {
                event.rename(
                    "azure.frontdoor.health_probe.operationName",
                    "azure.frontdoor.operation_name",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.category") {
                event.rename(
                    "azure.frontdoor.health_probe.category",
                    "azure.frontdoor.category",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.healthProbeId") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.healthProbeId",
                    "azure.frontdoor.health_probe.health_probe_id",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.pop") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.pop",
                    "azure.frontdoor.health_probe.pop",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.httpVerb") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.httpVerb",
                    "http.request.method",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.result") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.result",
                    "azure.frontdoor.health_probe.result",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.httpStatusCode") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.httpStatusCode",
                    "http.response.status_code",
                )?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.probeUrl") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.probeUrl",
                    "url.original",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("url.original") {
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

            if let Some(v) = event
                .get("url.original")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            if event.has_value("azure.frontdoor.health_probe.properties.originName") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.originName",
                    "azure.frontdoor.health_probe.origin_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("azure.frontdoor.health_probe.properties.totalLatencyMilliseconds")
                {
                    if let Some(val) = event
                        .get("azure.frontdoor.health_probe.properties.totalLatencyMilliseconds")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "azure.frontdoor.health_probe.properties.totalLatencyMilliseconds".into(),
                            message,
                        })?;
                        event.set(
                            "azure.frontdoor.health_probe.total_latency_milliseconds",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_latency")?;
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

            event.remove("azure.frontdoor.health_probe.properties.totalLatencyMilliseconds");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "azure.frontdoor.health_probe.properties.connectionLatencyMilliseconds",
                ) {
                    if let Some(val) = event.get(
                        "azure.frontdoor.health_probe.properties.connectionLatencyMilliseconds",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "azure.frontdoor.health_probe.properties.connectionLatencyMilliseconds".into(),
                            message,
                        })?;
                        event.set(
                            "azure.frontdoor.health_probe.connection_latency_milliseconds",
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
                    "convert_connection_latency",
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

            event.remove("azure.frontdoor.health_probe.properties.connectionLatencyMilliseconds");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.frontdoor.health_probe.properties.DNSLatencyMicroseconds")
                {
                    if let Some(val) =
                        event.get("azure.frontdoor.health_probe.properties.DNSLatencyMicroseconds")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "azure.frontdoor.health_probe.properties.DNSLatencyMicroseconds"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "azure.frontdoor.health_probe.dns_latency_microseconds",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dns_latency")?;
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

            event.remove("azure.frontdoor.health_probe.properties.DNSLatencyMicroseconds");

            if event.has_value("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

            if let Some(date_str) = event.get_as_string("azure.frontdoor.health_probe.time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "azure.frontdoor.health_probe.time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("azure.frontdoor.health_probe.properties.originIP") {
                event.rename(
                    "azure.frontdoor.health_probe.properties.originIP",
                    "destination.address",
                )?;
            }

            event.remove("azure.frontdoor.health_probe.time");
            event.remove("azure.frontdoor.health_probe.properties");

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: ^%{IPV4:destination.ip}$
                    // Grok pattern: ^%{IPV4:destination.ip}:(?P<destination_port>(?:[0-9]+))$
                    // Grok pattern: ^\\[%{IPV6:destination.ip}\\]:(?P<destination_port>(?:[0-9]+))$
                    // Grok pattern: ^(?P<destination_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<destination_port>(?:[0-9]+))$
                    // Grok pattern: ^%{IPV6:destination.ip}(?:(?: port |[p#.]))(?P<destination_port>(?:[0-9]+))$
                    // Grok pattern: ^(?P<destination_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}))$
                    // Grok pattern: ^%{IPV6:destination.ip}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{IPV4:destination.ip}$"),
                            cached_grok_mapped!(
                                "^%{IPV4:destination.ip}:(?P<destination_port>(?:[0-9]+))$",
                                [("destination_port", "destination.port:long")]
                            ),
                            cached_grok_mapped!(
                                "^\\[%{IPV6:destination.ip}\\]:(?P<destination_port>(?:[0-9]+))$",
                                [("destination_port", "destination.port:long")]
                            ),
                            cached_grok_mapped!(
                                "^(?P<destination_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<destination_port>(?:[0-9]+))$",
                                [
                                    ("destination_ip", "destination.ip"),
                                    ("destination_port", "destination.port:long")
                                ]
                            ),
                            cached_grok_mapped!(
                                "^%{IPV6:destination.ip}(?:(?: port |[p#.]))(?P<destination_port>(?:[0-9]+))$",
                                [("destination_port", "destination.port:long")]
                            ),
                            cached_grok_mapped!(
                                "^(?P<destination_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}))$",
                                [("destination_ip", "destination.ip")]
                            ),
                            cached_grok!("^%{IPV6:destination.ip}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("destination.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.ip".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_destination_ip")?;
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
                    if event.remove("destination.ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "destination.ip".into(),
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

            let _cond = { event.has_value("destination.ip") };
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

            let _cond = { event.has_value("destination.ip") };
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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
