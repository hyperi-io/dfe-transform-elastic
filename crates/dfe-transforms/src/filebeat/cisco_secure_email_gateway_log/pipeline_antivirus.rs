// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_antivirus` pipeline.
pub struct PipelineAntivirus;

impl Transform for PipelineAntivirus {
    fn name(&self) -> &str {
        "pipeline_antivirus"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' \\(\\)$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' '%{GREEDYDATA:cisco_secure_email_gateway.log.encrypted_hash}'$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' 'body.scan\\/%{GREEDYDATA:file.name}' 1 0$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}'$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' \\(\\)$"),
                            cached_grok!("^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' '%{GREEDYDATA:cisco_secure_email_gateway.log.encrypted_hash}'$"),
                            cached_grok!("^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' 'body.scan\\/%{GREEDYDATA:file.name}' 1 0$"),
                            cached_grok!("^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}'$"),
                            cached_grok!("^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.type") == Some("Error") };
            if _cond {
            event.set("event.type", json!("error"))?;
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
