// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dns_answer_v1` pipeline.
pub struct DnsAnswerV1;

impl Transform for DnsAnswerV1 {
    fn name(&self) -> &str {
        "dns_answer_v1"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n"#))?;

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
