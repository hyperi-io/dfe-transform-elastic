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

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.append("event.type", json!("access"))?;

            event.set("cloud.provider", json!("aws"))?;

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

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("#"))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if let Some(csv_str) = event.get_string("event.original") {
                let csv_str = csv_close_quote_gap(&csv_str, '\t', '\"');
                let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b'\t')
                    .quote(b'\"')
                    .has_headers(false)
                    .from_reader(csv_str.as_bytes());
                if let Some(Ok(record)) = rdr.records().next() {
                    if let Some(val) = record.get(0) {
                        if !val.is_empty() {
                            event.set("_tmp.date", val)?;
                        }
                    }
                    if let Some(val) = record.get(1) {
                        if !val.is_empty() {
                            event.set("_tmp.time", val)?;
                        }
                    }
                    if let Some(val) = record.get(2) {
                        if !val.is_empty() {
                            event.set("_tmp.x_edge_location", val)?;
                        }
                    }
                    if let Some(val) = record.get(3) {
                        if !val.is_empty() {
                            event.set("_tmp.sc_bytes", val)?;
                        }
                    }
                    if let Some(val) = record.get(4) {
                        if !val.is_empty() {
                            event.set("_tmp.c_ip", val)?;
                        }
                    }
                    if let Some(val) = record.get(5) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_method", val)?;
                        }
                    }
                    if let Some(val) = record.get(6) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_host", val)?;
                        }
                    }
                    if let Some(val) = record.get(7) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_uri_stem", val)?;
                        }
                    }
                    if let Some(val) = record.get(8) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_status", val)?;
                        }
                    }
                    if let Some(val) = record.get(9) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_referer", val)?;
                        }
                    }
                    if let Some(val) = record.get(10) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_user_agent", val)?;
                        }
                    }
                    if let Some(val) = record.get(11) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_uri_query", val)?;
                        }
                    }
                    if let Some(val) = record.get(12) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_cookie", val)?;
                        }
                    }
                    if let Some(val) = record.get(13) {
                        if !val.is_empty() {
                            event.set("_tmp.x_edge_result_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(14) {
                        if !val.is_empty() {
                            event.set("_tmp.x_edge_request_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(15) {
                        if !val.is_empty() {
                            event.set("_tmp.x_host_header", val)?;
                        }
                    }
                    if let Some(val) = record.get(16) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_protocol", val)?;
                        }
                    }
                    if let Some(val) = record.get(17) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_bytes", val)?;
                        }
                    }
                    if let Some(val) = record.get(18) {
                        if !val.is_empty() {
                            event.set("_tmp.time_taken", val)?;
                        }
                    }
                    if let Some(val) = record.get(19) {
                        if !val.is_empty() {
                            event.set("_tmp.x_forwarded_for", val)?;
                        }
                    }
                    if let Some(val) = record.get(20) {
                        if !val.is_empty() {
                            event.set("_tmp.ssl_protocol", val)?;
                        }
                    }
                    if let Some(val) = record.get(21) {
                        if !val.is_empty() {
                            event.set("_tmp.ssl_cipher", val)?;
                        }
                    }
                    if let Some(val) = record.get(22) {
                        if !val.is_empty() {
                            event.set("_tmp.x_edge_response_result_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(23) {
                        if !val.is_empty() {
                            event.set("_tmp.cs_protocol_version", val)?;
                        }
                    }
                    if let Some(val) = record.get(24) {
                        if !val.is_empty() {
                            event.set("_tmp.fle_status", val)?;
                        }
                    }
                    if let Some(val) = record.get(25) {
                        if !val.is_empty() {
                            event.set("_tmp.fle_encrypted_fields", val)?;
                        }
                    }
                    if let Some(val) = record.get(26) {
                        if !val.is_empty() {
                            event.set("_tmp.c_port", val)?;
                        }
                    }
                    if let Some(val) = record.get(27) {
                        if !val.is_empty() {
                            event.set("_tmp.time_to_first_byte", val)?;
                        }
                    }
                    if let Some(val) = record.get(28) {
                        if !val.is_empty() {
                            event.set("_tmp.x_edge_detailed_result_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(29) {
                        if !val.is_empty() {
                            event.set("_tmp.sc_content_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(30) {
                        if !val.is_empty() {
                            event.set("_tmp.sc_content_len", val)?;
                        }
                    }
                    if let Some(val) = record.get(31) {
                        if !val.is_empty() {
                            event.set("_tmp.sc_range_start", val)?;
                        }
                    }
                    if let Some(val) = record.get(32) {
                        if !val.is_empty() {
                            event.set("_tmp.sc_range_end", val)?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.date") && event.has_value("_tmp.time") };
            if _cond {
                // Painless script
                // Source: ctx._tmp.timestamp = ctx._tmp.date + 'T' + ctx._tmp.time;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(r#"ctx._tmp.timestamp = ctx._tmp.date + 'T' + ctx._tmp.time;"#),
                )?;
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })();
            }

            if event.has("_tmp.x_edge_location") {
                event.rename("_tmp.x_edge_location", "aws.cloudfront.edge_location")?;
            }

            if event.has_value("_tmp.sc_bytes") {
                if let Some(val) = event.get("_tmp.sc_bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_tmp.sc_bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_tmp.sc_bytes".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.sc_bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("http.response.bytes", converted)?;
                }
            }

            if event.has("_tmp.c_ip") {
                event.rename("_tmp.c_ip", "source.address")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(s) = event.get_string("source.address") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "source.address".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.ip", s)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("_tmp.cs_method") {
                event.rename("_tmp.cs_method", "http.request.method")?;
            }

            if event.has("_tmp.cs_host") {
                event.rename("_tmp.cs_host", "aws.cloudfront.domain")?;
            }

            let _cond = { event.has_value("aws.cloudfront.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("aws.cloudfront.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("_tmp.cs_uri_stem") {
                event.rename("_tmp.cs_uri_stem", "url.path")?;
            }

            if event.has_value("_tmp.cs_status") {
                if let Some(val) = event.get("_tmp.cs_status") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_tmp.cs_status".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_tmp.cs_status".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.cs_status".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("http.response.status_code", converted)?;
                }
            }

            let _cond = {
                event.has_value("_tmp.cs_referer") && event.get_str("_tmp.cs_referer") != Some("-")
            };
            if _cond {
                if event.has("_tmp.cs_referer") {
                    event.rename("_tmp.cs_referer", "http.request.referrer")?;
                }
            }

            if event.has_value("_tmp.cs_user_agent") {
                if let Some(s) = event.get_string("_tmp.cs_user_agent") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("_tmp.cs_user_agent", json!(decoded))?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.cs_user_agent".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("_tmp.cs_user_agent") {
                if let Some(ua_str) = event.get_string("_tmp.cs_user_agent") {
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

            let _cond = {
                event.has_value("_tmp.cs_uri_query")
                    && event.get_str("_tmp.cs_uri_query") != Some("-")
            };
            if _cond {
                event.rename("_tmp.cs_uri_query", "url.query")?;
            }

            let _cond = {
                event.has_value("_tmp.cs_cookie") && event.get_str("_tmp.cs_cookie") != Some("-")
            };
            if _cond {
                event.rename("_tmp.cs_cookie", "aws.cloudfront.cookies")?;
            }

            if event.has("_tmp.x_edge_result_type") {
                event.rename("_tmp.x_edge_result_type", "aws.cloudfront.edge_result_type")?;
            }

            if event.has("_tmp.x_edge_request_id") {
                event.rename("_tmp.x_edge_request_id", "http.request.id")?;
            }

            if event.has("_tmp.x_host_header") {
                event.rename("_tmp.x_host_header", "destination.address")?;
            }

            if let Some(v) = event
                .get("destination.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
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

            if event.has("_tmp.cs_protocol") {
                event.rename("_tmp.cs_protocol", "network.protocol")?;
            }

            if event.has_value("_tmp.cs_bytes") {
                if let Some(val) = event.get("_tmp.cs_bytes") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_tmp.cs_bytes".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_tmp.cs_bytes".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.cs_bytes".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("http.request.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("_tmp.time_taken") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = (Long)(Float.parseFloat(ctx._tmp.time_taken) * params.S_TO_NS);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"ctx.event.duration = (Long)(Float.parseFloat(ctx._tmp.time_taken) * params.S_TO_NS);"#
                    ),
                    cached_params!("{\"S_TO_NS\":1000000000}"),
                )?;
            }

            let _cond = {
                event.has_value("_tmp.x_forwarded_for")
                    && event.get_str("_tmp.x_forwarded_for") != Some("-")
            };
            if _cond {
                if let Some(s) = event.get_string("_tmp.x_forwarded_for") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("_tmp.split_x_forwarded_for", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("_tmp.split_x_forwarded_for") };
            if _cond {
                // Painless script
                // Source: for (int i = 0; i < ctx._tmp.split_x_forwarded_for.length; i++) {\n  ctx._tmp.split_x_forwarded_for[i] = ctx._tmp.split_x_forwarded_for[i].trim();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"for (int i = 0; i < ctx._tmp.split_x_forwarded_for.length; i++) {\n  ctx._tmp.split_x_forwarded_for[i] = ctx._tmp.split_x_forwarded_for[i].trim();\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("_tmp.split_x_forwarded_for")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.split_x_forwarded_for").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // SKIPPED: nested pipeline "pipeline_process_ip" is not in this pipeline set
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.split_x_forwarded_for", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("_tmp.invalid_ips") };
            if _cond {
                event.append(
                    "error.message",
                    json!(format!(
                        "Invalid IP addresses: {}",
                        event
                            .get("_tmp.invalid_ips")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("_tmp.split_x_forwarded_for")
                    && event.get_str("_tmp.split_x_forwarded_for") != Some("-")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.get('network') == null) {\n    ctx['network'] = new HashMap();\n  }\nfor (String item : ctx._tmp.split_x_forwarded_for ) {\n  // edge case observed in the wild. e.g. 'localhost:8081'\n  if (item.startsWith('localhost')) {\n  if (ctx.network.forwarded_ip == null) {\n    ctx['network']['forwarded_ip'] = new ArrayList();\n  }\n    ctx['network']['forwarded_ip'].add('127.0.0.1');\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.get('network') == null) {\n    ctx['network'] = new HashMap();\n  }\nfor (String item : ctx._tmp.split_x_forwarded_for ) {\n  // edge case observed in the wild. e.g. 'localhost:8081'\n  if (item.startsWith('localhost')) {\n  if (ctx.network.forwarded_ip == null) {\n    ctx['network']['forwarded_ip'] = new ArrayList();\n  }\n    ctx['network']['forwarded_ip'].add('127.0.0.1');\n  }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("network.forwarded_ip") {
                if let Some(Value::Array(items)) = event.get("network.forwarded_ip").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("network.forwarded_ip", Value::Array(out))?;
                }
            }

            if event.has_value("_tmp.ssl_protocol") {
                if let Some(input) = event.get_string("_tmp.ssl_protocol") {
                    // Grok pattern: (-|(?P<tls_version_protocol>(?:(TLS|SSL)))v%{NUMBER:tls.version})
                    if !cached_grok_mapped!(
                        "(-|(?P<tls_version_protocol>(?:(TLS|SSL)))v%{NUMBER:tls.version})",
                        [("tls_version_protocol", "tls.version_protocol")]
                    )
                    .extract_into(&input, event)?
                    {}
                }
            }

            if event.has_value("tls.version_protocol") {
                if let Some(s) = event.get_string("tls.version_protocol") {
                    let lowered = s.to_lowercase();
                    event.set("tls.version_protocol", lowered)?;
                }
            }

            let _cond = {
                event.has_value("_tmp.ssl_cipher") && event.get_str("_tmp.ssl_cipher") != Some("-")
            };
            if _cond {
                if event.has("_tmp.ssl_cipher") {
                    event.rename("_tmp.ssl_cipher", "tls.cipher")?;
                }
            }

            if event.has("_tmp.x_edge_response_result_type") {
                event.rename(
                    "_tmp.x_edge_response_result_type",
                    "aws.cloudfront.edge_response_result_type",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.cs_protocol_version") {
                    if let Some(input) = event.get_string("_tmp.cs_protocol_version") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
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
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("_tmp.fle_status") && event.get_str("_tmp.fle_status") != Some("-")
            };
            if _cond {
                event.rename("_tmp.fle_status", "aws.cloudfront.fle_status")?;
            }

            let _cond = {
                event.has_value("_tmp.encrypted_fields")
                    && event.get_str("_tmp.encrypted_fields") != Some("-")
            };
            if _cond {
                event.rename(
                    "_tmp.fle_encrypted_fields",
                    "aws.cloudfront.fle_encrypted_fields",
                )?;
            }

            let _cond =
                { event.has_value("_tmp.c_port") && event.get_str("_tmp.c_port") != Some("-") };
            if _cond {
                if let Some(val) = event.get("_tmp.c_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "_tmp.c_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "_tmp.c_port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.c_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
            }

            let _cond = {
                event.has_value("_tmp.time_to_first_byte")
                    && event.get_str("_tmp.time_to_first_byte") != Some("-")
            };
            if _cond {
                if let Some(val) = event.get("_tmp.time_to_first_byte") {
                    let converted = match val {
                        Value::String(s) => json!(s.trim().parse::<f64>().map_err(|_| {
                            TransformError::ParseError {
                                path: "_tmp.time_to_first_byte".into(),
                                message: format!("cannot convert '{}' to float", s),
                            }
                        })?),
                        Value::Number(n) => json!(n.as_f64().unwrap_or(0.0)),
                        Value::Bool(b) => json!(if *b { 1.0 } else { 0.0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.time_to_first_byte".into(),
                                message: "cannot convert to float".into(),
                            });
                        }
                    };
                    event.set("aws.cloudfront.time_to_first_byte", converted)?;
                }
            }

            let _cond = {
                event.has_value("_tmp.x_edge_detailed_result_type")
                    && event.get_str("_tmp.x_edge_detailed_result_type") != Some("-")
            };
            if _cond {
                event.rename(
                    "_tmp.x_edge_detailed_result_type",
                    "aws.cloudfront.edge_detailed_result_type",
                )?;
            }

            if event.has("_tmp.sc_content_type") {
                event.rename("_tmp.sc_content_type", "http.response.mime_type")?;
            }

            let _cond = {
                event.has_value("_tmp.sc_content_len")
                    && event.get_str("_tmp.sc_content_len") != Some("-")
            };
            if _cond {
                if event.has_value("_tmp.sc_content_len") {
                    if let Some(val) = event.get("_tmp.sc_content_len") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_tmp.sc_content_len".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_tmp.sc_content_len".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.sc_content_len".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("http.response.body.bytes", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("_tmp.sc_range_start")
                    && event.get_str("_tmp.sc_range_start") != Some("-")
            };
            if _cond {
                if event.has_value("_tmp.sc_range_start") {
                    if let Some(val) = event.get("_tmp.sc_range_start") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_tmp.sc_range_start".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_tmp.sc_range_start".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.sc_range_start".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("aws.cloudfront.range_start", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("_tmp.sc_range_end")
                    && event.get_str("_tmp.sc_range_end") != Some("-")
            };
            if _cond {
                if event.has_value("_tmp.sc_range_end") {
                    if let Some(val) = event.get("_tmp.sc_range_end") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_tmp.sc_range_end".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "_tmp.sc_range_end".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.sc_range_end".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("aws.cloudfront.range_end", converted)?;
                    }
                }
            }

            // Painless script
            // Source: def full = \"\";\nif(ctx.network?.protocol != null && ctx.network?.protocol != \"\") {\n    full += ctx.network.protocol+\"://\";\n}\nif(ctx.destination?.domain != null && ctx.destination?.domain != \"\") {\n    full += ctx.destination.domain;\n}\nif(ctx.url?.path != null && ctx.url?.path != \"\") {\n    full += ctx.url.path;\n}\nif(ctx.url?.query != null && ctx.url?.query != \"\") {\n    full += \"?\"+ctx.url.query;\n}\nif(full != \"\") {\n    ctx._tmp.url_full = full\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"def full = \"\";\nif(ctx.network?.protocol != null && ctx.network?.protocol != \"\") {\n    full += ctx.network.protocol+\"://\";\n}\nif(ctx.destination?.domain != null && ctx.destination?.domain != \"\") {\n    full += ctx.destination.domain;\n}\nif(ctx.url?.path != null && ctx.url?.path != \"\") {\n    full += ctx.url.path;\n}\nif(ctx.url?.query != null && ctx.url?.query != \"\") {\n    full += \"?\"+ctx.url.query;\n}\nif(full != \"\") {\n    ctx._tmp.url_full = full\n}\n"#
                ),
            )?;

            if event.has_value("_tmp.url_full") {
                uri_parts(event, "_tmp.url_full", "url", false, false)?;
            }

            if event.has("_tmp.url_full") {
                event.rename("_tmp.url_full", "url.full")?;
            }

            if event.has_value("url.domain") {
                if let Some(domain_str) = event.get_string("url.domain") {
                    let domain = domain_str.to_string();
                    event.set("url.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set("url.registered_domain", json!(rd.registered_domain))?;
                        event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("url.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if let Some(v) = event
                .get("http.request.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event
                    .get_i64("http.response.status_code")
                    .is_some_and(|n| n >= 400)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event
                    .get_i64("http.response.status_code")
                    .is_some_and(|n| n < 400)
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n > 000)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_i64("http.response.status_code") == Some(000) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
