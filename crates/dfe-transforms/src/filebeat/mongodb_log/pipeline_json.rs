// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_json` pipeline.
pub struct PipelineJson;

impl Transform for PipelineJson {
    fn name(&self) -> &str {
        "pipeline_json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                parse_json_field(event, "message", "mongodb.log")?;

                if let Some(date_str) = event.get_as_string("mongodb.log.t.$date") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSZZZZZ"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mongodb.log.t.$date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

                event.rename("mongodb.log.s", "log.level")?;

                event.rename("mongodb.log.c", "mongodb.log.component")?;

                event.rename("mongodb.log.ctx", "mongodb.log.context")?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

                event.rename("mongodb.log.msg", "message")?;

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

                event.remove("mongodb.log.t");
                event.remove("mongodb.log.tags");
                event.remove("mongodb.log.truncated");
                event.remove("mongodb.log.size");

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
