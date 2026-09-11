// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `http` pipeline.
pub struct Http;

impl Transform for Http {
    fn name(&self) -> &str {
        "http"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("json.EdgeStartTimestamp") };
            if _cond {
                // Painless script
                // Source: try {\n  long t;\n  if (ctx.json.EdgeStartTimestamp instanceof String) {\n    t = Long.parseLong(ctx.json.EdgeStartTimestamp);\n  } else if (ctx.json.EdgeStartTimestamp instanceof Number) {\n    t = (long)(ctx.json.EdgeStartTimestamp);\n  } else {\n    return;\n  }\n  if (t > (long)(1e18)) {\n    ctx.json.EdgeStartTimestamp = t/(long)(1e6)\n  } else if (t < (long)(1e10))  {\n    ctx.json.EdgeStartTimestamp = t*(long)(1e3)\n  }\n}\ncatch (Exception e) {}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"try {\n  long t;\n  if (ctx.json.EdgeStartTimestamp instanceof String) {\n    t = Long.parseLong(ctx.json.EdgeStartTimestamp);\n  } else if (ctx.json.EdgeStartTimestamp instanceof Number) {\n    t = (long)(ctx.json.EdgeStartTimestamp);\n  } else {\n    return;\n  }\n  if (t > (long)(1e18)) {\n    ctx.json.EdgeStartTimestamp = t/(long)(1e6)\n  } else if (t < (long)(1e10))  {\n    ctx.json.EdgeStartTimestamp = t*(long)(1e3)\n  }\n}\ncatch (Exception e) {}\n"#))?;
            }

            let _cond = { event.has_value("json.EdgeEndTimestamp") };
            if _cond {
                // Painless script
                // Source: try {\n  long t;\n  if (ctx.json.EdgeEndTimestamp instanceof String) {\n    t = Long.parseLong(ctx.json.EdgeEndTimestamp);\n  } else if (ctx.json.EdgeEndTimestamp instanceof Number) {\n    t = (long)(ctx.json.EdgeEndTimestamp);\n  } else {\n    return;\n  }\n  if (t > (long)(1e18)) {\n    ctx.json.EdgeEndTimestamp = t/(long)(1e6)\n  } else if (t < (long)(1e10))  {\n    ctx.json.EdgeEndTimestamp = t*(long)(1e3)\n  }\n}\ncatch (Exception e) {}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"try {\n  long t;\n  if (ctx.json.EdgeEndTimestamp instanceof String) {\n    t = Long.parseLong(ctx.json.EdgeEndTimestamp);\n  } else if (ctx.json.EdgeEndTimestamp instanceof Number) {\n    t = (long)(ctx.json.EdgeEndTimestamp);\n  } else {\n    return;\n  }\n  if (t > (long)(1e18)) {\n    ctx.json.EdgeEndTimestamp = t/(long)(1e6)\n  } else if (t < (long)(1e10))  {\n    ctx.json.EdgeEndTimestamp = t*(long)(1e3)\n  }\n}\ncatch (Exception e) {}\n"#))?;
            }

                if let Some(date_str) = event.get_as_string("json.EdgeStartTimestamp") {
                    match parse_date_out(&date_str, &["ISO8601", "uuuu-MM-dd'T'HH:mm:ssX", "uuuu-MM-dd'T'HH:mm:ss.SSSX", "yyyy-MM-dd'T'HH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ss.SSSZ", "UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.EdgeStartTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

            if let Some(v) = event.get("@timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.start", v)?;
            }

                if let Some(date_str) = event.get_as_string("json.EdgeEndTimestamp") {
                    match parse_date_out(&date_str, &["uuuu-MM-dd'T'HH:mm:ssX", "uuuu-MM-dd'T'HH:mm:ss.SSSX", "yyyy-MM-dd'T'HH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ss.SSSZ", "UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.EdgeEndTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // Painless script
                // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#))?;
            }

            let _cond = { !(event.get_str("json.ClientSSLProtocol").is_some_and(|s| s.to_lowercase() == "none")) };
            if _cond {
                if event.has_value("json.ClientSSLProtocol") {
                    event.rename("json.ClientSSLProtocol", "cloudflare.client.ssl.protocol")?;
                }
            }

            let _cond = { !(event.get_str("json.ClientSSLCipher").is_some_and(|s| s.to_lowercase() == "none")) };
            if _cond {
                if event.has_value("json.ClientSSLCipher") {
                    event.rename("json.ClientSSLCipher", "tls.cipher")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cloudflare.client.ssl.protocol") {
                if let Some(input) = event.get_string("cloudflare.client.ssl.protocol") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("v") else { break 'dissect false };
                        captured.push(("tls.version_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("v") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("tls.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
            }
                Ok(())
            })();

            if event.has_value("tls.version_protocol") {
                map_strings(event, "tls.version_protocol", "tls.version_protocol", str::to_lowercase)?;
            }

            let _cond = { event.has_value("json.ClientRequestURI") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "json.ClientRequestURI", "url", true, false)?;
                Ok(())
            })();
            }

            let _cond = { !event.has_value("url.domain") };
            if _cond {
            if let Some(v) = event.get("json.ClientRequestHost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.domain", v)?;
            }
            }

            let _cond = { !event.has_value("url.path") };
            if _cond {
            if let Some(v) = event.get("json.ClientRequestPath").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.path", v)?;
            }
            }

            let _cond = { !event.has_value("url.scheme") };
            if _cond {
            if let Some(v) = event.get("json.ClientRequestScheme").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.scheme", v)?;
            }
            }

