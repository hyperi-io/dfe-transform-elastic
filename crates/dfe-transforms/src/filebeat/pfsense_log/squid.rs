// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `squid` pipeline.
pub struct Squid;

impl Transform for Squid {
    fn name(&self) -> &str {
        "squid"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(input) = event.get_string("message") {
                // Grok pattern: %{IPORHOST:source.address} %{NOTSPACE:squid.request_status}/%{NUMBER:http.response.status_code:long} %{NUMBER:http.response.bytes:long} %{NOTSPACE:http.request.method} (?:%{URI:url.original}|(%{IPORHOST:url.domain}(?::%{DATA:url.port})?))?%{SPACE}%{NOTSPACE:http.request.referrer}%{SPACE}%{NOTSPACE:squid.hierarchy_status}/(?:%{IPORHOST:destination.address}|-)%{SPACE}%{NOTSPACE:http.response.mime_type}
                if !cached_grok!("%{IPORHOST:source.address} %{NOTSPACE:squid.request_status}/%{NUMBER:http.response.status_code:long} %{NUMBER:http.response.bytes:long} %{NOTSPACE:http.request.method} (?:%{URI:url.original}|(%{IPORHOST:url.domain}(?::%{DATA:url.port})?))?%{SPACE}%{NOTSPACE:http.request.referrer}%{SPACE}%{NOTSPACE:squid.hierarchy_status}/(?:%{IPORHOST:destination.address}|-)%{SPACE}%{NOTSPACE:http.response.mime_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
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

            if event.has_value("url.port") {
                if let Some(val) = event.get("url.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "url.port".into(),
                            message,
                        }
                    })?;
                    event.set("url.port", converted)?;
                }
            }

            event.append("event.category", json!("web"))?;

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
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
