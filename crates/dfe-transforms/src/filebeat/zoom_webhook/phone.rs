// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `phone` pipeline.
pub struct Phone;

impl Transform for Phone {
    fn name(&self) -> &str {
        "phone"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("info"))?;

            let _cond = { ["phone.caller_ringing", "phone.callee_ringing"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { ["phone.callee_answered", "phone.caller_connected"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { ["phone.callee_missed", "phone.callee_ended", "phone.caller_ended"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.phone")?;
                }

                if event.has_value("zoom.phone.download_url") {
                    event.rename("zoom.phone.download_url", "url.full")?;
                }

            let _cond = { ["phone.callee_ringing", "phone.caller_ringing", "phone.caller_ended"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.phone.ringing_start_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.phone.ringing_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("phone.caller_connected") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.phone.connected_start_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.phone.connected_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("zoom.phone.answer_start_time") && event.get_str("event.action") == Some("phone.callee_answered") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.phone.answer_start_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.phone.answer_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { ["phone.callee_missed", "phone.callee_ended", "phone.caller_ended", "phone.callee_rejected"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.phone.call_end_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.phone.call_end_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("phone.voicemail_received") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.phone.date_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.phone.date_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("zoom.phone.duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "zoom.phone.duration".into(),
                            message,
                        })?;
                    event.set("zoom.phone.duration", converted)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("zoom.phone.ringing_start_time") && !event.has_value("zoom.phone.answer_start_time") && event.has_value("zoom.phone.call_end_time") && !event.has_value("zoom.phone.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.start = ctx.zoom.phone.ringing_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.event.start = ctx.zoom.phone.ringing_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#))?;
            }

            let _cond = { !event.has_value("zoom.phone.ringing_start_time") && event.has_value("zoom.phone.answer_start_time") && event.has_value("zoom.phone.call_end_time") && !event.has_value("zoom.phone.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.start = ctx.zoom.phone.answer_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.event.start = ctx.zoom.phone.answer_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#))?;
            }

            let _cond = { event.has_value("zoom.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.zoom.phone.duration * 60L * 1000000000L;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.event.duration = ctx.zoom.phone.duration * 60L * 1000000000L;"#))?;
            }

                if event.has_value("zoom.phone.callee_user_id") {
                    event.rename("zoom.phone.callee_user_id", "zoom.phone.callee.user_id")?;
                }

                if event.has_value("zoom.phone.callee_extension_type") {
                    event.rename("zoom.phone.callee_extension_type", "zoom.phone.callee.extension_type")?;
                }

                if event.has_value("zoom.phone.callee_id") {
                    event.rename("zoom.phone.callee_id", "zoom.phone.callee.id")?;
                }

                if event.has_value("zoom.phone.callee_name") {
                    event.rename("zoom.phone.callee_name", "zoom.phone.callee.name")?;
                }

                if event.has_value("zoom.phone.callee_number") {
                    event.rename("zoom.phone.callee_number", "zoom.phone.callee.phone_number")?;
                }

                if event.has_value("zoom.phone.callee_number_type") {
                    event.rename("zoom.phone.callee_number_type", "zoom.phone.callee.number_type")?;
                }

                if event.has_value("zoom.phone.callee_user_id") {
                    event.rename("zoom.phone.callee_user_id", "zoom.phone.callee.user_id")?;
                }

                if event.has_value("zoom.phone.callee_extension_type") {
                    event.rename("zoom.phone.callee_extension_type", "zoom.phone.callee.extension_type")?;
                }

                if event.has_value("zoom.phone.caller_id") {
                    event.rename("zoom.phone.caller_id", "zoom.phone.caller.id")?;
                }

                if event.has_value("zoom.phone.caller_name") {
                    event.rename("zoom.phone.caller_name", "zoom.phone.caller.name")?;
                }

                if event.has_value("zoom.phone.caller_number") {
                    event.rename("zoom.phone.caller_number", "zoom.phone.caller.phone_number")?;
                }

                if event.has_value("zoom.phone.caller_number_type") {
                    event.rename("zoom.phone.caller_number_type", "zoom.phone.caller.number_type")?;
                }

            let _cond = { event.has_value("zoom.phone.callee.user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.phone.callee.user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.phone.callee_user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.phone.callee_user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.phone.caller.user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.phone.caller.user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("event.action") == Some("phone.voicemail_received") };
            if _cond {
                event.remove("zoom.phone.date_time");
            }

            let v = json!(event.get("zoom.phone.caller.user_id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("source.user.id", v)?;
            }

            let v = json!(event.get("zoom.phone.callee.user_id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("destination.user.id", v)?;
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
