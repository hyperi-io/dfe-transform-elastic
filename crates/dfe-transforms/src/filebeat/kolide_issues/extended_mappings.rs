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
            let _cond = { event.get("kolide.issues.value.version").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("kolide.issues.value.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.issues.detected_version", v)?;
            }
            }

            let _cond = { event.get("kolide.issues.detected_version").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("kolide.issues.detected_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.version", v)?;
            }
            }

            let _cond = { event.get("kolide.issues.value.newest_version").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("kolide.issues.value.newest_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.issues.expected_version", v)?;
            }
            }

            let _cond = { event.get("kolide.issues.value.key_type").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("kolide.issues.value.key_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("kolide.issues.ssh_key_type", v)?;
            }
            }

            let _cond = { event.get("kolide.issues.value.device").is_some_and(|v| v.is_string()) && !event.has_value("file.device") };
            if _cond {
            if let Some(v) = event.get("kolide.issues.value.device").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.device", v)?;
            }
            }

            let _cond = { event.get("kolide.issues.value.username").is_some_and(|v| v.is_string()) && !event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("kolide.issues.value.username").map_or_else(String::new, template_to_string)))?;
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
