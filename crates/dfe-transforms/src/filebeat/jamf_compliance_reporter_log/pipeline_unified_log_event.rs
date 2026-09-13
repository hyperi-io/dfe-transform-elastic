// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_unified_log_event` pipeline.
pub struct PipelineUnifiedLogEvent;

impl Transform for PipelineUnifiedLogEvent {
    fn name(&self) -> &str {
        "pipeline_unified_log_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.activityIdentifier") {
                if let Some(val) = event.get("json.event_attributes.activityIdentifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.activityIdentifier".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.activity_identifier", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.get("json.event_attributes.backtrace.frames").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.backtrace.frames", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("_ingest._value.imageOffset") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.imageOffset".into(),
                    message,
                    })?;
                    event.set("_ingest._value.image_offset", converted)?;
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.remove("_ingest._value.imageOffset");
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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

            let _cond = { event.get("json.event_attributes.backtrace.frames").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.event_attributes.backtrace.frames").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            if event.remove("_ingest._value.imageOffset").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.imageOffset".into() });
                            }
                            Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.event_attributes.backtrace.frames", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("json.event_attributes.backtrace.frames").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.backtrace.frames", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.rename("_ingest._value.imageUUID", "_ingest._value.image_uuid")?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

                if event.has_value("json.event_attributes.backtrace.frames") {
                    event.rename("json.event_attributes.backtrace.frames", "jamf_compliance_reporter.log.event_attributes.backtrace.frames")?;
                }

                if event.has_value("json.event_attributes.category") {
                    event.rename("json.event_attributes.category", "jamf_compliance_reporter.log.event_attributes.category")?;
                }

                if event.has_value("json.event_attributes.eventMessage") {
                    event.rename("json.event_attributes.eventMessage", "jamf_compliance_reporter.log.event_attributes.event.message")?;
                }

                if event.has_value("json.event_attributes.eventType") {
                    event.rename("json.event_attributes.eventType", "jamf_compliance_reporter.log.event_attributes.event.type")?;
                }

                if event.has_value("json.event_attributes.formatString") {
                    event.rename("json.event_attributes.formatString", "jamf_compliance_reporter.log.event_attributes.format_string")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.machTimestamp") {
                if let Some(val) = event.get("json.event_attributes.machTimestamp") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.machTimestamp".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.mach_timestamp", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.event_attributes.messageType") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("event.type", json!("info"))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.parentActivityIdentifier") {
                if let Some(val) = event.get("json.event_attributes.parentActivityIdentifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.parentActivityIdentifier".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event_attributes.processID") {
                if let Some(val) = event.get("json.event_attributes.processID") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.processID".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.process.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.event_attributes.processImagePath") {
                    event.rename("json.event_attributes.processImagePath", "jamf_compliance_reporter.log.event_attributes.process.image.path")?;
                }

                if event.has_value("json.event_attributes.processImageUUID") {
                    event.rename("json.event_attributes.processImageUUID", "jamf_compliance_reporter.log.event_attributes.process.image.uuid")?;
                }

                if event.has_value("json.event_attributes.senderImagePath") {
                    event.rename("json.event_attributes.senderImagePath", "jamf_compliance_reporter.log.event_attributes.sender.image.path")?;
                }

                if event.has_value("json.event_attributes.senderImageUUID") {
                    event.rename("json.event_attributes.senderImageUUID", "jamf_compliance_reporter.log.event_attributes.sender.image.uuid")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event_attributes.senderProgramCounter") {
                if let Some(val) = event.get("json.event_attributes.senderProgramCounter") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.senderProgramCounter".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.sender.program_counter", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.event_attributes.source") {
                    event.rename("json.event_attributes.source", "jamf_compliance_reporter.log.event_attributes.source")?;
                }

                if event.has_value("json.event_attributes.subsystem") {
                    event.rename("json.event_attributes.subsystem", "jamf_compliance_reporter.log.event_attributes.subsystem")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.threadID") {
                if let Some(val) = event.get("json.event_attributes.threadID") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.threadID".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.thread_id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.event_attributes.timestamp") && event.get_i64("json.event_attributes.timestamp") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSSZ"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_attributes.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.event_attributes.timezoneName") {
                    event.rename("json.event_attributes.timezoneName", "jamf_compliance_reporter.log.event_attributes.timezone_name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.traceID") {
                if let Some(val) = event.get("json.event_attributes.traceID") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.traceID".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.trace_id", converted)?;
                }
            }
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
