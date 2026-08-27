// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("alert"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "crowdstrike.alert")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.product") == Some("automated-lead")
                    || event.get_str("crowdstrike.alert.product") == Some("automated-lead-context")
            };
            if _cond {
                // Begin nested pipeline: "automated_lead"
                event.append("event.category", json!("threat"))?;
                event.append("event.type", json!("indicator"))?;
                if event.has_value("crowdstrike.alert.falcon_host_link") {
                    event.rename("crowdstrike.alert.falcon_host_link", "event.reference")?;
                }
                if let Some(v) = event
                    .get("crowdstrike.alert.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
                if _cond {
                    if let Some(v) = event
                        .get("crowdstrike.alert.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.alert.signal_start_timestamp")
                        && event.get_str("crowdstrike.alert.signal_start_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.alert.signal_start_timestamp")
                        {
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
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_signal_start_timestamp",
                        )?;
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
                let _cond = {
                    event.has_value("crowdstrike.alert.signal_end_timestamp")
                        && event.get_str("crowdstrike.alert.signal_end_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.alert.signal_end_timestamp")
                        {
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
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_signal_end_timestamp",
                        )?;
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
                let _cond = {
                    event.has_value("crowdstrike.alert.signal_updated_timestamp")
                        && event.get_str("crowdstrike.alert.signal_updated_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.alert.signal_updated_timestamp")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("crowdstrike.alert.signal_updated_timestamp", parsed)?,
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
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_signal_updated_timestamp",
                        )?;
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
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.score".into(),
                                    message,
                                }
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.score")
                        .is_some_and(|v| v.is_number())
                };
                if _cond {
                    // Painless script
                    // Source: long score = ctx.crowdstrike.alert.score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long score = ctx.crowdstrike.alert.score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                }
                if event.has_value("crowdstrike.alert.pattern_id") {
                    if let Some(val) = event.get("crowdstrike.alert.pattern_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.alert.pattern_id".into(),
                                message,
                            }
                        })?;
                        event.set("crowdstrike.alert.pattern_id", converted)?;
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.is_closed") {
                        if let Some(val) = event.get("crowdstrike.alert.is_closed") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.is_closed".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.is_closed", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_crowdstrike_alert_is_closed_81aa8eac",
                    )?;
                    event.remove("crowdstrike.alert.is_closed");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.pattern_id") {
                                if let Some(val) = event.get("_ingest._value.pattern_id") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.pattern_id".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.pattern_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_threatgraph_indicators_pattern_id_to_string",
                            )?;
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.template_instance_id") {
                                if let Some(val) = event.get("_ingest._value.template_instance_id")
                                {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.template_instance_id".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.template_instance_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_threatgraph_indicators_template_instance_id_to_string",
                            )?;
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.severity") {
                                if let Some(val) = event.get("_ingest._value.severity") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.severity".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.severity", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_threatgraph_indicators_severity_to_long",
                            )?;
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.pattern_disposition") {
                                if let Some(val) = event.get("_ingest._value.pattern_disposition") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.pattern_disposition".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.pattern_disposition", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_threatgraph_indicators_pattern_disposition_to_long",
                            )?;
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        event.append_unique(
                            "threat.indicator.id",
                            json!(
                                event
                                    .get("_ingest._value.indicator_id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        event.append_unique(
                            "threat.indicator.name",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        event.append_unique(
                            "threat.indicator.description",
                            json!(
                                event
                                    .get("_ingest._value.description")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.threatgraph_indicators")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: def indicator = ctx.crowdstrike.alert.threatgraph_indicators[0];\nif (indicator.host_id != null && indicator.host_id != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.host_id;\n}\nif (indicator.hostname != null && indicator.hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.hostname;\n}\nif (indicator.process_id != null && indicator.process_id != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.process_id;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def indicator = ctx.crowdstrike.alert.threatgraph_indicators[0];\nif (indicator.host_id != null && indicator.host_id != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.host_id;\n}\nif (indicator.hostname != null && indicator.hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.hostname;\n}\nif (indicator.process_id != null && indicator.process_id != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.process_id;\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.threatgraph_indicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.threatgraph_indicators", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.user_names")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.user_names", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                // End nested pipeline: "automated_lead"
            }

            let _cond = {
                event.get_str("crowdstrike.alert.product") == Some("ngsiem")
                    && event.get_str("crowdstrike.alert.type") == Some("correlation-detection")
            };
            if _cond {
                // Begin nested pipeline: "correlation_detection"
                event.append("event.type", json!("info"))?;
                if let Some(v) = event
                    .get("crowdstrike.alert.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
                if _cond {
                    if let Some(v) = event
                        .get("crowdstrike.alert.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                if let Some(v) = event
                    .get("crowdstrike.alert.falcon_host_link")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.url", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.alert.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.alert.correlation_rule_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.id", v)?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.source_products")
                        .is_some_and(|v| v.is_array())
                        && event.get("crowdstrike.alert.source_products").is_some_and(
                            |v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            },
                        )
                };
                if _cond {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nctx.event.provider = ctx.crowdstrike.alert.source_products[0];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nctx.event.provider = ctx.crowdstrike.alert.source_products[0];"#
                        ),
                    )?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.has_truncated_entities") {
                        if let Some(val) = event.get("crowdstrike.alert.has_truncated_entities") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.has_truncated_entities".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.has_truncated_entities", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_has_truncated_entities_to_boolean",
                    )?;
                    event.remove("crowdstrike.alert.has_truncated_entities");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.correlation_rule_create_case") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.correlation_rule_create_case")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.correlation_rule_create_case".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("crowdstrike.alert.correlation_rule_create_case", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_correlation_rule_create_case_to_boolean",
                    )?;
                    event.remove("crowdstrike.alert.correlation_rule_create_case");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.original_correlation_rules_entities_count")
                    {
                        if let Some(val) =
                            event.get("crowdstrike.alert.original_correlation_rules_entities_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                path: "crowdstrike.alert.original_correlation_rules_entities_count".into(),
                message,
                }
                            })?;
                            event.set(
                                "crowdstrike.alert.original_correlation_rules_entities_count",
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
                        "convert_original_correlation_rules_entities_count_to_long",
                    )?;
                    event.remove("crowdstrike.alert.original_correlation_rules_entities_count");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.original_indicator_entities_count") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.original_indicator_entities_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.original_indicator_entities_count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.original_indicator_entities_count",
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
                        "convert_original_indicator_entities_count_to_long",
                    )?;
                    event.remove("crowdstrike.alert.original_indicator_entities_count");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.users")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.users", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.full_name_is_enriched") {
                                if let Some(val) = event.get("_ingest._value.full_name_is_enriched")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.full_name_is_enriched".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.full_name_is_enriched", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_users_full_name_is_enriched_to_boolean",
                            )?;
                            event.remove("_ingest._value.full_name_is_enriched");
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.comments")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) =
                        event.get("crowdstrike.alert.comments").cloned()
                    {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.timestamp")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.timestamp", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.timestamp".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_comments_timestamp",
                                )?;
                                event.remove("_ingest._value.timestamp");
                                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        event.set("crowdstrike.alert.comments", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.users")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.users", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.idp_id_is_enriched") {
                                if let Some(val) = event.get("_ingest._value.idp_id_is_enriched") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.idp_id_is_enriched".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.idp_id_is_enriched", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_users_idp_id_is_enriched_to_boolean",
                            )?;
                            event.remove("_ingest._value.idp_id_is_enriched");
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.host_names")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.host_names")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: ctx.host = ctx.host ?: [:];\nif (ctx.host.name == null || ctx.host.name == '') {\n  ctx.host.name = ctx.crowdstrike.alert.host_names[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.host = ctx.host ?: [:];\nif (ctx.host.name == null || ctx.host.name == '') {\n  ctx.host.name = ctx.crowdstrike.alert.host_names[0];\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.host_names")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.host_names", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.source_hosts")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.source_hosts", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.destination_hosts")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.destination_hosts", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.source_hosts")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.source_hosts")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: ctx.source = ctx.source ?: [:];\nif (ctx.source.domain == null || ctx.source.domain == '') {\n  ctx.source.domain = ctx.crowdstrike.alert.source_hosts[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.source = ctx.source ?: [:];\nif (ctx.source.domain == null || ctx.source.domain == '') {\n  ctx.source.domain = ctx.crowdstrike.alert.source_hosts[0];\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.destination_hosts")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.destination_hosts")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.domain == null || ctx.destination.domain == '') {\n  ctx.destination.domain = ctx.crowdstrike.alert.destination_hosts[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.domain == null || ctx.destination.domain == '') {\n  ctx.destination.domain = ctx.crowdstrike.alert.destination_hosts[0];\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.source_ips")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.source_ips", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_ips_to_ip",
                            )?;
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.source_ips")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.source_ips")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: ctx.source = ctx.source ?: [:];\nif (ctx.source.ip == null || ctx.source.ip == '') {\n  ctx.source.ip = ctx.crowdstrike.alert.source_ips[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.source = ctx.source ?: [:];\nif (ctx.source.ip == null || ctx.source.ip == '') {\n  ctx.source.ip = ctx.crowdstrike.alert.source_ips[0];\n}"#
                        ),
                    )?;
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.source_ips")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.source_ips", |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.destination_ips")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.destination_ips", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ips_to_ip",
                            )?;
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
                let _cond = {
                    event
                        .get("crowdstrike.alert.destination_ips")
                        .is_some_and(|v| v.is_array())
                        && event.get("crowdstrike.alert.destination_ips").is_some_and(
                            |v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            },
                        )
                };
                if _cond {
                    // Painless script
                    // Source: ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.ip == null || ctx.destination.ip == '') {\n  ctx.destination.ip = ctx.crowdstrike.alert.destination_ips[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.ip == null || ctx.destination.ip == '') {\n  ctx.destination.ip = ctx.crowdstrike.alert.destination_ips[0];\n}"#
                        ),
                    )?;
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.destination_ips")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.destination_ips", |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event.has_value("crowdstrike.alert.correlation_rule_user_id")
                        && event.get_str("crowdstrike.alert.correlation_rule_user_id") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("crowdstrike.alert.correlation_rule_user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.user_names")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.user_names")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: ctx.user = ctx.user ?: [:];\nif (ctx.user.name == null || ctx.user.name == '') {\n  ctx.user.name = ctx.crowdstrike.alert.user_names[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.user = ctx.user ?: [:];\nif (ctx.user.name == null || ctx.user.name == '') {\n  ctx.user.name = ctx.crowdstrike.alert.user_names[0];\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.user_names")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "crowdstrike.alert.user_names", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.alert.users")
                        .is_some_and(|v| v.is_array())
                        && event
                            .get("crowdstrike.alert.users")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                };
                if _cond {
                    // Painless script
                    // Source: for (def userEntry : ctx.crowdstrike.alert.users) {\n  if (userEntry.user_name != null && userEntry.user_name != '') {\n    ctx.user = ctx.user ?: [:];\n    if (ctx.user.name == null || ctx.user.name == '') {\n      ctx.user.name = userEntry.user_name;\n    }\n\n    ctx.related = ctx.related ?: [:];\n    ctx.related.user = ctx.related.user ?: [];\n    if (!ctx.related.user.contains(userEntry.user_name)) {\n      ctx.related.user.add(userEntry.user_name);\n    }\n  }\n\n  ctx.user = ctx.user ?: [:];\n  if (userEntry.sid != null && userEntry.sid != '' && (ctx.user.id == null || ctx.user.id == '')) {\n    ctx.user.id = userEntry.sid;\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (def userEntry : ctx.crowdstrike.alert.users) {\n  if (userEntry.user_name != null && userEntry.user_name != '') {\n    ctx.user = ctx.user ?: [:];\n    if (ctx.user.name == null || ctx.user.name == '') {\n      ctx.user.name = userEntry.user_name;\n    }\n\n    ctx.related = ctx.related ?: [:];\n    ctx.related.user = ctx.related.user ?: [];\n    if (!ctx.related.user.contains(userEntry.user_name)) {\n      ctx.related.user.add(userEntry.user_name);\n    }\n  }\n\n  ctx.user = ctx.user ?: [:];\n  if (userEntry.sid != null && userEntry.sid != '' && (ctx.user.id == null || ctx.user.id == '')) {\n    ctx.user.id = userEntry.sid;\n  }\n}"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.set("event.kind", json!("pipeline_error"))?;
                }
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append_unique("tags", json!("preserve_original_event"))?;
                }
                // End nested pipeline: "correlation_detection"
            }

            let _cond = {
                event.get_str("crowdstrike.alert.product") != Some("automated-lead")
                    && event.get_str("crowdstrike.alert.product") != Some("automated-lead-context")
                    && (event.has_value("crowdstrike.alert.process_id")
                        || event.has_value("crowdstrike.alert.triggering_process_graph_id"))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("process")]))?;
            }

            let _cond = {
                event.get_str("crowdstrike.alert.product") != Some("automated-lead")
                    && event.get_str("crowdstrike.alert.product") != Some("automated-lead-context")
                    && event.has_value("crowdstrike.alert.process_start_time")
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = {
                event.get_str("crowdstrike.alert.active_directory_authentication_method")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.active_directory_authentication_method") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.active_directory_authentication_method")
                        {
                            let converted =
                                convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                            path: "crowdstrike.alert.active_directory_authentication_method".into(),
                            message,
                        }
                                })?;
                            event.set(
                                "crowdstrike.alert.active_directory_authentication_method",
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
                        "convert_active_directory_authentication_method_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.activity_browser") {
                event.rename(
                    "crowdstrike.alert.activity_browser",
                    "crowdstrike.alert.activity.browser",
                )?;
            }

            if event.has_value("crowdstrike.alert.activity_device") {
                event.rename(
                    "crowdstrike.alert.activity_device",
                    "crowdstrike.alert.activity.device",
                )?;
            }

            if event.has_value("crowdstrike.alert.activity_id") {
                event.rename(
                    "crowdstrike.alert.activity_id",
                    "crowdstrike.alert.activity.id",
                )?;
            }

            if event.has_value("crowdstrike.alert.activity_os") {
                event.rename(
                    "crowdstrike.alert.activity_os",
                    "crowdstrike.alert.activity.os",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.agent_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.get_str("crowdstrike.alert.alert_attributes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.alert_attributes") {
                        if let Some(val) = event.get("crowdstrike.alert.alert_attributes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.alert_attributes".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.alert_attributes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_attributes_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.assigned_to_name") {
                event.rename(
                    "crowdstrike.alert.assigned_to_name",
                    "crowdstrike.alert.assigned_to.name",
                )?;
            }

            if event.has_value("crowdstrike.alert.assigned_to_uid") {
                event.rename(
                    "crowdstrike.alert.assigned_to_uid",
                    "crowdstrike.alert.assigned_to.uid",
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.assigned_to.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.assigned_to.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.assigned_to_uuid") {
                event.rename(
                    "crowdstrike.alert.assigned_to_uuid",
                    "crowdstrike.alert.assigned_to.uuid",
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.associated_files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.associated_files", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("crowdstrike.alert.cloud_indicator") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.cloud_indicator") {
                        if let Some(val) = event.get("crowdstrike.alert.cloud_indicator") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.cloud_indicator".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.cloud_indicator", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cloud_indicator_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.prevented") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.prevented") {
                        if let Some(val) = event.get("crowdstrike.alert.prevented") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.prevented".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.prevented", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_prevented_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            let _cond = { event.get_str("crowdstrike.alert.confidence") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.confidence") {
                        if let Some(val) = event.get("crowdstrike.alert.confidence") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.confidence".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.confidence", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_confidence_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.context_timestamp")
                    && event.get_str("crowdstrike.alert.context_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.context_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.context_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.context_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_context_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.crawled_timestamp")
                    && event.get_str("crowdstrike.alert.crawled_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.crawled_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.crawled_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.crawled_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_crawled_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.created_timestamp")
                    && event.get_str("crowdstrike.alert.created_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.created_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.created_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.created_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.get_str("crowdstrike.alert.device.agent_load_flags") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.device.agent_load_flags") {
                        if let Some(val) = event.get("crowdstrike.alert.device.agent_load_flags") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.device.agent_load_flags".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.device.agent_load_flags", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_agent_load_flags_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.device.agent_local_time")
                    && event.get_str("crowdstrike.alert.device.agent_local_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.device.agent_local_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.device.agent_local_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.device.agent_local_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_device_agent_local_time",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond =
                { event.get_str("crowdstrike.alert.device.config_id_platform") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.device.config_id_platform") {
                        if let Some(val) = event.get("crowdstrike.alert.device.config_id_platform")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.device.config_id_platform".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.device.config_id_platform", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_config_id_platform_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.device.external_ip")
                    && event.get_str("crowdstrike.alert.device.external_ip") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.device.external_ip") {
                        if let Some(val) = event.get("crowdstrike.alert.device.external_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.device.external_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.device.external_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_external_ip_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.device.external_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("crowdstrike.alert.device.external_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.device.external_ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("crowdstrike.alert.device.external_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.device.first_seen")
                    && event.get_str("crowdstrike.alert.device.first_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.device.first_seen")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.device.first_seen", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.device.first_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_device_first_seen")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.device.hostinfo.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            let _cond = { event.has_value("host.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.device.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.device.device_id") {
                event.rename(
                    "crowdstrike.alert.device.device_id",
                    "crowdstrike.alert.device.id",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.device.last_seen")
                    && event.get_str("crowdstrike.alert.device.last_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.device.last_seen")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.device.last_seen", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.device.last_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_device_last_seen")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.device.local_ip")
                    && event.get_str("crowdstrike.alert.device.local_ip") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.device.local_ip") {
                        if let Some(val) = event.get("crowdstrike.alert.device.local_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.device.local_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.device.local_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_local_ip_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.device.local_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("crowdstrike.alert.device.local_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.device.local_ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("crowdstrike.alert.device.local_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("crowdstrike.alert.device.mac_address") {
                    gsub_field(
                        event,
                        "crowdstrike.alert.device.mac_address",
                        "crowdstrike.alert.device.mac_address",
                        cached_regex!("[:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_device_mac_address",
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.device.mac_address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.device.mac_address") {
                        map_strings(
                            event,
                            "crowdstrike.alert.device.mac_address",
                            "crowdstrike.alert.device.mac_address",
                            str::to_uppercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uppercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "uppercase_device_mac_address",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.device.mac_address") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("crowdstrike.alert.device.mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.device.machine_domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.alert.device.machine_domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.device.modified_timestamp")
                    && event.get_str("crowdstrike.alert.device.modified_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.device.modified_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.device.modified_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.device.modified_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_device_modified_timestamp",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.device.os_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.device.platform_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.device.system_manufacturer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.manufacturer", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.device.system_product_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.device.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.device.tags", |event| {
                    event.append_unique(
                        "tags",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.documents_accessed")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.alert.documents_accessed").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.timestamp")
                            {
                                match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                                    Some(parsed) => {
                                        event.set("_ingest._value.timestamp", parsed)?
                                    }
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_documents_accessed_timestamp",
                            )?;
                            event.remove("_ingest._value.timestamp");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("crowdstrike.alert.documents_accessed", Value::Array(out))?;
                }
            }

            let _cond = { event.get_str("crowdstrike.alert.email_sent") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.email_sent") {
                        if let Some(val) = event.get("crowdstrike.alert.email_sent") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.email_sent".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.email_sent", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_sent_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.end_time")
                    && event.get_str("crowdstrike.alert.end_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("crowdstrike.alert.end_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("crowdstrike.alert.end_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.end_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_end_time")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.executables_written")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.alert.executables_written").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.timestamp")
                            {
                                match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                                    Some(parsed) => {
                                        event.set("_ingest._value.timestamp", parsed)?
                                    }
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_executables_written_timestamp",
                            )?;
                            event.remove("_ingest._value.timestamp");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("crowdstrike.alert.executables_written", Value::Array(out))?;
                }
            }

            if let Some(v) = event
                .get("crowdstrike.alert.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.filepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.filepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.file_writes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.file_writes", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.files_accessed")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.alert.files_accessed").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.timestamp")
                            {
                                match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                                    Some(parsed) => {
                                        event.set("_ingest._value.timestamp", parsed)?
                                    }
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_files_accessed_timestamp",
                            )?;
                            event.remove("_ingest._value.timestamp");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("crowdstrike.alert.files_accessed", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.files_written")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.alert.files_written").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.timestamp")
                            {
                                match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                                    Some(parsed) => {
                                        event.set("_ingest._value.timestamp", parsed)?
                                    }
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_files_written_timestamp",
                            )?;
                            event.remove("_ingest._value.timestamp");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("crowdstrike.alert.files_written", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("crowdstrike.alert.grandparent_details.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.grandparent_details.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.grandparent_details.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.grandparent_details.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.grandparent_details.timestamp")
                    && event.get_str("crowdstrike.alert.grandparent_details.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.grandparent_details.timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("crowdstrike.alert.grandparent_details.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.grandparent_details.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_grandparent_details_timestamp",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.grandparent_details.user_graph_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.grandparent_details.user_graph_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.grandparent_details.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.grandparent_details.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.grandparent_details.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.grandparent_details.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("crowdstrike.alert.has_script_or_module_ioc") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.has_script_or_module_ioc") {
                        if let Some(val) = event.get("crowdstrike.alert.has_script_or_module_ioc") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.has_script_or_module_ioc".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.has_script_or_module_ioc", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_has_script_or_module_ioc_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.host_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            let _cond = {
                !event.has_value("crowdstrike.alert.has_script_or_module_ioc")
                    && event
                        .get("crowdstrike.alert.ioc_context")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: if (ctx.crowdstrike == null) {\n  ctx.crowdstrike = [:];\n}\nif (ctx.crowdstrike.alert == null) {\n  ctx.crowdstrike.alert = [:];\n}\nfor (def c: ctx.crowdstrike.alert.ioc_context) {\n  if (c.type == 'module' || c.type == 'script') {\n    ctx.crowdstrike.alert.has_script_or_module_ioc = true;\n    return;\n  }\n}\nctx.crowdstrike.alert.has_script_or_module_ioc = false;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.crowdstrike == null) {\n  ctx.crowdstrike = [:];\n}\nif (ctx.crowdstrike.alert == null) {\n  ctx.crowdstrike.alert = [:];\n}\nfor (def c: ctx.crowdstrike.alert.ioc_context) {\n  if (c.type == 'module' || c.type == 'script') {\n    ctx.crowdstrike.alert.has_script_or_module_ioc = true;\n    return;\n  }\n}\nctx.crowdstrike.alert.has_script_or_module_ioc = false;\n"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond =
                { event.get_str("crowdstrike.alert.idp_policy_enforced_externally") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.idp_policy_enforced_externally") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.idp_policy_enforced_externally")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.idp_policy_enforced_externally".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.idp_policy.enforced_externally",
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
                        "convert_idp_policy_enforced_externally_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond =
                { event.get_str("crowdstrike.alert.idp_policy_mfa_factor_type") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.idp_policy_mfa_factor_type") {
                        if let Some(val) = event.get("crowdstrike.alert.idp_policy_mfa_factor_type")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.idp_policy_mfa_factor_type".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.idp_policy.mfa_factor_type", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_idp_policy_mfa_factor_type_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.idp_policy_mfa_provider") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.idp_policy_mfa_provider") {
                        if let Some(val) = event.get("crowdstrike.alert.idp_policy_mfa_provider") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.idp_policy_mfa_provider".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.idp_policy.mfa_provider", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_idp_policy_mfa_provider_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.idp_policy_rule_action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.idp_policy_rule_action") {
                        if let Some(val) = event.get("crowdstrike.alert.idp_policy_rule_action") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.idp_policy_rule_action".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.idp_policy.rule_action", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_idp_policy_rule_action_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.idp_policy_rule_trigger") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.idp_policy_rule_trigger") {
                        if let Some(val) = event.get("crowdstrike.alert.idp_policy_rule_trigger") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.idp_policy_rule_trigger".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.idp_policy.rule_trigger", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_idp_policy_rule_trigger_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.idp_policy_rule_id") {
                event.rename(
                    "crowdstrike.alert.idp_policy_rule_id",
                    "crowdstrike.alert.idp_policy.rule_id",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.idp_policy.rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("crowdstrike.alert.idp_policy_rule_name") {
                event.rename(
                    "crowdstrike.alert.idp_policy_rule_name",
                    "crowdstrike.alert.idp_policy.rule_name",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.idp_policy.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.image_file_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.incident.created")
                    && event.get_str("crowdstrike.alert.incident.created") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.incident.created")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss'Z'"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.incident.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.incident.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_incident_created")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.incident.end")
                    && event.get_str("crowdstrike.alert.incident.end") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("crowdstrike.alert.incident.end") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss'Z'"], None, None) {
                            Some(parsed) => event.set("crowdstrike.alert.incident.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.incident.end".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_incident_end")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.incident.score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.incident.score") {
                        if let Some(val) = event.get("crowdstrike.alert.incident.score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.incident.score".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.incident.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_incident_score_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.incident.start")
                    && event.get_str("crowdstrike.alert.incident.start") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("crowdstrike.alert.incident.start")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss'Z'"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.incident.start", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.incident.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_incident_start")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event
                    .get("crowdstrike.alert.ioc_context")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "crowdstrike.alert.ioc_context", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.ioc_context")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "crowdstrike.alert.ioc_context", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("crowdstrike.alert.ioc_value") };
            if _cond {
                event.append_unique(
                    "crowdstrike.alert.ioc_values",
                    json!(
                        event
                            .get("crowdstrike.alert.ioc_value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.ioc_context")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "crowdstrike.alert.ioc_context", |event| {
                        event.append_unique(
                            "crowdstrike.alert.ioc_values",
                            json!(
                                event
                                    .get("_ingest._value.ioc_value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("crowdstrike.alert.is_synthetic_quarantine_disposition") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.is_synthetic_quarantine_disposition") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.is_synthetic_quarantine_disposition")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.is_synthetic_quarantine_disposition"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.is_synthetic_quarantine_disposition",
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
                        "convert_is_synthetic_quarantine_disposition_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                !event.has_value("crowdstrike.alert.is_synthetic_quarantine_disposition")
                    && event
                        .get("crowdstrike.alert.pattern_disposition_details")
                        .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: if (ctx.crowdstrike == null) {\n  ctx.crowdstrike = [:];\n}\nif (ctx.crowdstrike.alert == null) {\n  ctx.crowdstrike.alert = [:];\n}\nfor (def d: ctx.crowdstrike.alert.pattern_disposition_details.entrySet()) {\n  if (d.getKey() == 'quarantine_file') {\n    ctx.crowdstrike.alert.is_synthetic_quarantine_disposition = d.getValue();\n    return;\n  }\n}\nctx.crowdstrike.alert.is_synthetic_quarantine_disposition = false;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.crowdstrike == null) {\n  ctx.crowdstrike = [:];\n}\nif (ctx.crowdstrike.alert == null) {\n  ctx.crowdstrike.alert = [:];\n}\nfor (def d: ctx.crowdstrike.alert.pattern_disposition_details.entrySet()) {\n  if (d.getKey() == 'quarantine_file') {\n    ctx.crowdstrike.alert.is_synthetic_quarantine_disposition = d.getValue();\n    return;\n  }\n}\nctx.crowdstrike.alert.is_synthetic_quarantine_disposition = false;\n"#
                    ),
                )?;
            }

            let _cond = { event.get_str("crowdstrike.alert.ldap_search_query_attack") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.ldap_search_query_attack") {
                        if let Some(val) = event.get("crowdstrike.alert.ldap_search_query_attack") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.ldap_search_query_attack".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.ldap_search_query_attack", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ldap_search_query_attack_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.location_country_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.country_iso_code", v)?;
            }

            let _cond = { event.get_str("crowdstrike.alert.location_latitude_as_int") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.location_latitude_as_int") {
                        if let Some(val) = event.get("crowdstrike.alert.location_latitude_as_int") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.location_latitude_as_int".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.location_latitude_as_int", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_location_latitude_as_int_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond =
                { event.get_str("crowdstrike.alert.location_longitude_as_int") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.location_longitude_as_int") {
                        if let Some(val) = event.get("crowdstrike.alert.location_longitude_as_int")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.location_longitude_as_int".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.location_longitude_as_int", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_location_longitude_as_int_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.location_latitude_as_int")
                    && event.has_value("crowdstrike.alert.location_longitude_as_int")
            };
            if _cond {
                // Painless script
                // Source: def location = new HashMap();\nlocation.put('lat', ctx.crowdstrike.alert.location_latitude_as_int);\nlocation.put('lon', ctx.crowdstrike.alert.location_longitude_as_int);\nif(ctx.observer == null) {\n  ctx.put('observer', new HashMap());\n}\nif(ctx.observer.geo == null){\n  ctx.observer.put('geo', new HashMap());\n}\nctx.observer.geo.location = location;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def location = new HashMap();\nlocation.put('lat', ctx.crowdstrike.alert.location_latitude_as_int);\nlocation.put('lon', ctx.crowdstrike.alert.location_longitude_as_int);\nif(ctx.observer == null) {\n  ctx.put('observer', new HashMap());\n}\nif(ctx.observer.geo == null){\n  ctx.observer.put('geo', new HashMap());\n}\nctx.observer.geo.location = location;"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.md5", v)?;
            }

            let _cond = { event.has_value("crowdstrike.alert.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.alert.network_accesses").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("_ingest._value.access_timestamp")
                            {
                                match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                                    Some(parsed) => {
                                        event.set("_ingest._value.access_timestamp", parsed)?
                                    }
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "_ingest._value.access_timestamp".into(),
                                            message: format!("unable to parse date [{date_str}]"),
                                        });
                                    }
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_network_accesses_access_timestamp",
                            )?;
                            event.remove("_ingest._value.access_timestamp");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("crowdstrike.alert.network_accesses", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.access_type") {
                            if let Some(val) = event.get("_ingest._value.access_type") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.access_type".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.access_type", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_network_accesses_access_type_to_long",
                        )?;
                        event.remove("_ingest._value.access_type");
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
                                    .get("_ingest.on_failure_pipeline")
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.isIPV6") {
                            if let Some(val) = event.get("_ingest._value.isIPV6") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.isIPV6".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.isIPV6", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_network_accesses_isIPV6_to_boolean",
                        )?;
                        event.remove("_ingest._value.isIPV6");
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
                                    .get("_ingest.on_failure_pipeline")
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.local_address") {
                            if let Some(val) = event.get("_ingest._value.local_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.local_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.local_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_network_accesses_local_address_to_ip",
                        )?;
                        event.remove("_ingest._value.local_address");
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
                                    .get("_ingest.on_failure_pipeline")
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.local_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.local_port") {
                            if let Some(val) = event.get("_ingest._value.local_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.local_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.local_port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_network_accesses_local_port_to_long",
                        )?;
                        event.remove("_ingest._value.local_port");
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
                                    .get("_ingest.on_failure_pipeline")
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.remote_address") {
                            if let Some(val) = event.get("_ingest._value.remote_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.remote_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.remote_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_network_accesses_remote_address_to_ip",
                        )?;
                        event.remove("_ingest._value.remote_address");
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
                                    .get("_ingest.on_failure_pipeline")
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.remote_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.network_accesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.network_accesses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.remote_port") {
                            if let Some(val) = event.get("_ingest._value.remote_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.remote_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.remote_port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_network_accesses_remote_port_to_long",
                        )?;
                        event.remove("_ingest._value.remote_port");
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
                                    .get("_ingest.on_failure_pipeline")
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
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.operating_system")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            // Painless script
            // Source: if (ctx.crowdstrike?.alert?.device?.platform_name != null) {\n  String platform_name = ctx.crowdstrike.alert.device.platform_name.toLowerCase();\n  for (String os: params.os_type) {\n    if (platform_name.contains(os)) {\n      ctx.host.os.put('type', os);\n      return;\n    }\n  }\n} else if (ctx.crowdstrike?.alert?.operating_system != null) {\n  String operating_system = ctx.crowdstrike.alert.operating_system.toLowerCase();\n  for (String os: params.os_type) {\n    if (operating_system.contains(os)) {\n      ctx.host.os.put('type', os);\n      return;\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.crowdstrike?.alert?.device?.platform_name != null) {\n  String platform_name = ctx.crowdstrike.alert.device.platform_name.toLowerCase();\n  for (String os: params.os_type) {\n    if (platform_name.contains(os)) {\n      ctx.host.os.put('type', os);\n      return;\n    }\n  }\n} else if (ctx.crowdstrike?.alert?.operating_system != null) {\n  String operating_system = ctx.crowdstrike.alert.operating_system.toLowerCase();\n  for (String os: params.os_type) {\n    if (operating_system.contains(os)) {\n      ctx.host.os.put('type', os);\n      return;\n    }\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                ),
            )?;

            if let Some(v) = event
                .get("crowdstrike.alert.os_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.cmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.filepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            let _cond = { event.has_value("crowdstrike.alert.parent_details.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.parent_details.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha256", v)?;
            }

            let _cond = { event.has_value("crowdstrike.alert.parent_details.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.parent_details.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.parent_details.timestamp")
                    && event.get_str("crowdstrike.alert.parent_details.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.parent_details.timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.parent_details.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.parent_details.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_parent_details_timestamp",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.parent_details.user_graph_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.parent_details.user_graph_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.id", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_details.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.name", v)?;
            }

            let _cond = { event.has_value("crowdstrike.alert.parent_details.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.parent_details.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.parent_details.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.parent_details.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.parent_process_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.entity_id", v)?;
            }

            let _cond = { event.get_str("crowdstrike.alert.parent_process_id") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.parent_process_id") {
                        if let Some(val) = event.get("crowdstrike.alert.parent_process_id") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.parent_process_id".into(),
                                    message,
                                }
                            })?;
                            event.set("process.parent.pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_parent_process_id",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.pattern_disposition") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition") {
                        if let Some(val) = event.get("crowdstrike.alert.pattern_disposition") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.pattern_disposition".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.pattern_disposition", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.blocking_unsupported_or_disabled") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.blocking_unsupported_or_disabled") {
                if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.blocking_unsupported_or_disabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.blocking_unsupported_or_disabled".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.blocking_unsupported_or_disabled", converted)?;
                }
            }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_pattern_disposition_details_blocking_unsupported_or_disabled_to_boolean")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.bootup_safeguard_enabled",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.bootup_safeguard_enabled",
                    ) {
                        if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.bootup_safeguard_enabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.bootup_safeguard_enabled".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.bootup_safeguard_enabled", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_bootup_safeguard_enabled_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.containment_file_system",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.containment_file_system",
                    ) {
                        if let Some(val) = event.get(
                            "crowdstrike.alert.pattern_disposition_details.containment_file_system",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.containment_file_system".into(),
                            message,
                        })?;
                            event.set("crowdstrike.alert.pattern_disposition_details.containment_file_system", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_containment_file_system_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.critical_process_disabled",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.critical_process_disabled",
                    ) {
                        if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.critical_process_disabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.critical_process_disabled".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.critical_process_disabled", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_critical_process_disabled_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.detect") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.detect") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.detect")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.pattern_disposition_details.detect"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.detect",
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
                        "convert_pattern_disposition_details_detect_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.fs_operation_blocked")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.fs_operation_blocked",
                    ) {
                        if let Some(val) = event.get(
                            "crowdstrike.alert.pattern_disposition_details.fs_operation_blocked",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.fs_operation_blocked".into(),
                            message,
                        })?;
                            event.set("crowdstrike.alert.pattern_disposition_details.fs_operation_blocked", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_fs_operation_blocked_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.handle_operation_downgraded",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.handle_operation_downgraded",
                    ) {
                        if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.handle_operation_downgraded") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.handle_operation_downgraded".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.handle_operation_downgraded", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_pattern_disposition_details_handle_operation_downgraded_to_boolean")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.inddet_mask")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.inddet_mask")
                    {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.inddet_mask")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "crowdstrike.alert.pattern_disposition_details.inddet_mask"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.inddet_mask",
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
                        "convert_pattern_disposition_details_inddet_mask_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.indicator") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.indicator") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.indicator")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.pattern_disposition_details.indicator"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.indicator",
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
                        "convert_pattern_disposition_details_indicator_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.kill_action_failed")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.kill_action_failed",
                    ) {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.kill_action_failed")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.kill_action_failed".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.kill_action_failed",
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
                        "convert_pattern_disposition_details_kill_action_failed_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.kill_parent")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.kill_parent")
                    {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.kill_parent")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "crowdstrike.alert.pattern_disposition_details.kill_parent"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.kill_parent",
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
                        "convert_pattern_disposition_details_kill_parent_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.kill_process")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.kill_process")
                    {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.kill_process")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "crowdstrike.alert.pattern_disposition_details.kill_process"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.kill_process",
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
                        "convert_pattern_disposition_details_kill_process_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.kill_subprocess")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.pattern_disposition_details.kill_subprocess")
                    {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.kill_subprocess")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.kill_subprocess".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.kill_subprocess",
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
                        "convert_pattern_disposition_details_kill_subprocess_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.mfa_required")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.mfa_required")
                    {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.mfa_required")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "crowdstrike.alert.pattern_disposition_details.mfa_required"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.mfa_required",
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
                        "convert_pattern_disposition_details_mfa_required_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.operation_blocked")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.operation_blocked",
                    ) {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.operation_blocked")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.operation_blocked".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.operation_blocked",
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
                        "convert_pattern_disposition_details_operation_blocked_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.policy_disabled")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.pattern_disposition_details.policy_disabled")
                    {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.policy_disabled")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.policy_disabled".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.policy_disabled",
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
                        "convert_pattern_disposition_details_policy_disabled_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.prevention_provisioning_enabled",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.prevention_provisioning_enabled") {
                if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.prevention_provisioning_enabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.prevention_provisioning_enabled".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.prevention_provisioning_enabled", converted)?;
                }
            }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_pattern_disposition_details_prevention_provisioning_enabled_to_boolean")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.process_blocked")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.pattern_disposition_details.process_blocked")
                    {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.process_blocked")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.process_blocked".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.process_blocked",
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
                        "convert_pattern_disposition_details_process_blocked_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.quarantine_file")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.pattern_disposition_details.quarantine_file")
                    {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.quarantine_file")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.quarantine_file".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.quarantine_file",
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
                        "convert_pattern_disposition_details_quarantine_file_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.quarantine_machine")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.quarantine_machine",
                    ) {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.quarantine_machine")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.quarantine_machine".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.quarantine_machine",
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
                        "convert_pattern_disposition_details_quarantine_machine_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.registry_operation_blocked",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.registry_operation_blocked",
                    ) {
                        if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.registry_operation_blocked") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.registry_operation_blocked".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.registry_operation_blocked", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_registry_operation_blocked_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.response_action_already_applied",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.response_action_already_applied") {
                if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.response_action_already_applied") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.response_action_already_applied".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.response_action_already_applied", converted)?;
                }
            }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_pattern_disposition_details_response_action_already_applied_to_boolean")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event
                    .get_str("crowdstrike.alert.pattern_disposition_details.response_action_failed")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.response_action_failed",
                    ) {
                        if let Some(val) = event.get(
                            "crowdstrike.alert.pattern_disposition_details.response_action_failed",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.response_action_failed".into(),
                            message,
                        })?;
                            event.set("crowdstrike.alert.pattern_disposition_details.response_action_failed", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_response_action_failed_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str(
                    "crowdstrike.alert.pattern_disposition_details.response_action_triggered",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "crowdstrike.alert.pattern_disposition_details.response_action_triggered",
                    ) {
                        if let Some(val) = event.get("crowdstrike.alert.pattern_disposition_details.response_action_triggered") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.response_action_triggered".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.pattern_disposition_details.response_action_triggered", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_pattern_disposition_details_response_action_triggered_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.rooting") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.rooting") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.rooting")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.pattern_disposition_details.rooting"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.rooting",
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
                        "convert_pattern_disposition_details_rooting_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.sensor_only")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.pattern_disposition_details.sensor_only")
                    {
                        if let Some(val) =
                            event.get("crowdstrike.alert.pattern_disposition_details.sensor_only")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "crowdstrike.alert.pattern_disposition_details.sensor_only"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.sensor_only",
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
                        "convert_pattern_disposition_details_sensor_only_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.suspend_parent")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.pattern_disposition_details.suspend_parent")
                    {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.suspend_parent")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.suspend_parent".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.suspend_parent",
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
                        "convert_pattern_disposition_details_suspend_parent_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.get_str("crowdstrike.alert.pattern_disposition_details.suspend_process")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("crowdstrike.alert.pattern_disposition_details.suspend_process")
                    {
                        if let Some(val) = event
                            .get("crowdstrike.alert.pattern_disposition_details.suspend_process")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_disposition_details.suspend_process".into(),
                            message,
                        })?;
                            event.set(
                                "crowdstrike.alert.pattern_disposition_details.suspend_process",
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
                        "convert_pattern_disposition_details_suspend_process_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.pattern_id") {
                if let Some(val) = event.get("crowdstrike.alert.pattern_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.alert.pattern_id".into(),
                            message,
                        }
                    })?;
                    event.set("crowdstrike.alert.pattern_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.alert.process_end_time")
                    && event.get_str("crowdstrike.alert.process_end_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.process_end_time")
                    {
                        match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.process_end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.process_end_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_process_end_time")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.process_end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.end", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.process_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entity_id", v)?;
            }

            let _cond = { event.get_str("crowdstrike.alert.process_id") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.process_id") {
                        if let Some(val) = event.get("crowdstrike.alert.process_id") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.process_id".into(),
                                    message,
                                }
                            })?;
                            event.set("process.pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_process_id_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.process_start_time")
                    && event.get_str("crowdstrike.alert.process_start_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.process_start_time")
                    {
                        match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.process_start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.process_start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_process_start_time",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.process_start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.start", v)?;
            }

            let _cond =
                { event.get_str("crowdstrike.alert.protocol_anomaly_classification") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.protocol_anomaly_classification") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.protocol_anomaly_classification")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.protocol_anomaly_classification"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.protocol_anomaly_classification",
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
                        "convert_protocol_anomaly_classification_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.quarantined") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.quarantined") {
                        if let Some(val) = event.get("crowdstrike.alert.quarantined") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.quarantined".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.quarantined", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_quarantined_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event
                    .get("crowdstrike.alert.quarantined_files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "crowdstrike.alert.quarantined_files", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("crowdstrike.alert.seconds_to_resolved") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.seconds_to_resolved") {
                        if let Some(val) = event.get("crowdstrike.alert.seconds_to_resolved") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.seconds_to_resolved".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.seconds_to_resolved", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_seconds_to_resolved_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.seconds_to_triaged") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.seconds_to_triaged") {
                        if let Some(val) = event.get("crowdstrike.alert.seconds_to_triaged") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.seconds_to_triaged".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.seconds_to_triaged", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_seconds_to_triaged_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("crowdstrike.alert.severity") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.severity") {
                        if let Some(val) = event.get("crowdstrike.alert.severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.severity".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_severity_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event
                    .get("crowdstrike.alert.severity")
                    .is_some_and(|v| v.is_number())
                    && !event.has_value("crowdstrike.alert.severity_name")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: long severity = ctx.crowdstrike.alert.severity;\nif (0 <= severity && severity < 20) {\n  ctx.crowdstrike.alert.severity_name = \"info\";\n} else if (20 <= severity && severity < 40) {\n  ctx.crowdstrike.alert.severity_name = \"low\";\n} else if (40 <= severity && severity < 60) {\n  ctx.crowdstrike.alert.severity_name = \"medium\";\n} else if (60 <= severity && severity < 80) {\n  ctx.crowdstrike.alert.severity_name = \"high\";\n} else if (80 <= severity && severity <= 100) {\n  ctx.crowdstrike.alert.severity_name = \"critical\";\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long severity = ctx.crowdstrike.alert.severity;\nif (0 <= severity && severity < 20) {\n  ctx.crowdstrike.alert.severity_name = \"info\";\n} else if (20 <= severity && severity < 40) {\n  ctx.crowdstrike.alert.severity_name = \"low\";\n} else if (40 <= severity && severity < 60) {\n  ctx.crowdstrike.alert.severity_name = \"medium\";\n} else if (60 <= severity && severity < 80) {\n  ctx.crowdstrike.alert.severity_name = \"high\";\n} else if (80 <= severity && severity <= 100) {\n  ctx.crowdstrike.alert.severity_name = \"critical\";\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_severity_from_severity",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event
                    .get("crowdstrike.alert.severity_name")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.crowdstrike.alert.severity_name;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"info\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.crowdstrike.alert.severity_name;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"info\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_severity_from_severity_name",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("crowdstrike.alert.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.alert.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("crowdstrike.alert.show_in_ui") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.show_in_ui") {
                        if let Some(val) = event.get("crowdstrike.alert.show_in_ui") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.show_in_ui".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.show_in_ui", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_show_in_ui_to_boolean",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.source_account_azure_id") {
                event.rename(
                    "crowdstrike.alert.source_account_azure_id",
                    "crowdstrike.alert.source.account_azure_id",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.source.account_azure_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("crowdstrike.alert.source_account_domain") {
                event.rename(
                    "crowdstrike.alert.source_account_domain",
                    "crowdstrike.alert.source.account_domain",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.source.account_domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.domain", v)?;
            }

            let _cond = { event.has_value("source.user.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.source_account_name") {
                event.rename(
                    "crowdstrike.alert.source_account_name",
                    "crowdstrike.alert.source.account_name",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.source.account_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.source_account_object_guid") {
                event.rename(
                    "crowdstrike.alert.source_account_object_guid",
                    "crowdstrike.alert.source.account_object_guid",
                )?;
            }

            if event.has_value("crowdstrike.alert.source_account_object_sid") {
                event.rename(
                    "crowdstrike.alert.source_account_object_sid",
                    "crowdstrike.alert.source.account_object_sid",
                )?;
            }

            if event.has_value("crowdstrike.alert.source_account_sam_account_name") {
                event.rename(
                    "crowdstrike.alert.source_account_sam_account_name",
                    "crowdstrike.alert.source.account_sam_account_name",
                )?;
            }

            if event.has_value("crowdstrike.alert.source_account_upn") {
                event.rename(
                    "crowdstrike.alert.source_account_upn",
                    "crowdstrike.alert.source.account_upn",
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.source.account_upn") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.source.account_upn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.source_endpoint_account_object_guid") {
                event.rename(
                    "crowdstrike.alert.source_endpoint_account_object_guid",
                    "crowdstrike.alert.source.endpoint_account_object_guid",
                )?;
            }

            if event.has_value("crowdstrike.alert.source_endpoint_account_object_sid") {
                event.rename(
                    "crowdstrike.alert.source_endpoint_account_object_sid",
                    "crowdstrike.alert.source.endpoint_account_object_sid",
                )?;
            }

            let _cond =
                { event.get_str("crowdstrike.alert.source_endpoint_address_ip4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.source_endpoint_address_ip4") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.source_endpoint_address_ip4")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.source_endpoint_address_ip4".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("crowdstrike.alert.source.endpoint_address_ip4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_endpoint_address_ip4_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.source.endpoint_address_ip4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("crowdstrike.alert.source.endpoint_address_ip4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.source_endpoint_host_name") {
                event.rename(
                    "crowdstrike.alert.source_endpoint_host_name",
                    "crowdstrike.alert.source.endpoint_host_name",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.source.endpoint_host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("crowdstrike.alert.source_endpoint_ip_address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.source_endpoint_ip_address") {
                        if let Some(val) = event.get("crowdstrike.alert.source_endpoint_ip_address")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.source_endpoint_ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.alert.source.endpoint_ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_endpoint_ip_address_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.source.endpoint_ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("crowdstrike.alert.source_endpoint_ip_reputation") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.source_endpoint_ip_reputation") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.source_endpoint_ip_reputation")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.source_endpoint_ip_reputation".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.alert.source.endpoint_ip_reputation",
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
                        "convert_source_endpoint_ip_reputation_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.source_endpoint_sensor_id") {
                event.rename(
                    "crowdstrike.alert.source_endpoint_sensor_id",
                    "crowdstrike.alert.source.endpoint_sensor_id",
                )?;
            }

            let _cond =
                { event.get_str("crowdstrike.alert.source_ip_isp_classification") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.alert.source_ip_isp_classification") {
                        if let Some(val) =
                            event.get("crowdstrike.alert.source_ip_isp_classification")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.alert.source_ip_isp_classification".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("crowdstrike.alert.source.ip_isp_classification", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_ip_isp_classification_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("crowdstrike.alert.source_ip_isp_domain") {
                event.rename(
                    "crowdstrike.alert.source_ip_isp_domain",
                    "crowdstrike.alert.source.ip_isp_domain",
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.start_time")
                    && event.get_str("crowdstrike.alert.start_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("crowdstrike.alert.start_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("crowdstrike.alert.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_start_time")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def start = ZonedDateTime.parse(ctx.event.start);\ndef end = ZonedDateTime.parse(ctx.event.end);\ndef duration = ChronoUnit.NANOS.between(start, end);\nif (duration >= 0) {\n  ctx.event.duration = duration;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def start = ZonedDateTime.parse(ctx.event.start);\ndef end = ZonedDateTime.parse(ctx.event.end);\ndef duration = ChronoUnit.NANOS.between(start, end);\nif (duration >= 0) {\n  ctx.event.duration = duration;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_duration_from_start_and_end",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("crowdstrike.alert.tactic") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("crowdstrike.alert.tactic")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.tactic_id") };
            if _cond {
                event.append_unique(
                    "threat.tactic.id",
                    json!(
                        event
                            .get("crowdstrike.alert.tactic_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.tags", |event| {
                    event.append_unique(
                        "tags",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("crowdstrike.alert.target_endpoint_host_name") {
                event.rename(
                    "crowdstrike.alert.target_endpoint_host_name",
                    "crowdstrike.alert.target.endpoint_host_name",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.target.endpoint_host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.target_domain_controller_host_name") {
                event.rename(
                    "crowdstrike.alert.target_domain_controller_host_name",
                    "crowdstrike.alert.target.domain_controller_host_name",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.target.domain_controller_host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.domain", v)?;
            }

            let _cond = { event.has_value("destination.user.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.target_account_name") {
                event.rename(
                    "crowdstrike.alert.target_account_name",
                    "crowdstrike.alert.target.account_name",
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.target.account_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("crowdstrike.alert.target_domain_controller_object_guid") {
                event.rename(
                    "crowdstrike.alert.target_domain_controller_object_guid",
                    "crowdstrike.alert.target.domain_controller_object_guid",
                )?;
            }

            if event.has_value("crowdstrike.alert.target_domain_controller_object_sid") {
                event.rename(
                    "crowdstrike.alert.target_domain_controller_object_sid",
                    "crowdstrike.alert.target.domain_controller_object_sid",
                )?;
            }

            if event.has_value("crowdstrike.alert.target_endpoint_account_object_guid") {
                event.rename(
                    "crowdstrike.alert.target_endpoint_account_object_guid",
                    "crowdstrike.alert.target.endpoint_account_object_guid",
                )?;
            }

            if event.has_value("crowdstrike.alert.target_endpoint_account_object_sid") {
                event.rename(
                    "crowdstrike.alert.target_endpoint_account_object_sid",
                    "crowdstrike.alert.target.endpoint_account_object_sid",
                )?;
            }

            if event.has_value("crowdstrike.alert.target_endpoint_sensor_id") {
                event.rename(
                    "crowdstrike.alert.target_endpoint_sensor_id",
                    "crowdstrike.alert.target.endpoint_sensor_id",
                )?;
            }

            if event.has_value("crowdstrike.alert.target_service_access_identifier") {
                event.rename(
                    "crowdstrike.alert.target_service_access_identifier",
                    "crowdstrike.alert.target.service_access_identifier",
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.technique") };
            if _cond {
                event.append_unique(
                    "threat.technique.name",
                    json!(
                        event
                            .get("crowdstrike.alert.technique")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.technique_id") };
            if _cond {
                event.append_unique(
                    "threat.technique.id",
                    json!(
                        event
                            .get("crowdstrike.alert.technique_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.mitre_attack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.mitre_attack", |event| {
                    event.append_unique(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("_ingest._value.tactic")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.mitre_attack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.mitre_attack", |event| {
                    event.append_unique(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("_ingest._value.tactic_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.mitre_attack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.mitre_attack", |event| {
                    event.append_unique(
                        "threat.technique.name",
                        json!(
                            event
                                .get("_ingest._value.technique")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.mitre_attack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.alert.mitre_attack", |event| {
                    event.append_unique(
                        "threat.technique.id",
                        json!(
                            event
                                .get("_ingest._value.technique_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.alert.mitre_attack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.alert.mitre_attack").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.pattern_id") {
                                if let Some(val) = event.get("_ingest._value.pattern_id") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.pattern_id".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.pattern_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_mitre_attack_pattern_id_to_string",
                            )?;
                            if event.remove("_ingest._value.pattern_id").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.pattern_id".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("crowdstrike.alert.mitre_attack", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("threat") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def tid = ctx.threat.tactic?.id;\ndef nid = ctx.threat.technique?.id;\ndef tname = ctx.threat.tactic?.name;\nif ((tid == null || tid.isEmpty()) && (nid == null || nid.isEmpty()) && (tname == null || tname.isEmpty())) {\n  return;\n}\nSet frameworks = new HashSet();\n// Handling tactics prefixed with \"CS\" or \"TA\".\nif (tid != null && !tid.isEmpty()) {\n  for (String t: tid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n    else if (t.startsWith(\"TA\")) {\n      frameworks.add(params.framework_ma);\n    }\n  }\n}\n// Handling techniques prefixed with \"CS\".\nif (nid != null && !nid.isEmpty()) {\n  for (String t: nid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n// Handling falcon specific tactics.\nif (tname != null && !tname.isEmpty()) {\n  for (String t: tname) {\n    if (params.falcon_tactic_names.contains(t.toLowerCase())) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\nif (frameworks.isEmpty()) {\n  return;\n}\nif (frameworks.size() == 1) {\n  ctx.threat.framework = frameworks.iterator().next();\n  return;\n}\n\nfor (def preferred : params.framework_preference) {\n  if (frameworks.contains(preferred)) {\n    ctx.threat.framework = preferred;\n    return;\n  }\n}\n\n// fallback when new frameworks are added and not yet in preference list\nctx.threat.framework = frameworks.iterator().next();\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def tid = ctx.threat.tactic?.id;\ndef nid = ctx.threat.technique?.id;\ndef tname = ctx.threat.tactic?.name;\nif ((tid == null || tid.isEmpty()) && (nid == null || nid.isEmpty()) && (tname == null || tname.isEmpty())) {\n  return;\n}\nSet frameworks = new HashSet();\n// Handling tactics prefixed with \"CS\" or \"TA\".\nif (tid != null && !tid.isEmpty()) {\n  for (String t: tid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n    else if (t.startsWith(\"TA\")) {\n      frameworks.add(params.framework_ma);\n    }\n  }\n}\n// Handling techniques prefixed with \"CS\".\nif (nid != null && !nid.isEmpty()) {\n  for (String t: nid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n// Handling falcon specific tactics.\nif (tname != null && !tname.isEmpty()) {\n  for (String t: tname) {\n    if (params.falcon_tactic_names.contains(t.toLowerCase())) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\nif (frameworks.isEmpty()) {\n  return;\n}\nif (frameworks.size() == 1) {\n  ctx.threat.framework = frameworks.iterator().next();\n  return;\n}\n\nfor (def preferred : params.framework_preference) {\n  if (frameworks.contains(preferred)) {\n    ctx.threat.framework = preferred;\n    return;\n  }\n}\n\n// fallback when new frameworks are added and not yet in preference list\nctx.threat.framework = frameworks.iterator().next();\n"#
                        ),
                        cached_params!(
                            "{\"framework_preference\":[\"MITRE ATT&CK\",\"CrowdStrike Falcon Detections Framework\"],\"framework_cs\":\"CrowdStrike Falcon Detections Framework\",\"framework_ma\":\"MITRE ATT&CK\",\"falcon_tactic_names\":[\"malware\",\"exploit\",\"post-exploit\",\"machine learning\",\"custom intelligence\",\"falcon overwatch\",\"falcon intel\",\"ai powered ioa\",\"insecure security posture\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_threat_framework",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("crowdstrike.alert.timestamp")
                    && event.get_str("crowdstrike.alert.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("crowdstrike.alert.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("crowdstrike.alert.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("crowdstrike.alert.overwatch_note_timestamp")
                    && event.get_str("crowdstrike.alert.updated_timestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.overwatch_note_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.overwatch_note_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.overwatch_note_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("crowdstrike.alert.updated_timestamp")
                    && event.get_str("crowdstrike.alert.updated_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.alert.updated_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.alert.updated_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.alert.updated_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_updated_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("crowdstrike.alert.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.alert.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.alert.user_principal") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.alert.user_principal")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("crowdstrike.alert.name") {
                    if let Some(input) = event.get_string("crowdstrike.alert.name") {
                        // Grok pattern: %{NOTSPACE:_username_from_name} on %{NOTSPACE:_hostname_from_name}
                        let _ = cached_grok!(
                            "%{NOTSPACE:_username_from_name} on %{NOTSPACE:_hostname_from_name}"
                        )
                        .extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("user.name") || event.get_str("user.name") == Some("") };
            if _cond {
                if let Some(v) = event
                    .get("_username_from_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.has_value("_username_from_name")
                    && event.get_str("_username_from_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("_username_from_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.name") || event.get_str("host.name") == Some("") };
            if _cond {
                if let Some(v) = event
                    .get("_hostname_from_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event.has_value("_hostname_from_name")
                    && event.get_str("_hostname_from_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("_hostname_from_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("_username_from_name");
            event.remove("_hostname_from_name");

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("crowdstrike.alert.cid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.alert.indicator_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.alert.updated_timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("event.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.remove("crowdstrike.alert.attack_types");
            event.remove("crowdstrike.alert.aws_access_key_id");
            event.remove("crowdstrike.alert.aws_recipient_account_id");
            event.remove("crowdstrike.alert.aws_user_account_id");
            event.remove("crowdstrike.alert.cloud_account_id");
            event.remove("crowdstrike.alert.cloud_provider");
            event.remove("crowdstrike.alert.cloud_region");
            event.remove("crowdstrike.alert.device.instance_id");
            event.remove("crowdstrike.alert.device.service_provider");
            event.remove("crowdstrike.alert.device.service_provider_account_id");
            event.remove("crowdstrike.alert.event_category");
            event.remove("crowdstrike.alert.event_count");
            event.remove("crowdstrike.alert.event_name");
            event.remove("crowdstrike.alert.event_source");
            event.remove("crowdstrike.alert.event_type");
            event.remove("crowdstrike.alert.fcs_vertex_id");
            event.remove("crowdstrike.alert.first_timestamp");
            event.remove("crowdstrike.alert.idp_policy_mfa_factor_type");
            event.remove("crowdstrike.alert.idp_policy_mfa_provider");
            event.remove("crowdstrike.alert.last_timestamp");
            event.remove("crowdstrike.alert.mfa_authenticated");
            event.remove("crowdstrike.alert.origin_cid");
            event.remove("crowdstrike.alert.policy_id");
            event.remove("crowdstrike.alert.policy_statement");
            event.remove("crowdstrike.alert.priority_explanation");
            event.remove("crowdstrike.alert.priority_value");
            event.remove("crowdstrike.alert.request_parameters");
            event.remove("crowdstrike.alert.resource_gcrn");
            event.remove("crowdstrike.alert.resource_uuid");
            event.remove("crowdstrike.alert.response_elements");
            event.remove("crowdstrike.alert.service");
            event.remove("crowdstrike.alert.source_endpoint_address_ip4");
            event.remove("crowdstrike.alert.source_endpoint_ip_address");
            event.remove("crowdstrike.alert.source_endpoint_ip_reputation");
            event.remove("crowdstrike.alert.source_ip_address");
            event.remove("crowdstrike.alert.source_ip_isp_classification");
            event.remove("crowdstrike.alert.user_agent");
            event.remove("crowdstrike.alert.user_display_name");
            event.remove("crowdstrike.alert.user_principal_id");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("crowdstrike.alert.agent_id");
                event.remove("crowdstrike.alert.command_line");
                event.remove("crowdstrike.alert.description");
                event.remove("crowdstrike.alert.device.device_id");
                event.remove("crowdstrike.alert.device.external_ip");
                event.remove("crowdstrike.alert.device.hostinfo.domain");
                event.remove("crowdstrike.alert.device.hostname");
                event.remove("crowdstrike.alert.device.local_ip");
                event.remove("crowdstrike.alert.device.mac_address");
                event.remove("crowdstrike.alert.device.os_version");
                event.remove("crowdstrike.alert.device.platform_name");
                event.remove("crowdstrike.alert.device.system_manufacturer");
                event.remove("crowdstrike.alert.device.system_product_name");
                event.remove("crowdstrike.alert.device.tags");
                event.remove("crowdstrike.alert.end_time");
                event.remove("crowdstrike.alert.filename");
                event.remove("crowdstrike.alert.filepath");
                event.remove("crowdstrike.alert.host_name");
                event.remove("crowdstrike.alert.host_type");
                event.remove("crowdstrike.alert.id");
                event.remove("crowdstrike.alert.idp_policy_rule.id");
                event.remove("crowdstrike.alert.idp_policy_rule.name");
                event.remove("crowdstrike.alert.image_file_name");
                event.remove("crowdstrike.alert.location_country_code");
                event.remove("crowdstrike.alert.md5");
                event.remove("crowdstrike.alert.operating_system");
                event.remove("crowdstrike.alert.os_name");
                event.remove("crowdstrike.alert.parent_details.cmdline");
                event.remove("crowdstrike.alert.parent_details.filename");
                event.remove("crowdstrike.alert.parent_details.filepath");
                event.remove("crowdstrike.alert.parent_details.md5");
                event.remove("crowdstrike.alert.parent_details.sha256");
                event.remove("crowdstrike.alert.parent_details.user_id");
                event.remove("crowdstrike.alert.parent_details.user_name");
                event.remove("crowdstrike.alert.parent_process_id");
                event.remove("crowdstrike.alert.process_end_time");
                event.remove("crowdstrike.alert.process_id");
                event.remove("crowdstrike.alert.process_start_time");
                event.remove("crowdstrike.alert.severity");
                event.remove("crowdstrike.alert.sha1");
                event.remove("crowdstrike.alert.sha256");
                event.remove("crowdstrike.alert.source.account_azure_id");
                event.remove("crowdstrike.alert.source.account_domain");
                event.remove("crowdstrike.alert.source.account_name");
                event.remove("crowdstrike.alert.source.endpoint_host_name");
                event.remove("crowdstrike.alert.source.endpoint_ip_address");
                event.remove("crowdstrike.alert.start_time");
                event.remove("crowdstrike.alert.tactic");
                event.remove("crowdstrike.alert.tags");
                event.remove("crowdstrike.alert.target.account_name");
                event.remove("crowdstrike.alert.target.domain_controller_host_name");
                event.remove("crowdstrike.alert.target.endpoint_host_name");
                event.remove("crowdstrike.alert.technique");
                event.remove("crowdstrike.alert.timestamp");
                event.remove("crowdstrike.alert.user_id");
                event.remove("crowdstrike.alert.user_name");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
