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
            let _cond = { event.get_str("message") == Some("want_more") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.16.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("error message set and no data to process").to_string(),
                });
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

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("json.report.name") {
                event.rename("json.report.name", "o365.metrics.report.name")?;
            }

            if event.has_value("json.report.api_path") {
                event.rename("json.report.api_path", "o365.metrics.report.api_path")?;
            }

            let _cond = { event.get("json").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: String sanitize(String s) {\n  String t = /[ -]/.matcher(s).replaceAll('_');\n  return /[\\(\\)]/.matcher(t).replaceAll('').toLowerCase();\n}\n\ndef out = [:];\nfor (def item : ctx.json.entrySet()) {\n  // Remove control characters from CSV header\n  String key = /\\p{C}/.matcher(item.getKey()).replaceAll('');\n  // Replace spaces and hyphens with sanitize and convert to lowercase.\n  out[sanitize(key)] = item.getValue();\n}\nctx.json = out;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String sanitize(String s) {\n  String t = /[ -]/.matcher(s).replaceAll('_');\n  return /[\\(\\)]/.matcher(t).replaceAll('').toLowerCase();\n}\n\ndef out = [:];\nfor (def item : ctx.json.entrySet()) {\n  // Remove control characters from CSV header\n  String key = /\\p{C}/.matcher(item.getKey()).replaceAll('');\n  // Replace spaces and hyphens with sanitize and convert to lowercase.\n  out[sanitize(key)] = item.getValue();\n}\nctx.json = out;\n"#
                    ),
                )?;
            }

            if event.has_value("json") {
                event.rename("json", "o365.metrics.teams.user.activity.user.detail")?;
            }

            if event.has_value("o365.metrics.teams.user.activity.user.detail.report_period") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.report_period",
                    "o365.metrics.teams.user.activity.user.detail.report.period.day",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.report_refresh_date")
                    && event
                        .get_str("o365.metrics.teams.user.activity.user.detail.report_refresh_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "o365.metrics.teams.user.activity.user.detail.report_refresh_date",
                    ) {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.report_refresh_date".into(),
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
                        "date_o365.metrics_teams_user_activity_user_detail_report_refresh_date",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.report_refresh_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path:
                                "o365.metrics.teams.user.activity.user.detail.report_refresh_date"
                                    .into(),
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.report_refresh_date") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.report_refresh_date",
                    "o365.metrics.teams.user.activity.user.detail.report.refresh_date",
                )?;
            }

            if let Some(v) = event
                .get("o365.metrics.teams.user.activity.user.detail.report.refresh_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.last_activity_date")
                    && event
                        .get_str("o365.metrics.teams.user.activity.user.detail.last_activity_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "o365.metrics.teams.user.activity.user.detail.last_activity_date",
                    ) {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], Some("UTC"), None) {
                            Some(parsed) => event.set(
                                "o365.metrics.teams.user.activity.user.detail.last_activity_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.last_activity_date".into(),
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
                        "date_o365.metrics_teams_user_activity_user_detail_last_activity_date",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.last_activity_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.last_activity_date"
                                .into(),
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

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.is_deleted")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.is_deleted")
                        != Some("")
            };
            if _cond {
                map_strings(
                    event,
                    "o365.metrics.teams.user.activity.user.detail.is_deleted",
                    "o365.metrics.teams.user.activity.user.detail.is_deleted",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.is_deleted")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.is_deleted")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.is_deleted")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.is_deleted"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.is_deleted",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_is_deleted",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.is_deleted")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.is_deleted".into(),
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

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.deleted_date")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.deleted_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("o365.metrics.teams.user.activity.user.detail.deleted_date")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], Some("UTC"), None) {
                            Some(parsed) => event.set(
                                "o365.metrics.teams.user.activity.user.detail.deleted_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "o365.metrics.teams.user.activity.user.detail.deleted_date"
                                            .into(),
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
                        "date_o365.metrics_teams_user_activity_user_detail_deleted_date",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.deleted_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.deleted_date"
                                .into(),
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

            if let Some(v) = event
                .get("o365.metrics.teams.user.activity.user.detail.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.user_principal_name")
            };
            if _cond {
                if let Some(v) = event
                    .get("o365.metrics.teams.user.activity.user.detail.user_principal_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
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

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) =
                    event.get("o365.metrics.teams.user.activity.user.detail.last_activity_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.teams.user.activity.user.detail.last_activity_date"
                            .into(),
                    });
                }
                if let Some(v) =
                    event.get("o365.metrics.teams.user.activity.user.detail.report.refresh_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.teams.user.activity.user.detail.report.refresh_date"
                            .into(),
                    });
                }
                if let Some(v) = event.get("o365.metrics.teams.user.activity.user.detail.user_id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.teams.user.activity.user.detail.user_id".into(),
                    });
                }
                if let Some(v) =
                    event.get("o365.metrics.teams.user.activity.user.detail.user_principal_name")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.teams.user.activity.user.detail.user_principal_name"
                            .into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.team_chat_message_count",
                ) && event
                    .get_str("o365.metrics.teams.user.activity.user.detail.team_chat_message_count")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event
                        .get("o365.metrics.teams.user.activity.user.detail.team_chat_message_count")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.team_chat_message_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.team_chat_message_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_team_chat_message_count")?;
                    if event
                        .remove(
                            "o365.metrics.teams.user.activity.user.detail.team_chat_message_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.team_chat_message_count".into() });
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

            if event
                .has_value("o365.metrics.teams.user.activity.user.detail.team_chat_message_count")
            {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.team_chat_message_count",
                    "o365.metrics.teams.user.activity.user.detail.team_chat_message.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.private_chat_message_count",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.private_chat_message_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.teams.user.activity.user.detail.private_chat_message_count",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.private_chat_message_count".into(),
                            message,
                        })?;
                        event.set("o365.metrics.teams.user.activity.user.detail.private_chat_message_count", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_private_chat_message_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.private_chat_message_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.private_chat_message_count".into() });
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

            if event.has_value(
                "o365.metrics.teams.user.activity.user.detail.private_chat_message_count",
            ) {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.private_chat_message_count",
                    "o365.metrics.teams.user.activity.user.detail.private_chat_message.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.call_count")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.call_count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.call_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.call_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.call_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_call_count",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.call_count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.call_count".into(),
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.call_count") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.call_count",
                    "o365.metrics.teams.user.activity.user.detail.call.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.meeting_count")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.meeting_count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.meeting_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.meeting_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.meeting_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_meeting_count",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.meeting_count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.meeting_count"
                                .into(),
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.meeting_count") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.meeting_count",
                    "o365.metrics.teams.user.activity.user.detail.meeting.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.meetings_organized_count",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.meetings_organized_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.teams.user.activity.user.detail.meetings_organized_count",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.meetings_organized_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.meetings_organized_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_meetings_organized_count")?;
                    if event
                        .remove(
                            "o365.metrics.teams.user.activity.user.detail.meetings_organized_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.meetings_organized_count".into() });
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

            if event
                .has_value("o365.metrics.teams.user.activity.user.detail.meetings_organized_count")
            {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.meetings_organized_count",
                    "o365.metrics.teams.user.activity.user.detail.meetings_organized.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.meetings_attended_count",
                ) && event
                    .get_str("o365.metrics.teams.user.activity.user.detail.meetings_attended_count")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event
                        .get("o365.metrics.teams.user.activity.user.detail.meetings_attended_count")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.meetings_attended_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.meetings_attended_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_meetings_attended_count")?;
                    if event
                        .remove(
                            "o365.metrics.teams.user.activity.user.detail.meetings_attended_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.meetings_attended_count".into() });
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

            if event
                .has_value("o365.metrics.teams.user.activity.user.detail.meetings_attended_count")
            {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.meetings_attended_count",
                    "o365.metrics.teams.user.activity.user.detail.meetings_attended.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_ad_hoc_meetings_organized_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count".into() });
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

            if event.has_value(
                "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count",
            ) {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized_count",
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_organized.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_ad_hoc_meetings_attended_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count".into() });
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

            if event.has_value(
                "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count",
            ) {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended_count",
                    "o365.metrics.teams.user.activity.user.detail.ad_hoc_meetings_attended.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count") && event.get_str("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_scheduled_one_time_meetings_organized_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count".into() });
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count") {
                    event.rename("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized_count", "o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_organized.count")?;
                }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count") && event.get_str("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_scheduled_one_time_meetings_attended_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count".into() });
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count") {
                    event.rename("o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended_count", "o365.metrics.teams.user.activity.user.detail.scheduled_one_time_meetings_attended.count")?;
                }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count") && event.get_str("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_scheduled_recurring_meetings_organized_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count".into() });
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count") {
                    event.rename("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized_count", "o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_organized.count")?;
                }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count") && event.get_str("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_scheduled_recurring_meetings_attended_count")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count".into() });
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count") {
                    event.rename("o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended_count", "o365.metrics.teams.user.activity.user.detail.scheduled_recurring_meetings_attended.count")?;
                }

            if event.has_value("o365.metrics.teams.user.activity.user.detail.audio_duration") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.audio_duration",
                    "o365.metrics.teams.user.activity.user.detail.audio_duration.formatted",
                )?;
            }

            if event.has_value("o365.metrics.teams.user.activity.user.detail.video_duration") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.video_duration",
                    "o365.metrics.teams.user.activity.user.detail.video_duration.formatted",
                )?;
            }

            if event.has_value("o365.metrics.teams.user.activity.user.detail.screen_share_duration")
            {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.screen_share_duration",
                    "o365.metrics.teams.user.activity.user.detail.screen_share_duration.formatted",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds".into(),
                            message,
                        })?;
                        event.set("o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_audio_duration_in_seconds")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds".into() });
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

            if event
                .has_value("o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds")
            {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.audio_duration_in_seconds",
                    "o365.metrics.teams.user.activity.user.detail.audio_duration.seconds",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds".into(),
                            message,
                        })?;
                        event.set("o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_video_duration_in_seconds")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds".into() });
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

            if event
                .has_value("o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds")
            {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.video_duration_in_seconds",
                    "o365.metrics.teams.user.activity.user.detail.video_duration.seconds",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds",
                ) && event.get_str(
                    "o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds".into(),
                            message,
                        })?;
                    event.set("o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365.metrics_teams_user_activity_user_detail_screen_share_duration_in_seconds")?;
                    if event.remove("o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds".into() });
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

            if event.has_value(
                "o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds",
            ) {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.screen_share_duration_in_seconds",
                    "o365.metrics.teams.user.activity.user.detail.screen_share_duration.seconds",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.urgent_messages")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.urgent_messages")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.urgent_messages")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "o365.metrics.teams.user.activity.user.detail.urgent_messages"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.urgent_messages",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_urgent_messages",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.urgent_messages")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.urgent_messages"
                                .into(),
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.urgent_messages") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.urgent_messages",
                    "o365.metrics.teams.user.activity.user.detail.urgent_messages.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.post_messages")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.post_messages")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.post_messages")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.post_messages"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.post_messages",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_post_messages",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.post_messages")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.post_messages"
                                .into(),
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.post_messages") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.post_messages",
                    "o365.metrics.teams.user.activity.user.detail.post_messages.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.reply_messages")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.reply_messages")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.reply_messages")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.reply_messages"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.reply_messages",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_reply_messages",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.reply_messages")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.reply_messages"
                                .into(),
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

            if event.has_value("o365.metrics.teams.user.activity.user.detail.reply_messages") {
                event.rename(
                    "o365.metrics.teams.user.activity.user.detail.reply_messages",
                    "o365.metrics.teams.user.activity.user.detail.reply_messages.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.is_licensed")
                    && event
                        .get_str("o365.metrics.teams.user.activity.user.detail.is_licensed")
                        .is_some_and(|s| s.to_lowercase() == "yes")
            };
            if _cond {
                let v = json!("true");
                if !painless_is_empty_value(&v) {
                    event.set(
                        "o365.metrics.teams.user.activity.user.detail.is_licensed",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.is_licensed")
                    && event
                        .get_str("o365.metrics.teams.user.activity.user.detail.is_licensed")
                        .is_some_and(|s| s.to_lowercase() == "no")
            };
            if _cond {
                let v = json!("false");
                if !painless_is_empty_value(&v) {
                    event.set(
                        "o365.metrics.teams.user.activity.user.detail.is_licensed",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("o365.metrics.teams.user.activity.user.detail.is_licensed")
                    && event.get_str("o365.metrics.teams.user.activity.user.detail.is_licensed")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.teams.user.activity.user.detail.is_licensed")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.teams.user.activity.user.detail.is_licensed"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.teams.user.activity.user.detail.is_licensed",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_teams_user_activity_user_detail_is_licensed",
                    )?;
                    if event
                        .remove("o365.metrics.teams.user.activity.user.detail.is_licensed")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.teams.user.activity.user.detail.is_licensed".into(),
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

            event.remove("json");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_remove_null_values",
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
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
