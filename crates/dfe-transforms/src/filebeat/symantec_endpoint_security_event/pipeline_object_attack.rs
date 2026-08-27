// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_attack` pipeline.
pub struct PipelineObjectAttack;

impl Transform for PipelineObjectAttack {
    fn name(&self) -> &str {
        "pipeline_object_attack"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("ses.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.attacks", |event| {
                    if event.has_value("_ingest._value.tactic_ids") {
                    foreach_array(event, "_ingest._value.tactic_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_attacks_tactic_ids_to_string")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.attacks", |event| {
                    if event.has_value("_ingest._value.tactic_uids") {
                    foreach_array(event, "_ingest._value.tactic_uids", |event| {
                    event.append_unique("threat.tactic.id", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.attacks", |event| {
                    event.append_unique("threat.technique.id", json!(event.get("_ingest._value.technique_uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.attacks", |event| {
                    event.append_unique("threat.technique.name", json!(event.get("_ingest._value.technique_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.attacks", |event| {
                    event.append_unique("threat.technique.subtechnique.id", json!(event.get("_ingest._value.sub_technique_uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.attacks", |event| {
                    event.append_unique("threat.technique.subtechnique.name", json!(event.get("_ingest._value.sub_technique_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            // SKIPPED: condition not transpiled: ctx.ses?.attacks instanceof List && ctx.tags?.contains('preserve_duplicate_custom_fields') != true
            #[allow(unreachable_code, unused_variables)]
            if false {
                foreach_array(event, "ses.attacks", |event| {
                    event.remove("_ingest._value.tactic_uids");
                    event.remove("_ingest._value.technique_uid");
                    event.remove("_ingest._value.technique_name");
                    event.remove("_ingest._value.sub_technique_uid");
                    event.remove("_ingest._value.sub_technique_name");
                    Ok(())
                })?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
