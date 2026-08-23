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

            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json", parsed)?;
                }
                Ok(())
            })();

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.createdAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.updatedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("json.threatId")
                    && event.get_str("json.threatId") != Some("")
                    && event
                        .get_str("json.data.threatClassification")
                        .is_some_and(|s| ["exploit", "pua"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = {
                event.has_value("json.threatId")
                    && event.get_str("json.threatId") != Some("")
                    && event
                        .get_str("json.data.threatClassification")
                        .is_some_and(|s| ["exploit", "pua"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            let _cond = {
                event.has_value("json.threatId")
                    && event.get_str("json.threatId") != Some("")
                    && event
                        .get_str("json.data.threatClassification")
                        .is_some_and(|s| {
                            ["malware", "ransomware", "trojan", "downloader"]
                                .contains(&s.to_lowercase().as_str())
                        })
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("json.threatId")
                    && event.get_str("json.threatId") != Some("")
                    && event
                        .get_str("json.data.threatClassification")
                        .is_some_and(|s| {
                            ["malware", "ransomware", "trojan", "downloader"]
                                .contains(&s.to_lowercase().as_str())
                        })
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && (event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("logged in"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("logged out"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("failed to log in")))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("logged in"))
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("logged in"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("logged out"))
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && (event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("created"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("added"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("deleted"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("edited"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("updated"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("modified"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("enabled"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("recovery email")))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && (event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("created"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("added")))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("deleted"))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && (event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("edited"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("updated"))
                        || event
                            .get_str("json.primaryDescription")
                            .is_some_and(|s| s.to_lowercase().contains("modified")))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && (event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("verification email")))
            };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            let _cond = {
                event.has_value("json.primaryDescription")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && event
                        .get_str("json.primaryDescription")
                        .is_some_and(|s| s.to_lowercase().contains("failed to log in"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("json.data.confidenceLevel")
                    && (event.get_str("json.data.confidenceLevel").is_some_and(|s| {
                        ["suspicious", "malicious"].contains(&s.to_lowercase().as_str())
                    }))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                !event.has_value("event.type")
                    && (!event.has_value("json.threatId")
                        || event.get_str("json.threatId") == Some(""))
                    && event.has_value("json.primaryDescription")
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("json.updatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedAt") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.activity.updated_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_updatedAt_to_sentinel_one_activity_updated_at_d5e30bab",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.description") {
                event.rename(
                    "json.description",
                    "sentinel_one.activity.description_value",
                )?;
            }

            if event.has("json.hash") {
                event.rename("json.hash", "process.hash.sha1")?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.osFamily") {
                event.rename("json.osFamily", "observer.os.family")?;
            }

            if event.has("json.agentUpdatedVersion") {
                event.rename("json.agentUpdatedVersion", "observer.version")?;
            }

            if event.has("json.groupId") {
                event.rename("json.groupId", "user.group.id")?;
            }

            if let Some(v) = event
                .get("user.group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if event.has("json.groupName") {
                event.rename("json.groupName", "user.group.name")?;
            }

            if let Some(v) = event
                .get("user.group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.name", v)?;
            }

            if event.has("json.accountId") {
                event.rename("json.accountId", "sentinel_one.activity.account.id")?;
            }

            if event.has("json.userId") {
                event.rename("json.userId", "user.id")?;
            }

            if event.has("json.accountName") {
                event.rename("json.accountName", "sentinel_one.account.name")?;
            }

            if event.has("json.agentId") {
                event.rename("json.agentId", "sentinel_one.activity.agent.id")?;
            }

            if let Some(v) = event
                .get("sentinel_one.activity.agent.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has("json.comments") {
                event.rename("json.comments", "sentinel_one.activity.comments")?;
            }

            let _cond = { event.has_value("json.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_createdAt_2a5ac5b2",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.primaryDescription") {
                event.rename(
                    "json.primaryDescription",
                    "sentinel_one.activity.description.primary",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.activity.description.primary") };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one.activity.description.primary")
                    .cloned()
                {
                    event.set("message", v)?;
                }
            }

            if event.has("json.secondaryDescription") {
                event.rename(
                    "json.secondaryDescription",
                    "sentinel_one.activity.description.secondary",
                )?;
            }

            if event.has("json.id") {
                event.rename("json.id", "sentinel_one.activity.id")?;
            }

            if let Some(v) = event
                .get("sentinel_one.activity.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.siteId") {
                event.rename("json.siteId", "sentinel_one.site.id")?;
            }

            if event.has("json.siteName") {
                event.rename("json.siteName", "sentinel_one.site.name")?;
            }

            if event.has("json.threatId") {
                event.rename("json.threatId", "sentinel_one.activity.threat.id")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.activityType") {
                    if let Some(val) = event.get("json.activityType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.activityType".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.activity.type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_activityType_to_sentinel_one_activity_type_77a0c965",
                )?;
                event.remove("json.activityType");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                if event.has_value("json.data.accountId") {
                    if let Some(val) = event.get("json.data.accountId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.accountId".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.activity.data.account.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_data_accountId_to_sentinel_one_activity_data_account_id_6938dd10",
                )?;
                event.remove("json.data.accountId");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.data.accountName") {
                event.rename(
                    "json.data.accountName",
                    "sentinel_one.activity.data.account.name",
                )?;
            }

            if event.has("json.data.fullScopeDetails") {
                event.rename(
                    "json.data.fullScopeDetails",
                    "sentinel_one.activity.data.fullscope.details",
                )?;
            }

            if event.has("json.data.fullScopeDetailsPath") {
                event.rename(
                    "json.data.fullScopeDetailsPath",
                    "sentinel_one.activity.data.fullscope.details_path",
                )?;
            }

            if event.has("json.data.groupName") {
                event.rename(
                    "json.data.groupName",
                    "sentinel_one.activity.data.group_name",
                )?;
            }

            if event.has("json.data.scopeLevel") {
                event.rename(
                    "json.data.scopeLevel",
                    "sentinel_one.activity.data.scope.level",
                )?;
            }

            if event.has("json.data.scopeName") {
                event.rename(
                    "json.data.scopeName",
                    "sentinel_one.activity.data.scope.name",
                )?;
            }

            if event.has("json.data.siteName") {
                event.rename("json.data.siteName", "sentinel_one.activity.data.site.name")?;
            }

            if event.has("json.data.username") {
                event.rename("json.data.username", "user.full_name")?;
            }

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.full_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.data.byUser") {
                event.rename("json.data.byUser", "sentinel_one.activity.data.user.name")?;
            }

            let _cond = { event.has_value("sentinel_one.activity.data.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("sentinel_one.activity.data.user.name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.data.role") {
                event.rename("json.data.role", "sentinel_one.activity.data.role")?;
            }

            if event.has("json.data.roleName") {
                event.rename("json.data.roleName", "sentinel_one.activity.data.role_name")?;
            }

            if event.has("json.data.scopeLevelName") {
                event.rename(
                    "json.data.scopeLevelName",
                    "sentinel_one.activity.data.scope_level.name",
                )?;
            }

            if event.has("json.data.userScope") {
                event.rename(
                    "json.data.userScope",
                    "sentinel_one.activity.data.user.scope",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.newValue") {
                    if let Some(val) = event.get("json.data.newValue") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.newValue".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.activity.data.new.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_data_newValue_to_sentinel_one_activity_data_new_value_1fbcd0ff",
                )?;
                event.remove("json.data.newValue");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                if event.has_value("json.data.externalIp") {
                    if let Some(val) = event.get("json.data.externalIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.externalIp".into(),
                                message,
                            }
                        })?;
                        event.set("json.data.externalIp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_data_externalIp_025d5d36",
                )?;
                event.remove("json.data.externalIp");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.data.externalIp") {
                if let Some(ip_str) = event.get_string("json.data.externalIp") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.ipAddress") {
                    if let Some(val) = event.get("json.data.ipAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.ipAddress".into(),
                                message,
                            }
                        })?;
                        event.set("json.data.ipAddress", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_data_ipAddress_f8eb5a18",
                )?;
                event.remove("json.data.ipAddress");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("host.geo") };
            if _cond {
                if event.has_value("json.data.ipAddress") {
                    if let Some(ip_str) = event.get_string("json.data.ipAddress") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("host.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("host.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("host.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("host.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("host.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("host.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("host.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("host.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.data.ipAddress")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.data.externalIp")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("json.data.ipAddress") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.data.ipAddress")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.data.externalIp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.data.externalIp")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.data.reason") {
                event.rename("json.data.reason", "sentinel_one.activity.data.reason")?;
            }

            if event.has("json.data.source") {
                event.rename("json.data.source", "sentinel_one.activity.data.source")?;
            }

            if event.has("json.data.recoveryEmail") {
                event.rename("json.data.recoveryEmail", "user.email")?;
            }

            if event.has("json.data.computerName") {
                event.rename("json.data.computerName", "host.name")?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.system") {
                    if let Some(val) = event.get("json.data.system") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.system".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.activity.data.system", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_data_system_to_sentinel_one_activity_data_system_860fc3c2",
                )?;
                event.remove("json.data.system");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.data.uuid") {
                event.rename("json.data.uuid", "sentinel_one.activity.data.uuid")?;
            }

            if event.has("json.data.group") {
                event.rename("json.data.group", "sentinel_one.activity.data.group")?;
            }

            if event.has("json.data.optionalGroups") {
                event.rename(
                    "json.data.optionalGroups",
                    "sentinel_one.activity.data.optionals_groups",
                )?;
            }

            let _cond = { event.has_value("json.data.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.createdAt") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.activity.data.created_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_data_createdAt_to_sentinel_one_activity_data_created_at_1c9f8b95")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.data.status") {
                event.rename("json.data.status", "sentinel_one.activity.data.status")?;
            }

            if event.has("json.data.fileContentHash") {
                event.rename("json.data.fileContentHash", "file.hash.sha1")?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.data.osFamily") {
                event.rename("json.data.osFamily", "host.os.family")?;
            }

            let _cond = { event.has_value("host.os.family") };
            if _cond {
                // Painless script
                // Source: String os_family = ctx.host.os.family.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_family.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String os_family = ctx.host.os.family.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_family.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
            }

            if event.has("json.data.confidenceLevel") {
                event.rename(
                    "json.data.confidenceLevel",
                    "sentinel_one.activity.data.confidence.level",
                )?;
            }

            if event.has("json.data.escapedMaliciousProcessArguments") {
                event.rename(
                    "json.data.escapedMaliciousProcessArguments",
                    "sentinel_one.activity.data.malicious.process.arguments",
                )?;
            }

            if event.has("json.data.fileDisplayName") {
                event.rename("json.data.fileDisplayName", "file.name")?;
            }

            if event.has("json.data.filePath") {
                event.rename("json.data.filePath", "file.path")?;
            }

            if event.has("json.data.threatClassification") {
                event.rename(
                    "json.data.threatClassification",
                    "sentinel_one.threat_classification.name",
                )?;
            }

            if event.has("json.data.threatClassificationSource") {
                event.rename(
                    "json.data.threatClassificationSource",
                    "sentinel_one.threat_classification.source",
                )?;
            }

            if event.has("json.data.globalStatus") {
                event.rename(
                    "json.data.globalStatus",
                    "sentinel_one.activity.data.global.status",
                )?;
            }

            if event.has("json.data.newStatus") {
                event.rename(
                    "json.data.newStatus",
                    "sentinel_one.activity.data.new.status",
                )?;
            }

            if event.has("json.data.originalStatus") {
                event.rename(
                    "json.data.originalStatus",
                    "sentinel_one.activity.data.original.status",
                )?;
            }

            if event.has("json.data.downloadUrl") {
                event.rename(
                    "json.data.downloadUrl",
                    "sentinel_one.activity.data.downloaded.url",
                )?;
            }

            if event.has("json.data.description") {
                event.rename(
                    "json.data.description",
                    "sentinel_one.activity.data.description",
                )?;
            }

            if event.has("json.data.policy") {
                event.rename("json.data.policy", "sentinel_one.activity.data.policy")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.policyName") {
                    if let Some(val) = event.get("json.data.policyName") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.policyName".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.activity.data.policy_name", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_data_policyName_to_sentinel_one_activity_data_policy_name_e54fc1d0")?;
                event.remove("json.data.policyName");
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.data.changedKeys") {
                event.rename(
                    "json.data.changedKeys",
                    "sentinel_one.activity.data.changed_keys",
                )?;
            }

            if event.has("json.data.newConfidenceLevel") {
                event.rename(
                    "json.data.newConfidenceLevel",
                    "sentinel_one.activity.data.new.confidence_level",
                )?;
            }

            if event.has("json.data.oldConfidenceLevel") {
                event.rename(
                    "json.data.oldConfidenceLevel",
                    "sentinel_one.activity.data.old.confidence_level",
                )?;
            }

            if event.has("json.data.attr") {
                event.rename("json.data.attr", "sentinel_one.activity.data.attr")?;
            }

            event.remove("json.data.accountId");
            event.remove("json.data.newValue");
            event.remove("json.data.ipAddress");
            event.remove("json.data.externalIp");
            event.remove("json.data.system");
            event.remove("json.data.policyName");

            if event.has("json.data.ruledescription") {
                event.rename(
                    "json.data.ruledescription",
                    "sentinel_one.activity.rule_description",
                )?;
            }

            if event.has("json.data.ruleid") {
                event.rename("json.data.ruleid", "sentinel_one.activity.rule_id")?;
            }

            if event.has("json.data.rulename") {
                event.rename("json.data.rulename", "sentinel_one.activity.rule_name")?;
            }

            if let Some(v) = event
                .get("sentinel_one.activity.rule_description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.activity.rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.activity.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has("json.data.severity") {
                event.rename("json.data.severity", "sentinel_one.activity.severity")?;
            }

            if event.has("json.data") {
                event.rename("json.data", "sentinel_one.activity.data.flattened")?;
            }

            event.remove("json");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
