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

            let _cond = { !event.has_value("json.sub_event_type") };
            if _cond {
                if event.has_value("json.auth_event_type") {
                    event.rename("json.auth_event_type", "json.sub_event_type")?;
                }
            }

            let _cond = { !event.has_value("json.id") };
            if _cond {
            if let Some(v) = event.get("json.request_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("json.id", v)?;
            }
            }

                event.remove("json.request_id");

            let _cond = { event.get_bool("json.succeeded") == Some(true) };
            if _cond {
            event.set("json.result", json!("Success"))?;
            }

            let _cond = { event.get_bool("json.succeeded") == Some(false) };
            if _cond {
            event.set("json.result", json!("Fail"))?;
            }

                event.remove("json.succeeded");

                event.remove("json.authentication_events");

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
