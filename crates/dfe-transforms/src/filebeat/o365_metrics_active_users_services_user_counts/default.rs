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
            event.set("ecs.version", json!("8.16.0"))?;

            let _cond = {
                event.has_value("error.active_users_services_user_counts")
                    && !event.has_value("active_users_services_user_counts")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("error message set and no data to process").to_string(),
                });
            }

            let _cond = {
                !event.has_value("event.original")
                    && (event.has_value("tags")
                        && (event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_original_event")),
                            serde_json::Value::String(s) => s.contains("preserve_original_event"),
                            _ => false,
                        })))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("active_users_services_user_counts")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.original", v)?;
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(
                    event,
                    "active_users_services_user_counts",
                    "o365.metrics.active.users.services.user.counts",
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
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

            let _cond = { event.has_value("active_users_services_user_counts") };
            if _cond {
                event.remove("active_users_services_user_counts");
            }

            let _cond = {
                event
                    .get("o365.metrics.active.users.services.user.counts")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: String underscore(String s) {\n  String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n  return /[\\ufeff]/.matcher(result).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.active.users.services.user.counts.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.active.users.services.user.counts = out;                   \n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String underscore(String s) {\n  String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n  return /[\\ufeff]/.matcher(result).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.active.users.services.user.counts.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.active.users.services.user.counts = out;                   \n"#
                    ),
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.exchange_active") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.exchange_active",
                    "o365.metrics.active.users.services.user.counts.exchange.active.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.exchange_inactive") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.exchange_inactive",
                    "o365.metrics.active.users.services.user.counts.exchange.inactive.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.office_365_active") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.office_365_active",
                    "o365.metrics.active.users.services.user.counts.office365.active.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.office_365_inactive")
            {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.office_365_inactive",
                    "o365.metrics.active.users.services.user.counts.office365.inactive.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.onedrive_active") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.onedrive_active",
                    "o365.metrics.active.users.services.user.counts.onedrive.active.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.onedrive_inactive") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.onedrive_inactive",
                    "o365.metrics.active.users.services.user.counts.onedrive.inactive.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.report_period") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.report_period",
                    "o365.metrics.active.users.services.user.counts.report.period.day",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.report_refresh_date")
            {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.report_refresh_date",
                    "o365.metrics.active.users.services.user.counts.report.refresh_date",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.sharepoint_active") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.sharepoint_active",
                    "o365.metrics.active.users.services.user.counts.sharepoint.active.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.sharepoint_inactive")
            {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.sharepoint_inactive",
                    "o365.metrics.active.users.services.user.counts.sharepoint.inactive.count",
                )?;
            }

            if event.has_value(
                "o365.metrics.active.users.services.user.counts.skype_for_business_active",
            ) {
                event.rename("o365.metrics.active.users.services.user.counts.skype_for_business_active", "o365.metrics.active.users.services.user.counts.skype_for_business.active.count")?;
            }

            if event.has_value(
                "o365.metrics.active.users.services.user.counts.skype_for_business_inactive",
            ) {
                event.rename("o365.metrics.active.users.services.user.counts.skype_for_business_inactive", "o365.metrics.active.users.services.user.counts.skype_for_business.inactive.count")?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.teams_active") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.teams_active",
                    "o365.metrics.active.users.services.user.counts.teams.active.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.teams_inactive") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.teams_inactive",
                    "o365.metrics.active.users.services.user.counts.teams.inactive.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.yammer_active") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.yammer_active",
                    "o365.metrics.active.users.services.user.counts.yammer.active.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.yammer_inactive") {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.yammer_inactive",
                    "o365.metrics.active.users.services.user.counts.yammer.inactive.count",
                )?;
            }

            if event.has_value("o365.metrics.active.users.services.user.counts.skype_for_business")
            {
                event.rename(
                    "o365.metrics.active.users.services.user.counts.skype_for_business",
                    "o365.metrics.active.users.services.user.counts.skype_for_business.count",
                )?;
            }

            if let Some(v) = event
                .get("o365.metrics.active.users.services.user.counts.report.refresh_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) =
                    event.get("o365.metrics.active.users.services.user.counts.report.refresh_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.active.users.services.user.counts.report.refresh_date"
                            .into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.remove("o365.metrics.active.users.services.user.counts.skype_for_business");

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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
                event.append_unique("event.kind", json!("pipeline_error"))?;
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
