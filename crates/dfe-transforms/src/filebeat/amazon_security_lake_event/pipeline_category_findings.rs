// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_category_findings` pipeline.
pub struct PipelineCategoryFindings;

impl Transform for PipelineCategoryFindings {
    fn name(&self) -> &str {
        "pipeline_category_findings"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("ocsf.finding.src_url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            let _cond = { event.has_value("ocsf.finding.src_url") };
            if _cond {
                uri_parts(event, "ocsf.finding.src_url", "url", true, false)?;
            }

            if event.has_value("ocsf.analytic.type_id") {
                if let Some(val) = event.get("ocsf.analytic.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.analytic.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.analytic.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.confidence_id") {
                if let Some(val) = event.get("ocsf.confidence_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.confidence_id".into(),
                            message,
                        })?;
                    event.set("ocsf.confidence_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.confidence_score") {
                if let Some(val) = event.get("ocsf.confidence_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.confidence_score".into(),
                            message,
                        })?;
                    event.set("ocsf.confidence_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_confidence_score_to_long")?;
                        event.remove("ocsf.confidence_score");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.finding.created_time_dt") && event.get_str("ocsf.finding.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_created_time_dt")?;
                        event.remove("ocsf.finding.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.created_time") && event.get_str("ocsf.finding.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_created_time")?;
                        event.remove("ocsf.finding.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.created_time_dt") && event.get_str("ocsf.finding.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_created_time_dt")?;
                        event.remove("ocsf.finding.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.created_time") && event.get_str("ocsf.finding.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_created_time")?;
                        event.remove("ocsf.finding.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.first_seen_time_dt") && event.get_str("ocsf.finding.first_seen_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.first_seen_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.first_seen_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.first_seen_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_first_seen_time_dt")?;
                        event.remove("ocsf.finding.first_seen_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.first_seen_time") && event.get_str("ocsf.finding.first_seen_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.first_seen_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.first_seen_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.first_seen_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_first_seen_time")?;
                        event.remove("ocsf.finding.first_seen_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.last_seen_time_dt") && event.get_str("ocsf.finding.last_seen_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.last_seen_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.last_seen_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.last_seen_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_last_seen_time_dt")?;
                        event.remove("ocsf.finding.last_seen_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.last_seen_time") && event.get_str("ocsf.finding.last_seen_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.last_seen_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.last_seen_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.last_seen_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_last_seen_time")?;
                        event.remove("ocsf.finding.last_seen_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.modified_time_dt") && event.get_str("ocsf.finding.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_modified_time_dt")?;
                        event.remove("ocsf.finding.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.finding.modified_time") && event.get_str("ocsf.finding.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.finding.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.finding.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.finding.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_finding_modified_time")?;
                        event.remove("ocsf.finding.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.finding.related_events").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.finding.related_events", |event| {
                    if event.has_value("_ingest._value.type_uid") {
                    if let Some(val) = event.get("_ingest._value.type_uid") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.type_uid".into(),
                    message,
                    })?;
                    event.set("_ingest._value.type_uid", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.impact_score") {
                if let Some(val) = event.get("ocsf.impact_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.impact_score".into(),
                            message,
                        })?;
                    event.set("ocsf.impact_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_impact_score_to_long")?;
                        event.remove("ocsf.impact_score");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.state_id") {
                if let Some(val) = event.get("ocsf.state_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.state_id".into(),
                            message,
                        })?;
                    event.set("ocsf.state_id", converted)?;
                }
            }

            if event.has_value("ocsf.impact_id") {
                if let Some(val) = event.get("ocsf.impact_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.impact_id".into(),
                            message,
                        })?;
                    event.set("ocsf.impact_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.kill_chain").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.kill_chain", |event| {
                    if event.has_value("_ingest._value.phase_id") {
                    if let Some(val) = event.get("_ingest._value.phase_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.phase_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.phase_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.risk_level_id") {
                if let Some(val) = event.get("ocsf.risk_level_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.risk_level_id".into(),
                            message,
                        })?;
                    event.set("ocsf.risk_level_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.risk_score") {
                if let Some(val) = event.get("ocsf.risk_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.risk_score".into(),
                            message,
                        })?;
                    event.set("ocsf.risk_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_risk_score_to_long")?;
                        event.remove("ocsf.risk_score");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.risk_score").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.risk_score", v)?;
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    event.append_unique("vulnerability.id", json!(event.get("_ingest._value.cve.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    event.append_unique("vulnerability.score.version", json!(event.get("_ingest._value.cve.cvss.version").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    event.append("vulnerability.description", json!(event.get("_ingest._value.desc").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.references", |event| {
                    event.append("vulnerability.reference", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.cve.cvss.base_score") {
                    if let Some(val) = event.get("_ingest._value.cve.cvss.base_score") {
                    let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.cve.cvss.base_score".into(),
                    message,
                    })?;
                    event.set("_ingest._value.cve.cvss.base_score", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_cve_cvss_base_score_to_double")?;
                    event.remove("_ingest._value.cve.cvss.base_score");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    event.append_unique("vulnerability.score.base", json!(event.get("_ingest._value.cve.cvss.base_score").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    event.append_unique("vulnerability.severity", json!(event.get("_ingest._value.severity").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    event.append_unique("vulnerability.scanner.vendor", json!(event.get("_ingest._value.vendor_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.cve.cvss.overall_score") {
                    if let Some(val) = event.get("_ingest._value.cve.cvss.overall_score") {
                    let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.cve.cvss.overall_score".into(),
                    message,
                    })?;
                    event.set("_ingest._value.cve.cvss.overall_score", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_cve_cvss_overall_score_to_double")?;
                    event.remove("_ingest._value.cve.cvss.overall_score");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.cve.cvss.severity") {
                    if let Some(val) = event.get("_ingest._value.cve.cvss.severity") {
                    let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.cve.cvss.severity".into(),
                    message,
                    })?;
                    event.set("_ingest._value.cve.cvss.severity", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_cve_cvss_severity_to_double")?;
                    event.remove("_ingest._value.cve.cvss.severity");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ocsf.vulnerabilities").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.cve.created_time_dt") {
                            match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                            Some(parsed) => event.set("_ingest.cve.created_time_dt", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.cve.created_time_dt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_vulnerabilities_cve_created_time_dt")?;
                            event.remove("_ingest._value.cve.created_time_dt");
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
                        event.set("ocsf.vulnerabilities", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ocsf.vulnerabilities").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.cve.created_time") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("_ingest.cve.created_time", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.cve.created_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_vulnerabilities_cve_created_time")?;
                            event.remove("_ingest._value.cve.created_time");
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
                        event.set("ocsf.vulnerabilities", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ocsf.vulnerabilities").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.cve.modified_time_dt") {
                            match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                            Some(parsed) => event.set("_ingest.cve.modified_time_dt", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.cve.modified_time_dt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_vulnerabilities_cve_modified_time_dt")?;
                            event.remove("_ingest._value.cve.modified_time_dt");
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
                        event.set("ocsf.vulnerabilities", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ocsf.vulnerabilities").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.cve.modified_time") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("_ingest.cve.modified_time", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.cve.modified_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_vulnerabilities_cve_modified_time")?;
                            event.remove("_ingest._value.cve.modified_time");
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
                        event.set("ocsf.vulnerabilities", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.fix_available") {
                    if let Some(val) = event.get("_ingest._value.fix_available") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.fix_available".into(),
                    message,
                    })?;
                    event.set("_ingest._value.fix_available", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_fix_available_to_boolean")?;
                    event.remove("_ingest._value.fix_available");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.vulnerabilities").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.vulnerabilities", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.packages", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.epoch") {
                    if let Some(val) = event.get("_ingest._value.epoch") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.epoch".into(),
                    message,
                    })?;
                    event.set("_ingest._value.epoch", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_packages_epoch_to_long")?;
                    event.remove("_ingest._value.epoch");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.owner.email_addr").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.owner.full_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.owner.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.owner.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    if event.has_value("_ingest._value.owner.account.type_id") {
                    if let Some(val) = event.get("_ingest._value.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.owner.account.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.owner.account.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    if event.has_value("_ingest._value.owner.type_id") {
                    if let Some(val) = event.get("_ingest._value.owner.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.owner.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.owner.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.resources").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.resources", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
