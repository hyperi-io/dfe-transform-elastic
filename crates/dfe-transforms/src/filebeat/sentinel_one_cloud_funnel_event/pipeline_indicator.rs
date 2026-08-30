// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_indicator` pipeline.
pub struct PipelineIndicator;

impl Transform for PipelineIndicator {
    fn name(&self) -> &str {
        "pipeline_indicator"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("json.src.process.tid") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src.process.tid") {
                if let Some(val) = event.get("json.src.process.tid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src.process.tid".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.src.process.tid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_src_process_tid")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.src.process.tid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.thread.id", v)?;
            }

                if event.has_value("json.indicator.category") {
                    event.rename("json.indicator.category", "sentinel_one_cloud_funnel.event.indicator.category")?;
                }

                if event.has_value("json.indicator.description") {
                    event.rename("json.indicator.description", "sentinel_one_cloud_funnel.event.indicator.description")?;
                }

                if event.has_value("json.indicator.metadata") {
                    event.rename("json.indicator.metadata", "sentinel_one_cloud_funnel.event.indicator.metadata")?;
                }

                if event.has_value("json.indicator.name") {
                    event.rename("json.indicator.name", "sentinel_one_cloud_funnel.event.indicator.name")?;
                }

            // SKIPPED: condition not transpiled: ctx.json?.src?.process != null && ctx.json.src.process['isStoryline™Root'] != ''
            #[allow(unreachable_code, unused_variables)]
            if false {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src.process.isStoryline™Root") {
                if let Some(val) = event.get("json.src.process.isStoryline™Root") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src.process.isStoryline™Root".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.src.process.is_storyline_tm_root", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_src_process_isStoryline™Root")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // SKIPPED: condition not transpiled: ctx.json?.src?.process?.parent != null && ctx.json.src.process.parent['isStoryline™Root'] != ''
            #[allow(unreachable_code, unused_variables)]
            if false {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src.process.parent.isStoryline™Root") {
                if let Some(val) = event.get("json.src.process.parent.isStoryline™Root") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src.process.parent.isStoryline™Root".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_storyline_tm_root", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "json_src_process_parent_isStoryline™Root")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.src.process.parent.Storyline™.id") {
                    event.rename("json.src.process.parent.Storyline™.id", "sentinel_one_cloud_funnel.event.src.process.parent.storyline_tm_id")?;
                }

                if event.has_value("json.src.process.Storyline™.id") {
                    event.rename("json.src.process.Storyline™.id", "sentinel_one_cloud_funnel.event.src.process.storyline_tm_id")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
