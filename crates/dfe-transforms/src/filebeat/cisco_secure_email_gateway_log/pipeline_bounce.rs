// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bounce` pipeline.
pub struct PipelineBounce;

impl Transform for PipelineBounce {
    fn name(&self) -> &str {
        "pipeline_bounce"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.bounce_type}: DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}> RID %{NUMBER:cisco_secure_email_gateway.log.recipient_id} - %{DATA:cisco_secure_email_gateway.log.error_code} - %{GREEDYDATA:event.reason} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.response}\\)$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.bounce_type}: %{NUMBER:email.message_id}:%{NUMBER:cisco_secure_email_gateway.log.recipient_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}>$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{WORD:cisco_secure_email_gateway.log.bounce_type}: DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}> RID %{NUMBER:cisco_secure_email_gateway.log.recipient_id} - %{DATA:cisco_secure_email_gateway.log.error_code} - %{GREEDYDATA:event.reason} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.response}\\)$"),
                            cached_grok!("^%{WORD:cisco_secure_email_gateway.log.bounce_type}: %{NUMBER:email.message_id}:%{NUMBER:cisco_secure_email_gateway.log.recipient_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}>$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
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
