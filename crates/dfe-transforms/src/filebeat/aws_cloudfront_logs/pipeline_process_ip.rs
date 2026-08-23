// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_process_ip` pipeline.
pub struct PipelineProcessIp;

impl Transform for PipelineProcessIp {
    fn name(&self) -> &str {
        "pipeline_process_ip"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_ingest._value") {
                if let Some(input) = event.get_string("_ingest._value") {
                    // Grok pattern: ^%{IPV4:_tmp.valid_ip}$
                    // Grok pattern: ^%{IPV6:_tmp.valid_ip}$
                    // Grok pattern: ^(?P<_tmp_valid_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}))$
                    // Grok pattern: ^\\[%{IPV6:_tmp.valid_ip}\\]$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{IPV4:_tmp.valid_ip}$"),
                            cached_grok!("^%{IPV6:_tmp.valid_ip}$"),
                            cached_grok_mapped!("^(?P<_tmp_valid_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}))$", [("_tmp_valid_ip", "_tmp.valid_ip")]),
                            cached_grok!("^\\[%{IPV6:_tmp.valid_ip}\\]$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.valid_ip") && event.get_str("_tmp.valid_ip") != Some("") };
            if _cond {
                event.append_unique("network.forwarded_ip", json!(event.get("_tmp.valid_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.invalid_ip") && event.get_str("_tmp.invalid_ip") != Some("") };
            if _cond {
                event.append_unique("_tmp.invalid_ips", json!(event.get("_tmp.invalid_ip").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
