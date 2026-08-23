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

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.append("event.type", json!("access"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: %{BASE16NUM:aws.s3access.bucket_owner} %{HOSTNAME:aws.s3access.bucket} \\[%{HTTPDATE:_temp_.s3access_time}\\] (?:-|%{IP:aws.s3access.remote_ip}) (?:-|(?P<aws_s3access_requester>(?:[a-zA-Z0-9\\/_\\.\\-%:@]+))) (?P<aws_s3access_request_id>(?:[a-zA-Z0-9]+)) (?P<aws_s3access_operation>(?:%{WORD}.%{WORD}.%{WORD})) (?:-|(?P<aws_s3access_key>(?:[a-zA-Z0-9\\/\\_\\!\\-\\.\\*\\'\\(\\)\\%\\+]+))) (?:-|\\\"%{DATA:aws.s3access.request_uri}\\\") (?:-|%{NUMBER:aws.s3access.http_status:long}) (?:-|%{WORD:aws.s3access.error_code}) (?:-|%{NUMBER:aws.s3access.bytes_sent:long}) (?:-|%{NUMBER:aws.s3access.object_size:long}) (?:-|%{NUMBER:aws.s3access.total_time:long}) (?:-|%{NUMBER:aws.s3access.turn_around_time:long}) (?:-|\\\"-\\\"|\\\"%{DATA:aws.s3access.referrer}\\\") (?:-|\\\"(-|%{DATA:aws.s3access.user_agent})\\\") (?:-|(?P<aws_s3access_version_id>(?:[a-zA-Z0-9\\/\\_\\!\\-\\.\\*\\'\\(\\)\\%\\+]+))) (?:-|(?P<aws_s3access_host_id>(?:[a-zA-Z0-9\\/_\\.\\-%+=:]+))) (?:-|(?P<aws_s3access_signature_version>(?:[a-zA-Z0-9.]+))) (?:-|(?P<aws_s3access_cipher_suite>(?:[a-zA-Z0-9\\/\\_\\!\\-\\.\\*\\'\\(\\)\\%\\+]+))) (?:-|%{WORD:aws.s3access.authentication_type}) (?:-|(?P<aws_s3access_host_header>(?:[a-zA-Z0-9\\/_\\.\\-%+=:]+))) (?:-|(?P<aws_s3access_tls_version>(?:[a-zA-Z0-9.]+)))(?: (?:-|(?P<aws_s3access_access_point_arn>(?:[a-zA-Z0-9\\/_\\.\\-%:@]+))))?(?: (?:-|(?P<aws_s3access_aclrequired>(?:(-|Yes)))))?(?: (?:-|(?P<aws_s3access_source_region>(?:[a-zA-Z][a-zA-Z0-9-]*))))?(?: (?P<aws_s3access_source_region>(?:[a-zA-Z][a-zA-Z0-9-]*)))?
                let _ = cached_grok_mapped!("%{BASE16NUM:aws.s3access.bucket_owner} %{HOSTNAME:aws.s3access.bucket} \\[%{HTTPDATE:_temp_.s3access_time}\\] (?:-|%{IP:aws.s3access.remote_ip}) (?:-|(?P<aws_s3access_requester>(?:[a-zA-Z0-9\\/_\\.\\-%:@]+))) (?P<aws_s3access_request_id>(?:[a-zA-Z0-9]+)) (?P<aws_s3access_operation>(?:%{WORD}.%{WORD}.%{WORD})) (?:-|(?P<aws_s3access_key>(?:[a-zA-Z0-9\\/\\_\\!\\-\\.\\*\\'\\(\\)\\%\\+]+))) (?:-|\\\"%{DATA:aws.s3access.request_uri}\\\") (?:-|%{NUMBER:aws.s3access.http_status:long}) (?:-|%{WORD:aws.s3access.error_code}) (?:-|%{NUMBER:aws.s3access.bytes_sent:long}) (?:-|%{NUMBER:aws.s3access.object_size:long}) (?:-|%{NUMBER:aws.s3access.total_time:long}) (?:-|%{NUMBER:aws.s3access.turn_around_time:long}) (?:-|\\\"-\\\"|\\\"%{DATA:aws.s3access.referrer}\\\") (?:-|\\\"(-|%{DATA:aws.s3access.user_agent})\\\") (?:-|(?P<aws_s3access_version_id>(?:[a-zA-Z0-9\\/\\_\\!\\-\\.\\*\\'\\(\\)\\%\\+]+))) (?:-|(?P<aws_s3access_host_id>(?:[a-zA-Z0-9\\/_\\.\\-%+=:]+))) (?:-|(?P<aws_s3access_signature_version>(?:[a-zA-Z0-9.]+))) (?:-|(?P<aws_s3access_cipher_suite>(?:[a-zA-Z0-9\\/\\_\\!\\-\\.\\*\\'\\(\\)\\%\\+]+))) (?:-|%{WORD:aws.s3access.authentication_type}) (?:-|(?P<aws_s3access_host_header>(?:[a-zA-Z0-9\\/_\\.\\-%+=:]+))) (?:-|(?P<aws_s3access_tls_version>(?:[a-zA-Z0-9.]+)))(?: (?:-|(?P<aws_s3access_access_point_arn>(?:[a-zA-Z0-9\\/_\\.\\-%:@]+))))?(?: (?:-|(?P<aws_s3access_aclrequired>(?:(-|Yes)))))?(?: (?:-|(?P<aws_s3access_source_region>(?:[a-zA-Z][a-zA-Z0-9-]*))))?(?: (?P<aws_s3access_source_region>(?:[a-zA-Z][a-zA-Z0-9-]*)))?", [("aws_s3access_requester", "aws.s3access.requester"), ("aws_s3access_request_id", "aws.s3access.request_id"), ("aws_s3access_operation", "aws.s3access.operation"), ("aws_s3access_key", "aws.s3access.key"), ("aws_s3access_version_id", "aws.s3access.version_id"), ("aws_s3access_host_id", "aws.s3access.host_id"), ("aws_s3access_signature_version", "aws.s3access.signature_version"), ("aws_s3access_cipher_suite", "aws.s3access.cipher_suite"), ("aws_s3access_host_header", "aws.s3access.host_header"), ("aws_s3access_tls_version", "aws.s3access.tls_version"), ("aws_s3access_access_point_arn", "aws.s3access.access_point_arn"), ("aws_s3access_aclrequired", "aws.s3access.aclrequired"), ("aws_s3access_source_region", "aws.s3access.source_region"), ("aws_s3access_source_region", "aws.s3access.source_region")]).extract_into(&input, event)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("aws.s3access.host_header") {
                    if let Some(input) = event.get_string("aws.s3access.host_header") {
                        // Grok pattern: ^%{DATA}s3\\.%{DATA:cloud.region}\\.%{DATA}$
                        let _ = cached_grok!("^%{DATA}s3\\.%{DATA:cloud.region}\\.%{DATA}$")
                            .extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("aws.s3access.request_uri") {
                    // Grok pattern: %{NOTSPACE:http.request.method} %{NOTSPACE:_temp_.url} [hH][tT][tT][pP]/%{NOTSPACE:http.version}
                    let _ = cached_grok!("%{NOTSPACE:http.request.method} %{NOTSPACE:_temp_.url} [hH][tT][tT][pP]/%{NOTSPACE:http.version}").extract_into(&input, event)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("_temp_.url") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "_temp_.url", "url", true, false)?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("aws.s3access.bucket_owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("aws.s3access.bucket_owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_temp_.s3access_time") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["dd/MMM/yyyy:H:m:s Z"], None, None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            let v = json!(
                event
                    .get("aws.s3access.remote_ip")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.ip", v)?;
            }

            let _cond = { event.has_value("aws.s3access.remote_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("aws.s3access.remote_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("aws.s3access.remote_ip")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.address", v)?;
            }

            let _cond = { event.has_value("aws.s3access.remote_ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("aws.s3access.remote_ip") {
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

            if let Some(v) = event
                .get("client.geo")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("geo", v)?;
            }

            let v = json!(
                event
                    .get("aws.s3access.requester")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.user.id", v)?;
            }

            let v = json!(
                event
                    .get("aws.s3access.request_id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("event.id", v)?;
            }

            let v = json!(
                event
                    .get("aws.s3access.operation")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("event.action", v)?;
            }

            let v = json!(
                event
                    .get("aws.s3access.http_status")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("http.response.status_code", v)?;
            }

            let _cond = { event.has_value("http.response.status_code") };
            if _cond {
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

            let _cond = { event.has_value("aws.s3access.error_code") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let v = json!(
                event
                    .get("aws.s3access.error_code")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("event.code", v)?;
            }

            let _cond = { !event.has_value("aws.s3access.error_code") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("aws.s3access.bytes_sent") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "aws.s3access.bytes_sent".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.body.bytes", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("aws.s3access.total_time") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "aws.s3access.total_time".into(),
                            message,
                        }
                    })?;
                    event.set("event.duration", converted)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration *= params.MS_TO_NS;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(r#"ctx.event.duration *= params.MS_TO_NS;"#),
                    cached_params!("{\"MS_TO_NS\":1000000}"),
                )?;
            }

            let v = json!(
                event
                    .get("aws.s3access.referrer")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("http.request.referrer", v)?;
            }

            let _cond = { event.has_value("aws.s3access.user_agent") };
            if _cond {
                if let Some(ua_str) = event.get_string("aws.s3access.user_agent") {
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

            let v = json!(
                event
                    .get("aws.s3access.cipher_suite")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("tls.cipher", v)?;
            }

            let _cond = { event.has_value("aws.s3access.tls_version") };
            if _cond {
                // Painless script
                // Source: def parts = ctx.aws.s3access.tls_version.toLowerCase().splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} ctx.tls.version = parts[1]; ctx.tls.version_protocol = parts[0]
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parts = ctx.aws.s3access.tls_version.toLowerCase().splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} ctx.tls.version = parts[1]; ctx.tls.version_protocol = parts[0]"#
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("aws.s3access.access_point_arn")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("aws.s3access.access_point_arn", v)?;
            }

            let v = json!(
                event
                    .get("aws.s3access.aclrequired")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("aws.s3access.aclrequired", v)?;
            }

            let v = json!(
                event
                    .get("aws.s3access.source_region")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("aws.s3access.source_region", v)?;
            }

            event.set("cloud.provider", json!("aws"))?;

            event.set("event.kind", json!("event"))?;

            event.remove("_temp_");

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
