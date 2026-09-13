// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `user` pipeline.
pub struct User;

impl Transform for User {
    fn name(&self) -> &str {
        "user"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("event.action") == Some("user.settings_updated") };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = { !(["user.signed_in", "user.signed_out"].contains(&event.get_str("event.action").unwrap_or(""))) };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = { ["user.signed_in", "user.signed_out"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("event.action") == Some("user.created") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.action") == Some("user.deleted") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { ["user.updated", "user.settings_updated", "user.deactivated", "user.activated", "user.disassociated", "user.presence_status_updated", "user.personal_notes_updated"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("event.action") == Some("user.signed_in") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("event.action") == Some("user.signed_out") };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.user")?;
                }

            let _cond = { ["user.updated", "user.settings_updated"].contains(&event.get_str("event.action").unwrap_or("")) };
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

            let _cond = { ["user.signed_in", "user.signed_out", "user.personal_notes_updated", "user.presence_status_updated"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("zoom.user.date_time") {
                    match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zoom.user.date_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("zoom.user.id") };
            if _cond {
                event.append("related.user", json!(event.get("zoom.user.id").map_or_else(String::new, template_to_string)))?;
            }

                event.remove("zoom.time_stamp");
                event.remove("zoom.user.date_time");

            let v = json!(event.get("zoom.operator_id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
            }

            let _cond = { event.get("zoom.operator").is_some_and(|v| v.is_string()) && event.get("zoom.operator").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) };
            if _cond {
            let v = json!(event.get("zoom.operator").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
            }
            }

            if let Some(v) = event.get("zoom.user.email").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("user.email") {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { !event.has_value("zoom.operator") && !event.has_value("zoom.operator_id") };
            if _cond {
            let v = json!(event.get("zoom.user.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
            }
            }

            let _cond = { !event.has_value("zoom.operator") && !event.has_value("zoom.operator_id") };
            if _cond {
            let v = json!(event.get("zoom.user.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
            }
            }

            let _cond = { !event.has_value("zoom.operator") && !event.has_value("zoom.operator_id") && event.has_value("zoom.user.first_name") };
            if _cond {
            let v = json!(format!("{} {}", event.get("zoom.user.first_name").map_or_else(String::new, template_to_string), event.get("zoom.user.last_name").map_or_else(String::new, template_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
            }
            }

            let v = json!(event.get("zoom.old_values.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.target.id", v)?;
            }

            let v = json!(event.get("zoom.old_values.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.target.id", v)?;
            }

            let v = json!(event.get("zoom.old_values.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.target.email", v)?;
            }

            let v = json!(event.get("zoom.old_values.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.target.email", v)?;
            }

            let _cond = { event.has_value("zoom.old_values.first_name") };
            if _cond {
            event.set("user.target.full_name", json!(format!("{} {}", event.get("zoom.old_values.first_name").map_or_else(String::new, template_to_string), event.get("zoom.old_values.last_name").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { event.has_value("zoom.old_values") || event.has_value("zoom.operator") || event.has_value("zoom.operator_id") };
            if _cond {
            let v = json!(event.get("zoom.user.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    if !event.has("user.target.id") {
                        event.set("user.target.id", v)?;
                    }
            }
            }

            let _cond = { event.has_value("zoom.old_values") || event.has_value("zoom.operator") || event.has_value("zoom.operator_id") };
            if _cond {
            let v = json!(event.get("zoom.user.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    if !event.has("user.target.id") {
                        event.set("user.target.id", v)?;
                    }
            }
            }

            let _cond = { event.has_value("zoom.old_values") || event.has_value("zoom.operator") || event.has_value("zoom.operator_id") };
            if _cond {
            let v = json!(event.get("zoom.user.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    if !event.has("user.target.email") {
                        event.set("user.target.email", v)?;
                    }
            }
            }

            let _cond = { event.has_value("zoom.old_values") };
            if _cond {
            let v = json!(event.get("zoom.user.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    if !event.has("user.target.email") {
                        event.set("user.target.email", v)?;
                    }
            }
            }

            let _cond = { (event.has_value("zoom.old_values") || event.has_value("zoom.operator") || event.has_value("zoom.operator_id")) && event.has_value("zoom.user.first_name") };
            if _cond {
            if !event.has("user.target.full_name") {
                event.set("user.target.full_name", json!(format!("{} {}", event.get("zoom.user.first_name").map_or_else(String::new, template_to_string), event.get("zoom.user.last_name").map_or_else(String::new, template_to_string))))?;
            }
            }

            let _cond = { event.has_value("zoom.old_values.id") && !condition_eq(event.get("zoom.old_values.id"), event.get("zoom.user.id")) };
            if _cond {
            let v = json!(event.get("zoom.user.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.changes.id", v)?;
            }
            }

            let _cond = { event.has_value("zoom.old_values.email") && !condition_eq(event.get("zoom.old_values.email"), event.get("zoom.user.email")) };
            if _cond {
            let v = json!(event.get("zoom.user.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.changes.email", v)?;
            }
            }

            let _cond = { event.has_value("zoom.old_values.first_name") && event.has_value("zoom.old_values.last_name") && (!condition_eq(event.get("zoom.old_values.last_name"), event.get("zoom.user.last_name")) || !condition_eq(event.get("zoom.old_values.first_name"), event.get("zoom.user.first_name"))) };
            if _cond {
            let v = json!(format!("{} {}", event.get("zoom.user.first_name").map_or_else(String::new, template_to_string), event.get("zoom.user.last_name").map_or_else(String::new, template_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("user.changes.full_name", v)?;
            }
            }

            let _cond = { event.has_value("zoom.user.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.user.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.old_values.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.old_values.id").map_or_else(String::new, template_to_string)))?;
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
