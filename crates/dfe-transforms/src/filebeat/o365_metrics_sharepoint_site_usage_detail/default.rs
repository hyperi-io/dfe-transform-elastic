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
                event.has_value("error.sharepoint_site_usage_detail")
                    && !event.has_value("sharepoint_site_usage_detail")
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
                        .get("sharepoint_site_usage_detail")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.original", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(
                    event,
                    "sharepoint_site_usage_detail",
                    "o365.metrics.sharepoint.site.usage.detail",
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("sharepoint_site_usage_detail") };
            if _cond {
                event.remove("sharepoint_site_usage_detail");
            }

            let _cond = {
                event
                    .get("o365.metrics.sharepoint.site.usage.detail")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: String underscore(String s) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n  String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n  String result1 = /[\\ufeff]/.matcher(result).replaceAll('');\n  return /[()]/.matcher(result1).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.sharepoint.site.usage.detail.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.sharepoint.site.usage.detail = out;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String underscore(String s) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n  String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n  String result1 = /[\\ufeff]/.matcher(result).replaceAll('');\n  return /[()]/.matcher(result1).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.sharepoint.site.usage.detail.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.sharepoint.site.usage.detail = out;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.storage_allocated_byte") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.storage_allocated_byte",
                    "o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte")
                    && event
                        .get_str("o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event
                        .get("o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte".into(),
                            message,
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path:
                                "o365.metrics.sharepoint.site.usage.detail.storage_allocated.byte"
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

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.storage_used_byte") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.storage_used_byte",
                    "o365.metrics.sharepoint.site.usage.detail.storage_used.byte",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.storage_used.byte")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.storage_used.byte")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.sharepoint.site.usage.detail.storage_used.byte")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.sharepoint.site.usage.detail.storage_used.byte"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.storage_used.byte",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.storage_used.byte",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.storage_used.byte")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.sharepoint.site.usage.detail.storage_used.byte"
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

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.active_file_count") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.active_file_count",
                    "o365.metrics.sharepoint.site.usage.detail.active_file.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.active_file.count")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.active_file.count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.sharepoint.site.usage.detail.active_file.count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.sharepoint.site.usage.detail.active_file.count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.active_file.count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.active_file.count",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.active_file.count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.sharepoint.site.usage.detail.active_file.count"
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

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.file_count") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.file_count",
                    "o365.metrics.sharepoint.site.usage.detail.file.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.file.count")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.file.count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.sharepoint.site.usage.detail.file.count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.sharepoint.site.usage.detail.file.count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.file.count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.file.count",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.file.count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.sharepoint.site.usage.detail.file.count".into(),
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

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.page_view_count") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.page_view_count",
                    "o365.metrics.sharepoint.site.usage.detail.page_view.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.page_view.count")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.page_view.count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.sharepoint.site.usage.detail.page_view.count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.sharepoint.site.usage.detail.page_view.count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.page_view.count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.page_view.count",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.page_view.count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.sharepoint.site.usage.detail.page_view.count"
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

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.report_period") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.report_period",
                    "o365.metrics.sharepoint.site.usage.detail.report.period.day",
                )?;
            }

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.report_refresh_date") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.report_refresh_date",
                    "o365.metrics.sharepoint.site.usage.detail.report.refresh_date",
                )?;
            }

            if let Some(v) = event
                .get("o365.metrics.sharepoint.site.usage.detail.report.refresh_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) =
                    event.get("o365.metrics.sharepoint.site.usage.detail.last_activity_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.sharepoint.site.usage.detail.last_activity_date".into(),
                    });
                }
                if let Some(v) =
                    event.get("o365.metrics.sharepoint.site.usage.detail.owner_principal_name")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.sharepoint.site.usage.detail.owner_principal_name"
                            .into(),
                    });
                }
                if let Some(v) =
                    event.get("o365.metrics.sharepoint.site.usage.detail.report.refresh_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.sharepoint.site.usage.detail.report.refresh_date"
                            .into(),
                    });
                }
                if let Some(v) = event.get("o365.metrics.sharepoint.site.usage.detail.site_id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.sharepoint.site.usage.detail.site_id".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("o365.metrics.sharepoint.site.usage.detail.visited_page_count") {
                event.rename(
                    "o365.metrics.sharepoint.site.usage.detail.visited_page_count",
                    "o365.metrics.sharepoint.site.usage.detail.visited_page.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.visited_page.count")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.visited_page.count")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.sharepoint.site.usage.detail.visited_page.count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "o365.metrics.sharepoint.site.usage.detail.visited_page.count"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.visited_page.count",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.visited_page.count",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.visited_page.count")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.sharepoint.site.usage.detail.visited_page.count"
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
                event.has_value("o365.metrics.sharepoint.site.usage.detail.is_deleted")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.is_deleted")
                        != Some("")
            };
            if _cond {
                map_strings(
                    event,
                    "o365.metrics.sharepoint.site.usage.detail.is_deleted",
                    "o365.metrics.sharepoint.site.usage.detail.is_deleted",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.sharepoint.site.usage.detail.is_deleted")
                    && event.get_str("o365.metrics.sharepoint.site.usage.detail.is_deleted")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.sharepoint.site.usage.detail.is_deleted")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.sharepoint.site.usage.detail.is_deleted".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.sharepoint.site.usage.detail.is_deleted",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.sharepoint.site.usage.detail.is_deleted",
                    )?;
                    if event
                        .remove("o365.metrics.sharepoint.site.usage.detail.is_deleted")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.sharepoint.site.usage.detail.is_deleted".into(),
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
                event.append("event.type", json!("error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
