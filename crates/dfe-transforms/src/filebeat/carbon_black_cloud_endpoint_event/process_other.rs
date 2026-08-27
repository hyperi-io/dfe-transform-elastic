// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `process_other` pipeline.
pub struct ProcessOther;

impl Transform for ProcessOther {
    fn name(&self) -> &str {
        "process_other"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.parent_cmdline") {
                    event.rename("json.parent_cmdline", "process.parent.command_line")?;
                }

                if event.has_value("json.parent_path") {
                    event.rename("json.parent_path", "process.parent.executable")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.parent_pid") {
                if let Some(val) = event.get("json.parent_pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.parent_pid".into(),
                            message,
                        })?;
                    event.set("process.parent.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(format!("Processor {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.parent_guid") {
                    event.rename("json.parent_guid", "process.parent.entity_id")?;
                }

                if event.has_value("json.parent_reputation") {
                    event.rename("json.parent_reputation", "carbon_black_cloud.endpoint_event.process.parent.reputation")?;
                }

                if event.has_value("json.parent_hash_md5") {
                    event.rename("json.parent_hash_md5", "process.parent.hash.md5")?;
                }

                if event.has_value("json.parent_hash_sha256") {
                    event.rename("json.parent_hash_sha256", "process.parent.hash.sha256")?;
                }

                if event.has_value("json.process_cmdline") {
                    event.rename("json.process_cmdline", "process.command_line")?;
                }

                if event.has_value("json.process_path") {
                    event.rename("json.process_path", "process.executable")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.process_pid") {
                if let Some(val) = event.get("json.process_pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.process_pid".into(),
                            message,
                        })?;
                    event.set("process.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(format!("Processor {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.process_guid") {
                    event.rename("json.process_guid", "process.entity_id")?;
                }

                if event.has_value("json.process_username") {
                    event.rename("json.process_username", "carbon_black_cloud.endpoint_event.process.username")?;
                }

                if event.has_value("json.process_reputation") {
                    event.rename("json.process_reputation", "carbon_black_cloud.endpoint_event.process.reputation")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.process_publisher") {
                foreach_array(event, "json.process_publisher", |event| {
                    if event.has_value("_ingest._value.state") {
                    if let Some(s) = event.get_string("_ingest._value.state") {
                    let parts: Vec<Value> = cached_regex!(" \\| ")
                    .split(&s)
                    .into_iter()
                    .map(|p| json!(p))
                    .collect();
                    event.set("_ingest._value.state", Value::Array(parts))?;
                    }
                    }
                    Ok(())
                })?;
            }
                Ok(())
            })();

                if event.has_value("json.process_publisher") {
                    event.rename("json.process_publisher", "carbon_black_cloud.endpoint_event.process.publisher")?;
                }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.process_duration") {
                if let Some(val) = event.get("json.process_duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.process_duration".into(),
                            message,
                        })?;
                    event.set("carbon_black_cloud.endpoint_event.process.duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        if event.remove("json.process_duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "json.process_duration".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.process_terminated") {
                if let Some(val) = event.get("json.process_terminated") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.process_terminated".into(),
                            message,
                        })?;
                    event.set("carbon_black_cloud.endpoint_event.process.terminated", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(format!("Processor {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.process_hash_md5") {
                    event.rename("json.process_hash_md5", "process.hash.md5")?;
                }

                if event.has_value("json.process_hash_sha256") {
                    event.rename("json.process_hash_sha256", "process.hash.sha256")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
