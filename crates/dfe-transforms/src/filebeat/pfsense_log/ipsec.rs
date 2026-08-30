// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ipsec` pipeline.
pub struct Ipsec;

impl Transform for Ipsec {
    fn name(&self) -> &str {
        "ipsec"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:\\d+\\[%{WORD}\\])%{GREEDYDATA}(?:%{IP:source.address}\\[%{NONNEGINT:source.port:long}\\]) to (?:%{IP:destination.address}\\[%{NONNEGINT:destination.port:long}\\]) \\(%{NONNEGINT:network.bytes:long} bytes\\)
                    // Grok pattern: %{GREEDYDATA}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("(?:\\d+\\[%{WORD}\\])%{GREEDYDATA}(?:%{IP:source.address}\\[%{NONNEGINT:source.port:long}\\]) to (?:%{IP:destination.address}\\[%{NONNEGINT:destination.port:long}\\]) \\(%{NONNEGINT:network.bytes:long} bytes\\)"),
                            cached_grok!("%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )?;
                }

            let _cond = { event.has_value("source.address") };
            if _cond {
                event.append_unique("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("message").is_some_and(|s| s.to_lowercase().contains("disconnected")) };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            let v = json!(event.get("source.address").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("source.ip", v)?;
            }

            let v = json!(event.get("destination.address").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("destination.ip", v)?;
            }

            event.set("network.protocol", json!("ipsec"))?;

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
