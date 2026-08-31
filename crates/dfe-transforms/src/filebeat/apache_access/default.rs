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

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (%{IPORHOST:destination.domain}:?%{POSINT:destination.port}? )?(%{IPORHOST:source.address}:?%{POSINT:source.port}? )%{DATA:apache.access.identity} %{DATA:user.name} \\[%{HTTPDATE:apache.access.time}\\] \"(%{DATA:apache.access.tls_handshake.error})?((%{WORD:http.request.method}?) %{DATA:_tmp.url_orig}? HTTP/%{NUMBER:http.version})?(%{WORD:http.request.method}? ?%{DATA:apache.access.http.request_headers}?(-)? %{DATA:_tmp.url_orig} HTTP/%{NUMBER:http.version})?(-)?\" %{NUMBER:http.response.status_code:long} (?:%{NUMBER:http.response.body.bytes:long}|-)( %{NUMBER:apache.access.response_time})?( \"%{DATA:http.request.referrer}\")?( \"%{DATA:user_agent.original}\")?( X-Forwarded-For=\"(?P<apache_access_remote_addresses>(?:(%{IP})(\"?,?\\s*(%{IP}))*))\")?
                    // Grok pattern: %{IPORHOST:source.address} - %{DATA:user.name} \\[%{HTTPDATE:apache.access.time}\\] \"-\" %{NUMBER:http.response.status_code:long} -
                    // Grok pattern: \\[%{HTTPDATE:apache.access.time}\\] %{IPORHOST:source.address} %{DATA:apache.access.ssl.protocol} %{DATA:apache.access.ssl.cipher} \"%{WORD:http.request.method} %{DATA:_tmp.url_orig} HTTP/%{NUMBER:http.version}\" (-|%{NUMBER:http.response.body.bytes:long})
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(%{IPORHOST:destination.domain}:?%{POSINT:destination.port}? )?(%{IPORHOST:source.address}:?%{POSINT:source.port}? )%{DATA:apache.access.identity} %{DATA:user.name} \\[%{HTTPDATE:apache.access.time}\\] \"(%{DATA:apache.access.tls_handshake.error})?((%{WORD:http.request.method}?) %{DATA:_tmp.url_orig}? HTTP/%{NUMBER:http.version})?(%{WORD:http.request.method}? ?%{DATA:apache.access.http.request_headers}?(-)? %{DATA:_tmp.url_orig} HTTP/%{NUMBER:http.version})?(-)?\" %{NUMBER:http.response.status_code:long} (?:%{NUMBER:http.response.body.bytes:long}|-)( %{NUMBER:apache.access.response_time})?( \"%{DATA:http.request.referrer}\")?( \"%{DATA:user_agent.original}\")?( X-Forwarded-For=\"(?P<apache_access_remote_addresses>(?:(%{IP})(\"?,?\\s*(%{IP}))*))\")?",
                                [(
                                    "apache_access_remote_addresses",
                                    "apache.access.remote_addresses"
                                )]
                            ),
                            cached_grok!(
                                "%{IPORHOST:source.address} - %{DATA:user.name} \\[%{HTTPDATE:apache.access.time}\\] \"-\" %{NUMBER:http.response.status_code:long} -"
                            ),
                            cached_grok!(
                                "\\[%{HTTPDATE:apache.access.time}\\] %{IPORHOST:source.address} %{DATA:apache.access.ssl.protocol} %{DATA:apache.access.ssl.cipher} \"%{WORD:http.request.method} %{DATA:_tmp.url_orig} HTTP/%{NUMBER:http.version}\" (-|%{NUMBER:http.response.body.bytes:long})"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("apache.access.remote_addresses") {
                if let Some(s) = event.get_string("apache.access.remote_addresses") {
                    let mut parts: Vec<Value> = cached_regex!("\"?,\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("apache.access.remote_addresses", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("apache.access.remote_addresses") && event.get("apache.access.remote_addresses").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                event.set(
                    "network.forwarded_ip",
                    json!(
                        event
                            .get("apache.access.remote_addresses.0")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("apache.access.remote_addresses") && event.get("apache.access.remote_addresses").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: boolean isPrivateCIDR(def ip) {\n  CIDR class_a_network = new CIDR('10.0.0.0/8');\n  CIDR class_b_network = new CIDR('172.16.0.0/12');\n  CIDR class_c_network = new CIDR('192.168.0.0/16');\n\n  try {\n    return class_a_network.contains(ip) || class_b_network.contains(ip) || class_c_network.contains(ip);\n  } catch (IllegalArgumentException e) {\n    return false;\n  }\n}\ntry {\n  if (ctx.client == null) {\n    Map map = new HashMap();\n    ctx.put(\"client\", map);\n  }\n\n  def found = false;\n  for (def item : ctx.apache.access.remote_addresses) {\n    if (!isPrivateCIDR(item)) {\n      ctx.client.ip = item;\n      found = true;\n      break;\n    }\n  }\n  if (!found) {\n    ctx.client.ip = ctx.apache.access.remote_addresses[0];\n  }\n} catch (Exception e) {\n  ctx.client.ip = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean isPrivateCIDR(def ip) {\n  CIDR class_a_network = new CIDR('10.0.0.0/8');\n  CIDR class_b_network = new CIDR('172.16.0.0/12');\n  CIDR class_c_network = new CIDR('192.168.0.0/16');\n\n  try {\n    return class_a_network.contains(ip) || class_b_network.contains(ip) || class_c_network.contains(ip);\n  } catch (IllegalArgumentException e) {\n    return false;\n  }\n}\ntry {\n  if (ctx.client == null) {\n    Map map = new HashMap();\n    ctx.put(\"client\", map);\n  }\n\n  def found = false;\n  for (def item : ctx.apache.access.remote_addresses) {\n    if (!isPrivateCIDR(item)) {\n      ctx.client.ip = item;\n      found = true;\n      break;\n    }\n  }\n  if (!found) {\n    ctx.client.ip = ctx.apache.access.remote_addresses[0];\n  }\n} catch (Exception e) {\n  ctx.client.ip = null;\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                event.append(
                    "apache.access.remote_addresses",
                    json!(
                        event
                            .get("source.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "_tmp.url_orig", "url", true, false)?;
                Ok(())
            })();

            event.remove("_tmp");

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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("web"))?;

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
                        .is_some_and(|n| n > 399)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("source.address") {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(%{IP:source.ip}|%{HOSTNAME:source.domain})$
                    if !cached_grok!("^(%{IP:source.ip}|%{HOSTNAME:source.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.created");
                Ok(())
            })();

            event.rename("@timestamp", "event.created")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("apache.access.time") {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:H:m:s Z"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "apache.access.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("apache.access.time").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "apache.access.time".into(),
                    });
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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

            let _cond = { event.has_value("apache.access.ssl.cipher") };
            if _cond {
                event.set(
                    "tls.cipher",
                    json!(
                        event
                            .get("apache.access.ssl.cipher")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("apache.access.ssl.protocol") };
            if _cond {
                // Painless script
                // Source: def parts = ctx.apache.access.ssl.protocol.toLowerCase().splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} if (parts[1].contains(\".\")) {\n  ctx.tls.version = parts[1];\n} else {\n  ctx.tls.version = parts[1] + \".0\";\n} ctx.tls.version_protocol = parts[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parts = ctx.apache.access.ssl.protocol.toLowerCase().splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} if (parts[1].contains(\".\")) {\n  ctx.tls.version = parts[1];\n} else {\n  ctx.tls.version = parts[1] + \".0\";\n} ctx.tls.version_protocol = parts[0];"#
                    ),
                )?;
            }

            // on_failure: 3 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("tmp_host", v)?;
                }
                let _cond = { event.has_value("tmp_host") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("tmp_host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("tmp_host") };
                if _cond {
                    event.set("tmp_host", json!(""))?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("tmp_host") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 3 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.domain") {
                    if let Some(val) = event.get("destination.domain") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.domain".into(),
                                message,
                            }
                        })?;
                        event.set("destination.domain", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if let Some(v) = event.get("destination.domain").cloned() {
                    event.set("tmp_host", v)?;
                }
                let _cond = { event.has_value("tmp_host") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("tmp_host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("tmp_host") };
                if _cond {
                    event.set("tmp_host", json!(""))?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

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

            let _cond = { event.has_value("source.port") };
            if _cond {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            let _cond = { !event.has_value("tmp_host") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("apache.access.response_time") {
                    if let Some(val) = event.get("apache.access.response_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "apache.access.response_time".into(),
                                message,
                            }
                        })?;
                        event.set("apache.access.response_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_response_time_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
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

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '' || o == '-') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '' || o == '-') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
