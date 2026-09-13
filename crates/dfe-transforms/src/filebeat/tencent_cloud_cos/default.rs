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
            event.set("ecs.version", json!("8.17.0"))?;

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

            parse_json_field(event, "event.original", "tencent_cloud.cos")?;

            let _cond = { event.has_value("tencent_cloud.cos.__TIMESTAMP__") };
            if _cond {
                if let Some(date_str) = event.get_as_string("tencent_cloud.cos.__TIMESTAMP__") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tencent_cloud.cos.__TIMESTAMP__".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("tencent_cloud.cos.accountId") {
                if let Some(val) = event.get("tencent_cloud.cos.accountId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "tencent_cloud.cos.accountId".into(),
                            message,
                        }
                    })?;
                    event.set("cloud.account.id", converted)?;
                }
            }

            if event.has_value("tencent_cloud.cos.eventVersion") {
                if let Some(val) = event.get("tencent_cloud.cos.eventVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "tencent_cloud.cos.eventVersion".into(),
                            message,
                        }
                    })?;
                    event.set("tencent_cloud.cos.eventVersion", converted)?;
                }
            }

            if event.has_value("tencent_cloud.cos.eventName") {
                event.rename("tencent_cloud.cos.eventName", "event.action")?;
            }

            if event.has_value("tencent_cloud.cos.eventSource") {
                event.rename("tencent_cloud.cos.eventSource", "event.provider")?;
            }

            let _cond = { event.has_value("tencent_cloud.cos.eventTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("tencent_cloud.cos.eventTime") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss'Z'", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tencent_cloud.cos.eventTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("tencent_cloud.cos.referer") {
                event.rename("tencent_cloud.cos.referer", "http.request.referrer")?;
            }

            if event.has_value("tencent_cloud.cos.remoteIp") {
                event.rename("tencent_cloud.cos.remoteIp", "source.ip")?;
            }

            if event.has_value("tencent_cloud.cos.reqBytesSent") {
                event.rename("tencent_cloud.cos.reqBytesSent", "http.request.body.bytes")?;
            }

            if event.has_value("tencent_cloud.cos.reqMethod") {
                event.rename("tencent_cloud.cos.reqMethod", "http.request.method")?;
            }

            if event.has_value("tencent_cloud.cos.reqPath") {
                event.rename("tencent_cloud.cos.reqPath", "url.path")?;
            }

            if event.has_value("tencent_cloud.cos.requestId") {
                event.rename("tencent_cloud.cos.requestId", "event.id")?;
            }

            if event.has_value("tencent_cloud.cos.requestUri") {
                event.rename("tencent_cloud.cos.requestUri", "url.original")?;
            }

            if event.has_value("tencent_cloud.cos.requester") {
                event.rename("tencent_cloud.cos.requester", "user.id")?;
            }

            if event.has_value("tencent_cloud.cos.resBytesSent") {
                event.rename("tencent_cloud.cos.resBytesSent", "http.response.body.bytes")?;
            }

            if event.has_value("tencent_cloud.cos.resHttpCode") {
                event.rename("tencent_cloud.cos.resHttpCode", "http.response.status_code")?;
            }

            if event.has_value("tencent_cloud.cos.resTotalTime") {
                event.rename("tencent_cloud.cos.resTotalTime", "event.duration")?;
            }

            if event.has_value("tencent_cloud.cos.userAgent") {
                event.rename("tencent_cloud.cos.userAgent", "user_agent.original")?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\" || object == \"-\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["-".into()],
                    ..DropPolicy::none()
                },
                None,
            );

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

            event.append_unique("event.category", json!("database"))?;

            event.append_unique("event.type", json!("access"))?;

            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("connection"))?;

            event.append_unique("event.category", json!("file"))?;

            event.append_unique("event.type", json!("info"))?;

            let _cond = { event.get_str("http.request.method") == Some("GET") };
            if _cond {
                event.append_unique("event.category", json!("web"))?;
            }

            let _cond = { event.get_str("http.request.method") == Some("GET") };
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

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("tencent_cloud.cos.accountId");
            event.remove("tencent_cloud.cos.__TIMESTAMP__");

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
