// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_service_application` pipeline.
pub struct PipelineServiceApplication;

impl Transform for PipelineServiceApplication {
    fn name(&self) -> &str {
        "pipeline_service_application"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.Configuration.Message.UserReason").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.reason", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.Event.Action").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.action", v)?;
        }

        if event.has_value("event.action") {
            map_strings(event, "event.action", "event.action", str::to_lowercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.Event.Type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.ServiceControl.Service.DisplayName").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.error.code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("error.code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.error.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("error.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.error.stack_trace").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("error.stack_trace", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.error.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("error.type", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.ReceivedAt") && event.get_str("beyondtrust_epm.event.event.ReceivedAt") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.event.ReceivedAt") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.event.ReceivedAt", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.event.ReceivedAt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_event_ReceivedAt")?;
                    event.remove("beyondtrust_epm.event.event.ReceivedAt");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.action") };
        if _cond {
            event.append_unique("event.action", json!(event.get("beyondtrust_epm.event.event.action").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.agent_id_status").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.agent_id_status", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.code") };
        if _cond {
            event.append_unique("event.code", json!(event.get("beyondtrust_epm.event.event.code").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.created") && event.get_str("beyondtrust_epm.event.event.created") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.event.created") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.event.created", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.event.created".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_event_created")?;
                    event.remove("beyondtrust_epm.event.event.created");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.created").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.created", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.event.duration") {
            if let Some(val) = event.get("beyondtrust_epm.event.event.duration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.event.duration".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.event.duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_event_duration_to_long")?;
                    event.remove("beyondtrust_epm.event.event.duration");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.duration").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.duration", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.end") && event.get_str("beyondtrust_epm.event.event.end") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.event.end") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.event.end", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.event.end".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_event_end")?;
                    event.remove("beyondtrust_epm.event.event.end");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.end").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.end", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.hash").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.event.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.id") };
        if _cond {
        if let Some(v) = event.get("beyondtrust_epm.event.event.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.id", v)?;
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.RequestIdentifier").filter(|v| !painless_is_empty_value(v)).cloned() {
            if !event.has("event.id") {
                event.set("event.id", v)?;
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.ingested") && event.get_str("beyondtrust_epm.event.event.ingested") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.event.ingested") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.event.ingested", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.event.ingested".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_event_ingested")?;
                    event.remove("beyondtrust_epm.event.event.ingested");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.outcome").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.outcome", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.provider").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.provider", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.reason") };
        if _cond {
            event.append_unique("event.reason", json!(event.get("beyondtrust_epm.event.event.reason").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.reference").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.reference", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.event.risk_score") {
            if let Some(val) = event.get("beyondtrust_epm.event.event.risk_score") {
                let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.event.risk_score".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.event.risk_score", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_event_risk_score_to_float")?;
                    event.remove("beyondtrust_epm.event.event.risk_score");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.risk_score").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.risk_score", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.event.risk_score_norm") {
            if let Some(val) = event.get("beyondtrust_epm.event.event.risk_score_norm") {
                let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.event.risk_score_norm".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.event.risk_score_norm", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_event_risk_score_norm_to_float")?;
                    event.remove("beyondtrust_epm.event.event.risk_score_norm");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.risk_score_norm").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.risk_score_norm", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.event.sequence") {
            if let Some(val) = event.get("beyondtrust_epm.event.event.sequence") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.event.sequence".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.event.sequence", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_event_sequence_to_long")?;
                    event.remove("beyondtrust_epm.event.event.sequence");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.sequence").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.sequence", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.event.severity") {
            if let Some(val) = event.get("beyondtrust_epm.event.event.severity") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.event.severity".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.event.severity", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_event_severity_to_long")?;
                    event.remove("beyondtrust_epm.event.event.severity");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.severity", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.event.start") && event.get_str("beyondtrust_epm.event.event.start") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.event.start") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.event.start", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.event.start".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_event_start")?;
                    event.remove("beyondtrust_epm.event.event.start");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.start").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.start", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.timezone", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.url").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.url", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.log.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("log.file.path", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.log.level").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("log.level", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.log.logger").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("log.logger", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.log.origin.file.line") {
            if let Some(val) = event.get("beyondtrust_epm.event.log.origin.file.line") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.log.origin.file.line".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.log.origin.file.line", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_log_origin_file_line_to_long")?;
                    event.remove("beyondtrust_epm.event.log.origin.file.line");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.log.origin.file.line").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("log.origin.file.line", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.log.origin.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("log.origin.file.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.log.origin.function").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("log.origin.function", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.address", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.environment").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.environment", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.ephemeral_id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.ephemeral_id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.service.name") };
        if _cond {
            event.append_unique("service.name", json!(event.get("beyondtrust_epm.event.service.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.node.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.node.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.node.role").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.node.role", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.address", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.environment").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.environment", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.ephemeral_id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.ephemeral_id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.node.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.node.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.node.role").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.node.role", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.state").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.state", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.type", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.origin.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.origin.version", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.state").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.state", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.address", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.environment").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.environment", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.ephemeral_id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.ephemeral_id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.node.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.node.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.node.role").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.node.role", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.state").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.state", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.type", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.target.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.target.version", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.type", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.service.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.version", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.ReceivedAt").filter(|v| !painless_is_empty_value(v)).cloned() {
            if !event.has("@timestamp") {
                event.set("@timestamp", v)?;
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.event.original").filter(|v| !painless_is_empty_value(v)).cloned() {
            if !event.has("event.original") {
                event.set("event.original", v)?;
            }
        }

        let _cond = { event.get("beyondtrust_epm.event.event.category").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.event.category", |event| {
                event.append_unique("event.category", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.event.category").is_some_and(|v| v.is_string()) };
        if _cond {
            event.append_unique("event.category", json!(event.get("beyondtrust_epm.event.event.category").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.Message.UserReason");
            event.remove("beyondtrust_epm.event.EPMWinMac.Event.Action");
            event.remove("beyondtrust_epm.event.EPMWinMac.Event.Type");
            event.remove("beyondtrust_epm.event.EPMWinMac.ServiceControl.Service.DisplayName");
            event.remove("beyondtrust_epm.event.error.code");
            event.remove("beyondtrust_epm.event.error.id");
            event.remove("beyondtrust_epm.event.error.stack_trace");
            event.remove("beyondtrust_epm.event.error.type");
            event.remove("beyondtrust_epm.event.event.action");
            event.remove("beyondtrust_epm.event.event.agent_id_status");
            event.remove("beyondtrust_epm.event.event.code");
            event.remove("beyondtrust_epm.event.event.created");
            event.remove("beyondtrust_epm.event.event.duration");
            event.remove("beyondtrust_epm.event.event.end");
            event.remove("beyondtrust_epm.event.event.hash");
            event.remove("beyondtrust_epm.event.event.id");
            event.remove("beyondtrust_epm.event.event.outcome");
            event.remove("beyondtrust_epm.event.event.provider");
            event.remove("beyondtrust_epm.event.event.reason");
            event.remove("beyondtrust_epm.event.event.reference");
            event.remove("beyondtrust_epm.event.event.risk_score");
            event.remove("beyondtrust_epm.event.event.risk_score_norm");
            event.remove("beyondtrust_epm.event.event.sequence");
            event.remove("beyondtrust_epm.event.event.severity");
            event.remove("beyondtrust_epm.event.event.start");
            event.remove("beyondtrust_epm.event.event.timezone");
            event.remove("beyondtrust_epm.event.event.url");
            event.remove("beyondtrust_epm.event.log.file.path");
            event.remove("beyondtrust_epm.event.log.level");
            event.remove("beyondtrust_epm.event.log.logger");
            event.remove("beyondtrust_epm.event.log.origin.file.line");
            event.remove("beyondtrust_epm.event.log.origin.file.name");
            event.remove("beyondtrust_epm.event.log.origin.function");
            event.remove("beyondtrust_epm.event.service.address");
            event.remove("beyondtrust_epm.event.service.environment");
            event.remove("beyondtrust_epm.event.service.ephemeral_id");
            event.remove("beyondtrust_epm.event.service.id");
            event.remove("beyondtrust_epm.event.service.name");
            event.remove("beyondtrust_epm.event.service.node.name");
            event.remove("beyondtrust_epm.event.service.node.role");
            event.remove("beyondtrust_epm.event.service.origin.address");
            event.remove("beyondtrust_epm.event.service.origin.environment");
            event.remove("beyondtrust_epm.event.service.origin.ephemeral_id");
            event.remove("beyondtrust_epm.event.service.origin.id");
            event.remove("beyondtrust_epm.event.service.origin.name");
            event.remove("beyondtrust_epm.event.service.origin.node.name");
            event.remove("beyondtrust_epm.event.service.origin.node.role");
            event.remove("beyondtrust_epm.event.service.origin.state");
            event.remove("beyondtrust_epm.event.service.origin.type");
            event.remove("beyondtrust_epm.event.service.origin.version");
            event.remove("beyondtrust_epm.event.service.state");
            event.remove("beyondtrust_epm.event.service.target.address");
            event.remove("beyondtrust_epm.event.service.target.environment");
            event.remove("beyondtrust_epm.event.service.target.ephemeral_id");
            event.remove("beyondtrust_epm.event.service.target.id");
            event.remove("beyondtrust_epm.event.service.target.name");
            event.remove("beyondtrust_epm.event.service.target.node.name");
            event.remove("beyondtrust_epm.event.service.target.node.role");
            event.remove("beyondtrust_epm.event.service.target.state");
            event.remove("beyondtrust_epm.event.service.target.type");
            event.remove("beyondtrust_epm.event.service.target.version");
            event.remove("beyondtrust_epm.event.service.type");
            event.remove("beyondtrust_epm.event.service.version");

        Ok(TransformResult::Continue)
    }
}
