// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `automated_lead` pipeline.
pub struct AutomatedLead;

impl Transform for AutomatedLead {
    fn name(&self) -> &str {
        "automated_lead"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.category", json!("threat"))?;

                event.append("event.type", json!("indicator"))?;

                if event.has_value("crowdstrike.alert.falcon_host_link") {
                    event.rename("crowdstrike.alert.falcon_host_link", "event.reference")?;
                }

            if let Some(v) = event.get("crowdstrike.alert.display_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
            if let Some(v) = event.get("crowdstrike.alert.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }
            }

            let _cond = { event.has_value("crowdstrike.alert.signal_start_timestamp") && event.get_str("crowdstrike.alert.signal_start_timestamp") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("crowdstrike.alert.signal_start_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.alert.signal_start_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_signal_start_timestamp")?;
                        event.remove("crowdstrike.alert.signal_start_timestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("crowdstrike.alert.signal_end_timestamp") && event.get_str("crowdstrike.alert.signal_end_timestamp") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("crowdstrike.alert.signal_end_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.alert.signal_end_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_signal_end_timestamp")?;
                        event.remove("crowdstrike.alert.signal_end_timestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("crowdstrike.alert.signal_updated_timestamp") && event.get_str("crowdstrike.alert.signal_updated_timestamp") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("crowdstrike.alert.signal_updated_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("crowdstrike.alert.signal_updated_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.alert.signal_updated_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_signal_updated_timestamp")?;
                        event.remove("crowdstrike.alert.signal_updated_timestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("crowdstrike.alert.score") {
                if let Some(val) = event.get("crowdstrike.alert.score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.score".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_to_long")?;
                        event.remove("crowdstrike.alert.score");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get("crowdstrike.alert.score").is_some_and(|v| v.is_number()) };
            if _cond {
                // Painless script
                // Source: long score = ctx.crowdstrike.alert.score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"long score = ctx.crowdstrike.alert.score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}"#))?;
            }

            if event.has_value("crowdstrike.alert.pattern_id") {
                if let Some(val) = event.get("crowdstrike.alert.pattern_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_id".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("crowdstrike.alert.is_closed") {
                if let Some(val) = event.get("crowdstrike.alert.is_closed") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.is_closed".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.is_closed", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_crowdstrike_alert_is_closed_81aa8eac")?;
                        event.remove("crowdstrike.alert.is_closed");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.pattern_id") {
                    if let Some(val) = event.get("_ingest._value.pattern_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.pattern_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.pattern_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_threatgraph_indicators_pattern_id_to_string")?;
                    event.remove("_ingest._value.pattern_id");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.template_instance_id") {
                    if let Some(val) = event.get("_ingest._value.template_instance_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.template_instance_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.template_instance_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_threatgraph_indicators_template_instance_id_to_string")?;
                    event.remove("_ingest._value.template_instance_id");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.severity") {
                    if let Some(val) = event.get("_ingest._value.severity") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.severity".into(),
                    message,
                    })?;
                    event.set("_ingest._value.severity", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_threatgraph_indicators_severity_to_long")?;
                    event.remove("_ingest._value.severity");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.pattern_disposition") {
                    if let Some(val) = event.get("_ingest._value.pattern_disposition") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.pattern_disposition".into(),
                    message,
                    })?;
                    event.set("_ingest._value.pattern_disposition", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_threatgraph_indicators_pattern_disposition_to_long")?;
                    event.remove("_ingest._value.pattern_disposition");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    event.append_unique("threat.indicator.id", json!(event.get("_ingest._value.indicator_id").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    event.append_unique("threat.indicator.name", json!(event.get("_ingest._value.display_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    event.append_unique("threat.indicator.description", json!(event.get("_ingest._value.description").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: def indicator = ctx.crowdstrike.alert.threatgraph_indicators[0];\nif (indicator.host_id != null && indicator.host_id != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.host_id;\n}\nif (indicator.hostname != null && indicator.hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.hostname;\n}\nif (indicator.process_id != null && indicator.process_id != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.process_id;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def indicator = ctx.crowdstrike.alert.threatgraph_indicators[0];\nif (indicator.host_id != null && indicator.host_id != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.host_id;\n}\nif (indicator.hostname != null && indicator.hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.hostname;\n}\nif (indicator.process_id != null && indicator.process_id != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.process_id;\n}"#))?;
            }

            let _cond = { event.get("crowdstrike.alert.threatgraph_indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.hostname").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.user_names").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.user_names", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
