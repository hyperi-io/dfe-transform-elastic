// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `s3` pipeline.
pub struct S3;

impl Transform for S3 {
    fn name(&self) -> &str {
        "s3"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("json").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: def doc = ctx.json;\nSet reserved = new HashSet(['metadata', 'team_id', 'ip_address', 'user_email']);\nString eventKey = null;\nfor (def k : doc.keySet()) {\n  if (!reserved.contains(k)) {\n    eventKey = k;\n    break;\n  }\n}\nif (eventKey == null) {\n  return;\n}\ndef payload = doc.remove(eventKey);\ndoc.event_type = eventKey;\nif (payload instanceof Map) {\n  doc.event_data = payload;\n} else if (payload != null) {\n  def wrap = new HashMap();\n  wrap.put('value', payload);\n  doc.event_data = wrap;\n} else {\n  doc.event_data = new HashMap();\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def doc = ctx.json;\nSet reserved = new HashSet(['metadata', 'team_id', 'ip_address', 'user_email']);\nString eventKey = null;\nfor (def k : doc.keySet()) {\n  if (!reserved.contains(k)) {\n    eventKey = k;\n    break;\n  }\n}\nif (eventKey == null) {\n  return;\n}\ndef payload = doc.remove(eventKey);\ndoc.event_type = eventKey;\nif (payload instanceof Map) {\n  doc.event_data = payload;\n} else if (payload != null) {\n  def wrap = new HashMap();\n  wrap.put('value', payload);\n  doc.event_data = wrap;\n} else {\n  doc.event_data = new HashMap();\n}"#))?;
            }

                if event.has_value("json.metadata.id") {
                    event.rename("json.metadata.id", "json.event_id")?;
                }

                if event.has_value("json.metadata.timestamp") {
                    event.rename("json.metadata.timestamp", "json.timestamp")?;
                }

            let _cond = { event.get("json.metadata.context").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: def ctxMap = ctx.json.metadata.context;\nfor (def entry : ctxMap.entrySet()) {\n  if (!ctx.json.containsKey(entry.getKey())) {\n    ctx.json[entry.getKey()] = entry.getValue();\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def ctxMap = ctx.json.metadata.context;\nfor (def entry : ctxMap.entrySet()) {\n  if (!ctx.json.containsKey(entry.getKey())) {\n    ctx.json[entry.getKey()] = entry.getValue();\n  }\n}"#))?;
            }

            let _cond = { event.get("json.privacy_mode").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.privacy_mode", "_temp.privacy_json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "parse_privacy_mode_json_string")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("_temp.privacy_json.privacyMode") };
            if _cond {
            if let Some(v) = event.get("_temp.privacy_json.privacyMode").cloned() {
                event.set("json.privacy_mode", v)?;
            }
            }

                event.remove("json.metadata");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
