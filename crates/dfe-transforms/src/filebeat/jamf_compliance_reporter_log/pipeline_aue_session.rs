// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_aue_session` pipeline.
pub struct PipelineAueSession;

impl Transform for PipelineAueSession {
    fn name(&self) -> &str {
        "pipeline_aue_session"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.arguments.am_failure") {
                if let Some(val) = event.get("json.arguments.am_failure") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.am_failure".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.am_failure", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.arguments.am_success") {
                if let Some(val) = event.get("json.arguments.am_success") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.am_success".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.am_success", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.arguments.sflags") {
                if let Some(val) = event.get("json.arguments.sflags") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.sflags".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.sflags", converted)?;
                }
            }
                Ok(())
            })();

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
