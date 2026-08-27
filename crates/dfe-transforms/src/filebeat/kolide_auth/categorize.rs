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
                // Source: def action = ctx.event.action;\nctx.event.kind = 'event';\n\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def action = ctx.event.action;\nctx.event.kind = 'event';\n\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}"#), cached_params!("{\"exact\":{\"sign_in_attempt\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"]},\"sign_in_success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"auth_logs.success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"auth_logs.failure\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"failure\"},\"auth_session_summary\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"agent_detection_success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"agent_detection_failure\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"mobile_agent_detection_success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"agent_download_request\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"device_registration_request\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"device_registration_successful\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"device_registration_blocked\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"device_blocked\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"push_notification_sent\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"auth_log\":{\"category\":[\"authentication\"],\"type\":[\"info\"]}}}"))?;
            }

            let _cond = { event.get("json.events").is_some_and(|v| v.is_array()) && event.get("json.events").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has_value("json.timestamp") };
            if _cond {
                // Painless script
                // Source: def evs = ctx.json.events;\ndef ts = ctx.json.timestamp;\nint idx = -1;\nfor (int i = 0; i < evs.size(); i++) {\n  if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }\n}\nif (idx != -1) {\n  if (ctx.event == null) { ctx.event = [:]; }\n  if (idx == evs.size() - 1) {\n    ctx.event.type = ['end'];\n  } else if (idx == 0) {\n    ctx.event.type = ['start'];\n  } else {\n    ctx.event.type = ['info'];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def evs = ctx.json.events;\ndef ts = ctx.json.timestamp;\nint idx = -1;\nfor (int i = 0; i < evs.size(); i++) {\n  if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }\n}\nif (idx != -1) {\n  if (ctx.event == null) { ctx.event = [:]; }\n  if (idx == evs.size() - 1) {\n    ctx.event.type = ['end'];\n  } else if (idx == 0) {\n    ctx.event.type = ['start'];\n  } else {\n    ctx.event.type = ['info'];\n  }\n}"#))?;
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
