// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `chat_message` pipeline.
pub struct ChatMessage;

impl Transform for ChatMessage {
    fn name(&self) -> &str {
        "chat_message"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("info"))?;

            let _cond = { event.get_str("event.action") == Some("chat_message.sent") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.action") == Some("chat_message.deleted") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_str("event.action") == Some("chat_message.updated") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.chat_message")?;
                }

            let _cond = { event.has_value("zoom.chat_message.contact_id") };
            if _cond {
                event.append("related.user", json!(event.get("zoom.chat_message.contact_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.chat_message.timestamp") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.chat_message.timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.chat_message.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("zoom.chat_message.timestamp") };
            if _cond {
                event.remove("zoom.chat_message.date_time");
            }

            let _cond = { !event.has_value("zoom.chat_message.timestamp") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.chat_message.date_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.chat_message.date_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

                event.remove("zoom.chat_message.timestamp");

            let _cond = { !event.has_value("zoom.chat_message.message") };
            if _cond {
                event.remove("zoom.chat_message.message");
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
