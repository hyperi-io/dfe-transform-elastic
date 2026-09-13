// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_audit_class_verification_event` pipeline.
pub struct PipelineAuditClassVerificationEvent;

impl Transform for PipelineAuditClassVerificationEvent {
    fn name(&self) -> &str {
        "pipeline_audit_class_verification_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.audit_class_verification_info.contents") {
                    event.rename("json.audit_class_verification_info.contents", "jamf_compliance_reporter.log.audit_class_verification_info.contents")?;
                }

                if event.has_value("json.audit_class_verification_info.osversion") {
                    event.rename("json.audit_class_verification_info.osversion", "jamf_compliance_reporter.log.audit_class_verification_info.os.version")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.audit_class_verification_info.restored_default") {
                if let Some(val) = event.get("json.audit_class_verification_info.restored_default") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.audit_class_verification_info.restored_default".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.audit_class_verification_info.restored_default", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.audit_class_verification_info.status") {
                if let Some(val) = event.get("json.audit_class_verification_info.status") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.audit_class_verification_info.status".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.audit_class_verification_info.status", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.audit_class_verification_info.status_str") {
                    event.rename("json.audit_class_verification_info.status_str", "jamf_compliance_reporter.log.audit_class_verification_info.status_str")?;
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
