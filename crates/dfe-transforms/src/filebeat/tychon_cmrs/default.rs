// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // Begin nested pipeline: "common_init"
            parse_json_field(event, "message", "tychon")?;
            // Painless script
            // Source: def keys = new ArrayList(ctx.tychon.keySet());\nfor (key in keys) {\n  if (ctx.tychon[key] == \"\" || ctx.tychon[key] == null) {\n    ctx.tychon.remove(key);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def keys = new ArrayList(ctx.tychon.keySet());\nfor (key in keys) {\n  if (ctx.tychon[key] == \"\" || ctx.tychon[key] == null) {\n    ctx.tychon.remove(key);\n  }\n}\n"#
                ),
            )?;
            dot_expand(event, "tychon", "*")?;
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }
            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }
            // Painless script
            // Source: if (ctx.tychon?.script?.current_duration != null)\n{\n  ctx.tychon.script.current_duration =\n    (long) Double.parseDouble(ctx.tychon.script.current_duration.toString());\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.tychon?.script?.current_duration != null)\n{\n  ctx.tychon.script.current_duration =\n    (long) Double.parseDouble(ctx.tychon.script.current_duration.toString());\n}\n"#
                ),
            )?;
            event.set("ecs.version", json!("8.17.0"))?;
            event.set("event.module", json!("tychon"))?;
            event.set("event.kind", json!("state"))?;
            event.append("event.type", json!("info"))?;
            // End nested pipeline: "common_init"

            // Begin nested pipeline: "rest"
            event.set("event.category", Value::Array(vec![json!("host")]))?;
            // End nested pipeline: "rest"

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
