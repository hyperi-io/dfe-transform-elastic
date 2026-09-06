// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `audit_lc` pipeline.
pub struct AuditLc;

impl Transform for AuditLc {
    fn name(&self) -> &str {
        "audit_lc"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                parse_json_field(event, "event.original", "lyve_cloud.audit")?;

                event.rename("lyve_cloud.audit.serviceAccountName", "user.name")?;

                event.rename("lyve_cloud.audit.serviceAccountCreatorId", "user.id")?;

            if let Some(v) = event.get("user.id").cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("log.file.path") && event.get("log.file.path").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("seagate.com")), serde_json::Value::String(s) => s.contains("seagate.com"), _ => false }) };
            if _cond {
            event.set("cloud.provider", json!("lyvecloud"))?;
            }

                if let Some(date_str) = event.get_as_string("lyve_cloud.audit.auditEntry.time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SS'Z'", "yyyy-MM-dd'T'HH:mm:ss.S'Z'", "yyyy-MM-dd'T'HH:mm:ss'Z'"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "lyve_cloud.audit.auditEntry.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

                if let Some(ua_str) = event.get_string("lyve_cloud.audit.auditEntry.userAgent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }

                event.rename("lyve_cloud.audit.auditEntry.api.statusCode", "http.response.status_code")?;

            if event.has_value("lyve_cloud.audit.auditEntry.api.timeToFirstByte") {
                gsub_field(event, "lyve_cloud.audit.auditEntry.api.timeToFirstByte", "lyve_cloud.audit.auditEntry.api.timeToFirstByte", cached_regex!("ns"), "")?;
            }

                gsub_field(event, "lyve_cloud.audit.auditEntry.api.timeToResponse", "lyve_cloud.audit.auditEntry.api.timeToResponse", cached_regex!("ns"), "")?;

            if event.has_value("lyve_cloud.audit.auditEntry.api.timeToFirstByte") {
                if let Some(val) = event.get("lyve_cloud.audit.auditEntry.api.timeToFirstByte") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "lyve_cloud.audit.auditEntry.api.timeToFirstByte".into(),
                            message,
                        })?;
                    event.set("lyve_cloud.audit.auditEntry.api.timeToFirstByte", converted)?;
                }
            }

            if event.has_value("lyve_cloud.audit.auditEntry.api.timeToResponse") {
                if let Some(val) = event.get("lyve_cloud.audit.auditEntry.api.timeToResponse") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "lyve_cloud.audit.auditEntry.api.timeToResponse".into(),
                            message,
                        })?;
                    event.set("lyve_cloud.audit.auditEntry.api.timeToResponse", converted)?;
                }
            }

            if event.has_value("lyve_cloud.audit.auditEntry.responseHeader.Content-Length") {
                if let Some(val) = event.get("lyve_cloud.audit.auditEntry.responseHeader.Content-Length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "lyve_cloud.audit.auditEntry.responseHeader.Content-Length".into(),
                            message,
                        })?;
                    event.set("lyve_cloud.audit.auditEntry.responseHeader.Content-Length", converted)?;
                }
            }

            if event.has_value("lyve_cloud.audit.auditEntry.requestHeader.Content-Length") {
                if let Some(val) = event.get("lyve_cloud.audit.auditEntry.requestHeader.Content-Length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "lyve_cloud.audit.auditEntry.requestHeader.Content-Length".into(),
                            message,
                        })?;
                    event.set("lyve_cloud.audit.auditEntry.requestHeader.Content-Length", converted)?;
                }
            }

                if event.has_value("lyve_cloud.audit.auditEntry.requestHeader.Content-Length") {
                    event.rename("lyve_cloud.audit.auditEntry.requestHeader.Content-Length", "http.request.body.bytes")?;
                }

                if event.has_value("lyve_cloud.audit.auditEntry.responseHeader.Content-Length") {
                    event.rename("lyve_cloud.audit.auditEntry.responseHeader.Content-Length", "http.response.body.bytes")?;
                }

                if event.has_value("lyve_cloud.audit.auditEntry.responseHeader.Content-Type") {
                    event.rename("lyve_cloud.audit.auditEntry.responseHeader.Content-Type", "http.response.mime_type")?;
                }

            if event.has_value("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Port") {
                if let Some(val) = event.get("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Port".into(),
                            message,
                        })?;
                    event.set("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Port", converted)?;
                }
            }

                if event.has_value("lyve_cloud.audit.auditEntry.responseHeader.X-Amz-Object-Lock-Retain-Until-Date") {
                    event.rename("lyve_cloud.audit.auditEntry.responseHeader.X-Amz-Object-Lock-Retain-Until-Date", "lyve_cloud.audit.auditEntry.responseHeader.object_lock_retain_until_date")?;
                }

            let _cond = { event.has_value("lyve_cloud.audit.auditEntry.responseHeader.ObjectLockRetainUntilDate") && event.get_str("lyve_cloud.audit.auditEntry.responseHeader.ObjectLockRetainUntilDate") != Some("") };
            if _cond {
                if let Some(date_str) = event.get_as_string("lyve_cloud.audit.auditEntry.responseHeader.ObjectLockRetainUntilDate") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'", "yyyy-MM-dd'T'HH:mm:ss.SS'Z'", "yyyy-MM-dd'T'HH:mm:ss.S'Z'", "yyyy-MM-dd'T'HH:mm:ss'Z'"], None, None) {
                        Some(parsed) => event.set("lyve_cloud.audit.auditEntry.responseHeader.ObjectLockRetainUntilDate", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "lyve_cloud.audit.auditEntry.responseHeader.ObjectLockRetainUntilDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("lyve_cloud.audit.auditEntry.requestHeader") && event.get_str("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Host") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Host").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-For") {
                if let Some(s) = event.get_string("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-For") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("related.ip", Value::Array(parts))?;
                }
            }

            if event.has_value("related.ip") {
                foreach_array(event, "related.ip", |event| {
                    map_strings(event, "_ingest._value", "_ingest._value", |s| s.trim().to_string())?;
                    Ok(())
                })?;
            }

            if event.has_value("related.ip") {
                foreach_array(event, "related.ip", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append("_failed_ips", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("related.ip") && event.has_value("_failed_ips") };
            if _cond {
                // Painless script
                // Source: ctx.related.ip.removeAll(ctx[\"_failed_ips\"]);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.related.ip.removeAll(ctx[\"_failed_ips\"]);"#))?;
            }

            let _cond = { event.has_value("related.ip") && event.get("related.ip").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) };
            if _cond {
                // Painless script
                // Source: ctx.client = new HashMap(); ctx.client[\"ip\"] = ctx.related.ip[-1];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.client = new HashMap(); ctx.client[\"ip\"] = ctx.related.ip[-1];"#))?;
            }

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

                if event.has_value("client.as.asn") {
                    event.rename("client.as.asn", "client.as.number")?;
                }

                if event.has_value("client.as.organization_name") {
                    event.rename("client.as.organization_name", "client.as.organization.name")?;
                }

            let _cond = { event.has_value("client") };
            if _cond {
            if let Some(v) = event.get("client").cloned() {
                event.set("source", v)?;
            }
            }

                event.remove("lyve_cloud.audit.auditEntry.time");
                event.remove("lyve_cloud.audit.auditEntry.userAgent");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.User-Agent");
                event.remove("lyve_cloud.audit.auditEntry.deploymentid");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.Authorization");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.X-Amz-Content-Sha256");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.X-Amz-Date");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.X-Forwarded-Proto");
                event.remove("lyve_cloud.audit.auditEntry.requestID");
                event.remove("lyve_cloud.audit.auditEntry.requestQuery");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.Content-Security-Policy");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.ETag");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.X-Amz-Request-Id");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.X-Xss-Protection");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.Vary");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.Content-Length");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.Content-Length");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.Accept-Encoding");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.Amz-Sdk-Request");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.Amz-Sdk-Invocation-Id");
                event.remove("lyve_cloud.audit.auditEntry.responseHeader.Date");
                event.remove("lyve_cloud.audit.auditEntry.requestHeader.Content-Md5");
                event.remove("_failed_ips");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
