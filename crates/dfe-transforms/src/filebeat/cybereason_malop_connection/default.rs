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

            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("connection")]))?;

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

            let _cond = { event.has_value("event.original") };
            if _cond {
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
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.suspicionsMap = ctx.json[1]; ctx.evidenceMap = ctx.json[2]; ctx.json = ctx.json[0];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.suspicionsMap = ctx.json[1]; ctx.evidenceMap = ctx.json[2]; ctx.json = ctx.json[0];"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_parse_different_message_object",
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.guidString") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.elementValues.dnsQuery.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.dnsQuery.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.dnsQuery.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.dnsQuery.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_dnsQuery_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.dnsQuery.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.dnsQuery.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_dnsQuery_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.dnsQuery.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.dnsQuery.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.dnsQuery.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.dnsQuery.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.dnsQuery.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.dnsQuery.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.dnsQuery.elementValues") {
                event.rename(
                    "json.elementValues.dnsQuery.elementValues",
                    "cybereason.malop_connection.element_values.dns_query.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.dnsQuery.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.dnsQuery.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.dnsQuery.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.element_values.dns_query.guessed_total",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_dnsQuery_guessedTotal_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.dnsQuery.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.dnsQuery.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.dnsQuery.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.element_values.dns_query.total_malicious",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_dnsQuery_totalMalicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.dnsQuery.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.dnsQuery.totalSuspicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.dnsQuery.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.element_values.dns_query.total_suspicious",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_dnsQuery_totalSuspicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.dnsQuery.totalValues") {
                    if let Some(val) = event.get("json.elementValues.dnsQuery.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.dnsQuery.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.element_values.dns_query.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_dnsQuery_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "process.real_user.id",
                                json!(
                                    event
                                        .get("_ingest._value.guid")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.guid")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerMachine_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerMachine_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "process.real_user.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerMachine.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerMachine.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.ownerMachine.elementValues") {
                event.rename(
                    "json.elementValues.ownerMachine.elementValues",
                    "cybereason.malop_connection.element_values.owner_machine.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerMachine.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_machine.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_guessedTotal_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerMachine.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_machine.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_totalMalicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerMachine.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_machine.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_totalSuspicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerMachine.totalValues") {
                    if let Some(val) = event.get("json.elementValues.ownerMachine.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerMachine.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.element_values.owner_machine.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerMachine_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.ownerProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerProcess.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerProcess.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerProcess_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerProcess.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerProcess_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerProcess.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerProcess.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.elementValues.ownerProcess.elementValues") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.elementValues.ownerProcess.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.elementValues.ownerProcess.elementValues") {
                event.rename(
                    "json.elementValues.ownerProcess.elementValues",
                    "cybereason.malop_connection.element_values.owner_process.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerProcess.guessedTotal") {
                    if let Some(val) = event.get("json.elementValues.ownerProcess.guessedTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerProcess.guessedTotal".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_process.guessed_total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_guessedTotal_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerProcess.totalMalicious") {
                    if let Some(val) = event.get("json.elementValues.ownerProcess.totalMalicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerProcess.totalMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_process.total_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_totalMalicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerProcess.totalSuspicious") {
                    if let Some(val) = event.get("json.elementValues.ownerProcess.totalSuspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerProcess.totalSuspicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_process.total_suspicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_totalSuspicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.elementValues.ownerProcess.totalValues") {
                    if let Some(val) = event.get("json.elementValues.ownerProcess.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.elementValues.ownerProcess.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.element_values.owner_process.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_totalValues_to_long",
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

            let _cond = { event.has_value("json.elementValues.ownerProcess.user") };
            if _cond {
                // Painless script
                // Source: def obj = ctx.json.elementValues.remove(\"ownerProcess.user\"); ctx.cybereason.malop_connection.element_values.owner_process_user= obj;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.elementValues.remove(\"ownerProcess.user\"); ctx.cybereason.malop_connection.element_values.owner_process_user= obj;"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementType") {
                                event.rename(
                                    "_ingest._value.elementType",
                                    "_ingest._value.element_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasMalops") {
                                    if let Some(val) = event.get("_ingest._value.hasMalops") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasMalops".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_malops", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerProcess_user_elementValues_hasMalops_to_boolean")?;
                                event.remove("_ingest._value.hasMalops");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.hasSuspicions") {
                                    if let Some(val) = event.get("_ingest._value.hasSuspicions") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hasSuspicions".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.has_suspicions", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_elementValues_ownerProcess_user_elementValues_hasSuspicions_to_boolean")?;
                                event.remove("_ingest._value.hasSuspicions");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.elementValues") {
                                event.rename(
                                    "_ingest._value.elementValues",
                                    "_ingest._value.object",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                        |event| {
                            if event.has_value("_ingest._value.simpleValues") {
                                event.rename(
                                    "_ingest._value.simpleValues",
                                    "_ingest._value.simple_values",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                        |event| {
                            event.remove("_ingest._value.hasMalops");
                            event.remove("_ingest._value.hasSuspicions");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value(
                "cybereason.malop_connection.element_values.owner_process_user.elementValues",
            ) {
                event.rename(
                    "cybereason.malop_connection.element_values.owner_process_user.elementValues",
                    "cybereason.malop_connection.element_values.owner_process_user.element_values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.guessedTotal",
                ) {
                    if let Some(val) = event.get("cybereason.malop_connection.element_values.owner_process_user.guessedTotal") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.malop_connection.element_values.owner_process_user.guessedTotal".into(),
                            message,
                        })?;
                    event.set("cybereason.malop_connection.element_values.owner_process_user.guessed_total", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_user_guessedTotal_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.totalMalicious",
                ) {
                    if let Some(val) = event.get("cybereason.malop_connection.element_values.owner_process_user.totalMalicious") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.malop_connection.element_values.owner_process_user.totalMalicious".into(),
                            message,
                        })?;
                    event.set("cybereason.malop_connection.element_values.owner_process_user.total_malicious", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_user_totalMalicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.totalSuspicious",
                ) {
                    if let Some(val) = event.get("cybereason.malop_connection.element_values.owner_process_user.totalSuspicious") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.malop_connection.element_values.owner_process_user.totalSuspicious".into(),
                            message,
                        })?;
                    event.set("cybereason.malop_connection.element_values.owner_process_user.total_suspicious", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_user_totalSuspicious_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cybereason.malop_connection.element_values.owner_process_user.totalValues",
                ) {
                    if let Some(val) = event.get(
                        "cybereason.malop_connection.element_values.owner_process_user.totalValues",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cybereason.malop_connection.element_values.owner_process_user.totalValues".into(),
                            message,
                        })?;
                        event.set("cybereason.malop_connection.element_values.owner_process_user.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_elementValues_ownerProcess_user_totalValues_to_long",
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

            event.remove(
                "cybereason.malop_connection.element_values.owner_process_user.guessedTotal",
            );
            event.remove(
                "cybereason.malop_connection.element_values.owner_process_user.totalMalicious",
            );
            event.remove(
                "cybereason.malop_connection.element_values.owner_process_user.totalSuspicious",
            );
            event.remove(
                "cybereason.malop_connection.element_values.owner_process_user.totalValues",
            );

            if event.has_value("evidenceMap") {
                event.rename("evidenceMap", "cybereason.malop_connection.evidence_map")?;
            }

            if event.has_value("json.filterData.groupByValue") {
                event.rename(
                    "json.filterData.groupByValue",
                    "cybereason.malop_connection.filter_data.group_by_value",
                )?;
            }

            if event.has_value("json.filterData.sortInGroupValue") {
                event.rename(
                    "json.filterData.sortInGroupValue",
                    "cybereason.malop_connection.filter_data.sort_in_group_value",
                )?;
            }

            if event.has_value("json.guidString") {
                event.rename("json.guidString", "cybereason.malop_connection.guid_string")?;
            }

            if let Some(v) = event
                .get("cybereason.malop_connection.guid_string")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.isMalicious") {
                    if let Some(val) = event.get("json.isMalicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.is_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isMalicious_to_boolean",
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

            if event.has_value("json.labelsIds") {
                event.rename("json.labelsIds", "cybereason.malop_connection.labels_ids")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.malicious") {
                    if let Some(val) = event.get("json.malicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.malicious".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_malicious_to_boolean",
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

            if event.has_value("json.malopPriority") {
                event.rename(
                    "json.malopPriority",
                    "cybereason.malop_connection.malop_priority",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.accessedByMalwareEvidence.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.accessedByMalwareEvidence.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.accessedByMalwareEvidence.totalValues"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.accessed_by_malware_evidence.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_accessedByMalwareEvidence_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.accessedByMalwareEvidence.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.accessedByMalwareEvidence.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_accessedByMalwareEvidence_values_to_boolean")?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.accessedByMalwareEvidence.values") {
                event.rename(
                    "json.simpleValues.accessedByMalwareEvidence.values",
                    "cybereason.malop_connection.simple_values.accessed_by_malware_evidence.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.aggregatedReceivedBytesCount.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.aggregatedReceivedBytesCount.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.aggregatedReceivedBytesCount.totalValues"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.aggregated_received_bytes_count.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_aggregatedReceivedBytesCount_totalValues_to_long",
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

            let _cond =
                { event.has_value("json.simpleValues.aggregatedReceivedBytesCount.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.aggregatedReceivedBytesCount.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_aggregatedReceivedBytesCount_values_to_long")?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.aggregatedReceivedBytesCount.values") {
                event.rename("json.simpleValues.aggregatedReceivedBytesCount.values", "cybereason.malop_connection.simple_values.aggregated_received_bytes_count.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.aggregatedTransmittedBytesCount.totalValues")
                {
                    if let Some(val) =
                        event.get("json.simpleValues.aggregatedTransmittedBytesCount.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.simpleValues.aggregatedTransmittedBytesCount.totalValues"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.aggregated_transmitted_bytes_count.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_aggregatedTransmittedBytesCount_totalValues_to_long",
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

            let _cond =
                { event.has_value("json.simpleValues.aggregatedTransmittedBytesCount.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.aggregatedTransmittedBytesCount.values",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                event.set("_ingest.on_failure_processor_tag", "convert_simpleValues_aggregatedTransmittedBytesCount_values_to_long")?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.aggregatedTransmittedBytesCount.values") {
                event.rename("json.simpleValues.aggregatedTransmittedBytesCount.values", "cybereason.malop_connection.simple_values.aggregated_transmitted_bytes_count.values")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.calculatedCreationTime.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.calculatedCreationTime.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.calculatedCreationTime.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.calculated_creation_time.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_calculatedCreationTime_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.calculatedCreationTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("json.simpleValues.calculatedCreationTime.values")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) = event.get_as_string("_ingest._value") {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event.set("_ingest._value", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_simpleValues_calculatedCreationTime_values",
                                    )?;
                                    event.remove("_ingest._value");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.simpleValues.calculatedCreationTime.values",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.simpleValues.calculatedCreationTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.calculatedCreationTime.values",
                        |event| {
                            if let Some(v) = event
                                .get("_ingest._value")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("@timestamp", v)?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.calculatedCreationTime.values") {
                event.rename(
                    "json.simpleValues.calculatedCreationTime.values",
                    "cybereason.malop_connection.simple_values.calculated_creation_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.direction.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.direction.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.direction.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.direction.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_direction_totalValues_to_long",
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

            if event.has_value("json.simpleValues.direction.values") {
                event.rename(
                    "json.simpleValues.direction.values",
                    "cybereason.malop_connection.simple_values.direction.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.elementDisplayName.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.elementDisplayName.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.elementDisplayName.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.element_display_name.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_elementDisplayName_totalValues_to_long",
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

            if event.has_value("json.simpleValues.elementDisplayName.values") {
                event.rename(
                    "json.simpleValues.elementDisplayName.values",
                    "cybereason.malop_connection.simple_values.element_display_name.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.endTime.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.endTime.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.endTime.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.end_time.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_endTime_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.endTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.simpleValues.endTime.values").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) = event.get_as_string("_ingest._value") {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event.set("_ingest._value", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_simpleValues_endTime_values",
                                    )?;
                                    event.remove("_ingest._value");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.simpleValues.endTime.values",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.simpleValues.endTime.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.endTime.values", |event| {
                        if let Some(v) = event
                            .get("_ingest._value")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("event.end", v)?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.endTime.values") {
                event.rename(
                    "json.simpleValues.endTime.values",
                    "cybereason.malop_connection.simple_values.end_time.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.group.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.group.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.group.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.group.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_group_totalValues_to_long",
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

            if event.has_value("json.simpleValues.group.values") {
                event.rename(
                    "json.simpleValues.group.values",
                    "cybereason.malop_connection.simple_values.group.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.localPort.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.localPort.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.localPort.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.local_port.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_localPort_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.localPort.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.localPort.values", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
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
                                "convert_simpleValues_localPort_values_to_long",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.localPort.values") {
                event.rename(
                    "json.simpleValues.localPort.values",
                    "cybereason.malop_connection.simple_values.local_port.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.portType.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.portType.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.portType.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.port_type.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_portType_totalValues_to_long",
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

            if event.has_value("json.simpleValues.portType.values") {
                event.rename(
                    "json.simpleValues.portType.values",
                    "cybereason.malop_connection.simple_values.port_type.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.remoteAddressCountryName.totalValues") {
                    if let Some(val) =
                        event.get("json.simpleValues.remoteAddressCountryName.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.remoteAddressCountryName.totalValues"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.remote_address_country_name.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_remoteAddressCountryName_totalValues_to_long",
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

            if event.has_value("json.simpleValues.remoteAddressCountryName.values") {
                event.rename(
                    "json.simpleValues.remoteAddressCountryName.values",
                    "cybereason.malop_connection.simple_values.remote_address_country_name.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.remotePort.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.remotePort.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.remotePort.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.remote_port.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_remotePort_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.remotePort.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.remotePort.values", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
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
                                "convert_simpleValues_remotePort_values_to_long",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.remotePort.values") {
                event.rename(
                    "json.simpleValues.remotePort.values",
                    "cybereason.malop_connection.simple_values.remote_port.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.serverAddress.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.serverAddress.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.serverAddress.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.server_address.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_serverAddress_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.serverAddress.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.serverAddress.values", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
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
                                "convert_simpleValues_serverAddress_values_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.simpleValues.serverAddress.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.serverAddress.values", |event| {
                        event.append_unique(
                            "server.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.simpleValues.serverAddress.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.serverAddress.values", |event| {
                        if let Some(v) = event
                            .get("_ingest._value")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("server.ip", v)?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.simpleValues.serverAddress.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.serverAddress.values", |event| {
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
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.serverAddress.values") {
                event.rename(
                    "json.simpleValues.serverAddress.values",
                    "cybereason.malop_connection.simple_values.server_address.values",
                )?;
            }

            let _cond = {
                event.has_value("cybereason.malop_connection.simple_values.server_address.values")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.simple_values.server_address.values",
                        |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.serverPort.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.serverPort.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.serverPort.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.server_port.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_serverPort_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.serverPort.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.serverPort.values", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
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
                                "convert_simpleValues_serverPort_values_to_long",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.simpleValues.serverPort.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.simpleValues.serverPort.values", |event| {
                        if let Some(v) = event
                            .get("_ingest._value")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("server.port", v)?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.serverPort.values") {
                event.rename(
                    "json.simpleValues.serverPort.values",
                    "cybereason.malop_connection.simple_values.server_port.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.state.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.state.totalValues") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.state.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.malop_connection.simple_values.state.total_values",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_state_totalValues_to_long",
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

            if event.has_value("json.simpleValues.state.values") {
                event.rename(
                    "json.simpleValues.state.values",
                    "cybereason.malop_connection.simple_values.state.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.simpleValues.transportProtocol.totalValues") {
                    if let Some(val) = event.get("json.simpleValues.transportProtocol.totalValues")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.simpleValues.transportProtocol.totalValues".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.simple_values.transport_protocol.total_values", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_simpleValues_transportProtocol_totalValues_to_long",
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

            let _cond = { event.has_value("json.simpleValues.transportProtocol.values") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.simpleValues.transportProtocol.values",
                        |event| {
                            if let Some(v) = event
                                .get("_ingest._value")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("network.transport", v)?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.simpleValues.transportProtocol.values") {
                event.rename(
                    "json.simpleValues.transportProtocol.values",
                    "cybereason.malop_connection.simple_values.transport_protocol.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.suspect") {
                    if let Some(val) = event.get("json.suspect") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.suspect".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.suspect", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_suspect_to_boolean",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.suspicionCount") {
                    if let Some(val) = event.get("json.suspicionCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.suspicionCount".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.malop_connection.suspicion_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_suspicionCount_to_long",
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

            if event.has_value("json.suspicions") {
                event.rename("json.suspicions", "cybereason.malop_connection.suspicions")?;
            }

            if event.has_value("suspicionsMap") {
                event.rename(
                    "suspicionsMap",
                    "cybereason.malop_connection.suspicions_map",
                )?;
            }

            let _cond = {
                event.has_value(
                    "cybereason.malop_connection.element_values.owner_machine.element_values",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "cybereason.malop_connection.element_values.owner_machine.element_values",
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
                                event.remove("_ingest._value.guid");
                                event.remove("_ingest._value.name");
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
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
                event.remove("cybereason.malop_connection.simple_values.server_address.values");
                event.remove("cybereason.malop_connection.simple_values.server_port.values");
                event.remove("cybereason.malop_connection.simple_values.transport_protocol.values");
                event.remove(
                    "cybereason.malop_connection.simple_values.calculated_creation_time.values",
                );
                event.remove("cybereason.malop_connection.simple_values.end_time.values");
                event.remove("cybereason.malop_connection.guid_string");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
