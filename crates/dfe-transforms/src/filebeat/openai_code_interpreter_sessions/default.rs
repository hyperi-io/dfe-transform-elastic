// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("9.3.0"))?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("metric"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "openai.code_interpreter_sessions.results")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(
                event,
                "openai.code_interpreter_sessions.results",
                "openai.code_interpreter_sessions",
            )?;

            event.remove("openai.code_interpreter_sessions.results");

            if event.has_value("openai.code_interpreter_sessions.project_id") {
                event.rename(
                    "openai.code_interpreter_sessions.project_id",
                    "openai.base.project_id",
                )?;
            }

            if event.has_value("openai.code_interpreter_sessions.object") {
                event.rename(
                    "openai.code_interpreter_sessions.object",
                    "openai.base.usage_object_type",
                )?;
            }

            if let Some(date_str) =
                event.get_as_string("openai.code_interpreter_sessions.start_time")
            {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                    Some(parsed) => event.set("openai.base.start_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "openai.code_interpreter_sessions.start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if let Some(date_str) = event.get_as_string("openai.code_interpreter_sessions.end_time")
            {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                    Some(parsed) => event.set("openai.base.end_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "openai.code_interpreter_sessions.end_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if let Some(v) = event.get("openai.base.start_time").cloned() {
                event.set("@timestamp", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .remove("openai.code_interpreter_sessions.start_time")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "openai.code_interpreter_sessions.start_time".into(),
                    });
                }
                if event
                    .remove("openai.code_interpreter_sessions.end_time")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "openai.code_interpreter_sessions.end_time".into(),
                    });
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
                event.append("error.message", json!(format!("Processor '{}'\nwith tag '{}'\nin pipeline '{}'\nfailed with message '{}'\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
