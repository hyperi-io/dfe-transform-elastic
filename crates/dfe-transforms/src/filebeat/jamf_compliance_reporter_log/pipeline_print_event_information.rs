// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_print_event_information` pipeline.
pub struct PipelinePrintEventInformation;

impl Transform for PipelinePrintEventInformation {
    fn name(&self) -> &str {
        "pipeline_print_event_information"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("json.event_attributes.job_completed_time") && event.get_i64("json.event_attributes.job_completed_time") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.job_completed_time") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.completed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_attributes.job_completed_time".into(),
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

            let _cond = { event.has_value("json.event_attributes.job_creation_time") && event.get_i64("json.event_attributes.job_creation_time") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.job_creation_time") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.creation_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_attributes.job_creation_time".into(),
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

                if event.has_value("json.event_attributes.job_destination") {
                    event.rename("json.event_attributes.job_destination", "jamf_compliance_reporter.log.event_attributes.job.destination")?;
                }

                if event.has_value("json.event_attributes.job_format") {
                    event.rename("json.event_attributes.job_format", "jamf_compliance_reporter.log.event_attributes.job.format")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.job_id") {
                if let Some(val) = event.get("json.event_attributes.job_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.job_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.job.id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.event_attributes.job_processing_time") && event.get_i64("json.event_attributes.job_processing_time") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.job_processing_time") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.processing_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_attributes.job_processing_time".into(),
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

                if event.has_value("json.event_attributes.job_size") {
                    event.rename("json.event_attributes.job_size", "jamf_compliance_reporter.log.event_attributes.job.size")?;
                }

                if event.has_value("json.event_attributes.job_state") {
                    event.rename("json.event_attributes.job_state", "jamf_compliance_reporter.log.event_attributes.job.state")?;
                }

                if event.has_value("json.event_attributes.job_title") {
                    event.rename("json.event_attributes.job_title", "jamf_compliance_reporter.log.event_attributes.job.title")?;
                }

                if event.has_value("json.event_attributes.job_user") {
                    event.rename("json.event_attributes.job_user", "jamf_compliance_reporter.log.event_attributes.job.user")?;
                }

            let _cond = { event.has_value("jamf_compliance_reporter.log.event_attributes.job.user") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.event_attributes.job.user").map_or_else(String::new, template_to_string)))?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
