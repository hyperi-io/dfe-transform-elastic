// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `categorize` pipeline.
pub struct Categorize;

impl Transform for Categorize {
    fn name(&self) -> &str {
        "categorize"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n} else {\n  ctx.event.kind = 'state';\n  ctx.event.category = ['host'];\n  ctx.event.type = ['info'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n} else {\n  ctx.event.kind = 'state';\n  ctx.event.category = ['host'];\n  ctx.event.type = ['info'];\n}"#), cached_params!("{\"exact\":{\"check_result\":{\"kind\":\"state\",\"category\":[\"host\"],\"type\":[\"info\"]}}}"))?;
            }

            let _cond = { event.get_str("kolide.device_check.status") == Some("passing") && !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("kolide.device_check.status") == Some("failing") && !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { (event.get_str("kolide.device_check.status") == Some("inapplicable") || event.get_str("kolide.device_check.status") == Some("unknown")) && !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

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
