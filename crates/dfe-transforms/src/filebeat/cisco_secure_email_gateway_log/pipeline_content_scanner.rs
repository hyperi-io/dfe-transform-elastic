// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_content_scanner` pipeline.
pub struct PipelineContentScanner;

impl Transform for PipelineContentScanner {
    fn name(&self) -> &str {
        "pipeline_content_scanner"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{WORD:cisco_secure_email_gateway.log.object} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category}$
                    // Grok pattern: ^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(pid=%{NUMBER:process.pid:long}\\)$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{WORD:cisco_secure_email_gateway.log.object} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category}$"),
                            cached_grok!("^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(pid=%{NUMBER:process.pid:long}\\)$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
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
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
