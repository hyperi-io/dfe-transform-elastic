// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `admin_login` pipeline.
pub struct AdminLogin;

impl Transform for AdminLogin {
    fn name(&self) -> &str {
        "admin_login"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("arista.reason") {
                    event.rename("arista.reason", "event.reason")?;
                }

                if event.has_value("arista.login") {
                    event.rename("arista.login", "user.name")?;
                }

                if event.has_value("arista.clientAddress") {
                    event.rename("arista.clientAddress", "source.ip")?;
                }

                // Painless script
                // Source: if (ctx?.event == null) {\n  Map map = new HashMap();\n  ctx.put('event', map);\n} if (ctx.arista?.succeeded == null || !params.containsKey((ctx.arista.succeeded).toString())) {\n  return;\n} ctx.event.category = params.get((ctx.arista.succeeded).toString()).get('category').clone(); ctx.event.kind = params.get((ctx.arista.succeeded).toString()).get('kind'); ctx.event.outcome = params.get((ctx.arista.succeeded).toString()).get('outcome'); ctx.event.type = params.get((ctx.arista.succeeded).toString()).get('type').clone(); ctx.event.provider = params.get((ctx.arista.succeeded).toString()).get('provider'); ctx.arista.remove('succeeded');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx?.event == null) {\n  Map map = new HashMap();\n  ctx.put('event', map);\n} if (ctx.arista?.succeeded == null || !params.containsKey((ctx.arista.succeeded).toString())) {\n  return;\n} ctx.event.category = params.get((ctx.arista.succeeded).toString()).get('category').clone(); ctx.event.kind = params.get((ctx.arista.succeeded).toString()).get('kind'); ctx.event.outcome = params.get((ctx.arista.succeeded).toString()).get('outcome'); ctx.event.type = params.get((ctx.arista.succeeded).toString()).get('type').clone(); ctx.event.provider = params.get((ctx.arista.succeeded).toString()).get('provider'); ctx.arista.remove('succeeded');"#), cached_params!("{\"false\":{\"category\":[\"network\",\"authentication\",\"iam\"],\"kind\":\"event\",\"outcome\":\"failure\",\"type\":[\"denied\"],\"provider\":\"admin_login\"},\"true\":{\"category\":[\"network\",\"authentication\",\"iam\"],\"kind\":\"event\",\"outcome\":\"success\",\"type\":[\"allowed\"],\"provider\":\"admin_login\"}}"))?;

                event.remove("arista.local");

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
