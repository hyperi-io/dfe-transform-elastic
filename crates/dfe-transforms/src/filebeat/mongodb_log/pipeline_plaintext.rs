// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_plaintext` pipeline.
pub struct PipelinePlaintext;

impl Transform for PipelinePlaintext {
    fn name(&self) -> &str {
        "pipeline_plaintext"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{TIMESTAMP_ISO8601:mongodb.log.timestamp}%{SPACE}%{MONGO3_SEVERITY:log.level}%{SPACE}%{MONGO3_COMPONENT:mongodb.log.component}%{SPACE}(?:\\[%{DATA:mongodb.log.context}\\])?%{SPACE}%{GREEDYDATA:message}
                    let _ = cached_grok!("%{TIMESTAMP_ISO8601:mongodb.log.timestamp}%{SPACE}%{MONGO3_SEVERITY:log.level}%{SPACE}%{MONGO3_COMPONENT:mongodb.log.component}%{SPACE}(?:\\[%{DATA:mongodb.log.context}\\])?%{SPACE}%{GREEDYDATA:message}").extract_into(&input, event)?;
                }
            }

                if let Some(date_str) = event.get_as_string("mongodb.log.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSZZ"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mongodb.log.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

                if event.remove("mongodb.log.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound { path: "mongodb.log.timestamp".into() });
                }

            let _cond = { event.get_str("mongodb.log.component") == Some("ACCESS") };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("mongodb.log.component") == Some("WRITE") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("mongodb.log.component") != Some("WRITE") && event.get_str("mongodb.log.component") != Some("ACCESS") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("log.level") == Some("F") || event.get_str("log.level") == Some("E") };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
