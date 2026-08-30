// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dns_answer_v2` pipeline.
pub struct DnsAnswerV2;

impl Transform for DnsAnswerV2 {
    fn name(&self) -> &str {
        "dns_answer_v2"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("suricata.eve.dns.answers") {
                    event.rename("suricata.eve.dns.answers", "dns.answers")?;
                }

            let _cond = { event.has_value("dns.answers") };
            if _cond {
                // Painless script
                // Source: def resolvedIps = new ArrayList();\nfor (def answer : ctx?.dns?.answers) {\n    // Normalize field names to match ECS.\n    def name = answer.remove(\"rrname\");\n    if (name != null) {\n        answer[\"name\"] = name;\n    }\n    def type = answer.remove(\"rrtype\");\n    if (type != null) {\n        answer[\"type\"] = type;\n    }\n    def data = answer.remove(\"rdata\");\n    if (data != null) {\n        answer[\"data\"] = data;\n    }\n\n    if (type == \"A\" || type == \"AAAA\") {\n        resolvedIps.add(data);\n    }\n}\n\nif (resolvedIps.size() > 0) {\n    ctx.dns.resolved_ip = resolvedIps;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def resolvedIps = new ArrayList();\nfor (def answer : ctx?.dns?.answers) {\n    // Normalize field names to match ECS.\n    def name = answer.remove(\"rrname\");\n    if (name != null) {\n        answer[\"name\"] = name;\n    }\n    def type = answer.remove(\"rrtype\");\n    if (type != null) {\n        answer[\"type\"] = type;\n    }\n    def data = answer.remove(\"rdata\");\n    if (data != null) {\n        answer[\"data\"] = data;\n    }\n\n    if (type == \"A\" || type == \"AAAA\") {\n        resolvedIps.add(data);\n    }\n}\n\nif (resolvedIps.size() > 0) {\n    ctx.dns.resolved_ip = resolvedIps;\n}\n"#))?;
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
