// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `urls` pipeline.
pub struct Urls;

impl Transform for Urls {
    fn name(&self) -> &str {
        "urls"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" urls ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" urls ") {
                        remaining = rest;
                    }
                    event.set("message", remaining)?;
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
                if let Some(pos) = remaining.find(" urls ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" urls ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("src", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("src", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("dst", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("dst", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("mac", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" request: ") {
                    event.set("mac", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" request: ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("http.request.method", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                event.set("url.original", remaining)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("mac") {
                    if let Some(input) = event.get_string("mac") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" agent=") {
                            event.set("mac", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" agent=") {
                            remaining = rest;
                        }
                        event.set("user_agent.original", remaining)?;
                    }
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("src") {
                // Grok pattern: ^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$")
                    .extract_into(&input, event)?
                {
                    // Grok pattern: ^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$
                    if !cached_grok!("^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$
                        if !cached_grok_mapped!("^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$", [("_temp_src_ip", "_temp.src_ip")]).extract_into(&input, event)? {
                                // Grok pattern: ^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$
                                if !cached_grok!("^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$").extract_into(&input, event)? {
                                }
                            }
                    }
                }
            }

            if let Some(s) = event.get_string("_temp.src_ip") {
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "_temp.src_ip".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("source.ip", s)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("sport") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "sport".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "sport".into(),
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
                                path: "sport".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("dst") {
                // Grok pattern: ^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$")
                    .extract_into(&input, event)?
                {
                    // Grok pattern: ^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$
                    if !cached_grok!("^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$
                        if !cached_grok_mapped!("^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$", [("_temp_dst_ip", "_temp.dst_ip")]).extract_into(&input, event)? {
                                // Grok pattern: ^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$
                                if !cached_grok!("^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$").extract_into(&input, event)? {
                                }
                            }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("_temp.dst_ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp.dst_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("destination.ip", s)?;
                }
                Ok(())
            })();

            let _cond = {
                event.get_str("dport") != Some("0")
                    && event.get_str("cisco_meraki.event_subtype")
                        != Some("security_filtering_disposition_change")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("dport") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "dport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "dport".into(),
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
                                    path: "dport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.port", converted)?;
                    }
                    Ok(())
                })();
            }

            if let Some(s) = event.get_string("mac") {
                let re = cached_regex!("[-:.]");
                let replaced = re.replace_all(&s, "-").into_owned();
                event.set("cisco_meraki.urls.mac", replaced)?;
            }

            let _cond = {
                !(event
                    .get_str("http.request.method")
                    .is_some_and(|s| s.to_lowercase() == "unknown"))
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("http_access"))?;
            }

            let _cond = {
                event
                    .get_str("http.request.method")
                    .is_some_and(|s| s.to_lowercase() == "unknown")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("http_access_error"))?;
            }

            if event.has("user_agent.original") {
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

            let _cond = {
                event.has_value("url.original")
                    && event.get_str("url.original").is_some_and(|s| !s.is_empty())
            };
            if _cond {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            if event.has("url.domain") {
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
