// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `format_common` pipeline.
pub struct FormatCommon;

impl Transform for FormatCommon {
    fn name(&self) -> &str {
        "format_common"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("traefik.access.user_identifier", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" [") else { break 'dissect false };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] \"") else { break 'dissect false };
                        captured.push(("traefik.access.time", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] \"") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("http.request.method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" HTTP/") else { break 'dissect false };
                        captured.push(("url.original", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" HTTP/") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("\" ") else { break 'dissect false };
                        captured.push(("http.version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("http.response.status_code", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("traefik.access.message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }

            let _cond = { event.has_value("http.response.status_code") && event.get_str("http.response.status_code") == Some("-") };
            if _cond {
                if event.remove("http.response.status_code").is_none() {
                    return Err(TransformError::FieldNotFound { path: "http.response.status_code".into() });
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
                    return Err(TransformError::FieldNotFound { path: "traefik.access.time".into() });
                }

                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(%{IP:source.ip}|%{HOSTNAME:source.domain})$
                    if !cached_grok!("^(%{IP:source.ip}|%{HOSTNAME:source.domain})$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            if event.has_value("traefik.access.service.address") {
                if let Some(input) = event.get_string("traefik.access.service.address") {
                    // Grok pattern: ^(https?://)?%{DATA:destination.address}$
                    if !cached_grok!("^(https?://)?%{DATA:destination.address}$").extract_into(&input, event)? {
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
                painless_exec_plan_params(event, cached_painless!(r#"ctx.event.duration = Math.round(ctx.temp.duration * params.scale)"#), cached_params!("{\"scale\":1000000}"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
