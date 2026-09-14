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
            event.set("ecs.version", json!("8.4.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ((?:(?:(?P<destination_ip>(?:(?:\\[?%{IPV6}\\]?|%{IPV4})))|(?P<destination_domain>(?:[^\t ,:]+)))(:%{NUMBER:destination.port})?) )?\"?(?:(?P<nginx_ingress_controller_access_remote_ip_list>(?:(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))|%{WORD})(\"?,?\\s*(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))|%{WORD}))*))|%{NOTSPACE:source.address}) - (-|%{DATA:user.name}) \\[%{HTTPDATE:nginx_ingress_controller.access.time}\\] \"%{DATA:nginx_ingress_controller.access.info}\" %{NUMBER:http.response.status_code:long} %{NUMBER:http.response.body.bytes:long} \"(-|%{DATA:http.request.referrer})\" \"(-|%{DATA:user_agent.original})\" %{NUMBER:nginx_ingress_controller.access.http.request.length:long} %{NUMBER:nginx_ingress_controller.access.http.request.time:double} \\[%{DATA:nginx_ingress_controller.access.upstream.name}\\] \\[%{DATA:nginx_ingress_controller.access.upstream.alternative_name}\\] ((?P<nginx_ingress_controller_access_upstream_address_list>(?:(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))(:%{NUMBER})?)(\"?,?\\s*(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))(:%{NUMBER})?))*))|-) ((?P<nginx_ingress_controller_access_upstream_response_length_list>(?:(?:%{NUMBER})(\"?,?\\s*(?:%{NUMBER}))*))|-) ((?P<nginx_ingress_controller_access_upstream_response_time_list>(?:(?:%{NUMBER})(\"?,?\\s*(?:%{NUMBER}))*))|-) ((?P<nginx_ingress_controller_access_upstream_response_status_code_list>(?:(?:%{NUMBER})(\"?,?\\s*(?:%{NUMBER}))*))|-) %{GREEDYDATA:nginx_ingress_controller.access.http.request.id}
                    if !cached_grok_mapped!("((?:(?:(?P<destination_ip>(?:(?:\\[?%{IPV6}\\]?|%{IPV4})))|(?P<destination_domain>(?:[^\t ,:]+)))(:%{NUMBER:destination.port})?) )?\"?(?:(?P<nginx_ingress_controller_access_remote_ip_list>(?:(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))|%{WORD})(\"?,?\\s*(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))|%{WORD}))*))|%{NOTSPACE:source.address}) - (-|%{DATA:user.name}) \\[%{HTTPDATE:nginx_ingress_controller.access.time}\\] \"%{DATA:nginx_ingress_controller.access.info}\" %{NUMBER:http.response.status_code:long} %{NUMBER:http.response.body.bytes:long} \"(-|%{DATA:http.request.referrer})\" \"(-|%{DATA:user_agent.original})\" %{NUMBER:nginx_ingress_controller.access.http.request.length:long} %{NUMBER:nginx_ingress_controller.access.http.request.time:double} \\[%{DATA:nginx_ingress_controller.access.upstream.name}\\] \\[%{DATA:nginx_ingress_controller.access.upstream.alternative_name}\\] ((?P<nginx_ingress_controller_access_upstream_address_list>(?:(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))(:%{NUMBER})?)(\"?,?\\s*(?:(?:(?:\\[?%{IPV6}\\]?|%{IPV4}))(:%{NUMBER})?))*))|-) ((?P<nginx_ingress_controller_access_upstream_response_length_list>(?:(?:%{NUMBER})(\"?,?\\s*(?:%{NUMBER}))*))|-) ((?P<nginx_ingress_controller_access_upstream_response_time_list>(?:(?:%{NUMBER})(\"?,?\\s*(?:%{NUMBER}))*))|-) ((?P<nginx_ingress_controller_access_upstream_response_status_code_list>(?:(?:%{NUMBER})(\"?,?\\s*(?:%{NUMBER}))*))|-) %{GREEDYDATA:nginx_ingress_controller.access.http.request.id}", [("nginx_ingress_controller_access_remote_ip_list", "nginx_ingress_controller.access.remote_ip_list"), ("nginx_ingress_controller_access_upstream_address_list", "nginx_ingress_controller.access.upstream_address_list"), ("nginx_ingress_controller_access_upstream_response_length_list", "nginx_ingress_controller.access.upstream.response.length_list"), ("nginx_ingress_controller_access_upstream_response_time_list", "nginx_ingress_controller.access.upstream.response.time_list"), ("nginx_ingress_controller_access_upstream_response_status_code_list", "nginx_ingress_controller.access.upstream.response.status_code_list"), ("destination_ip", "destination.ip"), ("destination_domain", "destination.domain")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("nginx_ingress_controller.access.info") {
                if let Some(input) = event.get_string("nginx_ingress_controller.access.info") {
                    // Grok pattern: %{WORD:http.request.method} %{DATA:url.original} HTTP/%{NUMBER:http.version}
                    // Grok pattern:
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{WORD:http.request.method} %{DATA:url.original} HTTP/%{NUMBER:http.version}"
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
                uri_parts(event, "url.original", "url", true, false)?;
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

            if event.has_value("nginx_ingress_controller.access.remote_ip_list") {
                if let Some(s) = event.get_string("nginx_ingress_controller.access.remote_ip_list")
                {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "nginx_ingress_controller.access.remote_ip_list",
                        Value::Array(parts),
                    )?;
                }
            }

            if event.has_value("nginx_ingress_controller.access.upstream_address_list") {
                if let Some(s) =
                    event.get_string("nginx_ingress_controller.access.upstream_address_list")
                {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "nginx_ingress_controller.access.upstream_address_list",
                        Value::Array(parts),
                    )?;
                }
            }

            if event.has_value("nginx_ingress_controller.access.upstream.response.length_list") {
                if let Some(s) = event
                    .get_string("nginx_ingress_controller.access.upstream.response.length_list")
                {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "nginx_ingress_controller.access.upstream.response.length_list",
                        Value::Array(parts),
                    )?;
                }
            }

            if event.has_value("nginx_ingress_controller.access.upstream.response.time_list") {
                if let Some(s) =
                    event.get_string("nginx_ingress_controller.access.upstream.response.time_list")
                {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "nginx_ingress_controller.access.upstream.response.time_list",
                        Value::Array(parts),
                    )?;
                }
            }

            if event.has_value("nginx_ingress_controller.access.upstream.response.status_code_list")
            {
                if let Some(s) = event.get_string(
                    "nginx_ingress_controller.access.upstream.response.status_code_list",
                ) {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "nginx_ingress_controller.access.upstream.response.status_code_list",
                        Value::Array(parts),
                    )?;
                }
            }

            if event.has_value("nginx_ingress_controller.access.origin") {
                if let Some(s) = event.get_string("nginx_ingress_controller.access.origin") {
                    let mut parts: Vec<Value> = cached_regex!("\"?,?\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "nginx_ingress_controller.access.origin",
                        Value::Array(parts),
                    )?;
                }
            }

            let _cond = { !event.has_value("source.address") };
            if _cond {
                event.set("source.address", json!(""))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("nginx_ingress_controller.access.http.request.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("http.request.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("nginx_ingress_controller.access.http.request.length") {
                if let Some(val) = event.get("nginx_ingress_controller.access.http.request.length")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "nginx_ingress_controller.access.http.request.length".into(),
                            message,
                        }
                    })?;
                    event.set("http.request.bytes", converted)?;
                }
            }

            let _cond = {
                event.has_value("nginx_ingress_controller.access.upstream.response.length_list") && event.get("nginx_ingress_controller.access.upstream.response.length_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: try {\n  if (ctx.nginx_ingress_controller.access.upstream.response.length_list.length == null) {\n    return;\n  }\n  int last_length = 0;\n  for (def item : ctx.nginx_ingress_controller.access.upstream.response.length_list) {\n    last_length =  Integer.parseInt(item);\n  }\n  ctx.nginx_ingress_controller.access.upstream.response.length = last_length;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.response.length = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"try {\n  if (ctx.nginx_ingress_controller.access.upstream.response.length_list.length == null) {\n    return;\n  }\n  int last_length = 0;\n  for (def item : ctx.nginx_ingress_controller.access.upstream.response.length_list) {\n    last_length =  Integer.parseInt(item);\n  }\n  ctx.nginx_ingress_controller.access.upstream.response.length = last_length;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.response.length = null;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("nginx_ingress_controller.access.upstream.response.time_list") && event.get("nginx_ingress_controller.access.upstream.response.time_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: try {\n  if (ctx.nginx_ingress_controller.access.upstream.response.time_list.length == null) {\n    return;\n  }\n  float res_time = 0;\n  for (def item : ctx.nginx_ingress_controller.access.upstream.response.time_list) {\n    res_time = res_time + Float.parseFloat(item);\n  }\n  ctx.nginx_ingress_controller.access.upstream.response.time = res_time;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.response.time = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"try {\n  if (ctx.nginx_ingress_controller.access.upstream.response.time_list.length == null) {\n    return;\n  }\n  float res_time = 0;\n  for (def item : ctx.nginx_ingress_controller.access.upstream.response.time_list) {\n    res_time = res_time + Float.parseFloat(item);\n  }\n  ctx.nginx_ingress_controller.access.upstream.response.time = res_time;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.response.time = null;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("nginx_ingress_controller.access.upstream.response.status_code_list") && event.get("nginx_ingress_controller.access.upstream.response.status_code_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: try {\n  if (ctx.nginx_ingress_controller.access.upstream.response.status_code_list.length == null) {\n    return;\n  }\n  int last_status_code;\n  for (def item : ctx.nginx_ingress_controller.access.upstream.response.status_code_list) {\n    last_status_code = Integer.parseInt(item);\n  }\n  ctx.nginx_ingress_controller.access.upstream.response.status_code = last_status_code;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.response.status_code = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"try {\n  if (ctx.nginx_ingress_controller.access.upstream.response.status_code_list.length == null) {\n    return;\n  }\n  int last_status_code;\n  for (def item : ctx.nginx_ingress_controller.access.upstream.response.status_code_list) {\n    last_status_code = Integer.parseInt(item);\n  }\n  ctx.nginx_ingress_controller.access.upstream.response.status_code = last_status_code;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.response.status_code = null;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("nginx_ingress_controller.access.upstream_address_list") && event.get("nginx_ingress_controller.access.upstream_address_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: try {\n  if (ctx.nginx_ingress_controller.access.upstream_address_list.length == null) {\n    return;\n  }\n  def last_upstream = \"\";\n  for (def item : ctx.nginx_ingress_controller.access.upstream_address_list) {\n    last_upstream = item;\n  }\n  \n  ctx.nginx_ingress_controller.access.upstream.address = last_upstream;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.address = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"try {\n  if (ctx.nginx_ingress_controller.access.upstream_address_list.length == null) {\n    return;\n  }\n  def last_upstream = \"\";\n  for (def item : ctx.nginx_ingress_controller.access.upstream_address_list) {\n    last_upstream = item;\n  }\n  \n  ctx.nginx_ingress_controller.access.upstream.address = last_upstream;\n} catch (Exception e) {\n  ctx.nginx_ingress_controller.access.upstream.address = null;\n}"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nginx_ingress_controller.access.upstream.address") {
                    if let Some(input) =
                        event.get_string("nginx_ingress_controller.access.upstream.address")
                    {
                        // Grok pattern: ^%{IPV4:nginx_ingress_controller.access.upstream.ip}:%{NUMBER:nginx_ingress_controller.access.upstream.port}$
                        // Grok pattern: ^\\[%{IPV6:nginx_ingress_controller.access.upstream.ip}\\]:%{NUMBER:nginx_ingress_controller.access.upstream.port}$
                        // Grok pattern: ^(?P<nginx_ingress_controller_access_upstream_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):%{NUMBER:nginx_ingress_controller.access.upstream.port}$
                        // Grok pattern: ^%{IPV6:nginx_ingress_controller.access.upstream.ip}(?:(?: port |[p#.]))%{NUMBER:nginx_ingress_controller.access.upstream.port}$
                        // Grok pattern: ^%{IPV6:nginx_ingress_controller.access.upstream.ip}(?:(?: port |[p#.]))%{POSINT:nginx_ingress_controller.access.upstream.port}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{IPV4:nginx_ingress_controller.access.upstream.ip}:%{NUMBER:nginx_ingress_controller.access.upstream.port}$"
                                ),
                                cached_grok!(
                                    "^\\[%{IPV6:nginx_ingress_controller.access.upstream.ip}\\]:%{NUMBER:nginx_ingress_controller.access.upstream.port}$"
                                ),
                                cached_grok_mapped!(
                                    "^(?P<nginx_ingress_controller_access_upstream_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):%{NUMBER:nginx_ingress_controller.access.upstream.port}$",
                                    [(
                                        "nginx_ingress_controller_access_upstream_ip",
                                        "nginx_ingress_controller.access.upstream.ip"
                                    )]
                                ),
                                cached_grok!(
                                    "^%{IPV6:nginx_ingress_controller.access.upstream.ip}(?:(?: port |[p#.]))%{NUMBER:nginx_ingress_controller.access.upstream.port}$"
                                ),
                                cached_grok!(
                                    "^%{IPV6:nginx_ingress_controller.access.upstream.ip}(?:(?: port |[p#.]))%{POSINT:nginx_ingress_controller.access.upstream.port}$"
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("nginx_ingress_controller.access.upstream.ip") {
                    if let Some(val) = event.get("nginx_ingress_controller.access.upstream.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "nginx_ingress_controller.access.upstream.ip".into(),
                                message,
                            }
                        })?;
                        event.set("nginx_ingress_controller.access.upstream.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("nginx_ingress_controller.access.upstream.ip")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "nginx_ingress_controller.access.upstream.ip".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("nginx_ingress_controller.access.upstream.port") {
                    if let Some(val) = event.get("nginx_ingress_controller.access.upstream.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "nginx_ingress_controller.access.upstream.port".into(),
                                message,
                            }
                        })?;
                        event.set("nginx_ingress_controller.access.upstream.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("nginx_ingress_controller.access.upstream.port")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "nginx_ingress_controller.access.upstream.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("nginx_ingress_controller.access.remote_ip_list") && event.get("nginx_ingress_controller.access.remote_ip_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: boolean isPrivate(def dot, def ip) {\n  try {\n    StringTokenizer tok = new StringTokenizer(ip, dot);\n    int firstByte = Integer.parseInt(tok.nextToken());\n    int secondByte = Integer.parseInt(tok.nextToken());\n    if (firstByte == 10) {\n      return true;\n    }\n    if (firstByte == 192 && secondByte == 168) {\n      return true;\n    }\n    if (firstByte == 172 && secondByte >= 16 && secondByte <= 31) {\n      return true;\n    }\n    if (firstByte == 127) {\n      return true;\n    }\n    return false;\n  }\n  catch (Exception e) {\n    return false;\n  }\n} try {\n  ctx.source.address = null;\n  if (ctx.nginx_ingress_controller.access.remote_ip_list == null) {\n    return;\n  }\n  def found = false;\n  for (def item : ctx.nginx_ingress_controller.access.remote_ip_list) {\n    if (!isPrivate(params.dot, item)) {\n      ctx.source.address = item;\n      found = true;\n      break;\n    }\n  }\n  if (!found) {\n    ctx.source.address = ctx.nginx_ingress_controller.access.remote_ip_list[0];\n  }\n} catch (Exception e) {\n  ctx.source.address = null;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"boolean isPrivate(def dot, def ip) {\n  try {\n    StringTokenizer tok = new StringTokenizer(ip, dot);\n    int firstByte = Integer.parseInt(tok.nextToken());\n    int secondByte = Integer.parseInt(tok.nextToken());\n    if (firstByte == 10) {\n      return true;\n    }\n    if (firstByte == 192 && secondByte == 168) {\n      return true;\n    }\n    if (firstByte == 172 && secondByte >= 16 && secondByte <= 31) {\n      return true;\n    }\n    if (firstByte == 127) {\n      return true;\n    }\n    return false;\n  }\n  catch (Exception e) {\n    return false;\n  }\n} try {\n  ctx.source.address = null;\n  if (ctx.nginx_ingress_controller.access.remote_ip_list == null) {\n    return;\n  }\n  def found = false;\n  for (def item : ctx.nginx_ingress_controller.access.remote_ip_list) {\n    if (!isPrivate(params.dot, item)) {\n      ctx.source.address = item;\n      found = true;\n      break;\n    }\n  }\n  if (!found) {\n    ctx.source.address = ctx.nginx_ingress_controller.access.remote_ip_list[0];\n  }\n} catch (Exception e) {\n  ctx.source.address = null;\n}"#
                    ),
                    cached_params!("{\"dot\":\".\"}"),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("nginx_ingress_controller.access.time")
                {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:H:m:s Z"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "nginx_ingress_controller.access.time".into(),
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

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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

            event.append("event.type", json!("info"))?;

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

            let _cond = { event.has_value("nginx_ingress_controller.access.upstream.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("nginx_ingress_controller.access.upstream.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("nginx_ingress_controller.access.time");
            event.remove("nginx_ingress_controller.access.info");
            event.remove("nginx_ingress_controller.access.upstream.address");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
