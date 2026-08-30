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
            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.original = stringified_orig;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.original = stringified_orig;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("event.action") != Some("started")
                    && event.get_str("event.action") != Some("completed")
            };
            if _cond {
                event.remove("event.action");
            }

            event.set("event.kind", json!("asset"))?;

            event.set("event.category", Value::Array(vec![json!("iam")]))?;

            event.set(
                "event.type",
                Value::Array(vec![json!("user"), json!("info")]),
            )?;

            event.set("asset.category", json!("entity"))?;

            event.set("asset.type", json!("okta_user"))?;

            if event.has_value("okta.id") {
                event.rename("okta.id", "entityanalytics_okta.user.id")?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.id", v)?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("okta.status") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.user == null) {\n  ctx.user = new HashMap();\n}\nif (ctx.user.account == null) {\n  ctx.user.account = new HashMap();\n}\nif (ctx.user.account.status == null) {\n  ctx.user.account.status = new HashMap();\n}\nctx.user.account.status.put('recovery', false);\nctx.user.account.status.put('locked_out', false);\nctx.user.account.status.put('suspended', false);\nctx.user.account.status.put('password_expired', false);\nctx.user.account.status.put('deprovisioned', false);\ndef status = ctx.okta.status.toLowerCase();\nif (['recovery', 'locked_out', 'suspended', 'password_expired', 'deprovisioned'].contains(status)) {\n  ctx.user.account.status[status] = true;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.user == null) {\n  ctx.user = new HashMap();\n}\nif (ctx.user.account == null) {\n  ctx.user.account = new HashMap();\n}\nif (ctx.user.account.status == null) {\n  ctx.user.account.status = new HashMap();\n}\nctx.user.account.status.put('recovery', false);\nctx.user.account.status.put('locked_out', false);\nctx.user.account.status.put('suspended', false);\nctx.user.account.status.put('password_expired', false);\nctx.user.account.status.put('deprovisioned', false);\ndef status = ctx.okta.status.toLowerCase();\nif (['recovery', 'locked_out', 'suspended', 'password_expired', 'deprovisioned'].contains(status)) {\n  ctx.user.account.status[status] = true;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_set_user_account_status",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("okta.status") {
                event.rename("okta.status", "entityanalytics_okta.user.status")?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.status", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.status", v)?;
            }

            let _cond =
                { event.has_value("okta.created") && event.get_str("okta.created") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.user.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_user_created")?;
                    if event.remove("okta.created").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.created".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.account.create_date", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.create_date", v)?;
            }

            let _cond = {
                event.has_value("okta.activated") && event.get_str("okta.activated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.activated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.user.activated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.activated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_user_activated")?;
                    if event.remove("okta.activated").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.activated".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.activated")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.account.activated_date", v)?;
            }

            let _cond = {
                event.has_value("okta.statusChanged")
                    && event.get_str("okta.statusChanged") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.statusChanged") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.user.status_changed", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.statusChanged".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_user_status_changed",
                    )?;
                    if event.remove("okta.statusChanged").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.statusChanged".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.status_changed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.account.change_date", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.status_changed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_status_change_date", v)?;
            }

            let _cond = {
                event.has_value("okta.lastLogin") && event.get_str("okta.lastLogin") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.lastLogin") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.user.last_login", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.lastLogin".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_user_last_login")?;
                    if event.remove("okta.lastLogin").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.lastLogin".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.last_login")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_seen", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.last_login")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.entity.lifecycle.last_activity", v)?;
            }

            let _cond = {
                event.has_value("okta.lastUpdated") && event.get_str("okta.lastUpdated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.lastUpdated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.user.last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.lastUpdated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_user_last_updated")?;
                    if event.remove("okta.lastUpdated").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.lastUpdated".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.last_updated")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_updated", v)?;
            }

            let _cond = {
                event.has_value("okta.passwordChanged")
                    && event.get_str("okta.passwordChanged") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.passwordChanged") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.user.password_changed", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.passwordChanged".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_user_password_changed",
                    )?;
                    if event.remove("okta.passwordChanged").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.passwordChanged".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.password_changed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.account.password_change_date", v)?;
            }

            if event.has_value("okta.type") {
                event.rename("okta.type", "entityanalytics_okta.user.type")?;
            }

            if event.has_value("groups") {
                event.rename("groups", "entityanalytics_okta.groups")?;
            }

            let _cond = { event.has_value("entityanalytics_okta.groups") };
            if _cond {
                foreach_array(event, "entityanalytics_okta.groups", |event| {
                    event.append_unique(
                        "user.group.id",
                        json!(
                            event
                                .get("_ingest._value.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("entityanalytics_okta.groups") };
            if _cond {
                foreach_array(event, "entityanalytics_okta.groups", |event| {
                    event.append_unique(
                        "user.group.name",
                        json!(
                            event
                                .get("_ingest._value.profile.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("roles") {
                event.rename("roles", "entityanalytics_okta.roles")?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "entityanalytics_okta.roles", |event| {
                    event.append_unique(
                        "user.roles",
                        json!(
                            event
                                .get("_ingest._value.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "entityanalytics_okta.roles", |event| {
                    event.append_unique(
                        "user.roles",
                        json!(
                            event
                                .get("_ingest._value.label")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "entityanalytics_okta.roles", |event| {
                    if event.has_value("_ingest._value.assignmentType") {
                        event.rename(
                            "_ingest._value.assignmentType",
                            "_ingest._value.assignment_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "entityanalytics_okta.roles", |event| {
                    if event.has_value("_ingest._value.lastUpdated") {
                        event
                            .rename("_ingest._value.lastUpdated", "_ingest._value.last_updated")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("entityanalytics_okta.roles").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.last_updated")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_updated", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_updated".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_user_roles.lastUpdated",
                                )?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.rename(
                                        "_ingest._value.last_updated",
                                        "_ingest._value.lastUpdated",
                                    )?;
                                    Ok(())
                                })();
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "entityanalytics_okta.roles",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "entityanalytics_okta.roles", |event| {
                    if event.has_value("_ingest._value.permissions") {
                        foreach_array(event, "_ingest._value.permissions", |event| {
                            if event.has_value("_ingest._value.lastUpdated") {
                                event.rename(
                                    "_ingest._value.lastUpdated",
                                    "_ingest._value.last_updated",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: for (def role : ctx.entityanalytics_okta.roles) {\n  if (!(role?.permissions instanceof List)) {\n    continue;\n  }\n  for (def perm : role.permissions) {\n    def label = perm?.label;\n    if (label == null || label == '') {\n      continue;\n    }\n    ctx.user = ctx.user ?: new HashMap();\n    ctx.user.entity = ctx.user.entity ?: new HashMap();\n    ctx.user.entity.attributes = ctx.user.entity.attributes ?: new HashMap();\n    ctx.user.entity.attributes.permissions = ctx.user.entity.attributes.permissions ?: new HashSet();\n    ctx.user.entity.attributes.permissions.add(label);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (def role : ctx.entityanalytics_okta.roles) {\n  if (!(role?.permissions instanceof List)) {\n    continue;\n  }\n  for (def perm : role.permissions) {\n    def label = perm?.label;\n    if (label == null || label == '') {\n      continue;\n    }\n    ctx.user = ctx.user ?: new HashMap();\n    ctx.user.entity = ctx.user.entity ?: new HashMap();\n    ctx.user.entity.attributes = ctx.user.entity.attributes ?: new HashMap();\n    ctx.user.entity.attributes.permissions = ctx.user.entity.attributes.permissions ?: new HashSet();\n    ctx.user.entity.attributes.permissions.add(label);\n  }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("factors") {
                event.rename("factors", "entityanalytics_okta.user.factors")?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.user.factors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: for (def f : ctx.entityanalytics_okta.user.factors) {\n  if (f == null || f.status != \"ACTIVE\") {\n    continue;\n  }\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.attributes = ctx.user.entity.attributes ?: new HashMap();\n  ctx.user.entity.attributes.put(\"mfa_enabled\", true);\n  return;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (def f : ctx.entityanalytics_okta.user.factors) {\n  if (f == null || f.status != \"ACTIVE\") {\n    continue;\n  }\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.attributes = ctx.user.entity.attributes ?: new HashMap();\n  ctx.user.entity.attributes.put(\"mfa_enabled\", true);\n  return;\n}\n"#
                    ),
                )?;
            }

            if event.has_value("devices") {
                event.rename("devices", "entityanalytics_okta.user.devices")?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.user.devices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def ids = new ArrayList();\ndef names = new ArrayList();\nfor (def device : ctx.entityanalytics_okta.user.devices) {\n  if (device == null) { continue; }\n  if (device.id != null) { ids.add(device.id); }\n  def displayName = device?.profile?.displayName;\n  if (displayName != null) { names.add(displayName); }\n}\ndef hostObj = new HashMap();\nif (!ids.isEmpty()) { hostObj.put(\"id\", ids); }\nif (!names.isEmpty()) { hostObj.put(\"name\", names); }\nif (!hostObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n  ctx.user.entity.relationships.put(\"owns\", [\"host\": hostObj]);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def ids = new ArrayList();\ndef names = new ArrayList();\nfor (def device : ctx.entityanalytics_okta.user.devices) {\n  if (device == null) { continue; }\n  if (device.id != null) { ids.add(device.id); }\n  def displayName = device?.profile?.displayName;\n  if (displayName != null) { names.add(displayName); }\n}\ndef hostObj = new HashMap();\nif (!ids.isEmpty()) { hostObj.put(\"id\", ids); }\nif (!names.isEmpty()) { hostObj.put(\"name\", names); }\nif (!hostObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n  ctx.user.entity.relationships.put(\"owns\", [\"host\": hostObj]);\n}\n"#
                    ),
                )?;
            }

            if event.has_value("supervises") {
                event.rename("supervises", "entityanalytics_okta.user.supervises")?;
            }

            let _cond = {
                event
                    .get("entityanalytics_okta.user.supervises")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def ids = new ArrayList();\ndef names = new ArrayList();\ndef emails = new ArrayList();\ndef renamedSupervises = new ArrayList();\nfor (def supervised : ctx.entityanalytics_okta.user.supervises) {\n  if (supervised == null) { continue; }\n  def raw = new HashMap();\n  if (supervised?.user_id != null) {\n    ids.add(supervised.user_id);\n    raw.put(\"user.id\", supervised.user_id);\n  }\n  if (supervised?.username != null) {\n    names.add(supervised.username);\n    raw.put(\"user.name\", supervised.username);\n  }\n  if (supervised?.email != null) {\n    emails.add(supervised.email);\n    raw.put(\"user.email\", supervised.email);\n  }\n  renamedSupervises.add(raw);\n}\nctx.entityanalytics_okta.user.supervises = renamedSupervises;\ndef userObj = new HashMap();\nif (!ids.isEmpty()) { userObj.put(\"id\", ids); }\nif (!names.isEmpty()) { userObj.put(\"name\", names); }\nif (!emails.isEmpty()) { userObj.put(\"email\", emails); }\nif (!userObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n  ctx.user.entity.relationships.put(\"supervises\", [\"user\": userObj]);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def ids = new ArrayList();\ndef names = new ArrayList();\ndef emails = new ArrayList();\ndef renamedSupervises = new ArrayList();\nfor (def supervised : ctx.entityanalytics_okta.user.supervises) {\n  if (supervised == null) { continue; }\n  def raw = new HashMap();\n  if (supervised?.user_id != null) {\n    ids.add(supervised.user_id);\n    raw.put(\"user.id\", supervised.user_id);\n  }\n  if (supervised?.username != null) {\n    names.add(supervised.username);\n    raw.put(\"user.name\", supervised.username);\n  }\n  if (supervised?.email != null) {\n    emails.add(supervised.email);\n    raw.put(\"user.email\", supervised.email);\n  }\n  renamedSupervises.add(raw);\n}\nctx.entityanalytics_okta.user.supervises = renamedSupervises;\ndef userObj = new HashMap();\nif (!ids.isEmpty()) { userObj.put(\"id\", ids); }\nif (!names.isEmpty()) { userObj.put(\"name\", names); }\nif (!emails.isEmpty()) { userObj.put(\"email\", emails); }\nif (!userObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n  ctx.user.entity.relationships.put(\"supervises\", [\"user\": userObj]);\n}\n"#
                    ),
                )?;
            }

            if event.has_value("okta.transitioningToStatus") {
                event.rename(
                    "okta.transitioningToStatus",
                    "entityanalytics_okta.user.transitioning_to_status",
                )?;
            }

            if event.has_value("okta.profile.login") {
                event.rename(
                    "okta.profile.login",
                    "entityanalytics_okta.user.profile.login",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.login") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.login")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.login")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("okta.profile.email") {
                event.rename(
                    "okta.profile.email",
                    "entityanalytics_okta.user.profile.email",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("okta.profile.secondEmail") {
                event.rename(
                    "okta.profile.secondEmail",
                    "entityanalytics_okta.user.profile.second_email",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.second_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.second_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.second_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.other_identities", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.second_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.secondEmail", v)?;
            }

            if event.has_value("okta.profile.firstName") {
                event.rename(
                    "okta.profile.firstName",
                    "entityanalytics_okta.user.profile.first_name",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.first_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.first_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.first_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.first_name", v)?;
            }

            if event.has_value("okta.profile.lastName") {
                event.rename(
                    "okta.profile.lastName",
                    "entityanalytics_okta.user.profile.last_name",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.last_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.last_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.last_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.last_name", v)?;
            }

            if event.has_value("okta.profile.middleName") {
                event.rename(
                    "okta.profile.middleName",
                    "entityanalytics_okta.user.profile.middle_name",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.middle_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.middle_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("okta.profile.honorificPrefix") {
                event.rename(
                    "okta.profile.honorificPrefix",
                    "entityanalytics_okta.user.profile.honorific.prefix",
                )?;
            }

            if event.has_value("okta.profile.honorificSuffix") {
                event.rename(
                    "okta.profile.honorificSuffix",
                    "entityanalytics_okta.user.profile.honorific.suffix",
                )?;
            }

            if event.has_value("okta.profile.title") {
                event.rename(
                    "okta.profile.title",
                    "entityanalytics_okta.user.profile.title",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.title")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.job_title", v)?;
            }

            if event.has_value("okta.profile.displayName") {
                event.rename(
                    "okta.profile.displayName",
                    "entityanalytics_okta.user.profile.display_name",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.name", v)?;
            }

            if event.has_value("okta.profile.nickName") {
                event.rename(
                    "okta.profile.nickName",
                    "entityanalytics_okta.user.profile.nick_name",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.nick_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.nick_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("okta.profile.profileUrl") {
                event.rename(
                    "okta.profile.profileUrl",
                    "entityanalytics_okta.user.profile.url",
                )?;
            }

            if event.has_value("okta.profile.primaryPhone") {
                event.rename(
                    "okta.profile.primaryPhone",
                    "entityanalytics_okta.user.profile.primary_phone",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.primary_phone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.primaryPhone", v)?;
            }

            if event.has_value("okta.profile.mobilePhone") {
                event.rename(
                    "okta.profile.mobilePhone",
                    "entityanalytics_okta.user.profile.mobile_phone",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.mobile_phone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.mobile_phone", v)?;
            }

            if event.has_value("okta.profile.streetAddress") {
                event.rename(
                    "okta.profile.streetAddress",
                    "entityanalytics_okta.user.profile.street_address",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.street_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.geo.name", v)?;
            }

            if event.has_value("okta.profile.city") {
                event.rename(
                    "okta.profile.city",
                    "entityanalytics_okta.user.profile.city",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.geo.city_name", v)?;
            }

            if event.has_value("okta.profile.state") {
                event.rename(
                    "okta.profile.state",
                    "entityanalytics_okta.user.profile.state",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.state")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.geo.region_name", v)?;
            }

            if event.has_value("okta.profile.zipCode") {
                event.rename(
                    "okta.profile.zipCode",
                    "entityanalytics_okta.user.profile.zip_code",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.zip_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.geo.postal_code", v)?;
            }

            if event.has_value("okta.profile.countryCode") {
                event.rename(
                    "okta.profile.countryCode",
                    "entityanalytics_okta.user.profile.country_code",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.country_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.geo.country_iso_code", v)?;
            }

            if event.has_value("okta.profile.postalAddress") {
                event.rename(
                    "okta.profile.postalAddress",
                    "entityanalytics_okta.user.profile.postal_address",
                )?;
            }

            if event.has_value("okta.profile.preferredLanguage") {
                event.rename(
                    "okta.profile.preferredLanguage",
                    "entityanalytics_okta.user.profile.preferred_language",
                )?;
            }

            if event.has_value("okta.profile.locale") {
                event.rename(
                    "okta.profile.locale",
                    "entityanalytics_okta.user.profile.locale",
                )?;
            }

            if event.has_value("okta.profile.timezone") {
                event.rename(
                    "okta.profile.timezone",
                    "entityanalytics_okta.user.profile.timezone",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.geo.timezone", v)?;
            }

            if event.has_value("okta.profile.userType") {
                event.rename(
                    "okta.profile.userType",
                    "entityanalytics_okta.user.profile.user_type",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.user_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.type", v)?;
            }

            if event.has_value("okta.profile.employeeNumber") {
                event.rename(
                    "okta.profile.employeeNumber",
                    "entityanalytics_okta.user.profile.employee_number",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.employee_number") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.employee_number")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.employee_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.id", v)?;
            }

            if event.has_value("okta.profile.externalId") {
                event.rename(
                    "okta.profile.externalId",
                    "entityanalytics_okta.user.profile.external_id",
                )?;
            }

            if event.has_value("okta.profile.costCenter") {
                event.rename(
                    "okta.profile.costCenter",
                    "entityanalytics_okta.user.profile.cost_center",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.cost_center")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.costCenter", v)?;
            }

            if event.has_value("okta.profile.organization") {
                event.rename(
                    "okta.profile.organization",
                    "entityanalytics_okta.user.profile.organization",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.organization")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.organization.name", v)?;
            }

            if event.has_value("okta.profile.division") {
                event.rename(
                    "okta.profile.division",
                    "entityanalytics_okta.user.profile.division",
                )?;
            }

            if event.has_value("okta.profile.department") {
                event.rename(
                    "okta.profile.department",
                    "entityanalytics_okta.user.profile.department",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.department")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.department", v)?;
            }

            if event.has_value("okta.profile.managerId") {
                event.rename(
                    "okta.profile.managerId",
                    "entityanalytics_okta.user.profile.manager.id",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.manager.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.manager.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.profile.manager.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.profile.manager", v)?;
            }

            if event.has_value("okta.profile.manager") {
                event.rename(
                    "okta.profile.manager",
                    "entityanalytics_okta.user.profile.manager.name",
                )?;
            }

            let _cond = { event.has_value("entityanalytics_okta.user.profile.manager.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_okta.user.profile.manager.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("okta.credentials.recovery_question") };
            if _cond {
                event.set("okta.credentials.recovery_question.is_set", json!(true))?;
            }

            let _cond = { !event.has_value("okta.credentials.recovery_question") };
            if _cond {
                event.set("okta.credentials.recovery_question.is_set", json!(false))?;
            }

            if event.has_value("okta.credentials.recovery_question") {
                event.rename(
                    "okta.credentials.recovery_question",
                    "entityanalytics_okta.user.credentials.recovery_question",
                )?;
            }

            if event.has_value("okta.credentials.provider.type") {
                event.rename(
                    "okta.credentials.provider.type",
                    "entityanalytics_okta.user.credentials.provider.type",
                )?;
            }

            if event.has_value("okta.credentials.provider.name") {
                event.rename(
                    "okta.credentials.provider.name",
                    "entityanalytics_okta.user.credentials.provider.name",
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.user.credentials.provider.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.vendor", v)?;
            }

            if event.has_value("okta._links") {
                event.rename("okta._links", "entityanalytics_okta.user._links")?;
            }

            if event.has_value("okta._embedded") {
                event.rename("okta._embedded", "entityanalytics_okta.user._embedded")?;
            }

            event.remove("okta");

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
                event.remove("entityanalytics_okta.user.profile.cost_center");
                event.remove("entityanalytics_okta.user.last_login");
                event.remove("entityanalytics_okta.user.last_updated");
                event.remove("entityanalytics_okta.user.status");
                event.remove("entityanalytics_okta.user.credentials.provider.name");
                event.remove("entityanalytics_okta.user.activated");
                event.remove("entityanalytics_okta.user.status_changed");
                event.remove("entityanalytics_okta.user.created");
                event.remove("entityanalytics_okta.user.password_changed");
                event.remove("entityanalytics_okta.user.profile.email");
                event.remove("entityanalytics_okta.user.profile.display_name");
                event.remove("entityanalytics_okta.user.id");
                event.remove("entityanalytics_okta.user.profile.login");
                event.remove("entityanalytics_okta.user.profile.department");
                event.remove("entityanalytics_okta.user.profile.first_name");
                event.remove("entityanalytics_okta.user.profile.employee_number");
                event.remove("entityanalytics_okta.user.profile.title");
                event.remove("entityanalytics_okta.user.profile.last_name");
                event.remove("entityanalytics_okta.user.profile.manager.id");
                event.remove("entityanalytics_okta.user.profile.mobile_phone");
                event.remove("entityanalytics_okta.user.profile.primary_phone");
                event.remove("entityanalytics_okta.user.profile.organization");
                event.remove("entityanalytics_okta.user.profile.street_address");
                event.remove("entityanalytics_okta.user.profile.city");
                event.remove("entityanalytics_okta.user.profile.state");
                event.remove("entityanalytics_okta.user.profile.zip_code");
                event.remove("entityanalytics_okta.user.profile.timezone");
                event.remove("entityanalytics_okta.user.profile.country_code");
                event.remove("entityanalytics_okta.user.profile.second_email");
                event.remove("entityanalytics_okta.user.profile.user_type");
                event.remove("entityanalytics_okta.user.factors");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
