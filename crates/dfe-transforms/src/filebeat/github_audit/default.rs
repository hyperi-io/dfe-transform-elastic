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

            event.append("event.category", json!("configuration"))?;
            event.append("event.category", json!("web"))?;

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

            parse_json_field(event, "event.original", "json")?;

            let _cond = { !(event.get("json").is_some_and(|v| v.is_object())) };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Missing JSON object").to_string(),
                });
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json._document_id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json") {
                event.rename("json", "github")?;
            }

            let _cond = { event.has_value("github.created_at") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("github.created_at") {
                        match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "github.created_at".into(),
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
                    if event.remove("github.created_at").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "github.created_at".into(),
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

            let _cond = { event.has_value("github.@timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("github.@timestamp") {
                        match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "github.@timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_@timestamp")?;
                    if event.remove("github.@timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "github.@timestamp".into(),
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

            if event.has_value("github._document_id") {
                event.rename("github._document_id", "event.id")?;
            }

            if event.has_value("github.action") {
                event.rename("github.action", "event.action")?;
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.eq_ignore_ascii_case("code_scanning.alert_created"))
                    || event
                        .get_str("event.action")
                        .is_some_and(|s| s.eq_ignore_ascii_case("secret_scanning_alert.create"))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            if event.has_value("github.actor") {
                event.rename("github.actor", "user.name")?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("github.user") {
                event.rename("github.user", "user.target.name")?;
            }

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("github.repository_public") {
                    if let Some(val) = event.get("github.repository_public") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "github.repository_public".into(),
                                message,
                            }
                        })?;
                        event.set("github.repository_public", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_repository_public_to_boolean",
                )?;
                if event.remove("github.repository_public").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "github.repository_public".into(),
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

            let _cond = { !event.has_value("github.team") };
            if _cond {
                if event.has_value("github.data.team") {
                    event.rename("github.data.team", "github.team")?;
                }
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("team."))
            };
            if _cond {
                if let Some(v) = event
                    .get("github.team")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("team."))
                    && event.has_value("user.target.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("github.team")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.group.name", v)?;
                }
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("org."))
            };
            if _cond {
                if let Some(v) = event
                    .get("github.org")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("org."))
                    && event.has_value("user.target.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("github.org")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.group.name", v)?;
                }
            }

            if event.has_value("github.data.old_user") {
                event.rename("github.data.old_user", "user.target.group.name")?;
            }

            let _cond = { !event.has_value("user.target.group.name") };
            if _cond {
                if event.has_value("github.data.old_user") {
                    event.rename("github.data.old_user", "user.target.group.name")?;
                }
            }

            if event.has_value("github.actor_location.country_code") {
                event.rename(
                    "github.actor_location.country_code",
                    "client.geo.country_iso_code",
                )?;
            }

            let _cond = {
                event.has_value("github.actor_ip") && event.get_str("github.actor_ip") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("github.actor_ip") {
                        if let Some(val) = event.get("github.actor_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "github.actor_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("github.actor_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_actor_ip_to_ip")?;
                    if event.remove("github.actor_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "github.actor_ip".into(),
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

            let _cond = { event.has_value("github.actor_ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("github.actor_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("github.user_agent") {
                if let Some(ua_str) = event.get_string("github.user_agent") {
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

            if event.has_value("event.action") {
                if let Some(input) = event.get_string("event.action") {
                    // Grok pattern: ^(?P<github_category>(?:[a-z_]+))\\.%{GREEDYDATA:_temp.action}
                    if !cached_grok_mapped!(
                        "^(?P<github_category>(?:[a-z_]+))\\.%{GREEDYDATA:_temp.action}",
                        [("github_category", "github.category")]
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond =
                { ["team", "org"].contains(&event.get_str("github.category").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond =
                { ["team", "org"].contains(&event.get_str("github.category").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("group"))?;
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.get("_temp.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("create")),
                    serde_json::Value::String(s) => s.contains("create"),
                    _ => false,
                }) || event.get("_temp.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("add")),
                    serde_json::Value::String(s) => s.contains("add"),
                    _ => false,
                })
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get("_temp.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("delete")),
                    serde_json::Value::String(s) => s.contains("delete"),
                    _ => false,
                }) || event.get("_temp.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("remove")),
                    serde_json::Value::String(s) => s.contains("remove"),
                    _ => false,
                })
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                !event.has_value("event.type") || event.get("event.type").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("github.active") {
                    if let Some(val) = event.get("github.active") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "github.active".into(),
                                message,
                            }
                        })?;
                        event.set("github.active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_active_to_boolean",
                )?;
                if event.remove("github.active").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "github.active".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("github.actor_is_bot") {
                    if let Some(val) = event.get("github.actor_is_bot") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "github.actor_is_bot".into(),
                                message,
                            }
                        })?;
                        event.set("github.actor_is_bot", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_actor_is_bot_to_boolean",
                )?;
                if event.remove("github.actor_is_bot").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "github.actor_is_bot".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("github.audit_log_stream_enabled") {
                    if let Some(val) = event.get("github.audit_log_stream_enabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "github.audit_log_stream_enabled".into(),
                                message,
                            }
                        })?;
                        event.set("github.audit_log_stream_enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_audit_log_stream_enabled_to_boolean",
                )?;
                if event.remove("github.audit_log_stream_enabled").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "github.audit_log_stream_enabled".into(),
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

            if event.has_value("github.actor_id") {
                if let Some(val) = event.get("github.actor_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.actor_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.actor_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("github.actor_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("github.actor_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("github.business_id") {
                if let Some(val) = event.get("github.business_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.business_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.business_id", converted)?;
                }
            }

            if event.has_value("github.org_id") {
                if let Some(val) = event.get("github.org_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.org_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.org_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("github.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.id", v)?;
            }

            if event.has_value("github.user_id") {
                if let Some(val) = event.get("github.user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.user_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.user_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("github.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.id", v)?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("github.user_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get("github.events").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: if(ctx.github.events instanceof List) {\n  if(ctx.github.events.size() > 0 && ctx.github.events[0] instanceof Map) {\n    ctx.github.events_object = ctx.github.events;\n    ctx.github.remove(\"events\");\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if(ctx.github.events instanceof List) {\n  if(ctx.github.events.size() > 0 && ctx.github.events[0] instanceof Map) {\n    ctx.github.events_object = ctx.github.events;\n    ctx.github.remove(\"events\");\n  }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("github.events") {
                if let Some(val) = event.get("github.events") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.events".into(),
                            message,
                        }
                    })?;
                    event.set("github.events", converted)?;
                }
            }

            if event.has_value("github.hook_id") {
                if let Some(val) = event.get("github.hook_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.hook_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.hook_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("github.transport_protocol") {
                    if let Some(val) = event.get("github.transport_protocol") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "github.transport_protocol".into(),
                                message,
                            }
                        })?;
                        event.set("github.transport_protocol", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_transport_protocol_to_long",
                )?;
                if event.remove("github.transport_protocol").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "github.transport_protocol".into(),
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

            if let Some(v) = event
                .get("github.transport_protocol_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("github.version") {
                if let Some(val) = event.get("github.version") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.version".into(),
                            message,
                        }
                    })?;
                    event.set("github.version", converted)?;
                }
            }

            let _cond = {
                event.has_value("github.data.started_at")
                    && event.get_str("github.data.started_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("github.data.started_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("github.data.started_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "github.data.started_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_data_started_at")?;
                    if event.remove("github.data.started_at").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "github.data.started_at".into(),
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

            if event.has_value("github.data.trigger_id") {
                if let Some(val) = event.get("github.data.trigger_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.data.trigger_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.data.trigger_id", converted)?;
                }
            }

            if event.has_value("github.data.workflow_id") {
                if let Some(val) = event.get("github.data.workflow_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.data.workflow_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.data.workflow_id", converted)?;
                }
            }

            if event.has_value("github.data.workflow_run_id") {
                if let Some(val) = event.get("github.data.workflow_run_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.data.workflow_run_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.data.workflow_run_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("github.public_repo") {
                    if let Some(val) = event.get("github.public_repo") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "github.public_repo".into(),
                                message,
                            }
                        })?;
                        event.set("github.public_repo", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_public_repo_to_boolean",
                )?;
                if event.remove("github.public_repo").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "github.public_repo".into(),
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

            let _cond = { event.get_str("github.actor_location.ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("github.actor_location.ip") {
                        if let Some(val) = event.get("github.actor_location.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "github.actor_location.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("github.actor_location.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_actor_location_ip_to_ip",
                    )?;
                    if event.remove("github.actor_location.ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "github.actor_location.ip".into(),
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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("github.actor_location.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("github.pull_request_id") {
                if let Some(val) = event.get("github.pull_request_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.pull_request_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.pull_request_id", converted)?;
                }
            }

            if event.has_value("github.repo_id") {
                if let Some(val) = event.get("github.repo_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.repo_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.repo_id", converted)?;
                }
            }

            if event.has_value("github.repositories_added") {
                if let Some(val) = event.get("github.repositories_added") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.repositories_added".into(),
                            message,
                        }
                    })?;
                    event.set("github.repositories_added", converted)?;
                }
            }

            if event.has_value("github.repositories_removed") {
                if let Some(val) = event.get("github.repositories_removed") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.repositories_removed".into(),
                            message,
                        }
                    })?;
                    event.set("github.repositories_removed", converted)?;
                }
            }

            if event.has_value("github.token_id") {
                if let Some(val) = event.get("github.token_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.token_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.token_id", converted)?;
                }
            }

            if event.has_value("github.audit_log_stream_id") {
                if let Some(val) = event.get("github.audit_log_stream_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.audit_log_stream_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.audit_log_stream_id", converted)?;
                }
            }

            if event.has_value("github.ruleset_id") {
                if let Some(val) = event.get("github.ruleset_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.ruleset_id".into(),
                            message,
                        }
                    })?;
                    event.set("github.ruleset_id", converted)?;
                }
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("github.blocked_user")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("github.data.head_sha")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.remove("_temp");
            event.remove("github.created_at");
            event.remove("github.@timestamp");

            let _cond = { event.has_value("aws.s3.bucket") && event.has_value("aws.s3.object") };
            if _cond {
                event.remove("log.file.path");
                event.remove("log.offset");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
