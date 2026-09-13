// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `endace_netflow` pipeline.
pub struct EndaceNetflow;

impl Transform for EndaceNetflow {
    fn name(&self) -> &str {
        "endace_netflow"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { (event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")) && (event.has_value("source.ip") && event.get_str("source.ip") != Some("")) };
            if _cond {
            event.set("_conf.ip_conv", json!(format!("ip_conv={}%26{}", event.get("source.ip").map_or_else(String::new, template_to_string), event.get("destination.ip").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { (event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")) && (!event.has_value("source.ip") || event.get_str("source.ip") == Some("")) };
            if _cond {
            event.set("_conf.ip_conv", json!(format!("ip={}", event.get("destination.ip").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { (!event.has_value("destination.ip") || event.get_str("destination.ip") == Some("")) && (event.has_value("source.ip") && event.get_str("source.ip") != Some("")) };
            if _cond {
            event.set("_conf.ip_conv", json!(format!("ip={}", event.get("source.ip").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { event.has_value("netflow.exporter.timestamp") && event.get_str("netflow.exporter.timestamp") != Some("") };
            if _cond {
                if let Some(date_str) = event.get_as_string("netflow.exporter.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, Some("epoch_millis")) {
                        Some(parsed) => event.set("_conf.event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netflow.exporter.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_conf.event.start") && event.get_str("_conf.event.start") != Some("") };
            if _cond {
                if let Some(val) = event.get("_conf.event.start") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_conf.event.start".into(),
                            message,
                        })?;
                    event.set("_conf.event.start", converted)?;
                }
            }

            let _cond = { event.has_value("netflow.exporter.timestamp") && event.get_str("netflow.exporter.timestamp") != Some("") };
            if _cond {
                if let Some(date_str) = event.get_as_string("netflow.exporter.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, Some("epoch_millis")) {
                        Some(parsed) => event.set("_conf.event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netflow.exporter.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_conf.event.end") && event.get_str("_conf.event.end") != Some("") };
            if _cond {
                if let Some(val) = event.get("_conf.event.end") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_conf.event.end".into(),
                            message,
                        })?;
                    event.set("_conf.event.end", converted)?;
                }
            }

            let _cond = { event.has_value("_conf.endace_view_window") && event.get_str("_conf.endace_view_window") != Some("") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx._conf.timedelta = ctx._conf.endace_view_window * 60 * 1000
                scale_field(event, &ScaleField::new("_conf.endace_view_window", "_conf.timedelta", Factor::Long(60000)));
            }

            let _cond = { event.has_value("_conf.event.end") && event.get_str("_conf.event.end") != Some("") && event.has_value("_conf.timedelta") && event.get_str("_conf.timedelta") != Some("") };
            if _cond {
                // Painless script
                // Source: ctx._conf.event.end = ctx._conf.event.end + ctx._conf.timedelta/2
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx._conf.event.end = ctx._conf.event.end + ctx._conf.timedelta/2"#))?;
            }

            let _cond = { (event.has_value("_conf.event.start") && event.get_str("_conf.event.start") != Some("")) && (event.has_value("_conf.timedelta") && event.get_str("_conf.timedelta") != Some("")) };
            if _cond {
                // Painless script
                // Source: ctx._conf.event.start = ctx._conf.event.start - ctx._conf.timedelta/2
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx._conf.event.start = ctx._conf.event.start - ctx._conf.timedelta/2"#))?;
            }

            let _cond = { (event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")) || (event.has_value("source.ip") && event.get_str("source.ip") != Some("")) };
            if _cond {
            let v = json!(format!("{}/vision2/pivotintovision/?title=endace_pivot&datasources={}&start={}&end={}&tools={}&{}", event.get("_conf.endace_url").map_or_else(String::new, template_to_string), event.get("_conf.endace_datasources").map_or_else(String::new, template_to_string), event.get("_conf.event.start").map_or_else(String::new, template_to_string), event.get("_conf.event.end").map_or_else(String::new, template_to_string), event.get("_conf.endace_tools").map_or_else(String::new, template_to_string), event.get("_conf.ip_conv").map_or_else(String::new, template_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("event.reference", v)?;
            }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
