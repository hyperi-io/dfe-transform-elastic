// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `automated_lead_summary` pipeline.
pub struct AutomatedLeadSummary;

impl Transform for AutomatedLeadSummary {
    fn name(&self) -> &str {
        "automated_lead_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            if let Some(v) = event
                .get("crowdstrike.event.Name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
                if let Some(v) = event
                    .get("crowdstrike.event.Description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.SignalStartTimestamp")
                    && event.get_i64("crowdstrike.event.SignalStartTimestamp") != Some(0)
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.SignalStartTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                        {
                            event.set("crowdstrike.event.SignalStartTimestamp", parsed)?;
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
                    event.remove("crowdstrike.event.SignalStartTimestamp");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("crowdstrike.event.SignalStartTimestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.SignalEndTimestamp")
                    && event.get_i64("crowdstrike.event.SignalEndTimestamp") != Some(0)
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.SignalEndTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                        {
                            event.set("crowdstrike.event.SignalEndTimestamp", parsed)?;
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
                    event.remove("crowdstrike.event.SignalEndTimestamp");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("crowdstrike.event.SignalEndTimestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.SignalUpdatedTimestamp")
                    && event.get_i64("crowdstrike.event.SignalUpdatedTimestamp") != Some(0)
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.SignalUpdatedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                        {
                            event.set("crowdstrike.event.SignalUpdatedTimestamp", parsed)?;
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
                    event.remove("crowdstrike.event.SignalUpdatedTimestamp");
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
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.SignalAssociationTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                            {
                                event.set("_ingest._value.SignalAssociationTimestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_threatgraph_indicators_signal_association_timestamp",
                        )?;
                        event.remove("_ingest._value.SignalAssociationTimestamp");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("crowdstrike.event.Score") {
                    if let Some(val) = event.get("crowdstrike.event.Score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.Score".into(),
                                message,
                            }
                        })?;
                        event.set("crowdstrike.event.Score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_to_long")?;
                event.remove("crowdstrike.event.Score");
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
                    .get("crowdstrike.event.Score")
                    .is_some_and(|v| v.is_number())
            };
            if _cond {
                // Painless script
                // Source: long score = ctx.crowdstrike.event.Score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"long score = ctx.crowdstrike.event.Score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    if event.has_value("_ingest._value.PatternId") {
                        if let Some(val) = event.get("_ingest._value.PatternId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.PatternId".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.PatternId", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    if event.has_value("_ingest._value.TemplateInstanceId") {
                        if let Some(val) = event.get("_ingest._value.TemplateInstanceId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.TemplateInstanceId".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.TemplateInstanceId", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.Severity") {
                            if let Some(val) = event.get("_ingest._value.Severity") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.Severity".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.Severity", converted)?;
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
                        event.remove("_ingest._value.Severity");
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
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.PatternDisposition") {
                            if let Some(val) = event.get("_ingest._value.PatternDisposition") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.PatternDisposition".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.PatternDisposition", converted)?;
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
                        event.remove("_ingest._value.PatternDisposition");
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

            if event.has("crowdstrike.event.CompositeId") {
                event.rename("crowdstrike.event.CompositeId", "event.id")?;
            }

            if event.has("crowdstrike.event.FalconHostLink") {
                event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    event.append_unique(
                        "threat.indicator.id",
                        json!(
                            event
                                .get("_ingest._value.IndicatorId")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    event.append_unique(
                        "threat.indicator.name",
                        json!(
                            event
                                .get("_ingest._value.DisplayName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    event.append_unique(
                        "threat.indicator.description",
                        json!(
                            event
                                .get("_ingest._value.Description")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
                    && event
                        .get("crowdstrike.event.ThreatgraphIndicators")
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
                // Source: def indicator = ctx.crowdstrike.event.ThreatgraphIndicators[0];\nif (indicator.HostId != null && indicator.HostId != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.HostId;\n}\nif (indicator.Hostname != null && indicator.Hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.Hostname;\n}\nif (indicator.ProcessId != null && indicator.ProcessId != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.ProcessId;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def indicator = ctx.crowdstrike.event.ThreatgraphIndicators[0];\nif (indicator.HostId != null && indicator.HostId != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.HostId;\n}\nif (indicator.Hostname != null && indicator.Hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.Hostname;\n}\nif (indicator.ProcessId != null && indicator.ProcessId != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.ProcessId;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event.ThreatgraphIndicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "crowdstrike.event.ThreatgraphIndicators", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.Hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
