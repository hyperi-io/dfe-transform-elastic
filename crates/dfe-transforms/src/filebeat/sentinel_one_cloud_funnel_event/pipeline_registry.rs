// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_registry` pipeline.
pub struct PipelineRegistry;

impl Transform for PipelineRegistry {
    fn name(&self) -> &str {
        "pipeline_registry"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("registry")]))?;

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.type")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.type")
                        .is_some_and(|s| s.to_lowercase().contains("create"))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("creation")]))?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.type")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.type")
                        .is_some_and(|s| s.to_lowercase().contains("delet"))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("deletion")]))?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.type")
                    && (event
                        .get_str("sentinel_one_cloud_funnel.event.type")
                        .is_some_and(|s| s.to_lowercase().contains("change"))
                        || event
                            .get_str("sentinel_one_cloud_funnel.event.type")
                            .is_some_and(|s| s.to_lowercase().contains("modif")))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("change")]))?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.type")
                    && (event
                        .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                        .is_some_and(|s| s.to_lowercase().contains("regvaluecreate"))
                        || event
                            .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                            .is_some_and(|s| s.to_lowercase().contains("regkeycreate")))
            };
            if _cond {
                event.set("event.action", Value::Array(vec![json!("creation")]))?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.type")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                        .is_some_and(|s| s.to_lowercase().contains("regvaluemodified"))
            };
            if _cond {
                event.set("event.action", Value::Array(vec![json!("modification")]))?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.type")
                    && (event
                        .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                        .is_some_and(|s| s.to_lowercase().contains("regvaluedelete"))
                        || event
                            .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                            .is_some_and(|s| s.to_lowercase().contains("regkeydelete")))
            };
            if _cond {
                event.set("event.action", Value::Array(vec![json!("deletion")]))?;
            }

            if event.has_value("json.registry.keyPath") {
                event.rename(
                    "json.registry.keyPath",
                    "sentinel_one_cloud_funnel.event.registry.key.path",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.registry.key.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.path", v)?;
            }

            let _cond = {
                event.get("registry.path").is_some_and(|v| v.is_string())
                    && event.get_str("registry.path") != Some("")
            };
            if _cond {
                // Painless script
                // Source: def idx = ctx.registry.path.lastIndexOf('\\\\');\nif (idx >= 0) {\n  ctx.registry.key = ctx.registry.path.substring(0, idx);\n  ctx.registry.value = ctx.registry.path.substring(idx+1);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def idx = ctx.registry.path.lastIndexOf('\\\\');\nif (idx >= 0) {\n  ctx.registry.key = ctx.registry.path.substring(0, idx);\n  ctx.registry.value = ctx.registry.path.substring(idx+1);\n}"#
                    ),
                )?;
            }

            if event.has_value("json.registry.value") {
                event.rename(
                    "json.registry.value",
                    "sentinel_one_cloud_funnel.event.registry.val",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.registry.val")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.data.strings", v)?;
            }

            if event.has_value("json.registry.valueType") {
                event.rename(
                    "json.registry.valueType",
                    "sentinel_one_cloud_funnel.event.registry.value.type",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.registry.value.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.data.type", v)?;
            }

            if event.has_value("json.registry.keyUid") {
                event.rename(
                    "json.registry.keyUid",
                    "sentinel_one_cloud_funnel.event.registry.key.uid",
                )?;
            }

            if event.has_value("json.registry.oldValue") {
                event.rename(
                    "json.registry.oldValue",
                    "sentinel_one_cloud_funnel.event.registry.old_value.detail",
                )?;
            }

            let _cond = { event.get_str("json.registry.oldValueFullSize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.registry.oldValueFullSize") {
                        if let Some(val) = event.get("json.registry.oldValueFullSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.registry.oldValueFullSize".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.registry.old_value.full_size",
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
                        "convert_json_registry_oldValueFullSize",
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

            let _cond = { event.get_str("json.registry.oldValueIsComplete") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.registry.oldValueIsComplete") {
                        if let Some(val) = event.get("json.registry.oldValueIsComplete") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.registry.oldValueIsComplete".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.registry.old_value.is_complete",
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
                        "convert_json_registry_oldValueIsComplete",
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

            if event.has_value("json.registry.oldValueType") {
                event.rename(
                    "json.registry.oldValueType",
                    "sentinel_one_cloud_funnel.event.registry.old_value.type",
                )?;
            }

            let _cond = { event.get_str("json.registry.valueFullSize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.registry.valueFullSize") {
                        if let Some(val) = event.get("json.registry.valueFullSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.registry.valueFullSize".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.registry.value.full_size",
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
                        "convert_json_registry_valueFullSize",
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

            let _cond = { event.get_str("json.registry.valueIsComplete") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.registry.valueIsComplete") {
                        if let Some(val) = event.get("json.registry.valueIsComplete") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.registry.valueIsComplete".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.registry.value.is_complete",
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
                        "convert_json_registry_valueIsComplete",
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

            if event.has_value("json.registry.valueType") {
                event.rename(
                    "json.registry.valueType",
                    "sentinel_one_cloud_funnel.event.registry.value.type",
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
