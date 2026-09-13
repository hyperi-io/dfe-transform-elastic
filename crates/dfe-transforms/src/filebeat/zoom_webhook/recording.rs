// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `recording` pipeline.
pub struct Recording;

impl Transform for Recording {
    fn name(&self) -> &str {
        "recording"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("info"))?;

            let _cond = { event.get_str("event.action") == Some("recording.registration_created") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.action") == Some("recording.registration_approved") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("event.action") == Some("recording.registration_denied") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { ["recording.deleted", "recording.trashed"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { ["recording.paused", "recording.resumed", "recording.renamed", "recording.recovered"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("event.action") == Some("recording.started") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { ["recording.stopped", "recording.completed", "recording.transcript_completed"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.recording")?;
                }

                if event.has_value("zoom.recording.share_url") {
                    event.rename("zoom.recording.share_url", "url.full")?;
                }

            let _cond = { event.get_str("event.action") == Some("recording.renamed") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.time_stamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.time_stamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("zoom.recording.recording_file.recording_start") == Some("") };
            if _cond {
                if event.remove("zoom.recording.recording_file.recording_start").is_none() {
                    return Err(TransformError::FieldNotFound { path: "zoom.recording.recording_file.recording_start".into() });
                }
            }

            let _cond = { event.get_str("zoom.recording.recording_file.recording_end") == Some("") };
            if _cond {
                if event.remove("zoom.recording.recording_file.recording_end").is_none() {
                    return Err(TransformError::FieldNotFound { path: "zoom.recording.recording_file.recording_end".into() });
                }
            }

            let _cond = { event.get_str("event.action") == Some("recording.started") };
            if _cond {
            let v = json!(event.get("zoom.recording.recording_file.recording_start").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("event.start", v)?;
            }
            }

            let _cond = { event.get_str("event.action") == Some("recording.stopped") };
            if _cond {
            let v = json!(event.get("zoom.recording.recording_file.recording_end").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("event.end", v)?;
            }
            }

            let _cond = { event.has_value("event.end") && event.has_value("event.start") && event.get_str("event.action") == Some("recording.stopped") };
            if _cond {
                // Painless script
                // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#))?;
            }

            let _cond = { event.has_value("zoom.recording.recording_file.recording_start") && event.get_str("event.action") == Some("recording.started") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.recording.recording_file.recording_start") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.recording.recording_file.recording_start".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("zoom.recording.host_id") };
            if _cond {
                event.append("related.user", json!(event.get("zoom.recording.host_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.registrant.id") };
            if _cond {
                event.append("related.user", json!(event.get("zoom.registrant.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("event.action") == Some("recording.renamed") };
            if _cond {
                event.remove("zoom.time_stamp");
            }

            let _cond = { !event.has_value("user.id") && event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(event.get("zoom.registrant.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
            }
            }

            let _cond = { !event.has_value("user.id") && event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(format!("{} {}", event.get("zoom.registrant.first_name").map_or_else(String::new, template_to_string), event.get("zoom.registrant.last_name").map_or_else(String::new, template_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
            }
            }

            let _cond = { !event.has_value("user.id") && event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(event.get("zoom.registrant.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
            }
            }

            let _cond = { !event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(event.get("zoom.recording.host_id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
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
