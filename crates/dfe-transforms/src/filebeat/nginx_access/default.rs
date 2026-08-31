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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ((?:(?:%{IP:destination.ip}|(?P<destination_domain>(?:[^\t ,:]+)))(:%{NUMBER:destination.port})?) )?\"?(?:(?P<nginx_access_remote_ip_list>(?:(?:%{IP}|%{WORD})(\"?,?\\s*(?:%{IP}|%{WORD}))*))|%{NOTSPACE:source.address}) - (-|%{DATA:user.name}) \\[%{HTTPDATE:nginx.access.time}\\] \"%{DATA:nginx.access.info}\" %{NUMBER:http.response.status_code:long} %{NUMBER:http.response.body.bytes:long}(?:\\s%{NUMBER:nginx.access.response_time})? \"(-|%{DATA:http.request.referrer})\" \"(-|%{DATA:user_agent.original})\"
                    if !cached_grok_mapped!("((?:(?:%{IP:destination.ip}|(?P<destination_domain>(?:[^\t ,:]+)))(:%{NUMBER:destination.port})?) )?\"?(?:(?P<nginx_access_remote_ip_list>(?:(?:%{IP}|%{WORD})(\"?,?\\s*(?:%{IP}|%{WORD}))*))|%{NOTSPACE:source.address}) - (-|%{DATA:user.name}) \\[%{HTTPDATE:nginx.access.time}\\] \"%{DATA:nginx.access.info}\" %{NUMBER:http.response.status_code:long} %{NUMBER:http.response.body.bytes:long}(?:\\s%{NUMBER:nginx.access.response_time})? \"(-|%{DATA:http.request.referrer})\" \"(-|%{DATA:user_agent.original})\"", [("nginx_access_remote_ip_list", "nginx.access.remote_ip_list"), ("destination_domain", "destination.domain")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("nginx.access.info") {
                if let Some(input) = event.get_string("nginx.access.info") {
                    // Grok pattern: %{WORD:http.request.method} %{DATA:_tmp.url_orig} HTTP/%{NUMBER:http.version}
                    // Grok pattern:
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{WORD:http.request.method} %{DATA:_tmp.url_orig} HTTP/%{NUMBER:http.version}"
                            ),
                            cached_grok!(""),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "_tmp.url_orig", "url", true, false)?;
                Ok(())
            })();

            let _cond = { !event.has_value("url.domain") && event.has_value("destination.domain") };
            if _cond {
                event.set(
                    "url.domain",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("nginx.access.info");
            event.remove("_tmp.url_orig");

            if event.has_value("nginx.access.remote_ip_list") {
                if let Some(s) = event.get_string("nginx.access.remote_ip_list") {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("nginx.access.remote_ip_list", Value::Array(parts))?;
                }
            }

            if event.has_value("nginx.access.origin") {
                if let Some(s) = event.get_string("nginx.access.origin") {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("nginx.access.origin", Value::Array(parts))?;
                }
            }

            let _cond = { !event.has_value("source.address") };
            if _cond {
                event.set("source.address", json!(""))?;
            }

            let _cond = {
                event.has_value("nginx.access.remote_ip_list") && event.get("nginx.access.remote_ip_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: boolean isPrivate(def dot, def ip) {\n  try {\n    StringTokenizer tok = new StringTokenizer(ip, dot);\n    int firstByte = Integer.parseInt(tok.nextToken());\n    int secondByte = Integer.parseInt(tok.nextToken());\n    if (firstByte == 10) {\n      return true;\n    }\n    if (firstByte == 192 && secondByte == 168) {\n      return true;\n    }\n    if (firstByte == 172 && secondByte >= 16 && secondByte <= 31) {\n      return true;\n    }\n    if (firstByte == 127) {\n      return true;\n    }\n    return false;\n  }\n  catch (Exception e) {\n    return false;\n  }\n} try {\n  ctx.source.address = null;\n  if (ctx.nginx.access.remote_ip_list == null) {\n    return;\n  }\n  def found = false;\n  for (def item : ctx.nginx.access.remote_ip_list) {\n    if (!isPrivate(params.dot, item)) {\n      ctx.source.address = item;\n      found = true;\n      break;\n    }\n  }\n  if (!found) {\n    ctx.source.address = ctx.nginx.access.remote_ip_list[0];\n  }\n} catch (Exception e) {\n  ctx.source.address = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"boolean isPrivate(def dot, def ip) {\n  try {\n    StringTokenizer tok = new StringTokenizer(ip, dot);\n    int firstByte = Integer.parseInt(tok.nextToken());\n    int secondByte = Integer.parseInt(tok.nextToken());\n    if (firstByte == 10) {\n      return true;\n    }\n    if (firstByte == 192 && secondByte == 168) {\n      return true;\n    }\n    if (firstByte == 172 && secondByte >= 16 && secondByte <= 31) {\n      return true;\n    }\n    if (firstByte == 127) {\n      return true;\n    }\n    return false;\n  }\n  catch (Exception e) {\n    return false;\n  }\n} try {\n  ctx.source.address = null;\n  if (ctx.nginx.access.remote_ip_list == null) {\n    return;\n  }\n  def found = false;\n  for (def item : ctx.nginx.access.remote_ip_list) {\n    if (!isPrivate(params.dot, item)) {\n      ctx.source.address = item;\n      found = true;\n      break;\n    }\n  }\n  if (!found) {\n    ctx.source.address = ctx.nginx.access.remote_ip_list[0];\n  }\n} catch (Exception e) {\n  ctx.source.address = null;\n}"#
                    ),
                    cached_params!("{\"dot\":\".\"}"),
                )?;
            }

            let _cond = { !event.has_value("source.address") };
            if _cond {
                if event.remove("source.address").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.address".into(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^%{IP:source.ip}$
                    if !cached_grok!("^%{IP:source.ip}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.created");
                Ok(())
            })();

            event.rename("@timestamp", "event.created")?;

            let _cond = { event.has_value("destination.port") };
            if _cond {
                if let Some(val) = event.get("destination.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("nginx.access.time") {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:H:m:s Z"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "nginx.access.time".into(),
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

            if event.remove("nginx.access.time").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "nginx.access.time".into(),
                });
            }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("web"))?;

            event.append("event.type", json!("access"))?;

            let _cond = {
                event.has_value("http.response.status_code")
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n < 400)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("http.response.status_code")
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n >= 400)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
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
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("nginx.access.response_time") {
                if let Some(val) = event.get("nginx.access.response_time") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "nginx.access.response_time".into(),
                            message,
                        }
                    })?;
                    event.set("nginx.access.response_time", converted)?;
                }
            }

            let _cond = { event.has_value("nginx.access.response_time") };
            if _cond {
                // Painless script
                // Source: ctx.nginx.access.response_time = (long) (ctx.nginx.access.response_time * 1000)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.nginx.access.response_time = (long) (ctx.nginx.access.response_time * 1000)\n"#
                    ),
                )?;
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