            let _cond = { !event.has_value("url.scheme") && event.has_value("cloudflare.client.ssl.protocol") };
            if _cond {
            let v = json!("https");
            if !painless_is_empty_value(&v) {
                    event.set("url.scheme", v)?;
            }
            }

            let _cond = { !event.has_value("url.scheme") };
            if _cond {
            let v = json!("http");
            if !painless_is_empty_value(&v) {
                    event.set("url.scheme", v)?;
            }
            }

                // Painless script
                // Source: def full = \"\";\nif(ctx.url.scheme != null && ctx.url.scheme != \"\") {\n    full += ctx.url.scheme+\"://\";\n}\nif(ctx.url.domain != null && ctx.url.domain != \"\") {\n    full += ctx.url.domain;\n}\nif(ctx.url.path != null && ctx.url.path != \"\") {\n    full += ctx.url.path;\n}\nif(ctx.url.query != null && ctx.url.query != \"\") {\n    full += \"?\"+ctx.url.query;\n}\nif(full != \"\") {\n    ctx.url.full = full\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def full = \"\";\nif(ctx.url.scheme != null && ctx.url.scheme != \"\") {\n    full += ctx.url.scheme+\"://\";\n}\nif(ctx.url.domain != null && ctx.url.domain != \"\") {\n    full += ctx.url.domain;\n}\nif(ctx.url.path != null && ctx.url.path != \"\") {\n    full += ctx.url.path;\n}\nif(ctx.url.query != null && ctx.url.query != \"\") {\n    full += \"?\"+ctx.url.query;\n}\nif(full != \"\") {\n    ctx.url.full = full\n}\n"#))?;

