// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_kb_knowledge` pipeline.
pub struct PipelineKbKnowledge;

impl Transform for PipelineKbKnowledge {
    fn name(&self) -> &str {
        "pipeline_kb_knowledge"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                event.append("event.type", json!("info"))?;

                event.append("event.category", json!("database"))?;

            if let Some(v) = event.get("servicenow.event.author.display_value").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.full_name", v)?;
            }

            let _cond = { event.get_str("servicenow.event.direct.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.direct.display_value") {
                if let Some(val) = event.get("servicenow.event.direct.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.direct.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.direct.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_direct_display_value_to_boolean")?;
                        if event.remove("servicenow.event.direct.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.direct.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.direct.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.direct.value") {
                if let Some(val) = event.get("servicenow.event.direct.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.direct.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.direct.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_direct_value_to_boolean")?;
                        if event.remove("servicenow.event.direct.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.direct.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.disable_commenting.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.disable_commenting.display_value") {
                if let Some(val) = event.get("servicenow.event.disable_commenting.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.disable_commenting.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.disable_commenting.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_disable_commenting_display_value_to_boolean")?;
                        if event.remove("servicenow.event.disable_commenting.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.disable_commenting.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.disable_commenting.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.disable_commenting.value") {
                if let Some(val) = event.get("servicenow.event.disable_commenting.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.disable_commenting.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.disable_commenting.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_disable_commenting_value_to_boolean")?;
                        if event.remove("servicenow.event.disable_commenting.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.disable_commenting.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.disable_suggesting.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.disable_suggesting.display_value") {
                if let Some(val) = event.get("servicenow.event.disable_suggesting.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.disable_suggesting.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.disable_suggesting.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_disable_suggesting_display_value_to_boolean")?;
                        if event.remove("servicenow.event.disable_suggesting.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.disable_suggesting.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.disable_suggesting.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.disable_suggesting.value") {
                if let Some(val) = event.get("servicenow.event.disable_suggesting.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.disable_suggesting.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.disable_suggesting.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_disable_suggesting_value_to_boolean")?;
                        if event.remove("servicenow.event.disable_suggesting.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.disable_suggesting.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.display_attachments.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.display_attachments.display_value") {
                if let Some(val) = event.get("servicenow.event.display_attachments.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.display_attachments.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.display_attachments.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_display_attachments_display_value_to_boolean")?;
                        if event.remove("servicenow.event.display_attachments.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.display_attachments.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.display_attachments.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.display_attachments.value") {
                if let Some(val) = event.get("servicenow.event.display_attachments.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.display_attachments.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.display_attachments.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_display_attachments_value_to_boolean")?;
                        if event.remove("servicenow.event.display_attachments.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.display_attachments.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.flagged.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.flagged.display_value") {
                if let Some(val) = event.get("servicenow.event.flagged.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.flagged.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.flagged.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_flagged_display_value_to_boolean")?;
                        if event.remove("servicenow.event.flagged.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.flagged.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.flagged.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.flagged.value") {
                if let Some(val) = event.get("servicenow.event.flagged.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.flagged.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.flagged.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_flagged_value_to_boolean")?;
                        if event.remove("servicenow.event.flagged.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.flagged.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.generated_with_now_assist.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.generated_with_now_assist.display_value") {
                if let Some(val) = event.get("servicenow.event.generated_with_now_assist.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.generated_with_now_assist.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.generated_with_now_assist.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_generated_with_now_assist_display_value_to_boolean")?;
                        if event.remove("servicenow.event.generated_with_now_assist.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.generated_with_now_assist.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.generated_with_now_assist.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.generated_with_now_assist.value") {
                if let Some(val) = event.get("servicenow.event.generated_with_now_assist.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.generated_with_now_assist.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.generated_with_now_assist.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_generated_with_now_assist_value_to_boolean")?;
                        if event.remove("servicenow.event.generated_with_now_assist.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.generated_with_now_assist.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.sys_view_count.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.sys_view_count.value") {
                if let Some(val) = event.get("servicenow.event.sys_view_count.value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.sys_view_count.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.sys_view_count.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sys_view_count_value_to_long")?;
                        if event.remove("servicenow.event.sys_view_count.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.sys_view_count.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.use_count.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.use_count.value") {
                if let Some(val) = event.get("servicenow.event.use_count.value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.use_count.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.use_count.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_use_count_value_to_long")?;
                        if event.remove("servicenow.event.use_count.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.use_count.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.view_as_allowed.display_value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.view_as_allowed.display_value") {
                if let Some(val) = event.get("servicenow.event.view_as_allowed.display_value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.view_as_allowed.display_value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.view_as_allowed.display_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_view_as_allowed_display_value_to_boolean")?;
                        if event.remove("servicenow.event.view_as_allowed.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.view_as_allowed.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("servicenow.event.view_as_allowed.value") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.view_as_allowed.value") {
                if let Some(val) = event.get("servicenow.event.view_as_allowed.value") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "servicenow.event.view_as_allowed.value".into(),
                            message,
                        })?;
                    event.set("servicenow.event.view_as_allowed.value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_view_as_allowed_value_to_boolean")?;
                        if event.remove("servicenow.event.view_as_allowed.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.view_as_allowed.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.published.display_value") && event.get_str("servicenow.event.published.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.published.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.published.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.published.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_published_display_value")?;
                        if event.remove("servicenow.event.published.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.published.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.published.display_value") && event.get_str("servicenow.event.published.display_value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.published.display_value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "yyyy-MM-dd hh:mm:ss a", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.published.display_value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.published.display_value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_published_display_value")?;
                        if event.remove("servicenow.event.published.display_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.published.display_value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.published.value") && event.get_str("servicenow.event.published.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("ddMM") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.published.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "dd-MM-yyyy H:mm:ss", "dd-MM-yyyy HH:mm:ss", "dd-MM-yyyy", "dd/MM/yyyy H:mm:ss", "dd/MM/yyyy HH:mm:ss", "dd/MM/yyyy", "dd/MM/yy H:mm:ss", "dd/MM/yy HH:mm:ss", "dd/MM/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy", "yyyy-MM-dd hh:mm:ss a"], None, None) {
                        Some(parsed) => event.set("servicenow.event.published.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.published.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_published_value")?;
                        if event.remove("servicenow.event.published.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.published.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("servicenow.event.published.value") && event.get_str("servicenow.event.published.value") != Some("") && event.get_str("_conf.date_format_preference") == Some("MMdd") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("servicenow.event.published.value") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd H:mm:ss", "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601", "MM-dd-yyyy H:mm:ss", "MM-dd-yyyy HH:mm:ss", "MM-dd-yyyy", "MM/dd/yyyy H:mm:ss", "MM/dd/yyyy HH:mm:ss", "MM/dd/yyyy", "MM/dd/yy H:mm:ss", "MM/dd/yy HH:mm:ss", "MM/dd/yy", "dd.MM.yyyy H:mm:ss", "dd.MM.yyyy HH:mm:ss", "dd.MM.yyyy"], None, None) {
                        Some(parsed) => event.set("servicenow.event.published.value", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "servicenow.event.published.value".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_published_value")?;
                        if event.remove("servicenow.event.published.value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "servicenow.event.published.value".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.text.display_value") {
                map_strings(event, "servicenow.event.text.display_value", "servicenow.event.text.display_value", html_strip)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "html_strip")?;
                event.set("_ingest.on_failure_processor_tag", "html_strip_text_display_value")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("servicenow.event.text.value") {
                map_strings(event, "servicenow.event.text.value", "servicenow.event.text.value", html_strip)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "html_strip")?;
                event.set("_ingest.on_failure_processor_tag", "html_strip_text_value")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
