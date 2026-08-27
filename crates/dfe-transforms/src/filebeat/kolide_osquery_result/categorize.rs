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
            let _cond = { event.get_str("event.action") == Some("snapshot") };
            if _cond {
            event.set("event.kind", json!("state"))?;
            }

            let _cond = { event.get_str("event.action") == Some("snapshot") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = { event.get_str("event.action") == Some("snapshot") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("event.action") == Some("differential") };
            if _cond {
            event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.get_str("event.action") == Some("differential") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = { event.get_str("event.action") == Some("differential") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("event.action") != Some("snapshot") && event.get_str("event.action") != Some("differential") };
            if _cond {
            event.set("event.kind", json!("state"))?;
            }

            let _cond = { event.get_str("event.action") != Some("snapshot") && event.get_str("event.action") != Some("differential") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = { event.get_str("event.action") != Some("snapshot") && event.get_str("event.action") != Some("differential") };
            if _cond {
                event.append("event.type", json!("info"))?;
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
