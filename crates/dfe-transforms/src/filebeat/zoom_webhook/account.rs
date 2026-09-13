// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `account` pipeline.
pub struct Account;

impl Transform for Account {
    fn name(&self) -> &str {
        "account"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.category", json!("iam"))?;

            let _cond = { event.get_str("event.action") == Some("account.settings_updated") };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

                event.append("event.type", json!("user"))?;

            let _cond = { event.get_str("event.action") == Some("account.created") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { ["account.updated", "account.settings_updated", "account.disassociated"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

                if event.has_value("zoom.account_id") {
                    event.rename("zoom.account_id", "zoom.master_account_id")?;
                }

                if event.has_value("zoom.object.id") {
                    event.rename("zoom.object.id", "zoom.sub_account_id")?;
                }

            let _cond = { event.has_value("zoom.time_stamp") };
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

                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.account")?;
                }

            let v = json!(event.get("zoom.account.owner_id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.target.id", v)?;
            }

            let v = json!(event.get("zoom.account.owner_email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.target.email", v)?;
            }

            let _cond = { event.has_value("zoom.old_values.id") };
            if _cond {
            event.set("user.target.id", json!(event.get("zoom.old_values.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.old_values.account_email") };
            if _cond {
            event.set("user.target.email", json!(event.get("zoom.old_values.account_email").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.old_values.account_name") };
            if _cond {
            event.set("user.target.full_name", json!(event.get("zoom.old_values.account_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.old_values.account_alias") };
            if _cond {
            event.set("user.target.name", json!(event.get("zoom.old_values.account_alias").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.account.id") && !condition_eq(event.get("zoom.old_values.id"), event.get("zoom.account.id")) };
            if _cond {
            event.set("user.changes.id", json!(event.get("zoom.account.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.account.account_email") && !condition_eq(event.get("zoom.old_values.account_email"), event.get("zoom.account.account_email")) };
            if _cond {
            event.set("user.changes.email", json!(event.get("zoom.account.account_email").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.account.account_name") && !condition_eq(event.get("zoom.old_values.account_name"), event.get("zoom.account.account_name")) };
            if _cond {
            event.set("user.changes.full_name", json!(event.get("zoom.account.account_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.account.account_alias") && !condition_eq(event.get("zoom.old_values.account_alias"), event.get("zoom.account.account_alias")) };
            if _cond {
            event.set("user.changes.name", json!(event.get("zoom.account.account_alias").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.account.owner_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.account.owner_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.target.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.changes.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.changes.id").map_or_else(String::new, template_to_string)))?;
            }

                event.remove("zoom.time_stamp");

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
