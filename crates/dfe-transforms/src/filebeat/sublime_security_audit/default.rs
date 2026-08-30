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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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
                            .get("_ingest.on_failure_pipeline")
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("observer.vendor", json!("Sublime Security"))?;

            event.set("observer.product", json!("Sublime Security"))?;

            let _cond = {
                event.has_value("json.created_at") && event.get_str("json.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.audit.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_at")?;
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("sublime_security.audit.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.created_by.active") {
                    if let Some(val) = event.get("json.created_by.active") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.created_by.active".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.audit.created_by.active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_created_by_active_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("json.created_by.created_at")
                    && event.get_str("json.created_by.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_by.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.audit.created_by.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_by.created_at".into(),
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
                        "date_created_by_created_at",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = {
                event.has_value("json.created_by.deleted_at")
                    && event.get_str("json.created_by.deleted_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_by.deleted_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.audit.created_by.deleted_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_by.deleted_at".into(),
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
                        "date_created_by_deleted_at",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("json.created_by.email_address") {
                event.rename(
                    "json.created_by.email_address",
                    "sublime_security.audit.created_by.email_address",
                )?;
            }

            let _cond = { event.has_value("sublime_security.audit.created_by.email_address") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("sublime_security.audit.created_by.email_address")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "sublime_security.audit.created_by.email_address".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_created_by_email_address",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
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
                .get("sublime_security.audit.created_by.email_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("sublime_security.audit.created_by.email_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.audit.created_by.email_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.created_by.first_name") {
                event.rename(
                    "json.created_by.first_name",
                    "sublime_security.audit.created_by.first_name",
                )?;
            }

            let _cond = { event.has_value("sublime_security.audit.created_by.email_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.audit.created_by.first_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.created_by.last_name") {
                event.rename(
                    "json.created_by.last_name",
                    "sublime_security.audit.created_by.last_name",
                )?;
            }

            let _cond = {
                event.has_value("sublime_security.audit.created_by.first_name")
                    && event.has_value("sublime_security.audit.created_by.last_name")
            };
            if _cond {
                let v = json!(format!(
                    "{} {}",
                    event
                        .get("sublime_security.audit.created_by.first_name")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("sublime_security.audit.created_by.last_name")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
                }
            }

            if event.has_value("json.created_by.google_oauth_user_id") {
                event.rename(
                    "json.created_by.google_oauth_user_id",
                    "sublime_security.audit.created_by.google_oauth_user_id",
                )?;
            }

            let _cond =
                { event.has_value("sublime_security.audit.created_by.google_oauth_user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.audit.created_by.google_oauth_user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.created_by.id") {
                event.rename("json.created_by.id", "sublime_security.audit.created_by.id")?;
            }

            if let Some(v) = event
                .get("sublime_security.audit.created_by.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("sublime_security.audit.created_by.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.audit.created_by.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.created_by.is_enrolled") {
                    if let Some(val) = event.get("json.created_by.is_enrolled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.created_by.is_enrolled".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.audit.created_by.is_enrolled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_created_by_is_enrolled_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("json.created_by.microsoft_oauth_user_id") {
                event.rename(
                    "json.created_by.microsoft_oauth_user_id",
                    "sublime_security.audit.created_by.microsoft_oauth_user_id",
                )?;
            }

            let _cond =
                { event.has_value("sublime_security.audit.created_by.microsoft_oauth_user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.audit.created_by.microsoft_oauth_user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.created_by.phone_number") {
                event.rename(
                    "json.created_by.phone_number",
                    "sublime_security.audit.created_by.phone_number",
                )?;
            }

            if event.has_value("json.created_by.role") {
                event.rename(
                    "json.created_by.role",
                    "sublime_security.audit.created_by.role",
                )?;
            }

            let _cond = { event.has_value("sublime_security.audit.created_by.role") };
            if _cond {
                event.append_unique(
                    "user.roles",
                    json!(
                        event
                            .get("sublime_security.audit.created_by.role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.created_by.updated_at")
                    && event.get_str("json.created_by.updated_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_by.updated_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.audit.created_by.updated_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_by.updated_at".into(),
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
                        "date_created_by_updated_at",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("json.data.message.id") {
                event.rename(
                    "json.data.message.id",
                    "sublime_security.audit.data.message.id",
                )?;
            }

            if event.has_value("json.data.message_group.id") {
                event.rename(
                    "json.data.message_group.id",
                    "sublime_security.audit.data.message_group.id",
                )?;
            }

            if event.has_value("json.data.request.api_key_name") {
                event.rename(
                    "json.data.request.api_key_name",
                    "sublime_security.audit.data.request.api_key_name",
                )?;
            }

            if event.has_value("json.data.request.authentication_method") {
                event.rename(
                    "json.data.request.authentication_method",
                    "sublime_security.audit.data.request.authentication_method",
                )?;
            }

            if event.has_value("json.data.request.body") {
                event.rename(
                    "json.data.request.body",
                    "sublime_security.audit.data.request.body",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.audit.data.request.body")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.body.content", v)?;
            }

            if event.has_value("json.data.request.id") {
                event.rename(
                    "json.data.request.id",
                    "sublime_security.audit.data.request.id",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.audit.data.request.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.id", v)?;
            }

            let _cond = {
                event.has_value("json.data.request.ip")
                    && event.get_str("json.data.request.ip") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.data.request.ip") {
                        if let Some(val) = event.get("json.data.request.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.data.request.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("sublime_security.audit.data.request.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_data_request_ip_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("sublime_security.audit.data.request.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.data.request.method") {
                event.rename(
                    "json.data.request.method",
                    "sublime_security.audit.data.request.method",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.audit.data.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if event.has_value("json.data.request.path") {
                event.rename(
                    "json.data.request.path",
                    "sublime_security.audit.data.request.path",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.audit.data.request.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.path", v)?;
            }

            if event.has_value("json.data.request.query") {
                event.rename(
                    "json.data.request.query",
                    "sublime_security.audit.data.request.query",
                )?;
            }

            let _cond = { event.has_value("sublime_security.audit.data.request.query") };
            if _cond {
                // Painless script
                // Source: StringBuilder sb = new StringBuilder();\nMap attributes = ctx.sublime_security.audit.data.request.query;\nfor (entry in attributes.entrySet()) {\n  sb.append(entry.getKey()).append('=').append(entry.getValue()).append('&');\n}\nif(ctx.url == null) {\n    ctx.put(\"url\", new HashMap());\n}\nctx.url.query = sb.length() > 0 ? sb.substring(0, sb.length() - 1) : null;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"StringBuilder sb = new StringBuilder();\nMap attributes = ctx.sublime_security.audit.data.request.query;\nfor (entry in attributes.entrySet()) {\n  sb.append(entry.getKey()).append('=').append(entry.getValue()).append('&');\n}\nif(ctx.url == null) {\n    ctx.put(\"url\", new HashMap());\n}\nctx.url.query = sb.length() > 0 ? sb.substring(0, sb.length() - 1) : null;"#
                    ),
                )?;
            }

            if event.has_value("json.data.request.user_agent") {
                event.rename(
                    "json.data.request.user_agent",
                    "sublime_security.audit.data.request.user_agent",
                )?;
            }

            if event.has_value("sublime_security.audit.data.request.user_agent") {
                if let Some(ua_str) =
                    event.get_string("sublime_security.audit.data.request.user_agent")
                {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has_value("json.id") {
                event.rename("json.id", "sublime_security.audit.id")?;
            }

            if let Some(v) = event
                .get("sublime_security.audit.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "sublime_security.audit.type")?;
            }

            if event.has_value("sublime_security.audit.type") {
                if let Some(input) = event.get_string("sublime_security.audit.type") {
                    // Grok pattern: .*.%{WORD:event.action}
                    let _ = cached_grok!(".*.%{WORD:event.action}").extract_into(&input, event)?;
                }
            }

            let _cond = { event.has_value("sublime_security.audit.type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String t = ctx.sublime_security.audit.type;\nint dot = t.indexOf('.');\nString res = dot >= 0 ? t.substring(0, dot) : t;\n\nArrayList cats = new ArrayList();\nArrayList types = new ArrayList();\n\nif (params.email_resources.contains(res)) {\n  cats.add('email');\n  types.add('info');\n} else if (res == 'auth') {\n  cats.add('authentication');\n  if (t.startsWith('auth.login')) { types.add('start'); }\n  else if (t == 'auth.logout') { types.add('end'); }\n  else { types.add('info'); }\n} else if (params.iam_resources.contains(res)) {\n  cats.add('iam');\n  if (res == 'user') { types.add('user'); }\n  if (res == 'rbac') { types.add('group'); }\n  if (t.contains('create') || t.contains('connect') || t.contains('upload')) { types.add('creation'); }\n  else if (t.contains('delete')) { types.add('deletion'); }\n  else { types.add('change'); }\n} else if (params.configuration_resources.contains(res) || params.configuration_types.contains(t)) {\n  cats.add('configuration');\n  if (t.contains('create') || t.contains('connect') || t.contains('upload')) { types.add('creation'); }\n  else if (t.contains('delete')) { types.add('deletion'); }\n  else if (t == 'inline_signed_headers_retrieved') { types.add('info'); }\n  else { types.add('change'); }\n}\n\nif (!cats.isEmpty()) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.category = cats;\n  ctx.event.type = types;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"String t = ctx.sublime_security.audit.type;\nint dot = t.indexOf('.');\nString res = dot >= 0 ? t.substring(0, dot) : t;\n\nArrayList cats = new ArrayList();\nArrayList types = new ArrayList();\n\nif (params.email_resources.contains(res)) {\n  cats.add('email');\n  types.add('info');\n} else if (res == 'auth') {\n  cats.add('authentication');\n  if (t.startsWith('auth.login')) { types.add('start'); }\n  else if (t == 'auth.logout') { types.add('end'); }\n  else { types.add('info'); }\n} else if (params.iam_resources.contains(res)) {\n  cats.add('iam');\n  if (res == 'user') { types.add('user'); }\n  if (res == 'rbac') { types.add('group'); }\n  if (t.contains('create') || t.contains('connect') || t.contains('upload')) { types.add('creation'); }\n  else if (t.contains('delete')) { types.add('deletion'); }\n  else { types.add('change'); }\n} else if (params.configuration_resources.contains(res) || params.configuration_types.contains(t)) {\n  cats.add('configuration');\n  if (t.contains('create') || t.contains('connect') || t.contains('upload')) { types.add('creation'); }\n  else if (t.contains('delete')) { types.add('deletion'); }\n  else if (t == 'inline_signed_headers_retrieved') { types.add('info'); }\n  else { types.add('change'); }\n}\n\nif (!cats.isEmpty()) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.category = cats;\n  ctx.event.type = types;\n}"#
                        ),
                        cached_params!(
                            "{\"email_resources\":[\"message\",\"message_group\",\"email_bomb\"],\"iam_resources\":[\"user\",\"rbac\",\"api_key\",\"external_api_credential\",\"api_key_max_duration_set\",\"api_key_max_duration_cleared\",\"generate_password_reset\"],\"configuration_resources\":[\"rules\",\"list\",\"global_exclusion\",\"auth_settings\",\"org\",\"org_settings\",\"child_org\",\"descendant_org\",\"mailbox\",\"message_source\",\"abuse_mailbox\",\"quarantine_digest\",\"ip_allowlist\",\"trusted_sender_ips\",\"folder_routing\",\"github_app\",\"vendor\",\"logo_image\",\"create_child_organization\",\"dlp_toggled\",\"graymail_user_preference_changed\",\"auto_confirm_discovered_vendors_toggled\",\"synchronous_processing_monitoring_toggled\",\"dlp_alerts_webhook_deleted\",\"dlp_alerts_webhook_upserted\",\"inline_alerts_webhook_deleted\",\"inline_alerts_webhook_upserted\",\"inline_processing_toggled\",\"inline_signing_key_created\",\"inline_signing_key_deleted\",\"inline_signed_headers_retrieved\",\"inline_probe_injected\"],\"configuration_types\":[\"link_clicks.exceptions_updated\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_ecs_categorization",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

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
                event.remove("sublime_security.audit.created_at");
                event.remove("sublime_security.audit.created_by.email_address");
                event.remove("sublime_security.audit.created_by.id");
                event.remove("sublime_security.audit.created_by.role");
                event.remove("sublime_security.audit.data.request.body");
                event.remove("sublime_security.audit.data.request.id");
                event.remove("sublime_security.audit.data.request.ip");
                event.remove("sublime_security.audit.data.request.method");
                event.remove("sublime_security.audit.data.request.path");
                event.remove("sublime_security.audit.data.request.user_agent");
                event.remove("sublime_security.audit.id");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                            .get("_ingest.on_failure_pipeline")
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
