// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_application_settings` pipeline.
pub struct PipelineApplicationSettings;

impl Transform for PipelineApplicationSettings {
    fn name(&self) -> &str {
        "pipeline_application_settings"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("axonius.application.configuration_values").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "axonius.application.configuration_values", |event| {
                    event.append_unique("rule.description", json!(event.get("_ingest._value.configuration_value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("axonius.application.configuration_values").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "axonius.application.configuration_values", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.is_valid") {
                    if let Some(val) = event.get("_ingest._value.is_valid") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.is_valid".into(),
                    message,
                    })?;
                    event.set("_ingest._value.is_valid", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_configuration_values_is_valid_to_boolean")?;
                    event.remove("_ingest._value.is_valid");
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

            let _cond = { event.get("axonius.application.configuration_values").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "axonius.application.configuration_values", |event| {
                    event.append_unique("rule.id", json!(event.get("_ingest._value.role.remote_id").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("axonius.application.is_excluded") {
                if let Some(val) = event.get("axonius.application.is_excluded") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "axonius.application.is_excluded".into(),
                            message,
                        })?;
                    event.set("axonius.application.is_excluded", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_is_excluded_to_boolean")?;
                        event.remove("axonius.application.is_excluded");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("axonius.application.raw_setting_name") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("axonius.application.raw_setting_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.application.recommendation_description") };
            if _cond {
                event.append_unique("rule.description", json!(event.get("axonius.application.recommendation_description").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.application.setting_description") };
            if _cond {
                event.append_unique("message", json!(event.get("axonius.application.setting_description").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("axonius.application.settings_score") {
                if let Some(val) = event.get("axonius.application.settings_score") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "axonius.application.settings_score".into(),
                            message,
                        })?;
                    event.set("axonius.application.settings_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_settings_score_to_double")?;
                        event.remove("axonius.application.settings_score");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("axonius.application.vendor_setting._id") };
            if _cond {
                event.append_unique("rule.id", json!(event.get("axonius.application.vendor_setting._id").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("axonius.application.vendor_setting.is_relevant") {
                if let Some(val) = event.get("axonius.application.vendor_setting.is_relevant") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "axonius.application.vendor_setting.is_relevant".into(),
                            message,
                        })?;
                    event.set("axonius.application.vendor_setting.is_relevant", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vendor_setting_is_relevant_to_boolean")?;
                        event.remove("axonius.application.vendor_setting.is_relevant");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("axonius.application.vendor_setting.level").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.ruleset", v)?;
            }

            if let Some(v) = event.get("axonius.application.vendor_setting.link").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.reference", v)?;
            }

            let _cond = { event.has_value("axonius.application.vendor_setting.raw_setting_name") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("axonius.application.vendor_setting.raw_setting_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.application.vendor_setting.recommendation_reason") };
            if _cond {
                event.append_unique("rule.description", json!(event.get("axonius.application.vendor_setting.recommendation_reason").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("axonius.application.vendor_setting.xsetting.impact") {
                if let Some(val) = event.get("axonius.application.vendor_setting.xsetting.impact") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "axonius.application.vendor_setting.xsetting.impact".into(),
                            message,
                        })?;
                    event.set("axonius.application.vendor_setting.xsetting.impact", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vendor_setting_xsetting_impact_to_long")?;
                        event.remove("axonius.application.vendor_setting.xsetting.impact");
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
                    event.append("error.message", json!(format!("Processor '{}'\n{}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}'\n", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
