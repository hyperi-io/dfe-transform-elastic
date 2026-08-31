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

            event.set("ecs.version", json!("8.3.0"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?P<first_char>(?:.))
                if !cached_grok!("^(?P<first_char>(?:.))").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = { event.get_str("first_char") != Some("{") };
            if _cond {
                // Begin nested pipeline: "format-common"
                let _cond = { event.get_str("first_char") != Some("[") };
                if _cond {
                    return Ok(TransformResult::Drop);
                }
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("] ") else {
                            break 'dissect false;
                        };
                        captured.push(("istio.access.start_time", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("istio.access.message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
                if event.has_value("istio.access.message") {
                    if let Some(input) = event.get_string("istio.access.message") {
                        // Grok pattern: \"(-|%{DATA:http.request.method}) (-|%{DATA:url.original}) (-|%{DATA:istio.access.protocol})\" (-|%{NUMBER:http.response.status_code}) (-|%{DATA:istio.access.response.flags}) (-|%{DATA:istio.access.response.code_details}) (-|%{DATA:istio.access.connection_termination_details}) \"(-|%{DATA:istio.access.upstream.transport_failure_reason})\" %{NUMBER:istio.access.bytes.received} %{NUMBER:istio.access.bytes.sent} (-|%{NUMBER:istio.access.duration}) (-|%{NUMBER:istio.access.upstream.service_time}) \"(-|%{DATA:istio.access.x_forwarded_for})\" \"(-|%{DATA:user_agent.original})\" \"(-|%{DATA:http.request.id})\" \"(-|%{DATA:istio.access.authority})\" \"(-|%{DATA:istio.access.upstream.host})\" (-|%{DATA:istio.access.upstream.cluster}) (-|%{DATA:istio.access.upstream.local_address}) (-|%{DATA:istio.access.downstream.local_address}) (-|%{DATA:istio.access.downstream.remote_address}) (-|%{DATA:istio.access.requested_server_name}) (-|%{GREEDYDATA:istio.access.route_name})
                        if !cached_grok!("\"(-|%{DATA:http.request.method}) (-|%{DATA:url.original}) (-|%{DATA:istio.access.protocol})\" (-|%{NUMBER:http.response.status_code}) (-|%{DATA:istio.access.response.flags}) (-|%{DATA:istio.access.response.code_details}) (-|%{DATA:istio.access.connection_termination_details}) \"(-|%{DATA:istio.access.upstream.transport_failure_reason})\" %{NUMBER:istio.access.bytes.received} %{NUMBER:istio.access.bytes.sent} (-|%{NUMBER:istio.access.duration}) (-|%{NUMBER:istio.access.upstream.service_time}) \"(-|%{DATA:istio.access.x_forwarded_for})\" \"(-|%{DATA:user_agent.original})\" \"(-|%{DATA:http.request.id})\" \"(-|%{DATA:istio.access.authority})\" \"(-|%{DATA:istio.access.upstream.host})\" (-|%{DATA:istio.access.upstream.cluster}) (-|%{DATA:istio.access.upstream.local_address}) (-|%{DATA:istio.access.downstream.local_address}) (-|%{DATA:istio.access.downstream.remote_address}) (-|%{DATA:istio.access.requested_server_name}) (-|%{GREEDYDATA:istio.access.route_name})").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                event.remove("istio.access.message");
                // End nested pipeline: "format-common"
            }

            let _cond = { event.get_str("first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "format-json"
                parse_json_field(event, "event.original", "json")?;
                if event.has_value("json.start_time") {
                    event.rename("json.start_time", "istio.access.start_time")?;
                }
                if event.has_value("json.method") {
                    event.rename("json.method", "http.request.method")?;
                }
                if event.has_value("json.path") {
                    event.rename("json.path", "url.original")?;
                }
                if event.has_value("json.protocol") {
                    event.rename("json.protocol", "istio.access.protocol")?;
                }
                if event.has_value("json.response_code") {
                    event.rename("json.response_code", "http.response.status_code")?;
                }
                if event.has_value("json.response_flags") {
                    event.rename("json.response_flags", "istio.access.response.flags")?;
                }
                if event.has_value("json.response_code_details") {
                    event.rename(
                        "json.response_code_details",
                        "istio.access.response.code_details",
                    )?;
                }
                if event.has_value("json.connection_termination_details") {
                    event.rename(
                        "json.connection_termination_details",
                        "istio.access.connection_termination_details",
                    )?;
                }
                if event.has_value("json.upstream_transport_failure_reason") {
                    event.rename(
                        "json.upstream_transport_failure_reason",
                        "istio.access.upstream.transport_failure_reason",
                    )?;
                }
                if event.has_value("json.bytes_received") {
                    event.rename("json.bytes_received", "istio.access.bytes.received")?;
                }
                if event.has_value("json.bytes_sent") {
                    event.rename("json.bytes_sent", "istio.access.bytes.sent")?;
                }
                if event.has_value("json.duration") {
                    event.rename("json.duration", "istio.access.duration")?;
                }
                if event.has_value("json.upstream_service_time") {
                    event.rename(
                        "json.upstream_service_time",
                        "istio.access.upstream.service_time",
                    )?;
                }
                if event.has_value("json.x_forwarded_for") {
                    event.rename("json.x_forwarded_for", "istio.access.x_forwarded_for")?;
                }
                if event.has_value("json.user_agent") {
                    event.rename("json.user_agent", "user_agent.original")?;
                }
                if event.has_value("json.request_id") {
                    event.rename("json.request_id", "http.request.id")?;
                }
                if event.has_value("json.authority") {
                    event.rename("json.authority", "istio.access.authority")?;
                }
                if event.has_value("json.upstream_host") {
                    event.rename("json.upstream_host", "istio.access.upstream.host")?;
                }
                if event.has_value("json.upstream_cluster") {
                    event.rename("json.upstream_cluster", "istio.access.upstream.cluster")?;
                }
                if event.has_value("json.upstream_local_address") {
                    event.rename(
                        "json.upstream_local_address",
                        "istio.access.upstream.local_address",
                    )?;
                }
                if event.has_value("json.downstream_local_address") {
                    event.rename(
                        "json.downstream_local_address",
                        "istio.access.downstream.local_address",
                    )?;
                }
                if event.has_value("json.downstream_remote_address") {
                    event.rename(
                        "json.downstream_remote_address",
                        "istio.access.downstream.remote_address",
                    )?;
                }
                if event.has_value("json.requested_server_name") {
                    event.rename(
                        "json.requested_server_name",
                        "istio.access.requested_server_name",
                    )?;
                }
                if event.has_value("json.route_name") {
                    event.rename("json.route_name", "istio.access.route_name")?;
                }
                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n  map.values().removeIf(v -> v == \"-\");\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n  map.values().removeIf(v -> v == \"-\");\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
                event.remove("json");
                // End nested pipeline: "format-json"
            }

            if event.remove("first_char").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "first_char".into(),
                });
            }

            if let Some(date_str) = event.get_as_string("istio.access.start_time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "istio.access.start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("istio.access.start_time").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "istio.access.start_time".into(),
                });
            }

            if event.has_value("istio.access.protocol") {
                if let Some(input) = event.get_string("istio.access.protocol") {
                    // Grok pattern: (-|HTTP/%{NUMBER:http.version})
                    if !cached_grok!("(-|HTTP/%{NUMBER:http.version})")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("istio.access.protocol") };
            if _cond {
                event.set("network.protocol", json!("http"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("istio.access.protocol").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "istio.access.protocol".into(),
                    });
                }
                Ok(())
            })();

            event.set("network.transport", json!("tcp"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("http.response.status_code").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "http.response.status_code".into(),
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
                if event.has_value("istio.access.bytes.received") {
                    if let Some(val) = event.get("istio.access.bytes.received") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "istio.access.bytes.received".into(),
                                message,
                            }
                        })?;
                        event.set("istio.access.bytes.received", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("istio.access.bytes.received").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "istio.access.bytes.received".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("network.protocol") == Some("http") };
            if _cond {
                if let Some(v) = event.get("istio.access.bytes.received").cloned() {
                    event.set("http.response.body.bytes", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("istio.access.bytes.sent") {
                    if let Some(val) = event.get("istio.access.bytes.sent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "istio.access.bytes.sent".into(),
                                message,
                            }
                        })?;
                        event.set("istio.access.bytes.sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("istio.access.bytes.sent").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "istio.access.bytes.sent".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("network.protocol") == Some("http") };
            if _cond {
                if let Some(v) = event.get("istio.access.bytes.sent").cloned() {
                    event.set("http.request.body.bytes", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("istio.access.duration") {
                    if let Some(val) = event.get("istio.access.duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "istio.access.duration".into(),
                                message,
                            }
                        })?;
                        event.set("istio.access.duration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("istio.access.duration").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "istio.access.duration".into(),
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
                if event.has_value("istio.access.upstream.service_time") {
                    if let Some(val) = event.get("istio.access.upstream.service_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "istio.access.upstream.service_time".into(),
                                message,
                            }
                        })?;
                        event.set("istio.access.upstream.service_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("istio.access.upstream.service_time").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "istio.access.upstream.service_time".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("istio.access.upstream.service_time") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx.istio.access.upstream.service_time * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Math.round(ctx.istio.access.upstream.service_time * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1000000}"),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("istio.access.upstream.host").cloned() {
                    event.set("destination.address", v)?;
                }
                Ok(())
            })();

            if event.has_value("destination.address") {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: ^-$
                    // Grok pattern: ^%{HOSTNAME:destination.domain}$
                    // Grok pattern: ^%{IPV4:destination.ip}:%{NUMBER:destination.port}$
                    // Grok pattern: ^\\[%{IPV6:destination.ip}\\]:%{NUMBER:destination.port}$
                    // Grok pattern: ^(?P<destination_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):%{NUMBER:destination.port}$
                    // Grok pattern: ^%{IPV6:destination.ip}(?:(?: port |[p#.]))%{NUMBER:destination.port}$
                    // Grok pattern: ^%{IPV6:destination.ip}(?:(?: port |[p#.]))%{POSINT:destination.port}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^-$"),
                            cached_grok!("^%{HOSTNAME:destination.domain}$"),
                            cached_grok!("^%{IPV4:destination.ip}:%{NUMBER:destination.port}$"),
                            cached_grok!(
                                "^\\[%{IPV6:destination.ip}\\]:%{NUMBER:destination.port}$"
                            ),
                            cached_grok_mapped!(
                                "^(?P<destination_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):%{NUMBER:destination.port}$",
                                [("destination_ip", "destination.ip")]
                            ),
                            cached_grok!(
                                "^%{IPV6:destination.ip}(?:(?: port |[p#.]))%{NUMBER:destination.port}$"
                            ),
                            cached_grok!(
                                "^%{IPV6:destination.ip}(?:(?: port |[p#.]))%{POSINT:destination.port}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.ip") {
                    if let Some(val) = event.get("destination.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.ip".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.port") {
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("destination.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("istio.access.downstream.remote_address").cloned() {
                event.set("source.address", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(input) = event.get_string("source.address") {
                        // Grok pattern: ^-$
                        // Grok pattern: ^%{HOSTNAME:source.domain}$
                        // Grok pattern: ^%{IPV4:source.ip}:%{NUMBER:source.port}$
                        // Grok pattern: ^\\[%{IPV6:source.ip}\\]:%{NUMBER:source.port}$
                        // Grok pattern: ^(?P<source_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):%{NUMBER:source.port}$
                        // Grok pattern: ^%{IPV6:source.ip}(?:(?: port |[p#.]))%{NUMBER:source.port}$
                        // Grok pattern: ^%{IPV6:source.ip}(?:(?: port |[p#.]))%{POSINT:source.port}$
                        if !extract_first_match(
                            &[
                                cached_grok!("^-$"),
                                cached_grok!("^%{HOSTNAME:source.domain}$"),
                                cached_grok!("^%{IPV4:source.ip}:%{NUMBER:source.port}$"),
                                cached_grok!("^\\[%{IPV6:source.ip}\\]:%{NUMBER:source.port}$"),
                                cached_grok_mapped!(
                                    "^(?P<source_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):%{NUMBER:source.port}$",
                                    [("source_ip", "source.ip")]
                                ),
                                cached_grok!(
                                    "^%{IPV6:source.ip}(?:(?: port |[p#.]))%{NUMBER:source.port}$"
                                ),
                                cached_grok!(
                                    "^%{IPV6:source.ip}(?:(?: port |[p#.]))%{POSINT:source.port}$"
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
                if event.has_value("source.ip") {
                    if let Some(val) = event.get("source.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("source.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.ip".into(),
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
                if event.has_value("source.port") {
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("source.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_i64("source.port") == Some(0) };
            if _cond {
                if event.remove("source.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.port".into(),
                    });
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("http.request.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            event.set("event.module", json!("istio"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("http.request.method")
                    && event.get_str("http.request.method") != Some("-")
            };
            if _cond {
                event.append_unique("event.category", json!("web"))?;
            }

            let _cond = {
                event.has_value("http.request.method")
                    && event.get_str("http.request.method") != Some("-")
            };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

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

            let _cond = {
                event.has_value("source.ip")
                    && event.has_value("source.port")
                    && event.has_value("destination.ip")
                    && event.has_value("destination.port")
            };
            if _cond {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
            }

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
