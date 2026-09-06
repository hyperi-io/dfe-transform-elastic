// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `json` pipeline.
pub struct Json;

impl Transform for Json {
    fn name(&self) -> &str {
        "json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                parse_json_field(event, "message", "json")?;

                if event.remove("message").is_none() {
                    return Err(TransformError::FieldNotFound { path: "message".into() });
                }

                event.rename("json.message", "message")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.kubernetes", "kubernetes")?;
                Ok(())
            })();

                if event.remove("json").is_none() {
                    return Err(TransformError::FieldNotFound { path: "json".into() });
                }

                // Begin nested pipeline: "plaintext"
                // Painless script, resolved to its runners at generation time
                // Source: if (ctx.message.charAt(0) == (char)(\"[\")) {\n  ctx.temp_message = \"ACCESS \" + ctx.message;\n} else if (ctx.message.substring(0, 7) == \"ACCESS \") {\n  ctx.temp_message = ctx.message;\n} else {\n  throw new Exception(\"Not a valid envoyproxy access log\");\n}
                ensure_prefix(event, &EnsurePrefix::new("message".into(), "temp_message".into(), '[', "ACCESS ".into()));
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("temp_message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                let Some(pos) = remaining.find(" [") else { break 'dissect false };
                captured.push(("envoyproxy.log.log_type", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" [") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("] \"") else { break 'dissect false };
                captured.push(("timestamp", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("] \"") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("method", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("path", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("\" ") else { break 'dissect false };
                captured.push(("proto", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("\" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("response_code", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("envoyproxy.log.response_flags", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("bytes_received", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("bytes_sent", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" ") else { break 'dissect false };
                captured.push(("duration", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find(" \"") else { break 'dissect false };
                captured.push(("upstream_service_time", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix(" \"") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("\" \"") else { break 'dissect false };
                captured.push(("source.address", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("\" \"") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("\" \"") else { break 'dissect false };
                captured.push(("user_agent.original", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("\" \"") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("\" \"") else { break 'dissect false };
                captured.push(("envoyproxy.log.request_id", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("\" \"") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("\" \"") else { break 'dissect false };
                captured.push(("envoyproxy.log.authority", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("\" \"") else { break 'dissect false };
                remaining = rest;
                let Some(pos) = remaining.find("\"") else { break 'dissect false };
                captured.push(("dest", &remaining[..pos]));
                remaining = &remaining[pos..];
                let Some(rest) = remaining.strip_prefix("\"") else { break 'dissect false };
                remaining = rest;
                true
                };
                if matched {
                for (path, value) in captured {
                event.set(path, value)?;
                }
                }
                else {
                return Err(TransformError::ParseError {
                path: "temp_message".into(),
                message: "dissect pattern did not match".into(),
                });
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                // Painless script
                // Source: ctx.remove('temp_message'); throw new Exception(\"Dissect error: Not a valid envoyproxy access log\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.remove('temp_message'); throw new Exception(\"Dissect error: Not a valid envoyproxy access log\");"#))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.has_value("dest") };
                if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: if (ctx.dest == \"-\") {\n  ctx.remove('dest');\n} else {\n  ctx['destination'] = new HashMap();\n  def p = ctx.dest.indexOf (':');\n  def l = ctx.dest.length();\n  ctx.destination.address = ctx.dest.substring(0, p);\n  ctx.destination.port = ctx.dest.substring(p+1, l);\n} ctx.remove('dest');
                split_at_delimiter(event, &SplitAtDelimiter::new("dest".into(), ":".into(), Some("destination.address".into()), Some("destination.port".into()), Some("-".into()), true));
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                if let Some(val) = event.get("destination.port") {
                let converted = convert_value(val, "integer")
                .map_err(|message| TransformError::ParseError {
                path: "destination.port".into(),
                message,
                })?;
                event.set("destination.port", converted)?;
                }
                }
                let _cond = { event.has_value("duration") };
                if _cond {
                if let Some(val) = event.get("duration") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "duration".into(),
                message,
                })?;
                event.set("duration", converted)?;
                }
                }
                let _cond = { event.has_value("duration") };
                if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.event.duration = Math.round(ctx.duration * params.scale)"#), cached_params!("{\"scale\":1000000}"))?;
                }
                event.remove("json");
                event.remove("duration");
                event.remove("time");
                event.remove("temp_message");
                let _cond = { event.get_str("proto").and_then(|s| s.chars().nth(0)).is_some_and(|c| c != '-') };
                if _cond {
                // Begin nested pipeline: "http"
                let _cond = { event.has_value("proto") && event.get_str("proto") != Some("-") };
                if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx['http'] = new HashMap(); def p = ctx.proto.indexOf ('/'); def l = ctx.proto.length(); ctx.http.version = ctx.proto.substring(p+1, l);
                split_at_delimiter(event, &SplitAtDelimiter::new("proto".into(), "/".into(), None, Some("http.version".into()), None, false));
                }
                event.rename("method", "http.request.method")?;
                event.rename_over("path", "url.path")?;
                if let Some(val) = event.get("response_code") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "response_code".into(),
                message,
                })?;
                event.set("response_code", converted)?;
                }
                event.rename("response_code", "http.response.status_code")?;
                event.rename("bytes_received", "http.response.body.bytes")?;
                if let Some(val) = event.get("http.response.body.bytes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "http.response.body.bytes".into(),
                message,
                })?;
                event.set("http.response.body.bytes", converted)?;
                }
                event.rename("bytes_sent", "http.request.body.bytes")?;
                if let Some(val) = event.get("http.request.body.bytes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "http.request.body.bytes".into(),
                message,
                })?;
                event.set("http.request.body.bytes", converted)?;
                }
                let _cond = { event.has_value("upstream_service_time") && event.get_str("upstream_service_time") != Some("-") };
                if _cond {
                // Painless script
                // Source: ctx.envoyproxy.log.upstream_service_time = Math.round(Double.parseDouble(ctx.upstream_service_time) * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.envoyproxy.log.upstream_service_time = Math.round(Double.parseDouble(ctx.upstream_service_time) * params.scale)"#), cached_params!("{\"scale\":1000000}"))?;
                }
                event.set("envoyproxy.log.proxy_type", json!("http"))?;
                let _cond = { event.has_value("envoyproxy.log.authority") && event.get_str("envoyproxy.log.authority") != Some("-") };
                if _cond {
                event.set("url.domain", json!(event.get("envoyproxy.log.authority").map_or_else(String::new, template_to_string)))?;
                }
                if let Some(domain_str) = event.get_string("url.domain") {
                let domain = domain_str.to_string();
                event.set("url.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                if let Some(registered) = rd.registered_domain {
                event.set("url.registered_domain", json!(registered))?;
                }
                event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                if let Some(sub) = rd.subdomain {
                event.set("url.subdomain", json!(sub))?;
                }
                }
                }
                if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                let ua_str = ua_str.to_string();
                // User agent parsing
                if let Ok(ua) = parse_user_agent(&ua_str) {
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
                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;
                event.set("network.protocol", json!("http"))?;
                // End nested pipeline: "http"
                }
                let _cond = { event.get_str("proto").and_then(|s| s.chars().nth(0)).is_some_and(|c| c == '-') };
                if _cond {
                // Begin nested pipeline: "tcp"
                if event.remove("upstream_service_time").is_none() {
                return Err(TransformError::FieldNotFound { path: "upstream_service_time".into() });
                }
                if event.remove("method").is_none() {
                return Err(TransformError::FieldNotFound { path: "method".into() });
                }
                if event.remove("user_agent").is_none() {
                return Err(TransformError::FieldNotFound { path: "user_agent".into() });
                }
                if event.remove("path").is_none() {
                return Err(TransformError::FieldNotFound { path: "path".into() });
                }
                if event.remove("response_code").is_none() {
                return Err(TransformError::FieldNotFound { path: "response_code".into() });
                }
                event.rename("bytes_received", "destination.bytes")?;
                if let Some(val) = event.get("destination.bytes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "destination.bytes".into(),
                message,
                })?;
                event.set("destination.bytes", converted)?;
                }
                event.rename_over("bytes_sent", "source.bytes")?;
                if let Some(val) = event.get("source.bytes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "source.bytes".into(),
                message,
                })?;
                event.set("source.bytes", converted)?;
                }
                event.set("envoyproxy.log.proxy_type", json!("tcp"))?;
                event.append_unique("event.type", json!("connection"))?;
                event.set("network.transport", json!("tcp"))?;
                // End nested pipeline: "tcp"
                }
                event.remove("proto");
                event.remove("upstream_service_time");
                let _cond = { event.get_str("source.address") == Some("-") };
                if _cond {
                if event.remove("source.address").is_none() {
                return Err(TransformError::FieldNotFound { path: "source.address".into() });
                }
                }
                let _cond = { event.get_str("envoyproxy.log.response_flags") == Some("-") };
                if _cond {
                if event.remove("envoyproxy.log.response_flags").is_none() {
                return Err(TransformError::FieldNotFound { path: "envoyproxy.log.response_flags".into() });
                }
                }
                let _cond = { event.has_value("envoyproxy.log.response_flags") };
                if _cond {
                if let Some(s) = event.get_string("envoyproxy.log.response_flags") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                parts.pop();
                }
                event.set("envoyproxy.log.response_flags", Value::Array(parts))?;
                }
                }
                let _cond = { event.has_value("destination.address") };
                if _cond {
                event.set("destination.ip", json!(event.get("destination.address").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = { event.has_value("source.address") };
                if _cond {
                event.set("source.ip", json!(event.get("source.address").map_or_else(String::new, template_to_string)))?;
                }
                // End nested pipeline: "plaintext"

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
