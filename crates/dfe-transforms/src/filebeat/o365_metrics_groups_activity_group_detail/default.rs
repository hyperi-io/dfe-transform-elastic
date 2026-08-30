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
                event.rename("json", "o365.metrics.groups.activity.group.detail")?;
            }

            if event.has_value("o365.metrics.groups.activity.group.detail.report_period") {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.report_period",
                    "o365.metrics.groups.activity.group.detail.report.period.day",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.groups.activity.group.detail.report_refresh_date")
                    && event
                        .get_str("o365.metrics.groups.activity.group.detail.report_refresh_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "o365.metrics.groups.activity.group.detail.report_refresh_date",
                    ) {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "o365.metrics.groups.activity.group.detail.report_refresh_date".into(),
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
                        "date_o365.metrics_groups_activity_group_detail_report_refresh_date",
                    )?;
                    if event
                        .remove("o365.metrics.groups.activity.group.detail.report_refresh_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.groups.activity.group.detail.report_refresh_date"
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

            if event.has_value("o365.metrics.groups.activity.group.detail.report_refresh_date") {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.report_refresh_date",
                    "o365.metrics.groups.activity.group.detail.report.refresh_date",
                )?;
            }

            if let Some(v) = event
                .get("o365.metrics.groups.activity.group.detail.report.refresh_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("o365.metrics.groups.activity.group.detail.last_activity_date")
                    && event.get_str("o365.metrics.groups.activity.group.detail.last_activity_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "o365.metrics.groups.activity.group.detail.last_activity_date",
                    ) {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], Some("UTC"), None) {
                            Some(parsed) => event.set(
                                "o365.metrics.groups.activity.group.detail.last_activity_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "o365.metrics.groups.activity.group.detail.last_activity_date".into(),
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
                        "date_o365.metrics_groups_activity_group_detail_last_activity_date",
                    )?;
                    if event
                        .remove("o365.metrics.groups.activity.group.detail.last_activity_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.groups.activity.group.detail.last_activity_date"
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
                event.has_value("o365.metrics.groups.activity.group.detail.is_deleted")
                    && event.get_str("o365.metrics.groups.activity.group.detail.is_deleted")
                        != Some("")
            };
            if _cond {
                map_strings(
                    event,
                    "o365.metrics.groups.activity.group.detail.is_deleted",
                    "o365.metrics.groups.activity.group.detail.is_deleted",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.groups.activity.group.detail.is_deleted")
                    && event.get_str("o365.metrics.groups.activity.group.detail.is_deleted")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.groups.activity.group.detail.is_deleted")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.groups.activity.group.detail.is_deleted".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.is_deleted",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics_groups_activity_group_detail_is_deleted",
                    )?;
                    if event
                        .remove("o365.metrics.groups.activity.group.detail.is_deleted")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.groups.activity.group.detail.is_deleted".into(),
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
                event.has_value("o365.metrics.groups.activity.group.detail.owner_principal_name")
            };
            if _cond {
                if let Some(v) = event
                    .get("o365.metrics.groups.activity.group.detail.owner_principal_name")
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

            let _cond = { event.has_value("o365.metrics.groups.activity.group.detail.group_id") };
            if _cond {
                if let Some(v) = event
                    .get("o365.metrics.groups.activity.group.detail.group_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.id", v)?;
                }
            }

            if let Some(v) = event
                .get("group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.id", v)?;
            }

            let _cond =
                { event.has_value("o365.metrics.groups.activity.group.detail.group_display_name") };
            if _cond {
                if let Some(v) = event
                    .get("o365.metrics.groups.activity.group.detail.group_display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
            }

            if let Some(v) = event
                .get("group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.name", v)?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) =
                    event.get("o365.metrics.groups.activity.group.detail.group_display_name")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.groups.activity.group.detail.group_display_name".into(),
                    });
                }
                if let Some(v) = event.get("o365.metrics.groups.activity.group.detail.group_id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.groups.activity.group.detail.group_id".into(),
                    });
                }
                if let Some(v) =
                    event.get("o365.metrics.groups.activity.group.detail.last_activity_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.groups.activity.group.detail.last_activity_date".into(),
                    });
                }
                if let Some(v) =
                    event.get("o365.metrics.groups.activity.group.detail.report.refresh_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.groups.activity.group.detail.report.refresh_date"
                            .into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("o365.metrics.groups.activity.group.detail.member_count")
                    && event.get_str("o365.metrics.groups.activity.group.detail.member_count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.groups.activity.group.detail.member_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.groups.activity.group.detail.member_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.member_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365_reports_m365 _groups_activity_group_member_count",
                    )?;
                    if event
                        .remove("o365.metrics.groups.activity.group.detail.member_count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.groups.activity.group.detail.member_count".into(),
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

            if event.has_value("o365.metrics.groups.activity.group.detail.member_count") {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.member_count",
                    "o365.metrics.groups.activity.group.detail.member.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.groups.activity.group.detail.external_member_count")
                    && event
                        .get_str("o365.metrics.groups.activity.group.detail.external_member_count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.groups.activity.group.detail.external_member_count")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.external_member_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.external_member_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365_reports_m365 _groups_activity_group_external_member_count",
                    )?;
                    if event
                        .remove("o365.metrics.groups.activity.group.detail.external_member_count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.groups.activity.group.detail.external_member_count"
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

            if event.has_value("o365.metrics.groups.activity.group.detail.external_member_count") {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.external_member_count",
                    "o365.metrics.groups.activity.group.detail.external_member.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.exchange_received_email_count",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.exchange_received_email_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.groups.activity.group.detail.exchange_received_email_count",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.exchange_received_email_count".into(),
                            message,
                        })?;
                        event.set("o365.metrics.groups.activity.group.detail.exchange_received_email_count", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_exchange_received_email_count")?;
                    if event.remove("o365.metrics.groups.activity.group.detail.exchange_received_email_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.exchange_received_email_count".into() });
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
                "o365.metrics.groups.activity.group.detail.exchange_received_email_count",
            ) {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.exchange_received_email_count",
                    "o365.metrics.groups.activity.group.detail.exchange_received_email.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.sharepoint_active_file_count",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.sharepoint_active_file_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.groups.activity.group.detail.sharepoint_active_file_count",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.sharepoint_active_file_count".into(),
                            message,
                        })?;
                        event.set("o365.metrics.groups.activity.group.detail.sharepoint_active_file_count", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_sharepoint_active_file_count")?;
                    if event.remove("o365.metrics.groups.activity.group.detail.sharepoint_active_file_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.sharepoint_active_file_count".into() });
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
                .has_value("o365.metrics.groups.activity.group.detail.sharepoint_active_file_count")
            {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.sharepoint_active_file_count",
                    "o365.metrics.groups.activity.group.detail.sharepoint_active_file.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.yammer_liked_message_count",
                ) && event
                    .get_str("o365.metrics.groups.activity.group.detail.yammer_liked_message_count")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event
                        .get("o365.metrics.groups.activity.group.detail.yammer_liked_message_count")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.yammer_liked_message_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.yammer_liked_message_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_yammer_liked_message_count")?;
                    if event
                        .remove(
                            "o365.metrics.groups.activity.group.detail.yammer_liked_message_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.yammer_liked_message_count".into() });
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
                .has_value("o365.metrics.groups.activity.group.detail.yammer_liked_message_count")
            {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.yammer_liked_message_count",
                    "o365.metrics.groups.activity.group.detail.yammer_liked_message.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.yammer_posted_message_count",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.yammer_posted_message_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.groups.activity.group.detail.yammer_posted_message_count",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.yammer_posted_message_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.yammer_posted_message_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_yammer_posted_message_count")?;
                    if event
                        .remove(
                            "o365.metrics.groups.activity.group.detail.yammer_posted_message_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.yammer_posted_message_count".into() });
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
                .has_value("o365.metrics.groups.activity.group.detail.yammer_posted_message_count")
            {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.yammer_posted_message_count",
                    "o365.metrics.groups.activity.group.detail.yammer_posted_message.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.yammer_read_message_count",
                ) && event
                    .get_str("o365.metrics.groups.activity.group.detail.yammer_read_message_count")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event
                        .get("o365.metrics.groups.activity.group.detail.yammer_read_message_count")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.yammer_read_message_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.yammer_read_message_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_yammer_read_message_count")?;
                    if event
                        .remove(
                            "o365.metrics.groups.activity.group.detail.yammer_read_message_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.yammer_read_message_count".into() });
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
                .has_value("o365.metrics.groups.activity.group.detail.yammer_read_message_count")
            {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.yammer_read_message_count",
                    "o365.metrics.groups.activity.group.detail.yammer_read_message.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count".into(),
                            message,
                        })?;
                    event.set("o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_exchange_mailbox_total_item_count")?;
                    if event.remove("o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count".into() });
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
                "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count",
            ) {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item_count",
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_total_item.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get(
                        "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_sharepoint_total_file_count")?;
                    if event
                        .remove(
                            "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count".into() });
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
                .has_value("o365.metrics.groups.activity.group.detail.sharepoint_total_file_count")
            {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.sharepoint_total_file_count",
                    "o365.metrics.groups.activity.group.detail.sharepoint_total_file.count",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte".into(),
                            message,
                        })?;
                    event.set("o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_exchange_mailbox_storage_used_byte")?;
                    if event.remove("o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte".into() });
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
                "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte",
            ) {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used_byte",
                    "o365.metrics.groups.activity.group.detail.exchange_mailbox_storage_used.byte",
                )?;
            }

            let _cond = {
                event.has_value(
                    "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte",
                ) && event.get_str(
                    "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte".into(),
                            message,
                        })?;
                    event.set("o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte", converted)?;
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_o365_reports_m365 _groups_activity_group_sharepoint_site_storage_used_byte")?;
                    if event.remove("o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte").is_none() {
                            return Err(TransformError::FieldNotFound { path: "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte".into() });
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
                "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte",
            ) {
                event.rename(
                    "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used_byte",
                    "o365.metrics.groups.activity.group.detail.sharepoint_site_storage_used.byte",
                )?;
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
