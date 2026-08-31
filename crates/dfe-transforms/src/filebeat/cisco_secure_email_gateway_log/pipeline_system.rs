// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_system` pipeline.
pub struct PipelineSystem;

impl Transform for PipelineSystem {
    fn name(&self) -> &str {
        "pipeline_system"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^PID %{NUMBER:process.pid:long}: User %{USERNAME:user.name} commit changes:%{GREEDYDATA:cisco_secure_email_gateway.log.commit_changes}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.name}: qname:%{DATA:cisco_secure_email_gateway.log.qname} ns_name:%{DATA:cisco_secure_email_gateway.log.ns_name} zone:%{DATA:cisco_secure_email_gateway.log.zone} ref_zone:%{DATA:cisco_secure_email_gateway.log.ref_zone} referrals:%{GREEDYDATA:cisco_secure_email_gateway.log.referrals}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} to %{GREEDYDATA:cisco_secure_email_gateway.log.description} ' '$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^PID %{NUMBER:process.pid:long}: User %{USERNAME:user.name} commit changes:%{GREEDYDATA:cisco_secure_email_gateway.log.commit_changes}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.name}: qname:%{DATA:cisco_secure_email_gateway.log.qname} ns_name:%{DATA:cisco_secure_email_gateway.log.ns_name} zone:%{DATA:cisco_secure_email_gateway.log.zone} ref_zone:%{DATA:cisco_secure_email_gateway.log.ref_zone} referrals:%{GREEDYDATA:cisco_secure_email_gateway.log.referrals}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} to %{GREEDYDATA:cisco_secure_email_gateway.log.description} ' '$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            let _cond = { event.has_value("user.name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
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
