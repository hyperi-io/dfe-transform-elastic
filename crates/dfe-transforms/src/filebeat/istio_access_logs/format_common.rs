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
            let _cond = { event.get_str("first_char") != Some("[") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] ") else { break 'dissect false };
                        captured.push(("istio.access.start_time", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("istio.access.message", remaining));
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

            if event.has_value("istio.access.message") {
                if let Some(input) = event.get_string("istio.access.message") {
                    // Grok pattern: \"(-|%{DATA:http.request.method}) (-|%{DATA:url.original}) (-|%{DATA:istio.access.protocol})\" (-|%{NUMBER:http.response.status_code}) (-|%{DATA:istio.access.response.flags}) (-|%{DATA:istio.access.response.code_details}) (-|%{DATA:istio.access.connection_termination_details}) \"(-|%{DATA:istio.access.upstream.transport_failure_reason})\" %{NUMBER:istio.access.bytes.received} %{NUMBER:istio.access.bytes.sent} (-|%{NUMBER:istio.access.duration}) (-|%{NUMBER:istio.access.upstream.service_time}) \"(-|%{DATA:istio.access.x_forwarded_for})\" \"(-|%{DATA:user_agent.original})\" \"(-|%{DATA:http.request.id})\" \"(-|%{DATA:istio.access.authority})\" \"(-|%{DATA:istio.access.upstream.host})\" (-|%{DATA:istio.access.upstream.cluster}) (-|%{DATA:istio.access.upstream.local_address}) (-|%{DATA:istio.access.downstream.local_address}) (-|%{DATA:istio.access.downstream.remote_address}) (-|%{DATA:istio.access.requested_server_name}) (-|%{GREEDYDATA:istio.access.route_name})
                    let _ = cached_grok!("\"(-|%{DATA:http.request.method}) (-|%{DATA:url.original}) (-|%{DATA:istio.access.protocol})\" (-|%{NUMBER:http.response.status_code}) (-|%{DATA:istio.access.response.flags}) (-|%{DATA:istio.access.response.code_details}) (-|%{DATA:istio.access.connection_termination_details}) \"(-|%{DATA:istio.access.upstream.transport_failure_reason})\" %{NUMBER:istio.access.bytes.received} %{NUMBER:istio.access.bytes.sent} (-|%{NUMBER:istio.access.duration}) (-|%{NUMBER:istio.access.upstream.service_time}) \"(-|%{DATA:istio.access.x_forwarded_for})\" \"(-|%{DATA:user_agent.original})\" \"(-|%{DATA:http.request.id})\" \"(-|%{DATA:istio.access.authority})\" \"(-|%{DATA:istio.access.upstream.host})\" (-|%{DATA:istio.access.upstream.cluster}) (-|%{DATA:istio.access.upstream.local_address}) (-|%{DATA:istio.access.downstream.local_address}) (-|%{DATA:istio.access.downstream.remote_address}) (-|%{DATA:istio.access.requested_server_name}) (-|%{GREEDYDATA:istio.access.route_name})").extract_into(&input, event)?;
                }
            }

                event.remove("istio.access.message");

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
