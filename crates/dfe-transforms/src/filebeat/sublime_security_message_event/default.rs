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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Sublime Security"))?;

            event.set("observer.product", json!("Sublime Security"))?;

            if event.has_value("json.data.message.canonical_id") {
                event.rename(
                    "json.data.message.canonical_id",
                    "sublime_security.message_event.canonical_id",
                )?;
            }

            if event.has_value("json.canonical_id") {
                event.rename(
                    "json.canonical_id",
                    "sublime_security.message_event.canonical_id",
                )?;
            }

            let _cond = {
                event.has_value("json.created_at") && event.get_str("json.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.message_event.created_at", parsed)?
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
                .get("sublime_security.message_event.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event
                    .get("json.data.flagged_rules")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.data.flagged_rules", |event| {
                    event.append_unique(
                        "rule.id",
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
                    .get("json.data.flagged_rules")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.data.flagged_rules", |event| {
                    event.append_unique(
                        "rule.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.data.flagged_rules") {
                event.rename(
                    "json.data.flagged_rules",
                    "sublime_security.message_event.data.flagged_rules",
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.message_event.data.flagged_rules")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.message_event.data.flagged_rules",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.id");
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.data.triggered_actions") {
                event.rename(
                    "json.data.triggered_actions",
                    "sublime_security.message_event.data.triggered_actions",
                )?;
            }

            if event.has_value("json.data.message.external_id") {
                event.rename(
                    "json.data.message.external_id",
                    "sublime_security.message_event.external_id",
                )?;
            }

            if event.has_value("json.external_id") {
                event.rename(
                    "json.external_id",
                    "sublime_security.message_event.external_id",
                )?;
            }

            if event.has_value("json.forward_recipients") {
                event.rename(
                    "json.forward_recipients",
                    "sublime_security.message_event.forward_recipients",
                )?;
            }

            let _cond = {
                event.has_value("json.forwarded_at")
                    && event.get_str("json.forwarded_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.forwarded_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.message_event.forwarded_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.forwarded_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_forwarded_at")?;
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
                event.rename("json.data.message.id", "sublime_security.message_event.id")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "sublime_security.message_event.id")?;
            }

            if let Some(v) = event
                .get("sublime_security.message_event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.landed_in_spam") {
                    if let Some(val) = event.get("json.landed_in_spam") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.landed_in_spam".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.message_event.landed_in_spam", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_landed_in_spam_to_boolean",
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

            if event.has_value("json.mailbox.email") {
                event.rename(
                    "json.mailbox.email",
                    "sublime_security.message_event.mailbox.email",
                )?;
            }

            if event.has_value("json.data.message.mailbox.external_id") {
                event.rename(
                    "json.data.message.mailbox.external_id",
                    "sublime_security.message_event.mailbox.external_id",
                )?;
            }

            if event.has_value("json.mailbox.external_id") {
                event.rename(
                    "json.mailbox.external_id",
                    "sublime_security.message_event.mailbox.external_id",
                )?;
            }

            if event.has_value("json.data.message.mailbox.id") {
                event.rename(
                    "json.data.message.mailbox.id",
                    "sublime_security.message_event.mailbox.id",
                )?;
            }

            if event.has_value("json.mailbox.id") {
                event.rename(
                    "json.mailbox.id",
                    "sublime_security.message_event.mailbox.id",
                )?;
            }

            if event.has_value("json.data.message.message_source_id") {
                event.rename(
                    "json.data.message.message_source_id",
                    "sublime_security.message_event.message_source_id",
                )?;
            }

            if event.has_value("json.message_source_id") {
                event.rename(
                    "json.message_source_id",
                    "sublime_security.message_event.message_source_id",
                )?;
            }

            let _cond =
                { event.has_value("json.read_at") && event.get_str("json.read_at") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.read_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.message_event.read_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.read_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_read_at")?;
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

            let _cond = { event.get("json.recipients").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.recipients", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.recipients").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.recipients", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.recipients") {
                event.rename(
                    "json.recipients",
                    "sublime_security.message_event.recipients",
                )?;
            }

            let _cond = {
                event.has_value("json.replied_at") && event.get_str("json.replied_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.replied_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.message_event.replied_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.replied_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_replied_at")?;
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

            if event.has_value("json.sender.display_name") {
                event.rename(
                    "json.sender.display_name",
                    "sublime_security.message_event.sender.display_name",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.message_event.sender.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            let _cond = { event.has_value("sublime_security.message_event.sender.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.message_event.sender.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.sender.email") {
                event.rename(
                    "json.sender.email",
                    "sublime_security.message_event.sender.email",
                )?;
            }

            let _cond = { event.has_value("sublime_security.message_event.sender.email") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("sublime_security.message_event.sender.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sublime_security.message_event.sender.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sublime_security.message_event.sender.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.subject") {
                event.rename("json.subject", "sublime_security.message_event.subject")?;
            }

            if let Some(v) = event
                .get("sublime_security.message_event.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "sublime_security.message_event.type")?;
            }

            let _cond = {
                event
                    .get("sublime_security.message_event.data.flagged_rules")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.message_event.data.flagged_rules",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.id");
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.message_event.recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.message_event.recipients",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.email");
                        }
                        Ok(())
                    },
                )?;
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
                event.remove("sublime_security.message_event.created_at");
                event.remove("sublime_security.message_event.id");
                event.remove("sublime_security.message_event.sender.display_name");
                event.remove("sublime_security.message_event.sender.email");
                event.remove("sublime_security.message_event.subject");
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
