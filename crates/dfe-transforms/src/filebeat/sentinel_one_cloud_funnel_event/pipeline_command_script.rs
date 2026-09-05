// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_command_script` pipeline.
pub struct PipelineCommandScript;

impl Transform for PipelineCommandScript {
    fn name(&self) -> &str {
        "pipeline_command_script"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("process")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if event.has_value("json.tgt.file.sha1") {
                event.rename(
                    "json.tgt.file.sha1",
                    "sentinel_one_cloud_funnel.event.tgt.file.sha1",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.tgt.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.cmdScript.applicationName") {
                event.rename(
                    "json.cmdScript.applicationName",
                    "sentinel_one_cloud_funnel.event.cmd_script.application_name",
                )?;
            }

            if event.has_value("json.cmdScript.content") {
                event.rename(
                    "json.cmdScript.content",
                    "sentinel_one_cloud_funnel.event.cmd_script.content",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.cmd_script.content")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("powershell.file.script_block_text", v)?;
            }

            let _cond = { event.get_str("json.cmdScript.isComplete") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cmdScript.isComplete") {
                        if let Some(val) = event.get("json.cmdScript.isComplete") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cmdScript.isComplete".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.cmd_script.is_complete",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_cmdScript_isComplete",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("json.cmdScript.originalSize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cmdScript.originalSize") {
                        if let Some(val) = event.get("json.cmdScript.originalSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cmdScript.originalSize".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.cmd_script.original_size",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_cmdScript_originalSize",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.cmdScript.sha256") {
                event.rename(
                    "json.cmdScript.sha256",
                    "sentinel_one_cloud_funnel.event.cmd_script.sha256",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.cmd_script.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.cmd_script.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.src.process")
                    && event.get_str("json.src.process.crossProcessOutOfStoryline™Count")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessOutOfStoryline™Count") {
                        if let Some(val) =
                            event.get("json.src.process.crossProcessOutOfStoryline™Count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.crossProcessOutOfStoryline™Count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.out_of_storyline_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_crossProcessOutOfStoryline™Count",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.src.process.isStoryline™Root") {
                event.rename(
                    "json.src.process.isStoryline™Root",
                    "sentinel_one_cloud_funnel.event.src.process.is_storyline_tm_root",
                )?;
            }

            let _cond = {
                event.has_value("json.src.process.parent")
                    && event.get_str("json.src.process.parent.isStoryline™Root") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.isStoryline™Root") {
                        if let Some(val) = event.get("json.src.process.parent.isStoryline™Root") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.isStoryline™Root".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_storyline_tm_root", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_parent_isStoryline™Root",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.src.process.parent.Storyline™.id") {
                event.rename(
                    "json.src.process.parent.Storyline™.id",
                    "sentinel_one_cloud_funnel.event.src.process.parent.storyline_tm_id",
                )?;
            }

            if event.has_value("json.src.process.Storyline™.id") {
                event.rename(
                    "json.src.process.Storyline™.id",
                    "sentinel_one_cloud_funnel.event.src.process.storyline_tm_id",
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
