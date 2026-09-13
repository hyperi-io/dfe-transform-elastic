// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_encryption` pipeline.
pub struct PipelineEncryption;

impl Transform for PipelineEncryption {
    fn name(&self) -> &str {
        "pipeline_encryption"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                    // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg},%{SPACE}sent from:%{SPACE}\\'%{DATA:fortinet_fortimail.log.sent_from}\\',%{SPACE}subject:%{SPACE}\\'%{GREEDYDATA:temp.subject}\\'(?:%{SPACE}%{GREEDYDATA:temp.msg2})$
                    // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA}(?i)user %{NOTSPACE:temp.user} %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"),
                            cached_grok!("^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg},%{SPACE}sent from:%{SPACE}\\'%{DATA:fortinet_fortimail.log.sent_from}\\',%{SPACE}subject:%{SPACE}\\'%{GREEDYDATA:temp.subject}\\'(?:%{SPACE}%{GREEDYDATA:temp.msg2})$"),
                            cached_grok!("^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("fortinet_fortimail.log.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("fortinet_fortimail.log.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("fortinet_fortimail.log.user", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("user.name", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("fortinet_fortimail.log.sent_from") };
            if _cond {
                event.append_unique("related.user", json!(event.get("fortinet_fortimail.log.sent_from").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("fortinet_fortimail.log.sent_from") };
            if _cond {
                event.append_unique("email.from.address", json!(event.get("fortinet_fortimail.log.sent_from").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.subject") };
            if _cond {
                event.append_unique("fortinet_fortimail.log.subject", json!(event.get("temp.subject").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.subject") };
            if _cond {
                event.append_unique("email.subject", json!(event.get("temp.subject").map_or_else(String::new, template_to_string)))?;
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
