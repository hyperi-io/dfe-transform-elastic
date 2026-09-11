// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `http` pipeline.
pub struct Http;

impl Transform for Http {
    fn name(&self) -> &str {
        "http"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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

                if let Some(domain) = event.get_string("url.domain") {
                    // Public suffix list lookup for registered domain extraction.
                    // A failed lookup writes NO target field, which is what
                    // Elasticsearch does.
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set("url.domain", json!(domain))?;
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
                        event.remove("user_agent");
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