            if event.has_value("json.ClientRequestUserAgent") {
                if let Some(ua_str) = event.get_string("json.ClientRequestUserAgent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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
            }

            let _cond = { event.get_str("json.EdgeServerIP") != Some("") };
            if _cond {
                event.append("observer.ip", json!(event.get("json.EdgeServerIP").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("observer.ip") {
                if let Some(ip_str) = event.get_string("observer.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("observer.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("observer.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("observer.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("observer.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("observer.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("observer.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("observer.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("observer.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.BotScore") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.BotScore") {
                if let Some(val) = event.get("json.BotScore") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.BotScore".into(),
                            message,
                        })?;
                    event.set("cloudflare.bot.score.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bot_score_value")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.BotScoreSrc") {
                    event.rename("json.BotScoreSrc", "cloudflare.bot.score.src")?;
                }

                if event.has_value("json.CacheCacheStatus") {
                    event.rename("json.CacheCacheStatus", "cloudflare.cache.status")?;
                }

                if event.has_value("json.CacheTieredFill") {
                    event.rename("json.CacheTieredFill", "cloudflare.cache.tiered_fill")?;
                }

            let _cond = { event.get_i64("json.CacheResponseBytes") != Some(0) };
            if _cond {
            if event.has_value("json.CacheResponseBytes") {
                if let Some(val) = event.get("json.CacheResponseBytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.CacheResponseBytes".into(),
                            message,
                        })?;
                    event.set("cloudflare.cache.bytes", converted)?;
                }
            }
            }

            let _cond = { event.get_i64("json.CacheResponseStatus") != Some(0) };
            if _cond {
            if event.has_value("json.CacheResponseStatus") {
                if let Some(val) = event.get("json.CacheResponseStatus") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.CacheResponseStatus".into(),
                            message,
                        })?;
                    event.set("cloudflare.cache.status_code", converted)?;
                }
            }
            }

                if event.has_value("json.EdgeColoCode") {
                    event.rename("json.EdgeColoCode", "cloudflare.edge.colo.code")?;
                }

                if event.has_value("json.EdgeColoID") {
                    event.rename("json.EdgeColoID", "cloudflare.edge.colo.id")?;
                }

                if event.has_value("json.EdgePathingOp") {
                    event.rename("json.EdgePathingOp", "cloudflare.edge.pathing.op")?;
                }

                if event.has_value("json.EdgePathingSrc") {
                    event.rename("json.EdgePathingSrc", "cloudflare.edge.pathing.src")?;
                }

                if event.has_value("json.EdgePathingStatus") {
                    event.rename("json.EdgePathingStatus", "cloudflare.edge.pathing.status")?;
                }

                if event.has_value("json.EdgeRateLimitAction") {
                    event.rename("json.EdgeRateLimitAction", "cloudflare.edge.rate_limit.action")?;
                }

                if event.has_value("json.EdgeRateLimitID") {
                    event.rename("json.EdgeRateLimitID", "cloudflare.edge.rate_limit.id")?;
                }

                if event.has_value("json.EdgeRequestHost") {
                    event.rename("json.EdgeRequestHost", "cloudflare.edge.request.host")?;
                }

            if event.has_value("json.EdgeResponseBytes") {
                if let Some(val) = event.get("json.EdgeResponseBytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.EdgeResponseBytes".into(),
                            message,
                        })?;
                    event.set("cloudflare.edge.response.bytes", converted)?;
                }
            }

                if event.has_value("json.EdgeResponseStatus") {
                    event.rename("json.EdgeResponseStatus", "cloudflare.edge.response.status_code")?;
                }

                if event.has_value("json.EdgeResponseCompressionRatio") {
                    event.rename("json.EdgeResponseCompressionRatio", "cloudflare.edge.response.compression_ratio")?;
                }

                if event.has_value("json.EdgeResponseContentType") {
                    event.rename("json.EdgeResponseContentType", "cloudflare.edge.response.content_type")?;
                }

            if event.has_value("json.EdgeResponseBodyBytes") {
                if let Some(val) = event.get("json.EdgeResponseBodyBytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.EdgeResponseBodyBytes".into(),
                            message,
                        })?;
                    event.set("cloudflare.edge.response.body.bytes", converted)?;
                }
            }

                if event.has_value("json.FirewallMatchesActions") {
                    event.rename("json.FirewallMatchesActions", "cloudflare.firewall.actions")?;
                }

                if event.has_value("json.FirewallMatchesSources") {
                    event.rename("json.FirewallMatchesSources", "cloudflare.firewall.sources")?;
                }

                if event.has_value("json.FirewallMatchesRuleIDs") {
                    event.rename("json.FirewallMatchesRuleIDs", "cloudflare.firewall.rule_ids")?;
                }

                if event.has_value("json.WAFAction") {
                    event.rename("json.WAFAction", "cloudflare.waf.action")?;
                }

                if event.has_value("json.WAFFlags") {
                    event.rename("json.WAFFlags", "cloudflare.waf.flags")?;
                }

                if event.has_value("json.WAFMatchedVar") {
                    event.rename("json.WAFMatchedVar", "cloudflare.waf.matched_var")?;
                }

                if event.has_value("json.WAFProfile") {
                    event.rename("json.WAFProfile", "cloudflare.waf.profile")?;
                }

                if event.has_value("json.WAFRuleID") {
                    event.rename("json.WAFRuleID", "cloudflare.waf.rule.id")?;
                }

                if event.has_value("json.WAFRuleMessage") {
                    event.rename("json.WAFRuleMessage", "cloudflare.waf.rule.message")?;
                }

            if event.has_value("json.WorkerCPUTime") {
                if let Some(val) = event.get("json.WorkerCPUTime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.WorkerCPUTime".into(),
                            message,
                        })?;
                    event.set("cloudflare.worker.cpu_time", converted)?;
                }
            }

                if event.has_value("json.WorkerStatus") {
                    event.rename("json.WorkerStatus", "cloudflare.worker.status")?;
                }

                if event.has_value("json.WorkerSubrequest") {
                    event.rename("json.WorkerSubrequest", "cloudflare.worker.subrequest")?;
                }

            if event.has_value("json.WorkerSubrequestCount") {
                if let Some(val) = event.get("json.WorkerSubrequestCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.WorkerSubrequestCount".into(),
                            message,
                        })?;
                    event.set("cloudflare.worker.subrequest_count", converted)?;
                }
            }

                if event.has_value("json.OriginResponseBytes") {
                    event.rename("json.OriginResponseBytes", "cloudflare.origin.response.bytes")?;
                }

            let _cond = { event.has_value("json.OriginResponseHTTPExpires") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.OriginResponseHTTPExpires") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss z"], Some("UTC"), None) {
                        Some(parsed) => event.set("cloudflare.origin.response.expires", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.OriginResponseHTTPExpires".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.OriginResponseHTTPLastModified") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.OriginResponseHTTPLastModified") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss z"], Some("UTC"), None) {
                        Some(parsed) => event.set("cloudflare.origin.response.last_modified", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.OriginResponseHTTPLastModified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

                if event.has_value("json.OriginResponseStatus") {
                    event.rename("json.OriginResponseStatus", "cloudflare.origin.response.status_code")?;
                }

            if event.has_value("json.OriginResponseTime") {
                if let Some(val) = event.get("json.OriginResponseTime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.OriginResponseTime".into(),
                            message,
                        })?;
                    event.set("cloudflare.origin.response.time", converted)?;
                }
            }

                if event.has_value("json.OriginSSLProtocol") {
                    event.rename("json.OriginSSLProtocol", "cloudflare.origin.ssl.protocol")?;
                }

                if event.has_value("json.ParentRayID") {
                    event.rename("json.ParentRayID", "cloudflare.parent.ray_id")?;
                }

                if event.has_value("json.RayID") {
                    event.rename("json.RayID", "cloudflare.ray_id")?;
                }

                if event.has_value("json.ZoneID") {
                    event.rename("json.ZoneID", "cloudflare.zone.id")?;
                }

                if event.has_value("json.ZoneName") {
                    event.rename("json.ZoneName", "cloudflare.zone.name")?;
                }

                if event.has_value("json.SecurityLevel") {
                    event.rename("json.SecurityLevel", "cloudflare.security_level")?;
                }

                if event.has_value("json.ClientDeviceType") {
                    event.rename("json.ClientDeviceType", "cloudflare.device_type")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.ClientRequestProtocol") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("/") else { break 'dissect false };
                        captured.push(("network.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else { break 'dissect false };
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

            if let Some(v) = event.get("cloudflare.edge.response.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.bytes", v)?;
            }

            if let Some(v) = event.get("cloudflare.edge.response.body.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.body.bytes", v)?;
            }

            if event.has_value("json.ClientRequestBytes") {
                if let Some(val) = event.get("json.ClientRequestBytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ClientRequestBytes".into(),
                            message,
                        })?;
                    event.set("http.request.bytes", converted)?;
                }
            }

                if event.has_value("json.ClientRequestMethod") {
                    event.rename("json.ClientRequestMethod", "http.request.method")?;
                }

                if event.has_value("json.ClientRequestReferer") {
                    event.rename("json.ClientRequestReferer", "http.request.referrer")?;
                }

            if let Some(v) = event.get("cloudflare.edge.response.status_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.status_code", v)?;
            }

            let _cond = { !event.has_value("http.response.status_code") && event.get_i64("cloudflare.origin.response.status_code") != Some(0) };
            if _cond {
            if let Some(v) = event.get("cloudflare.origin.response.status_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.status_code", v)?;
            }
            }

                if event.has_value("json.ClientIP") {
                    event.rename("json.ClientIP", "source.address")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })();

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

            let _cond = { !event.has_value("source.geo.country_iso_code") };
            if _cond {
                if event.has_value("json.ClientCountry") {
                    event.rename("json.ClientCountry", "source.geo.country_iso_code")?;
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

            let _cond = { !event.has_value("source.as.number") };
            if _cond {
                if event.has_value("json.ClientASN") {
                    event.rename("json.ClientASN", "source.as.number")?;
                }
            }

                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }

            if let Some(v) = event.get("http.request.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.bytes", v)?;
            }

            if event.has_value("json.ClientSrcPort") {
                if let Some(val) = event.get("json.ClientSrcPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ClientSrcPort".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }

            if let Some(v) = event.get("source").cloned() {
                event.set("client", v)?;
            }

                if event.has_value("json.ClientIPClass") {
                    event.rename("json.ClientIPClass", "cloudflare.client.ip_class")?;
                }

                if event.has_value("json.OriginIP") {
                    event.rename("json.OriginIP", "destination.address")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("destination.address") {
                if let Some(val) = event.get("destination.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }
                Ok(())
            })();

            if let Some(v) = event.get("cloudflare.edge.response.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.bytes", v)?;
            }

            if let Some(v) = event.get("destination").cloned() {
                event.set("server", v)?;
            }

                event.append("event.category", json!("network"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("cloudflare.firewall.actions") && event.get("cloudflare.firewall.actions").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("block")), serde_json::Value::String(s) => s.contains("block"), _ => false }) };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.has_value("cloudflare.firewall.actions") && event.get("cloudflare.firewall.actions").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("allow")), serde_json::Value::String(s) => s.contains("allow"), _ => false }) };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.has_value("cloudflare.firewall.actions") };
            if _cond {
            if let Some(v) = event.get("cloudflare.firewall.actions").cloned() {
                event.set("event.action", v)?;
            }
            }

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
            }

            let _cond = { event.has_value("network.protocol") && event.get_str("network.protocol") == Some("http") };
            if _cond {
            event.set("network.transport", json!("tcp"))?;
            }

            let _cond = { event.has_value("source.bytes") && event.has_value("destination.bytes") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                sum_directions_into_existing(event, &["bytes"]);
                Ok(())
            })();
            }

                event.remove("json.EdgeServerIP");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
