// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `format_json` pipeline.
pub struct FormatJson;

impl Transform for FormatJson {
    fn name(&self) -> &str {
        "format_json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
                    event.rename("json.response_code_details", "istio.access.response.code_details")?;
                }

                if event.has_value("json.connection_termination_details") {
                    event.rename("json.connection_termination_details", "istio.access.connection_termination_details")?;
                }

                if event.has_value("json.upstream_transport_failure_reason") {
                    event.rename("json.upstream_transport_failure_reason", "istio.access.upstream.transport_failure_reason")?;
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
                    event.rename("json.upstream_service_time", "istio.access.upstream.service_time")?;
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
                    event.rename("json.upstream_local_address", "istio.access.upstream.local_address")?;
                }

                if event.has_value("json.downstream_local_address") {
                    event.rename("json.downstream_local_address", "istio.access.downstream.local_address")?;
                }

                if event.has_value("json.downstream_remote_address") {
                    event.rename("json.downstream_remote_address", "istio.access.downstream.remote_address")?;
                }

                if event.has_value("json.requested_server_name") {
                    event.rename("json.requested_server_name", "istio.access.requested_server_name")?;
                }

                if event.has_value("json.route_name") {
                    event.rename("json.route_name", "istio.access.route_name")?;
                }

                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n  map.values().removeIf(v -> v == \"-\");\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n  map.values().removeIf(v -> v == \"-\");\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#))?;

                event.remove("json");

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
