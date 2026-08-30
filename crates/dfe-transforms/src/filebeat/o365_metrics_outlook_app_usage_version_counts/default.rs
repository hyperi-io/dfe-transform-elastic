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
                if let Some(v) = event
                    .get("message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.original", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(
                    event,
                    "message",
                    "o365.metrics.outlook.app.usage.version.counts",
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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

            let _cond = { event.has_value("message") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                event
                    .get("o365.metrics.outlook.app.usage.version.counts")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: String underscore(String s) {\n  String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n  return /[\\ufeff]/.matcher(result).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.outlook.app.usage.version.counts.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.outlook.app.usage.version.counts = out;           \n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String underscore(String s) {\n  String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n  return /[\\ufeff]/.matcher(result).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.outlook.app.usage.version.counts.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.outlook.app.usage.version.counts = out;           \n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2007")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.outlook_2007")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.outlook_2007")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.outlook_2007"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.outlook_2007",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.outlook_2007",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.outlook_2007")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.outlook_2007"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2007") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2007",
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2007.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2010")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.outlook_2010")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.outlook_2010")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.outlook_2010"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.outlook_2010",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.outlook_2010",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.outlook_2010")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.outlook_2010"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2010") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2010",
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2010.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2013")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.outlook_2013")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.outlook_2013")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.outlook_2013"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.outlook_2013",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.outlook_2013",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.outlook_2013")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.outlook_2013"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2013") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2013",
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2013.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2016")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.outlook_2016")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.outlook_2016")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.outlook_2016"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.outlook_2016",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.outlook_2016",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.outlook_2016")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.outlook_2016"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2016") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2016",
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2016.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2019")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.outlook_2019")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.outlook_2019")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.outlook_2019"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.outlook_2019",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.outlook_2019",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.outlook_2019")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.outlook_2019"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_2019") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2019",
                    "o365.metrics.outlook.app.usage.version.counts.outlook_2019.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_m365")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.outlook_m365")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.outlook_m365")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.outlook_m365"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.outlook_m365",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.outlook_m365",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.outlook_m365")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.outlook_m365"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.outlook_m365") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.outlook_m365",
                    "o365.metrics.outlook.app.usage.version.counts.outlook_m365.count",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.version.counts.undetermined")
                    && event.get_str("o365.metrics.outlook.app.usage.version.counts.undetermined")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("o365.metrics.outlook.app.usage.version.counts.undetermined")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.undetermined"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "o365.metrics.outlook.app.usage.version.counts.undetermined",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365.metrics.outlook.app.usage.version.counts.undetermined",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.undetermined")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "o365.metrics.outlook.app.usage.version.counts.undetermined"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.undetermined") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.undetermined",
                    "o365.metrics.outlook.app.usage.version.counts.undetermined.count",
                )?;
            }

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.report_period") {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.report_period",
                    "o365.metrics.outlook.app.usage.version.counts.report.period.day",
                )?;
            }

            let _cond = {
                event.has_value("o365.metrics.outlook.app.usage.report_refresh_date")
                    && event.get_str("o365.metrics.outlook.app.usage.report_refresh_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "o365.metrics.outlook.app.usage.version.counts.report_refresh_date",
                    ) {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "o365.metrics.outlook.app.usage.version.counts.report_refresh_date".into(),
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
                        "date_o365.metrics.outlook.app.usage.version.counts.report_refresh_date",
                    )?;
                    if event
                        .remove("o365.metrics.outlook.app.usage.version.counts.report_refresh_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path:
                                "o365.metrics.outlook.app.usage.version.counts.report_refresh_date"
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

            if event.has_value("o365.metrics.outlook.app.usage.version.counts.report_refresh_date")
            {
                event.rename(
                    "o365.metrics.outlook.app.usage.version.counts.report_refresh_date",
                    "o365.metrics.outlook.app.usage.version.counts.report.refresh_date",
                )?;
            }

            if let Some(v) = event
                .get("o365.metrics.outlook.app.usage.version.counts.report.refresh_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) =
                    event.get("o365.metrics.outlook.app.usage.version.counts.report.refresh_date")
                {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "o365.metrics.outlook.app.usage.version.counts.report.refresh_date"
                            .into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append_unique("event.kind", json!("pipeline_error"))?;
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
