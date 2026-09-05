// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `snort` pipeline.
pub struct Snort;

impl Transform for Snort {
    fn name(&self) -> &str {
        "snort"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(input) = event.get_string("message") {
                // Grok pattern: \\[%{NUMBER:snort.generator_id}:%{NUMBER:snort.signature_id}:%{NUMBER:snort.signature_revision}\\] \\(%{DATA:snort.preprocessor}\\) %{GREEDYDATA:snort.alert_message} \\[Classification: %{DATA:snort.classification}\\] \\[Priority: %{NONNEGINT:snort.priority:long}\\] \\{%{WORD:network.protocol}\\} %{IP:source.address}:%{NUMBER:source.port:long} -> %{IP:destination.address}:%{NUMBER:destination.port:long}
                if !cached_grok!("\\[%{NUMBER:snort.generator_id}:%{NUMBER:snort.signature_id}:%{NUMBER:snort.signature_revision}\\] \\(%{DATA:snort.preprocessor}\\) %{GREEDYDATA:snort.alert_message} \\[Classification: %{DATA:snort.classification}\\] \\[Priority: %{NONNEGINT:snort.priority:long}\\] \\{%{WORD:network.protocol}\\} %{IP:source.address}:%{NUMBER:source.port:long} -> %{IP:destination.address}:%{NUMBER:destination.port:long}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
