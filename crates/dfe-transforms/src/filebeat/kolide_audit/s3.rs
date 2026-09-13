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
            let _cond = { event.get("json.data").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: Map data = (Map) ctx.json.remove('data');\nfor (def entry : data.entrySet()) {\n  ctx.json[entry.getKey()] = entry.getValue();\n}\nctx.json.remove('type');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"Map data = (Map) ctx.json.remove('data');\nfor (def entry : data.entrySet()) {\n  ctx.json[entry.getKey()] = entry.getValue();\n}\nctx.json.remove('type');"#))?;
            }

            let _cond = { !event.has_value("user.email") && event.has_value("json.actor_email") && event.get_str("json.actor_email") != Some("") };
            if _cond {
                if event.has_value("json.actor_email") {
                    event.rename("json.actor_email", "user.email")?;
                }
            }

                event.remove("json.actor_email");

            let _cond = { !event.has_value("kolide.audit.actor_type") };
            if _cond {
                if event.has_value("json.actor_type") {
                    event.rename("json.actor_type", "kolide.audit.actor_type")?;
                }
            }

            let _cond = { !event.has_value("source.ip") && event.has_value("json.ip_address") && event.get_str("json.ip_address") != Some("") };
            if _cond {
                if event.has_value("json.ip_address") {
                    event.rename("json.ip_address", "source.ip")?;
                }
            }

                event.remove("json.ip_address");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
