// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_xprotect_event_log` pipeline.
pub struct PipelineXprotectEventLog;

impl Transform for PipelineXprotectEventLog {
    fn name(&self) -> &str {
        "pipeline_xprotect_event_log"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.activity_identifier", json!(event.get("_ingest._value.activityIdentifier").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.activity_identifier") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.activity_identifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.activity_identifier".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.activity_identifier", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    foreach_array(event, "_ingest._value.backtrace.frames", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset", json!(event.get("_ingest._value.imageOffset").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                    })?;
                    Ok(())
                })?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset", converted)?;
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

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    foreach_array(event, "_ingest._value.backtrace.frames", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_uuid", json!(event.get("_ingest._value.imageUUID").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.category", json!(event.get("_ingest._value.category").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.event.message", json!(event.get("_ingest._value.eventMessage").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.event.type", json!(event.get("_ingest._value.eventType").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.format_string", json!(event.get("_ingest._value.formatString").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("_ingest._value.machTimestamp") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.machTimestamp".into(),
                    message,
                    })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.mach_timestamp", converted)?;
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.remove("_ingest._value.machTimestamp");
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

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("event.type", json!("info"))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", json!(event.get("_ingest._value.parentActivityIdentifier").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.parent_activity_identifier".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.process.id", json!(event.get("_ingest._value.processID").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.process.id") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.process.id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.process.id".into(),
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

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.process.image.path", json!(event.get("_ingest._value.processImagePath").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.process.image.uuid", json!(event.get("_ingest._value.processImageUUID").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.image.path", json!(event.get("_ingest._value.senderImagePath").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.image.uuid", json!(event.get("_ingest._value.senderImageUUID").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.program_counter", json!(event.get("_ingest._value.senderProgramCounter").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.sender.program_counter") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.sender.program_counter") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.sender.program_counter".into(),
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

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.source", json!(event.get("_ingest._value.source").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.subsystem", json!(event.get("_ingest._value.subsystem").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.thread_id", json!(event.get("_ingest._value.threadID").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.thread_id") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.thread_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.thread_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.thread_id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.event_attributes").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.timestamp") {
                            match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSSZ"], None, None) {
                            Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.timestamp", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.remove("_ingest._value.timestamp");
                            event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
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
                        event.set("json.event_attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.timezone_name", json!(event.get("_ingest._value.timezone_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.timezoneName", json!(event.get("_ingest._value.timezoneName").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("jamf_compliance_reporter.log.event_attributes.trace_id", json!(event.get("_ingest._value.traceID").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("jamf_compliance_reporter.log.event_attributes.trace_id") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.trace_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "jamf_compliance_reporter.log.event_attributes.trace_id".into(),
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
