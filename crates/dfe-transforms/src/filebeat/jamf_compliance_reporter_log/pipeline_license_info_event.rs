// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_license_info_event` pipeline.
pub struct PipelineLicenseInfoEvent;

impl Transform for PipelineLicenseInfoEvent {
    fn name(&self) -> &str {
        "pipeline_license_info_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.ComplianceReporter_license_info.email") {
                    event.rename("json.ComplianceReporter_license_info.email", "user.email")?;
                }

            let _cond = { event.has_value("user.email") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.email").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.ComplianceReporter_license_info.expiration_date") && event.get_i64("json.ComplianceReporter_license_info.expiration_date") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.ComplianceReporter_license_info.expiration_date") {
                    match parse_date_out(&date_str, &["dd/MM/yyyy"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.compliancereporter_license_info.expiration_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.ComplianceReporter_license_info.expiration_date".into(),
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

                if event.has_value("json.ComplianceReporter_license_info.status") {
                    event.rename("json.ComplianceReporter_license_info.status", "jamf_compliance_reporter.log.compliancereporter_license_info.status")?;
                }

            let _cond = { event.has_value("json.ComplianceReporter_license_info.time_seconds_epoch") && event.get_str("json.ComplianceReporter_license_info.time_seconds_epoch") != Some("0") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.ComplianceReporter_license_info.time_seconds_epoch") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.compliancereporter_license_info.time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.ComplianceReporter_license_info.time_seconds_epoch".into(),
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

                if event.has_value("json.ComplianceReporter_license_info.type") {
                    event.rename("json.ComplianceReporter_license_info.type", "jamf_compliance_reporter.log.compliancereporter_license_info.type")?;
                }

                if event.has_value("json.ComplianceReporter_license_info.version") {
                    event.rename("json.ComplianceReporter_license_info.version", "jamf_compliance_reporter.log.compliancereporter_license_info.version")?;
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
