// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `gpunifiedlogevent` pipeline.
pub struct Gpunifiedlogevent;

impl Transform for Gpunifiedlogevent {
    fn name(&self) -> &str {
        "gpunifiedlogevent"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUnifiedLogEvent") && event.has_value("jamf_protect.alerts.input.match.event.process") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.process", "process.name")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUnifiedLogEvent") && event.has_value("jamf_protect.alerts.input.match.event.processIdentifier") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.processIdentifier", "process.pid")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUnifiedLogEvent") && event.has_value("jamf_protect.alerts.input.match.event.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("jamf_protect.alerts.input.match.event.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("process.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jamf_protect.alerts.input.match.event.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
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
