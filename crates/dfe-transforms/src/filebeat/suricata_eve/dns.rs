// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dns` pipeline.
pub struct Dns;

impl Transform for Dns {
    fn name(&self) -> &str {
        "dns"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let v = json!(event.get("suricata.eve.dns.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("dns.id", v)?;
            }

            let v = json!(event.get("suricata.eve.dns.rcode").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("dns.response_code", v)?;
            }

            let v = json!(event.get("suricata.eve.dns.type").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("dns.type", v)?;
            }

            let _cond = { event.get_str("dns.type") == Some("query") || event.get_i64("suricata.eve.dns.version") == Some(2) };
            if _cond {
            let v = json!(event.get("suricata.eve.dns.rrname").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("dns.question.name", v)?;
            }
            }

            let _cond = { event.get_str("dns.type") == Some("query") || event.get_i64("suricata.eve.dns.version") == Some(2) };
            if _cond {
            let v = json!(event.get("suricata.eve.dns.rrtype").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("dns.question.type", v)?;
            }
            }

            let _cond = { event.get_str("dns.type") == Some("answer") && !event.has_value("suricata.eve.dns.version") };
            if _cond {
                // Begin nested pipeline: "dns-answer-v1"
                // Painless script
                // Source: def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n"#))?;
                // End nested pipeline: "dns-answer-v1"
            }

            let _cond = { event.get_str("dns.type") == Some("answer") && event.get_i64("suricata.eve.dns.version") == Some(2) };
            if _cond {
                // Begin nested pipeline: "dns-answer-v2"
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
                // End nested pipeline: "dns-answer-v2"
            }

            if event.has_value("dns.resolved_ip") {
                foreach_array(event, "dns.resolved_ip", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("dns.question.registered_domain") };
            if _cond {
                // Painless script
                // Source: def rd = ctx.dns.question.registered_domain;\ndef firstDot = rd.indexOf(\".\");\nif (firstDot == -1) {\n    return;\n}\nctx.dns.question.top_level_domain = rd.substring(firstDot + 1);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def rd = ctx.dns.question.registered_domain;\ndef firstDot = rd.indexOf(\".\");\nif (firstDot == -1) {\n    return;\n}\nctx.dns.question.top_level_domain = rd.substring(firstDot + 1);\n"#))?;
            }

            let _cond = { event.get_bool("suricata.eve.dns.aa") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("AA"))?;
            }

            let _cond = { event.get_bool("suricata.eve.dns.tc") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("TC"))?;
            }

            let _cond = { event.get_bool("suricata.eve.dns.rd") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("RD"))?;
            }

            let _cond = { event.get_bool("suricata.eve.dns.ra") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("RA"))?;
            }

                event.remove("suricata.eve.dns.aa");
                event.remove("suricata.eve.dns.tc");
                event.remove("suricata.eve.dns.rd");
                event.remove("suricata.eve.dns.ra");
                event.remove("suricata.eve.dns.qr");
                event.remove("suricata.eve.dns.version");
                event.remove("suricata.eve.dns.flags");
                event.remove("suricata.eve.dns.grouped");

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
