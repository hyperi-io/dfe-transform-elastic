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
            parse_json_field(event, "message", "json")?;

            let _cond =
                { !event.has_value("json") || !(event.get("json").is_some_and(|v| v.is_object())) };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("missing json object in input document").to_string(),
                });
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if event.has_value("json") {
                event.rename("json", "tines.time_saved")?;
            }

            if event.has_value("_tmp.tenant_url") {
                event.rename("_tmp.tenant_url", "tines.tenant_url")?;
            }

            if event.has_value("_tmp.team_id") {
                event.rename("_tmp.team_id", "tines.time_saved.team_id")?;
            }

            if event.has_value("_tmp.story_id") {
                event.rename("_tmp.story_id", "tines.time_saved.story_id")?;
            }

            let _cond = {
                event.has_value("tines.time_saved.date")
                    && event.get_str("tines.time_saved.date") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("tines.time_saved.date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tines.time_saved.date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                !event.has_value("tines.time_saved.meta.next_page")
                    || event.get_str("tines.time_saved.meta.next_page") == Some("")
            };
            if _cond {
                event.remove("tines.time_saved.meta.next_page");
            }

            let _cond = {
                !event.has_value("tines.time_saved.meta.previous_page")
                    || event.get_str("tines.time_saved.meta.previous_page") == Some("")
            };
            if _cond {
                event.remove("tines.time_saved.meta.previous_page");
            }

            event.remove("_tmp");
            event.remove("message");
            event.remove("json");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("tines.time_saved.date");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
