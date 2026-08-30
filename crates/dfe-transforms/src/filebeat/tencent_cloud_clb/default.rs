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

            if let Some(v) = event.get("message").cloned() {
                event.set("originalMessage", v)?;
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("originalMessage") {
                    event.rename("originalMessage", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("originalMessage");
            }

            event.remove("message");

            parse_json_field(event, "event.original", "tencent_cloud.clb")?;

            if event.has_value("tencent_cloud.clb.bytes_sent") {
                event.rename("tencent_cloud.clb.bytes_sent", "http.response.body.bytes")?;
            }

            if event.has_value("tencent_cloud.clb.http_host") {
                event.rename("tencent_cloud.clb.http_host", "url.domain")?;
            }

            if event.has_value("tencent_cloud.clb.http_referer") {
                event.rename("tencent_cloud.clb.http_referer", "http.request.referrer")?;
            }

            if event.has_value("tencent_cloud.clb.http_user_agent") {
                event.rename("tencent_cloud.clb.http_user_agent", "user_agent.original")?;
            }

            if event.has_value("tencent_cloud.clb.http_x_forwarded_for") {
                event.rename(
                    "tencent_cloud.clb.http_x_forwarded_for",
                    "network.forwarded_ip",
                )?;
            }

            if event.has_value("tencent_cloud.clb.protocol_type") {
                event.rename("tencent_cloud.clb.protocol_type", "network.protocol")?;
            }

            if event.has_value("tencent_cloud.clb.remote_addr") {
                event.rename("tencent_cloud.clb.remote_addr", "source.ip")?;
            }

            if event.has_value("tencent_cloud.clb.remote_port") {
                event.rename("tencent_cloud.clb.remote_port", "source.port")?;
            }

            if event.has_value("tencent_cloud.clb.request_length") {
                event.rename(
                    "tencent_cloud.clb.request_length",
                    "http.request.body.bytes",
                )?;
            }

            if event.has_value("tencent_cloud.clb.request_method") {
                event.rename("tencent_cloud.clb.request_method", "http.request.method")?;
            }

            if event.has_value("tencent_cloud.clb.server_addr") {
                event.rename("tencent_cloud.clb.server_addr", "destination.ip")?;
            }

            if event.has_value("tencent_cloud.clb.server_name") {
                event.rename("tencent_cloud.clb.server_name", "server.domain")?;
            }

            if event.has_value("tencent_cloud.clb.server_port") {
                event.rename("tencent_cloud.clb.server_port", "destination.port")?;
            }

            if event.has_value("tencent_cloud.clb.server_protocol") {
                event.rename("tencent_cloud.clb.server_protocol", "http.version")?;
            }

            if event.has_value("tencent_cloud.clb.status") {
                event.rename("tencent_cloud.clb.status", "http.response.status_code")?;
            }

            if event.has_value("tencent_cloud.clb.stgw_request_id") {
                event.rename("tencent_cloud.clb.stgw_request_id", "event.id")?;
            }

            let _cond = { event.has_value("tencent_cloud.clb.time_local") };
            if _cond {
                if let Some(date_str) = event.get_as_string("tencent_cloud.clb.time_local") {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:HH:mm:ss Z"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tencent_cloud.clb.time_local".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("tencent_cloud.clb.upstream_addr") {
                event.rename("tencent_cloud.clb.upstream_addr", "destination.address")?;
            }

            if event.has_value("tencent_cloud.clb.upstream_connect_time") {
                event.rename("tencent_cloud.clb.upstream_connect_time", "event.duration")?;
            }

            if event.has_value("tencent_cloud.clb.uri") {
                event.rename("tencent_cloud.clb.uri", "url.path")?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            event.append_unique("event.category", json!("web"))?;

            event.append_unique("event.type", json!("access"))?;

            let _cond = { event.get_i64("http.response.status_code") == Some(200) };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = { event.get_i64("http.response.status_code") == Some(405) };
            if _cond {
                event.append_unique("event.type", json!("error"))?;
            }

            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("connection"))?;

            let _cond = {
                event.get_str("http.request.method") == Some("GET")
                    || event.get_str("http.request.method") == Some("POST")
            };
            if _cond {
                event.append_unique("event.category", json!("database"))?;
            }

            let _cond = { event.get_str("http.request.method") == Some("GET") };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

            let _cond = {
                event.get_str("http.request.method") == Some("POST")
                    || event.get_str("http.request.method") == Some("PUT")
                    || event.get_str("http.request.method") == Some("PATCH")
                    || event.get_str("http.request.method") == Some("DELETE")
            };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("http.request.method") == Some("HEAD") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
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

            let _cond = { event.has_value("network.forwarded_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("network.forwarded_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("server.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            event.remove("tencent_cloud.clb.__FILENAME__");
            event.remove("tencent_cloud.clb.__TIMESTAMP__");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("event.original");
                    Ok(())
                })();
            }

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
