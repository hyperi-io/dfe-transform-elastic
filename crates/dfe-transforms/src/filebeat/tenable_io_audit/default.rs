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
                if let Some(v) = event.get("json.action") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.received") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json") {
                event.rename("json", "tenable_io.audit")?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event.get_str("tenable_io.audit.action").is_some_and(|s| {
                        [
                            "api-access-control-permissions.create",
                            "api-access-control-permissions.delete",
                            "api-access-control-permissions.update",
                            "api-exports-jobs.create",
                            "api-findings-vulnerabilities-cloud_resource-count.create",
                            "api-findings-vulnerabilities-cloud_resource.create",
                            "api-findings-vulnerabilities-host_audit-count.create",
                            "api-findings-vulnerabilities-host_audit.create",
                            "api-findings-vulnerabilities-host-count.create",
                            "api-findings-vulnerabilities-host.create",
                            "api-findings-vulnerabilities-webapp-count.create",
                            "api-findings-vulnerabilities-webapp.create",
                            "asset.hard.delete",
                            "credentials.create",
                            "dashboards-activity.create",
                            "filters-workbenches-assets.create",
                            "filters-workbenches-vulnerabilities.create",
                            "networks.create",
                            "networks.update",
                            "permissions-agent-group.update",
                            "permissions-scanner-pool.update",
                            "permissions-scanner.update",
                            "scanner-groups.create",
                            "scanner-groups.update",
                            "scanners-agent-groups-agents.update",
                            "scanners-agent-groups.create",
                            "scanners-agents-directives.create",
                            "scanners-agents.delete",
                            "scanners.delete",
                            "scans-pause.create",
                            "scans-remediation.create",
                            "scans-resume.create",
                            "scans.create",
                            "scans.delete",
                            "scans.update",
                            "settings-scanner-rekey.create",
                            "workbenches-assets.create",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event.get_str("tenable_io.audit.action").is_some_and(|s| {
                        [
                            "groups.create",
                            "user.create",
                            "user.roles.update",
                            "user.update",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event.get_str("tenable_io.audit.action").is_some_and(|s| {
                        [
                            "session.create",
                            "session.delete",
                            "session.impersonation.end",
                            "session.impersonation.start",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.append("event.category", json!("session"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event.get_str("tenable_io.audit.action").is_some_and(|s| {
                        [
                            "user.authenticate.api_keys",
                            "user.authenticate.mfa",
                            "user.authenticate.password",
                            "user.impersonation.end",
                            "user.impersonation.start",
                            "user.logout",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event
                        .get_str("tenable_io.audit.action")
                        .is_some_and(|s| s.to_lowercase().contains("create"))
                    && event.has_value("event.category")
                    && (event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("configuration"))
                        }
                        serde_json::Value::String(s) => s.contains("configuration"),
                        _ => false,
                    }) || event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("iam")),
                        serde_json::Value::String(s) => s.contains("iam"),
                        _ => false,
                    }))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event
                        .get_str("tenable_io.audit.action")
                        .is_some_and(|s| s.to_lowercase().contains("delete"))
                    && event.has_value("event.category")
                    && (event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("configuration"))
                        }
                        serde_json::Value::String(s) => s.contains("configuration"),
                        _ => false,
                    }) || event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("iam")),
                        serde_json::Value::String(s) => s.contains("iam"),
                        _ => false,
                    }))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event
                        .get_str("tenable_io.audit.action")
                        .is_some_and(|s| s.to_lowercase().contains("update"))
                    && event.has_value("event.category")
                    && (event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("configuration"))
                        }
                        serde_json::Value::String(s) => s.contains("configuration"),
                        _ => false,
                    }) || event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("iam")),
                        serde_json::Value::String(s) => s.contains("iam"),
                        _ => false,
                    }))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event
                        .get_str("tenable_io.audit.action")
                        .is_some_and(|s| s.to_lowercase().contains("user"))
                    && event.has_value("event.category")
                    && event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("iam")),
                        serde_json::Value::String(s) => s.contains("iam"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event
                        .get_str("tenable_io.audit.action")
                        .is_some_and(|s| s.to_lowercase().contains("group"))
                    && event.has_value("event.category")
                    && event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("iam")),
                        serde_json::Value::String(s) => s.contains("iam"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event.get_str("tenable_io.audit.action").is_some_and(|s| {
                        [
                            "session.create",
                            "session.impersonation.start",
                            "user.authenticate.password",
                            "user.authenticate.api_keys",
                            "user.impersonation.start",
                            "user.authenticate.mfa",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.action")
                    && event.get_str("tenable_io.audit.action").is_some_and(|s| {
                        [
                            "session.delete",
                            "session.impersonation.end",
                            "user.impersonation.end",
                            "user.logout",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            if let Some(v) = event
                .get("tenable_io.audit.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("tenable_io.audit.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                gsub_field(
                    event,
                    "event.action",
                    "event.action",
                    cached_regex!("\\."),
                    "-",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("tenable_io.audit.is_failure") {
                    if let Some(val) = event.get("tenable_io.audit.is_failure") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "tenable_io.audit.is_failure".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_io.audit.is_failure", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_failure_to_boolean",
                )?;
                event.remove("tenable_io.audit.is_failure");
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
                event.has_value("tenable_io.audit.is_failure")
                    && event.get_bool("tenable_io.audit.is_failure") == Some(false)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.is_failure")
                    && event.get_bool("tenable_io.audit.is_failure") == Some(true)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.received")
                    && event.get_str("tenable_io.audit.received") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("tenable_io.audit.received") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.audit.received", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "tenable_io.audit.received".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_received")?;
                    event.remove("tenable_io.audit.received");
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
                .get("tenable_io.audit.received")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("tenable_io.audit.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("tenable_io.audit.is_anonymous") {
                    if let Some(val) = event.get("tenable_io.audit.is_anonymous") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "tenable_io.audit.is_anonymous".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_io.audit.is_anonymous", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_anonymous_to_boolean",
                )?;
                event.remove("tenable_io.audit.is_anonymous");
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
                .get("tenable_io.audit.actor.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if let Some(v) = event
                .get("tenable_io.audit.actor.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
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
                            }
                        }
                    }
                    Ok(())
                })();
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("user.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("user.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("user.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("tenable_io.audit.target.type")
                    && event.get_str("tenable_io.audit.target.type") == Some("User")
            };
            if _cond {
                if let Some(v) = event
                    .get("tenable_io.audit.target.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.id", v)?;
                }
            }

            let _cond = {
                event.has_value("tenable_io.audit.target.type")
                    && event.get_str("tenable_io.audit.target.type") == Some("User")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("tenable_io.audit.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tenable_io.audit.target.type")
                    && event.get_str("tenable_io.audit.target.type") == Some("User")
            };
            if _cond {
                if let Some(v) = event
                    .get("tenable_io.audit.target.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.name", v)?;
                }
            }

            let _cond = {
                event.has_value("user.target.name")
                    && event
                        .get_str("user.target.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.target.name", "user.target.email")?;
            }

            let _cond = { !event.has_value("user.target.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.target.email") {
                        if let Some(input) = event.get_string("user.target.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.target.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.target.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("tenable_io.audit.fields")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def fields = new HashMap();\nfor (f in ctx.tenable_io.audit.fields) {\n  fields.put(f.key.toLowerCase(), f.value);\n}\nctx.tenable_io.audit.fields = fields;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def fields = new HashMap();\nfor (f in ctx.tenable_io.audit.fields) {\n  fields.put(f.key.toLowerCase(), f.value);\n}\nctx.tenable_io.audit.fields = fields;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_fields_object")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("tenable_io.audit.fields.x-forwarded-for") {
                    if let Some(s) = event.get_string("tenable_io.audit.fields.x-forwarded-for") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "tenable_io.audit.fields.x_forwarded_for",
                            Value::Array(parts),
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "split")?;
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

            if event.has_value("tenable_io.audit.fields.x_forwarded_for") {
                map_strings(
                    event,
                    "tenable_io.audit.fields.x_forwarded_for",
                    "tenable_io.audit.fields.x_forwarded_for",
                    |s| s.trim().to_string(),
                )?;
            }

            let _cond = {
                event
                    .get("tenable_io.audit.fields.x_forwarded_for")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "tenable_io.audit.fields.x_forwarded_for", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_x_forwarded_for_to_ip",
                        )?;
                        event.remove("_ingest._value");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event.get("tenable_io.audit.fields.x_forwarded_for").is_some_and(|v| v.is_array()) && event.get("tenable_io.audit.fields.x_forwarded_for").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("tenable_io.audit.fields.x_forwarded_for.0")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("source.ip", v)?;
                    }
                    Ok(())
                })();
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

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = {
                event
                    .get("tenable_io.audit.fields.x_forwarded_for")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "tenable_io.audit.fields.x_forwarded_for", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            event.remove("tenable_io.audit.fields.x-forwarded-for");

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
                event.remove("tenable_io.audit.id");
                event.remove("tenable_io.audit.action");
                event.remove("tenable_io.audit.received");
                event.remove("tenable_io.audit.description");
                event.remove("tenable_io.audit.actor.id");
                event.remove("tenable_io.audit.actor.name");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
