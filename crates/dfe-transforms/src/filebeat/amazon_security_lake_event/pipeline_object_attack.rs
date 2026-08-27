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
            let _cond = { event.get("ocsf.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.attacks", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.tactics", |event| {
                    event.append_unique("threat.tactic.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.attacks", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.tactics", |event| {
                    event.append_unique("threat.tactic.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.attacks", |event| {
                    event.append_unique("threat.technique.name", json!(event.get("_ingest._value.technique.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.attacks").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.attacks", |event| {
                    event.append_unique("threat.technique.id", json!(event.get("_ingest._value.technique.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
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
