// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `openvpn` pipeline.
pub struct Openvpn;

impl Transform for Openvpn {
    fn name(&self) -> &str {
        "openvpn"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}peer%{SPACE}info:%{SPACE}%{GREEDYDATA:pfsense.openvpn.peer_info}
                    // Grok pattern: (?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}\\[(?P<user_name>(?:[a-zA-Z0-9._-]+))\\]%{SPACE}%{GREEDYDATA}
                    // Grok pattern: user%{SPACE}'(?P<user_name>(?:[a-zA-Z0-9._-]+))'%{GREEDYDATA}
                    // Grok pattern: (?P<user_name>(?:[a-zA-Z0-9._-]+))/(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{DATA}IPv4=(%{IP:source.nat.ip}|%{GREEDYDATA}),%{SPACE}IPv6=(%{IP:source.nat.ip}|%{GREEDYDATA})
                    // Grok pattern: %{GREEDYDATA}(?:%{IP:source.address}:%{NONNEGINT:source.port:long})
                    // Grok pattern: %{GREEDYDATA}
                    if !extract_first_match(
                        &[
                            cached_grok!("(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}peer%{SPACE}info:%{SPACE}%{GREEDYDATA:pfsense.openvpn.peer_info}"),
                            cached_grok_mapped!("(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}\\[(?P<user_name>(?:[a-zA-Z0-9._-]+))\\]%{SPACE}%{GREEDYDATA}", [("user_name", "user.name")]),
                            cached_grok_mapped!("user%{SPACE}'(?P<user_name>(?:[a-zA-Z0-9._-]+))'%{GREEDYDATA}", [("user_name", "user.name")]),
                            cached_grok_mapped!("(?P<user_name>(?:[a-zA-Z0-9._-]+))/(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{DATA}IPv4=(%{IP:source.nat.ip}|%{GREEDYDATA}),%{SPACE}IPv6=(%{IP:source.nat.ip}|%{GREEDYDATA})", [("user_name", "user.name")]),
                            cached_grok!("%{GREEDYDATA}(?:%{IP:source.address}:%{NONNEGINT:source.port:long})"),
                            cached_grok!("%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("auth")), serde_json::Value::String(s) => s.contains("auth"), _ => false }) };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                event.append_unique("event.type", json!("connection"))?;
            }

                event.append_unique("event.type", json!("info"))?;

            let _cond = { event.get_str("message").is_some_and(|s| s.to_lowercase().contains("error")) || event.get_str("message").is_some_and(|s| s.to_lowercase().contains("not auth")) };
            if _cond {
                event.append_unique("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("message").is_some_and(|s| s.to_lowercase().contains("initiat")) };
            if _cond {
                event.append_unique("event.type", json!("start"))?;
            }

            let v = json!(event.get("source.address").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("source.ip", v)?;
            }

            event.set("network.protocol", json!("openvpn"))?;

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
