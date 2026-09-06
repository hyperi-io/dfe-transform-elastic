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

            if event.has_value("@timestamp") {
                event.rename("@timestamp", "event.created")?;
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
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
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("traefik.access.user_identifier", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" [") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" [") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("] \"") else {
                            break 'dissect false;
                        };
                        captured.push(("traefik.access.time", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] \"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("http.request.method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" HTTP/") else {
                            break 'dissect false;
                        };
                        captured.push(("url.original", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" HTTP/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("\" ") else {
                            break 'dissect false;
                        };
                        captured.push(("http.version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("http.response.status_code", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("traefik.access.message", remaining));
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
                let _cond = {
                    event.has_value("http.response.status_code")
                        && event.get_str("http.response.status_code") == Some("-")
                };
                if _cond {
                    if event.remove("http.response.status_code").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "http.response.status_code".into(),
                        });
                    }
                }
                if event.has_value("traefik.access.message") {
                    if let Some(input) = event.get_string("traefik.access.message") {
                        // Grok pattern: (?:%{NUMBER:http.response.body.bytes:long}|-)( (?:\"%{DATA:http.request.referrer}\"|-)?( (?:\"%{DATA:user_agent.original}\"|-)?)?( (?:%{NUMBER:traefik.access.request_count:long}|-)?)?( (?:\"%{DATA:traefik.access.router.name}\"|-)?)?( \"%{DATA:traefik.access.service.address}\")?( %{NUMBER:temp.duration:long}ms)?)?
                        if !cached_grok!("(?:%{NUMBER:http.response.body.bytes:long}|-)( (?:\"%{DATA:http.request.referrer}\"|-)?( (?:\"%{DATA:user_agent.original}\"|-)?)?( (?:%{NUMBER:traefik.access.request_count:long}|-)?)?( (?:\"%{DATA:traefik.access.router.name}\"|-)?)?( \"%{DATA:traefik.access.service.address}\")?( %{NUMBER:temp.duration:long}ms)?)?").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                event.remove("traefik.access.message");
                if let Some(date_str) = event.get_as_string("traefik.access.time") {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:H:m:s Z"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "traefik.access.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                if event.remove("traefik.access.time").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "traefik.access.time".into(),
                    });
                }
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(%{IP:source.ip}|%{HOSTNAME:source.domain})$
                    if !cached_grok!("^(%{IP:source.ip}|%{HOSTNAME:source.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                if event.has_value("traefik.access.service.address") {
                    if let Some(input) = event.get_string("traefik.access.service.address") {
                        // Grok pattern: ^(https?://)?%{DATA:destination.address}$
                        if !cached_grok!("^(https?://)?%{DATA:destination.address}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("destination.address") {
                        // Grok pattern: ^(%{IP:destination.ip}|%{HOSTNAME:destination.domain})(:%{POSINT:destination.port:long})?$
                        if !cached_grok!("^(%{IP:destination.ip}|%{HOSTNAME:destination.domain})(:%{POSINT:destination.port:long})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("temp.duration") };
                if _cond {
                    // Painless script
                    // Source: ctx.event.duration = Math.round(ctx.temp.duration * params.scale)
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event.duration = Math.round(ctx.temp.duration * params.scale)"#
                        ),
                        cached_params!("{\"scale\":1000000}"),
                    )?;
                }
                // End nested pipeline: "format-common"
            }

            let _cond = { event.get_str("first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "format-json"
                parse_json_field(event, "event.original", "temp")?;
                if event.has_value("temp.time") {
                    event.rename("temp.time", "@timestamp")?;
                }
                let _cond = { event.has_value("temp.StartUTC") };
                if _cond {
                    if let Some(v) = event.get("temp.StartUTC").cloned() {
                        event.set("@timestamp", v)?;
                    }
                }
                let _cond =
                    { !event.has_value("temp.StartUTC") && event.has_value("temp.StarLocal") };
                if _cond {
                    if let Some(v) = event.get("temp.StartLocal").cloned() {
                        event.set("@timestamp", v)?;
                    }
                }
                if event.has_value("temp.Duration") {
                    event.rename("temp.Duration", "event.duration")?;
                }
                if event.has_value("temp.RouterName") {
                    event.rename("temp.RouterName", "traefik.access.router.name")?;
                }
                let _cond = { !event.has_value("traefik.access.router.name") };
                if _cond {
                    if event.has_value("temp.FrontendName") {
                        event.rename("temp.FrontendName", "traefik.access.router.name")?;
                    }
                }
                if event.has_value("temp.ClientUsername") {
                    event.rename("temp.ClientUsername", "user.name")?;
                }
                if event.has_value("temp.ServiceName") {
                    event.rename("temp.ServiceName", "observer.egress.interface.name")?;
                }
                let _cond = { !event.has_value("observer.egress.interface.name") };
                if _cond {
                    if event.has_value("temp.BackendName") {
                        event.rename("temp.BackendName", "observer.egress.interface.name")?;
                    }
                }
                let _cond = { !event.has_value("temp.ServiceURL") };
                if _cond {
                    if event.has_value("temp.BackendURL") {
                        event.rename("temp.BackendURL", "temp.ServiceURL")?;
                    }
                }
                if event.has_value("temp.ServiceURL.Opaque") {
                    event.rename(
                        "temp.ServiceURL.Opaque",
                        "traefik.access.service.url.opaque",
                    )?;
                }
                if event.has_value("temp.ServiceURL.User") {
                    event.rename("temp.ServiceURL.User", "traefik.access.service.url.user")?;
                }
                if event.has_value("temp.ServiceURL.Host") {
                    event.rename("temp.ServiceURL.Host", "traefik.access.service.url.domain")?;
                }
                if event.has_value("temp.ServiceURL.Path") {
                    event.rename("temp.ServiceURL.Path", "traefik.access.service.url.path")?;
                }
                if event.has_value("temp.ServiceURL.RawPath") {
                    event.rename(
                        "temp.ServiceURL.RawPath",
                        "traefik.access.service.url.raw_path",
                    )?;
                }
                if event.has_value("temp.ServiceURL.ForceQuery") {
                    event.rename(
                        "temp.ServiceURL.ForceQuery",
                        "traefik.access.service.url.force_query",
                    )?;
                }
                if event.has_value("temp.ServiceURL.RawQuery") {
                    event.rename(
                        "temp.ServiceURL.RawQuery",
                        "traefik.access.service.url.raw_query",
                    )?;
                }
                if event.has_value("temp.ServiceURL.Fragment") {
                    event.rename(
                        "temp.ServiceURL.Fragment",
                        "traefik.access.service.url.fragment",
                    )?;
                }
                if event.has_value("temp.RequestCount") {
                    event.rename("temp.RequestCount", "traefik.access.request_count")?;
                }
                if event.has_value("temp.ClientAddr") {
                    event.rename("temp.ClientAddr", "source.address")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.address") {
                        if let Some(input) = event.get_string("source.address") {
                            // Grok pattern: ^\\\\[?%{IP:source.ip}\\\\]?:%{POSINT:source.port}$
                            if !cached_grok!("^\\\\[?%{IP:source.ip}\\\\]?:%{POSINT:source.port}$")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = { !event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("temp.ClientHost") {
                            if let Some(val) = event.get("temp.ClientHost") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.ClientHost".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("source.ip")
                        && !condition_eq(event.get("source.ip"), event.get("temp.ClientHost"))
                };
                if _cond {
                    if event.has_value("temp.ClientHost") {
                        if let Some(val) = event.get("temp.ClientHost") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "temp.ClientHost".into(),
                                    message,
                                }
                            })?;
                            event.set("network.forwarded_ip", converted)?;
                        }
                    }
                }
                let _cond = { event.has_value("network.forwarded_ip") };
                if _cond {
                    event.append(
                        "related.ip",
                        json!(
                            event
                                .get("network.forwarded_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { !event.has_value("source.port") };
                if _cond {
                    if event.has_value("temp.ClientPort") {
                        event.rename("temp.ClientPort", "source.port")?;
                    }
                }
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("temp.ServiceAddr") {
                        if let Some(input) = event.get_string("temp.ServiceAddr") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(":") else {
                                    break 'dissect false;
                                };
                                captured.push(("destination.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(":") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("destination.port", remaining));
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
                let _cond = { !event.has_value("temp.SourceAddr") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("temp.BackendAddr") {
                            if let Some(input) = event.get_string("temp.BackendAddr") {
                                let mut remaining: &str = &input;
                                let mut captured: Vec<(&str, &str)> = Vec::new();
                                let matched = 'dissect: {
                                    let Some(pos) = remaining.find(":") else {
                                        break 'dissect false;
                                    };
                                    captured.push(("destination.ip", &remaining[..pos]));
                                    remaining = &remaining[pos..];
                                    let Some(rest) = remaining.strip_prefix(":") else {
                                        break 'dissect false;
                                    };
                                    remaining = rest;
                                    captured.push(("destination.port", remaining));
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
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    if let Some(v) = event.get("destination.ip").cloned() {
                        event.set("destination.address", v)?;
                    }
                }
                if event.has_value("temp.RequestPath") {
                    event.rename("temp.RequestPath", "url.path")?;
                }
                if event.has_value("temp.RequestMethod") {
                    event.rename("temp.RequestMethod", "http.request.method")?;
                }
                if event.has_value("temp.RequestHost") {
                    event.rename("temp.RequestHost", "url.domain")?;
                }
                let _cond = { !event.has_value("url.domain") };
                if _cond {
                    if event.has_value("temp.RequestAddr") {
                        event.rename("temp.RequestAddr", "url.domain")?;
                    }
                }
                let _cond = {
                    event.has_value("temp.RequestPort")
                        && event.get_str("temp.RequestPort") != Some("-")
                };
                if _cond {
                    if let Some(val) = event.get("temp.RequestPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "temp.RequestPort".into(),
                                message,
                            }
                        })?;
                        event.set("url.port", converted)?;
                    }
                }
                if let Some(input) = event.get_string("temp.RequestProtocol") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("HTTP/") else {
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
                    } else {
                        return Err(TransformError::ParseError {
                            path: "temp.RequestProtocol".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
                if event.has_value("temp.RequestScheme") {
                    event.rename("temp.RequestScheme", "url.scheme")?;
                }
                let _cond = { event.has_value("url.scheme") };
                if _cond {
                    event.append(
                        "url.original",
                        json!(format!(
                            "{}://",
                            event
                                .get("url.scheme")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.has_value("url.domain") };
                if _cond {
                    event.append(
                        "url.original",
                        json!(
                            event
                                .get("url.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("url.port") };
                if _cond {
                    event.append(
                        "url.original",
                        json!(format!(
                            ":{}",
                            event
                                .get("url.port")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.has_value("url.path") };
                if _cond {
                    event.append(
                        "url.original",
                        json!(
                            event
                                .get("url.path")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let joined = event.get("url.original").and_then(|v| join_values(v, ""));
                if let Some(joined) = joined {
                    event.set("url.original", json!(joined))?;
                }
                let _cond = { event.get_str("url.original") == Some("") };
                if _cond {
                    if event.remove("url.original").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "url.original".into(),
                        });
                    }
                }
                let _cond = {
                    event.has_value("temp.DownstreamStatus")
                        && event.get_str("temp.DownstreamStatus") != Some("-")
                };
                if _cond {
                    event.rename("temp.DownstreamStatus", "http.response.status_code")?;
                }
                if event.has_value("temp.DownstreamContentSize") {
                    event.rename("temp.DownstreamContentSize", "http.response.body.bytes")?;
                }
                if event.has_value("temp.request_User-Agent") {
                    event.rename("temp.request_User-Agent", "user_agent.original")?;
                }
                if event.has_value("temp.OriginContentSize") {
                    event.rename(
                        "temp.OriginContentSize",
                        "traefik.access.origin.content_size",
                    )?;
                }
                if event.has_value("temp.OriginDuration") {
                    event.rename("temp.OriginDuration", "traefik.access.origin.duration")?;
                }
                let _cond = {
                    event.has_value("temp.OriginStatus")
                        && event.get_str("temp.OriginStatus") != Some("-")
                };
                if _cond {
                    event.rename("temp.OriginStatus", "traefik.access.origin.status_code")?;
                }
                if event.has_value("temp.Overhead") {
                    event.rename("temp.Overhead", "traefik.access.overhead")?;
                }
                if event.has_value("temp.RequestContentSize") {
                    event.rename("temp.RequestContentSize", "http.request.body.bytes")?;
                }
                if event.has_value("temp.RetryAttempts") {
                    event.rename("temp.RetryAttempts", "traefik.access.retry_attempts")?;
                }
                if event.has_value("temp.request_User-Agent") {
                    if let Some(ua_str) = event.get_string("temp.request_User-Agent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            let device_kind = ua.device_type();
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
                            if let Some(kind) = device_kind {
                                event.set("user_agent.device.type", json!(kind))?;
                            }
                        }
                    }
                }
                // Painless script
                // Source: Map downstream_headers = new HashMap();\nMap origin_headers = new HashMap();\nMap request_headers = new HashMap();\n\n// Get the headers\nfor (fieldname in ctx?.temp?.keySet()){\n  if (fieldname.startsWith('downstream_')){\n    downstream_headers.put(fieldname.replace('downstream_', '').toLowerCase(), ctx.temp[fieldname]);\n  }\n  else if (fieldname.startsWith('request_')){\n    request_headers.put(fieldname.replace('request_', '').toLowerCase(), ctx.temp[fieldname]);\n  }\n  else if (fieldname.startsWith('origin_')){\n    origin_headers.put(fieldname.replace('origin_', '').toLowerCase(), ctx.temp[fieldname]);\n  }\n}\n\nif (!request_headers.isEmpty()){\n  // Make sure http.request object exists\n  if (ctx.http == null){\n    ctx.put('http', new HashMap());\n  }\n  if (ctx.http.request == null){\n    ctx.http.put('request', new HashMap());\n  }\n\n  // Add headers\n  ctx.http.request.put('headers', request_headers);\n}\n\nif (!downstream_headers.isEmpty()){\n  // Make sure http.response object exists\n  if (ctx.http == null){\n    ctx.put('http', new HashMap());\n  }\n  if (ctx.http.response == null){\n    ctx.http.put('response', new HashMap());\n  }\n  // Add headers\n  ctx.http.response.put('headers', downstream_headers);\n}\n\nif (!origin_headers.isEmpty()){\n  // Make sure traefik.access.origin object exists\n  if (ctx.traefik == null){\n    ctx.put('traefik', new HashMap());\n  }\n  if (ctx.traefik.access == null){\n    ctx.traefik.put('access', new HashMap());\n  }\n  if (ctx.traefik.access.origin == null){\n    ctx.traefik.access.put('origin', new HashMap());\n  }\n  // Add headers\n  ctx.traefik.access.origin.put('headers', origin_headers);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map downstream_headers = new HashMap();\nMap origin_headers = new HashMap();\nMap request_headers = new HashMap();\n\n// Get the headers\nfor (fieldname in ctx?.temp?.keySet()){\n  if (fieldname.startsWith('downstream_')){\n    downstream_headers.put(fieldname.replace('downstream_', '').toLowerCase(), ctx.temp[fieldname]);\n  }\n  else if (fieldname.startsWith('request_')){\n    request_headers.put(fieldname.replace('request_', '').toLowerCase(), ctx.temp[fieldname]);\n  }\n  else if (fieldname.startsWith('origin_')){\n    origin_headers.put(fieldname.replace('origin_', '').toLowerCase(), ctx.temp[fieldname]);\n  }\n}\n\nif (!request_headers.isEmpty()){\n  // Make sure http.request object exists\n  if (ctx.http == null){\n    ctx.put('http', new HashMap());\n  }\n  if (ctx.http.request == null){\n    ctx.http.put('request', new HashMap());\n  }\n\n  // Add headers\n  ctx.http.request.put('headers', request_headers);\n}\n\nif (!downstream_headers.isEmpty()){\n  // Make sure http.response object exists\n  if (ctx.http == null){\n    ctx.put('http', new HashMap());\n  }\n  if (ctx.http.response == null){\n    ctx.http.put('response', new HashMap());\n  }\n  // Add headers\n  ctx.http.response.put('headers', downstream_headers);\n}\n\nif (!origin_headers.isEmpty()){\n  // Make sure traefik.access.origin object exists\n  if (ctx.traefik == null){\n    ctx.put('traefik', new HashMap());\n  }\n  if (ctx.traefik.access == null){\n    ctx.traefik.put('access', new HashMap());\n  }\n  if (ctx.traefik.access.origin == null){\n    ctx.traefik.access.put('origin', new HashMap());\n  }\n  // Add headers\n  ctx.traefik.access.origin.put('headers', origin_headers);\n}\n"#
                    ),
                )?;
                if event.has_value("temp.TLSCipher") {
                    event.rename("temp.TLSCipher", "tls.cipher")?;
                }
                if event.has_value("temp.TLSVersion") {
                    event.rename("temp.TLSVersion", "tls.version")?;
                }
                if event.has_value("temp.GzipRatio") {
                    event.rename("temp.GzipRatio", "traefik.access.gzip_ratio")?;
                }
                if event.has_value("temp.entryPointName") {
                    event.rename("temp.entryPointName", "observer.ingress.interface.name")?;
                }
                if let Some(v) = event
                    .get("temp.level")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("log.level", v)?;
                }
                if let Some(v) = event
                    .get("temp.msg")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                event.set("observer.vendor", json!("traefik"))?;
                event.set("observer.product", json!("traefik"))?;
                event.set("observer.type", json!("proxy"))?;
                // End nested pipeline: "format-json"
            }

            if event.remove("first_char").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "first_char".into(),
                });
            }

            event.remove("temp");

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

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("-") };
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

            event.set("network.transport", json!("tcp"))?;

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

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
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
