// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `haproxy` pipeline.
pub struct Haproxy;

impl Transform for Haproxy {
    fn name(&self) -> &str {
        "haproxy"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Connect from (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} %{WORD} %{IPORHOST:destination.address}:%{POSINT:destination.port:long} \\(%{NOTSPACE:haproxy.frontend_name}/%{WORD:haproxy.mode}\\)
                    // Grok pattern: (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.http.request.time_wait_ms:long}/%{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:haproxy.http.request.time_wait_without_data_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:http.response.status_code:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.http.request.captured_cookie} %{NOTSPACE:haproxy.http.response.captured_cookie} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long} (\\{%{DATA:haproxy.http.request.captured_headers}\\} \\{%{DATA:haproxy.http.response.captured_headers}\\} |\\{%{DATA}\\} )?\"%{GREEDYDATA:haproxy.http.request.raw_request_line}\"
                    // Grok pattern: (%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long}
                    // Grok pattern: (%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name}/(?P<haproxy_bind_name>(?:((%{IP:destination.address})?(:%{POSINT:destination.port:long})?|%{NOTSPACE}))):? %{GREEDYDATA:haproxy.error_message}
                    if !extract_first_match(
                        &[
                            cached_grok!("Connect from (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} %{WORD} %{IPORHOST:destination.address}:%{POSINT:destination.port:long} \\(%{NOTSPACE:haproxy.frontend_name}/%{WORD:haproxy.mode}\\)"),
                            cached_grok!("(%{IPORHOST:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.http.request.time_wait_ms:long}/%{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:haproxy.http.request.time_wait_without_data_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:http.response.status_code:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.http.request.captured_cookie} %{NOTSPACE:haproxy.http.response.captured_cookie} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long} (\\{%{DATA:haproxy.http.request.captured_headers}\\} \\{%{DATA:haproxy.http.response.captured_headers}\\} |\\{%{DATA}\\} )?\"%{GREEDYDATA:haproxy.http.request.raw_request_line}\""),
                            cached_grok!("(%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long}"),
                            cached_grok_mapped!("(%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name}/(?P<haproxy_bind_name>(?:((%{IP:destination.address})?(:%{POSINT:destination.port:long})?|%{NOTSPACE}))):? %{GREEDYDATA:haproxy.error_message}", [("haproxy_bind_name", "haproxy.bind_name")]),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_message_27190f7e")?;
                        return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("haproxy.request_date") && !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("haproxy.request_date") {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:HH:mm:ss.SSS", "MMM dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "haproxy.request_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("haproxy.request_date") && event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("haproxy.request_date") {
                    match parse_date_out(&date_str, &["dd/MMM/yyyy:HH:mm:ss.SSS", "MMM dd HH:mm:ss"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "haproxy.request_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

                event.remove("haproxy.request_date");

            let _cond = { event.has_value("haproxy.http.request.raw_request_line") && event.get("haproxy.http.request.raw_request_line").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) && event.get_str("haproxy.http.request.raw_request_line") != Some("<BADREQ>") };
            if _cond {
            if event.has_value("haproxy.http.request.raw_request_line") {
                if let Some(input) = event.get_string("haproxy.http.request.raw_request_line") {
                    // Grok pattern: %{WORD:http.request.method}%{SPACE}%{URIPATHPARAM:url.original}%{SPACE}HTTP/%{NUMBER:http.version}
                    if !cached_grok!("%{WORD:http.request.method}%{SPACE}%{URIPATHPARAM:url.original}%{SPACE}HTTP/%{NUMBER:http.version}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "url.original", "url", true, false)?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("haproxy.http.request.captured_headers") {
                if let Some(s) = event.get_string("haproxy.http.request.captured_headers") {
                    let mut parts: Vec<Value> = cached_regex!("\\|")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("haproxy.http.request.captured_headers", Value::Array(parts))?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("haproxy.http.response.captured_headers") {
                if let Some(s) = event.get_string("haproxy.http.response.captured_headers") {
                    let mut parts: Vec<Value> = cached_regex!("\\|")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("haproxy.http.response.captured_headers", Value::Array(parts))?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("_temp.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx._temp.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.event.duration = Math.round(ctx._temp.duration * params.scale)"#), cached_params!("{\"scale\":1000000}"))?;
            }

            let _cond = { event.has("http") };
            if _cond {
            if event.has_value("haproxy.bytes_read") {
                if let Some(val) = event.get("haproxy.bytes_read") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "haproxy.bytes_read".into(),
                            message,
                        })?;
                    event.set("http.response.bytes", converted)?;
                }
            }
            }

            let _cond = { event.get_str("haproxy.mode") == Some("HTTP") || event.has_value("haproxy.http") };
            if _cond {
                event.append("event.category", json!("web"))?;
            }

            let _cond = { event.has_value("source.address") && event.has_value("destination.address") };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = { event.has_value("http.response.status_code") && event.get_i64("http.response.status_code").is_some_and(|n| n < 400) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("http.response.status_code") && event.get_i64("http.response.status_code").is_some_and(|n| n >= 400) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

                event.remove("_temp");
                event.remove("haproxy.request_date");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
