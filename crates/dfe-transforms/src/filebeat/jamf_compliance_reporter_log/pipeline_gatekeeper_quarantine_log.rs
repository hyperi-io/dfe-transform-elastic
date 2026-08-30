// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_gatekeeper_quarantine_log` pipeline.
pub struct PipelineGatekeeperQuarantineLog;

impl Transform for PipelineGatekeeperQuarantineLog {
    fn name(&self) -> &str {
        "pipeline_gatekeeper_quarantine_log"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.rename("_ingest._value.QuarantineAgentBundleIdentifier", "_ingest._value.quarantine.agent_bundle_identifier")?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.rename("_ingest._value.QuarantineAgentName", "_ingest._value.quarantine.agent_name")?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.rename("_ingest._value.QuarantineDataURLString", "_ingest._value.quarantine.data_url_string")?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.rename("_ingest._value.QuarantineEventIdentifier", "_ingest._value.quarantine.event_identifier")?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.rename("_ingest._value.QuarantineOriginURLString", "_ingest._value.quarantine.origin_url_string")?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.event_attributes.attributes").cloned();
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
                            if let Some(date_str) = event.get_as_string("_ingest._value.QuarantineTimeStamp") {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("_ingest._value.quarantine.timestamp", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.QuarantineTimeStamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.remove("_ingest._value.QuarantineTimeStamp");
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
                        event.set("json.event_attributes.attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.event_attributes.attributes").cloned();
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
                            if event.remove("_ingest._value.QuarantineTimeStamp").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.QuarantineTimeStamp".into() });
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
                        event.set("json.event_attributes.attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

                if event.has_value("json.event_attributes.attributes") {
                    event.rename("json.event_attributes.attributes", "jamf_compliance_reporter.log.event_attributes.attributes")?;
                }

                if event.has_value("json.event_attributes.path") {
                    event.rename("json.event_attributes.path", "jamf_compliance_reporter.log.event_attributes.path")?;
                }

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
