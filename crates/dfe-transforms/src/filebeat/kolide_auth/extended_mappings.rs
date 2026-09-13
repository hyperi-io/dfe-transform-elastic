// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `extended_mappings` pipeline.
pub struct ExtendedMappings;

impl Transform for ExtendedMappings {
    fn name(&self) -> &str {
        "extended_mappings"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("json.events").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def prefix = 'Downloaded package: ';\ndef packages = new ArrayList();\nfor (def ev : ctx.json.events) {\n  def desc = ev?.event_description;\n  if (desc instanceof String && desc.startsWith(prefix)) {\n    packages.add(desc.substring(prefix.length()));\n  }\n}\nif (!packages.isEmpty()) {\n  if (ctx.kolide.auth.downloaded_packages == null) {\n    ctx.kolide.auth.downloaded_packages = packages;\n  } else {\n    ctx.kolide.auth.downloaded_packages.addAll(packages);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def prefix = 'Downloaded package: ';\ndef packages = new ArrayList();\nfor (def ev : ctx.json.events) {\n  def desc = ev?.event_description;\n  if (desc instanceof String && desc.startsWith(prefix)) {\n    packages.add(desc.substring(prefix.length()));\n  }\n}\nif (!packages.isEmpty()) {\n  if (ctx.kolide.auth.downloaded_packages == null) {\n    ctx.kolide.auth.downloaded_packages = packages;\n  } else {\n    ctx.kolide.auth.downloaded_packages.addAll(packages);\n  }\n}"#))?;
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
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
