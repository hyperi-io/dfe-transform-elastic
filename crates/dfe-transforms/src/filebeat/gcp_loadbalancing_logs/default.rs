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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(s) = event.get_string("event.original") {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "event.original".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("json", parsed)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("info"))?;

            if let Some(date_str) = event.get_as_string("json.timestamp") {
                if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    event.set("@timestamp", parsed)?;
                }
            }

            if let Some(date_str) = event.get_as_string("json.receiveTimestamp") {
                if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    event.set("event.created", parsed)?;
                }
            }

            if event.has("json.logName") {
                event.rename("json.logName", "log.logger")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.insertId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.resource.labels.project_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.resource.labels.project_id".into(),
                            message,
                        }
                    })?;
                    event.set("cloud.project.id", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.resource.labels.zone") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.resource.labels.zone".into(),
                            message,
                        }
                    })?;
                    event.set("cloud.region", converted)?;
                }
                Ok(())
            })();

            if event.has_value("json.httpRequest.remoteIp") {
                if let Some(input) = event.get_string("json.httpRequest.remoteIp") {
                    // Grok pattern: ^%{IP:source.address}(:%{POSINT:source.port:long})?$
                    if !cached_grok!("^%{IP:source.address}(:%{POSINT:source.port:long})?$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();

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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has("json.httpRequest.requestMethod") {
                event.rename("json.httpRequest.requestMethod", "http.request.method")?;
            }

            if event.has_value("json.httpRequest.requestSize") {
                if let Some(val) = event.get("json.httpRequest.requestSize") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.httpRequest.requestSize".into(),
                            message,
                        }
                    })?;
                    event.set("http.request.bytes", converted)?;
                }
            }

            if event.has_value("json.httpRequest.responseSize") {
                if let Some(val) = event.get("json.httpRequest.responseSize") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.httpRequest.responseSize".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.bytes", converted)?;
                }
            }

            if event.has("json.httpRequest.status") {
                event.rename("json.httpRequest.status", "http.response.status_code")?;
            }

            let _cond = { event.has_value("json.httpRequest.protocol") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("json.httpRequest.protocol") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("network.protocol", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("http.version", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            if event.has_value("json.httpRequest.userAgent") {
                if let Some(ua_str) = event.get_string("json.httpRequest.userAgent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.httpRequest.requestUrl") };
            if _cond {
                uri_parts(event, "json.httpRequest.requestUrl", "url", true, false)?;
            }

            if event.has("json.httpRequest.referer") {
                event.rename("json.httpRequest.referer", "http.request.referrer")?;
            }

            if event.has_value("json.httpRequest.serverIp") {
                if let Some(input) = event.get_string("json.httpRequest.serverIp") {
                    // Grok pattern: ^%{IP:destination.nat.ip}(:%{POSINT:destination.nat.port:long})?$
                    if !cached_grok!(
                        "^%{IP:destination.nat.ip}(:%{POSINT:destination.nat.port:long})?$"
                    )
                    .extract_into(&input, event)?
                    {}
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("url.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("url.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.port", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("url.domain") {
                        event.rename("url.domain", "destination.domain")?;
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.severity") {
                event.rename("json.severity", "log.level")?;
            }

            if event.has("json.jsonPayload.cacheId") {
                event.rename("json.jsonPayload.cacheId", "gcp.load_balancer.cache_id")?;
            }

            if event.has("json.jsonPayload.statusDetails") {
                event.rename(
                    "json.jsonPayload.statusDetails",
                    "gcp.load_balancer.status_details",
                )?;
            }

            if event.has("json.httpRequest.cacheHit") {
                event.rename("json.httpRequest.cacheHit", "gcp.load_balancer.cache_hit")?;
            }

            if event.has("json.httpRequest.cacheLookup") {
                event.rename(
                    "json.httpRequest.cacheLookup",
                    "gcp.load_balancer.cache_lookup",
                )?;
            }

            if event.has("json.resource.labels.url_map_name") {
                event.rename(
                    "json.resource.labels.url_map_name",
                    "gcp.load_balancer.url_map_name",
                )?;
            }

            if event.has("json.resource.labels.forwarding_rule_name") {
                event.rename(
                    "json.resource.labels.forwarding_rule_name",
                    "gcp.load_balancer.forwarding_rule_name",
                )?;
            }

            if event.has("json.resource.labels.target_proxy_name") {
                event.rename(
                    "json.resource.labels.target_proxy_name",
                    "gcp.load_balancer.target_proxy_name",
                )?;
            }

            if event.has("json.resource.labels.backend_service_name") {
                event.rename(
                    "json.resource.labels.backend_service_name",
                    "gcp.load_balancer.backend_service_name",
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            event.remove("json");

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
