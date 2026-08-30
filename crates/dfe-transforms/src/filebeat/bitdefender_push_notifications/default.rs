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
                !event.has_value("json")
                    && event.has_value("message")
                    && event.get_str("message") != Some("")
            };
            if _cond {
                parse_json_field(event, "message", "bitdefender.event")?;
            }

            let _cond = { event.has_value("json") && !event.has_value("json.event") };
            if _cond {
                event.rename("json", "bitdefender.event")?;
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                event.rename("json.event", "bitdefender.event")?;
            }

            let _cond = { event.has_value("json.jsonrpc.version") };
            if _cond {
                event.rename("json.jsonrpc.version", "bitdefender.event.jsonrpc.version")?;
            }

            let _cond = { event.has_value("json.jsonrpc.method") };
            if _cond {
                event.rename("json.jsonrpc.method", "bitdefender.event.jsonrpc.method")?;
            }

            let _cond = { event.has_value("json.id") };
            if _cond {
                event.rename("json.id", "bitdefender.event.id")?;
            }

            let _cond = {
                event.has_value("_tmp.bitdefender_id")
                    && event.get_str("_tmp.bitdefender_id") != Some("")
            };
            if _cond {
                event.rename("_tmp.bitdefender_id", "bitdefender.id")?;
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

            if event.has_value("bitdefender.event.status") {
                if let Some(val) = event.get("bitdefender.event.status") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.status".into(),
                            message,
                        }
                    })?;
                    event.set("bitdefender.event.status", converted)?;
                }
            }

            if event.has_value("bitdefender.event.main_action") {
                if let Some(val) = event.get("bitdefender.event.main_action") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.main_action".into(),
                            message,
                        }
                    })?;
                    event.set("bitdefender.event.main_action", converted)?;
                }
            }

            if event.has_value("bitdefender.event.detected_on") {
                if let Some(val) = event.get("bitdefender.event.detected_on") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.detected_on".into(),
                            message,
                        }
                    })?;
                    event.set("bitdefender.event.detected_on", converted)?;
                }
            }

            if event.has_value("bitdefender.event.detectionTime") {
                if let Some(val) = event.get("bitdefender.event.detectionTime") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.detectionTime".into(),
                            message,
                        }
                    })?;
                    event.set("bitdefender.event.detectionTime", converted)?;
                }
            }

            if event.has_value("bitdefender.event.attack_entry") {
                if let Some(val) = event.get("bitdefender.event.attack_entry") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.attack_entry".into(),
                            message,
                        }
                    })?;
                    event.set("bitdefender.event.attack_entry", converted)?;
                }
            }

            if event.has_value("bitdefender.event.taskType") {
                if let Some(val) = event.get("bitdefender.event.taskType") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.taskType".into(),
                            message,
                        }
                    })?;
                    event.set("bitdefender.event.taskType", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("bitdefender.event.auto_renew_period") {
                    if let Some(val) = event.get("bitdefender.event.auto_renew_period") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.auto_renew_period".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.auto_renew_period", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_auto_renew_period_to_long",
                )?;
                event.remove("bitdefender.event.auto_renew_period");
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
                if event.has_value("bitdefender.event.blocked") {
                    if let Some(val) = event.get("bitdefender.event.blocked") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.blocked".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.blocked", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_blocked_to_long",
                )?;
                event.remove("bitdefender.event.blocked");
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
                if event.has_value("bitdefender.event.cleaned") {
                    if let Some(val) = event.get("bitdefender.event.cleaned") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.cleaned".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.cleaned", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cleaned_to_long",
                )?;
                event.remove("bitdefender.event.cleaned");
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
                if event.has_value("bitdefender.event.deleted") {
                    if let Some(val) = event.get("bitdefender.event.deleted") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.deleted".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.deleted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_deleted_to_long",
                )?;
                event.remove("bitdefender.event.deleted");
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
                if event.has_value("bitdefender.event.ignored") {
                    if let Some(val) = event.get("bitdefender.event.ignored") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.ignored".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.ignored", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ignored_to_long",
                )?;
                event.remove("bitdefender.event.ignored");
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
                if event.has_value("bitdefender.event.deviceClass") {
                    if let Some(val) = event.get("bitdefender.event.deviceClass") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.deviceClass".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.deviceClass", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_deviceClass_to_int",
                )?;
                event.remove("bitdefender.event.deviceClass");
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
                if event.has_value("bitdefender.event.license_limit") {
                    if let Some(val) = event.get("bitdefender.event.license_limit") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.license_limit".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.license_limit", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_license_limit_to_long",
                )?;
                event.remove("bitdefender.event.license_limit");
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
                if event.has_value("bitdefender.event.mailboxes") {
                    if let Some(val) = event.get("bitdefender.event.mailboxes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.mailboxes".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.mailboxes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_mailboxes_to_long",
                )?;
                event.remove("bitdefender.event.mailboxes");
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
                if event.has_value("bitdefender.event.minimal_commitment_usage_endpoints") {
                    if let Some(val) =
                        event.get("bitdefender.event.minimal_commitment_usage_endpoints")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.minimal_commitment_usage_endpoints".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "bitdefender.event.minimal_commitment_usage_endpoints",
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
                    "convert_minimal_commitment_usage_endpoints_to_long",
                )?;
                event.remove("bitdefender.event.minimal_commitment_usage_endpoints");
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
                if event.has_value("bitdefender.event.parent_process_id") {
                    if let Some(val) = event.get("bitdefender.event.parent_process_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.parent_process_id".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.parent_process_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_parent_process_id_to_long",
                )?;
                event.remove("bitdefender.event.parent_process_id");
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
                if event.has_value("bitdefender.event.present") {
                    if let Some(val) = event.get("bitdefender.event.present") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.present".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.present", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_present_to_long",
                )?;
                event.remove("bitdefender.event.present");
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
                if event.has_value("bitdefender.event.productId") {
                    if let Some(val) = event.get("bitdefender.event.productId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.productId".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.productId", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_productId_to_long",
                )?;
                event.remove("bitdefender.event.productId");
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
                if event.has_value("bitdefender.event.quarantined") {
                    if let Some(val) = event.get("bitdefender.event.quarantined") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.quarantined".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.quarantined", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_quarantined_to_long",
                )?;
                event.remove("bitdefender.event.quarantined");
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
                if event.has_value("bitdefender.event.vendorId") {
                    if let Some(val) = event.get("bitdefender.event.vendorId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.vendorId".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.vendorId", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_vendorId_to_long",
                )?;
                event.remove("bitdefender.event.vendorId");
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
                if event.has_value("bitdefender.event.storage_ip") {
                    if let Some(val) = event.get("bitdefender.event.storage_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.storage_ip".into(),
                                message,
                            }
                        })?;
                        event.set("bitdefender.event.storage_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_storage_ip_to_ip",
                )?;
                event.remove("bitdefender.event.storage_ip");
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
                event.has_value("bitdefender.event.lastAdReportDate")
                    && event.get_str("bitdefender.event.lastAdReportDate") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.lastAdReportDate") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.lastAdReportDate", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.lastAdReportDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.created")
                    && event.get_str("bitdefender.event.created") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.created") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.detectionTime")
                    && event.get_str("bitdefender.event.detectionTime") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.detectionTime") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.detectionTime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.detectionTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.detection_time")
                    && event.get_str("bitdefender.event.detection_time") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.detection_time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.detection_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.detection_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.detected_on")
                    && event.get_str("bitdefender.event.detected_on") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.detected_on") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.detected_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.detected_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.last_blocked")
                    && event.get_str("bitdefender.event.last_blocked") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.last_blocked") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.last_blocked", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.last_blocked".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.signature_update")
                    && event.get_str("bitdefender.event.signature_update") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.signature_update") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.signature_update", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.signature_update".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.timestamp")
                    && event.get_str("bitdefender.event.timestamp") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.date")
                    && event.get_str("bitdefender.event.date") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.date") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.startDate")
                    && event.get_str("bitdefender.event.startDate") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.startDate") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.startDate", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.startDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.endDate")
                    && event.get_str("bitdefender.event.endDate") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("bitdefender.event.endDate") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("bitdefender.event.endDate", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.endDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.end_subscription_date")
                    && event.get_str("bitdefender.event.end_subscription_date") != Some("")
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("bitdefender.event.end_subscription_date")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:' 'mm:ss.SSS'Z'",
                            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
                            "yyyy- MM - dd'T'HH: mm:ss.SSS'Z'",
                            "ISO8601",
                            "UNIX",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("bitdefender.event.end_subscription_date", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "bitdefender.event.end_subscription_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("bitdefender.event.companyId") {
                if let Some(val) = event.get("bitdefender.event.companyId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.companyId".into(),
                            message,
                        }
                    })?;
                    event.set("organization.id", converted)?;
                }
            }

            if event.has_value("bitdefender.event.moved_company_id") {
                if let Some(val) = event.get("bitdefender.event.moved_company_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.moved_company_id".into(),
                            message,
                        }
                    })?;
                    event.set("organization.id", converted)?;
                }
            }

            let _cond = { event.has_value("organization.id") && event.has_value("_tmp.tenants") };
            if _cond {
                // Painless script
                // Source: def conftenants = ctx._tmp.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def conftenants = ctx._tmp.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n}\n"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("organization.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.moved_company_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("organization.name", v)?;
                }
            }

            if event.has_value("bitdefender.event.id") {
                if let Some(val) = event.get("bitdefender.event.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.id".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.module")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.module", v)?;
            }

            // Painless script
            // Source: def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.kind = schema;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.kind = schema;\n}\n"#
                ),
                cached_params!(
                    "{\"adcloud\":\"event\",\"aph\":\"alert\",\"av\":\"alert\",\"avc\":\"alert\",\"dp\":\"event\",\"exchange-malware\":\"alert\",\"exchange-organization-info\":\"alert\",\"exchange-user-credentials\":\"alert\",\"fw\":\"event\",\"hd\":\"alert\",\"modules\":\"event\",\"network-sandboxing\":\"alert\",\"registration\":\"event\",\"supa-update-status\":\"event\",\"sva-load\":\"alert\",\"sva\":\"event\",\"antiexploit\":\"alert\",\"network-monitor\":\"alert\",\"task-status\":\"event\",\"uc\":\"event\",\"storage-antimalware\":\"alert\",\"install\":\"event\",\"uninstall\":\"event\",\"hwid-change\":\"event\",\"endpoint-moved-in\":\"event\",\"endpoint-moved-out\":\"event\",\"troubleshooting-activity\":\"event\",\"device-control\":\"event\",\"ransomware-mitigation\":\"alert\",\"new-incident\":\"alert\",\"security-container-update-available\":\"event\"}"
                ),
            )?;

            // Painless script
            // Source: def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.category = schema;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.category = schema;\n}\n"#
                ),
                cached_params!(
                    "{\"adcloud\":[\"database\"],\"aph\":[\"threat\",\"network\"],\"av\":[\"threat\",\"malware\"],\"avc\":[\"threat\",\"malware\"],\"dp\":[\"threat\",\"network\"],\"exchange-malware\":[\"threat\",\"malware\"],\"exchange-organization-info\":[\"configuration\",\"package\"],\"exchange-user-credentials\":[\"configuration\",\"package\",\"host\"],\"fw\":[\"threat\",\"intrusion_detection\",\"network\"],\"hd\":[\"threat\",\"intrusion_detection\",\"malware\"],\"modules\":[\"configuration\",\"package\"],\"network-sandboxing\":[\"threat\",\"intrusion_detection\"],\"registration\":[\"configuration\",\"package\"],\"supa-update-status\":[\"configuration\",\"package\"],\"sva-load\":[\"configuration\",\"host\"],\"sva\":[\"configuration\",\"host\"],\"antiexploit\":[\"threat\",\"intrusion_detection\"],\"network-monitor\":[\"threat\",\"intrusion_detection\"],\"task-status\":[\"configuration\",\"host\",\"process\"],\"uc\":[\"threat\",\"network\",\"web\"],\"storage-antimalware\":[\"threat\",\"intrusion_detection\"],\"install\":[\"configuration\",\"package\"],\"uninstall\":[\"configuration\",\"package\"],\"hwid-change\":[\"configuration\",\"host\"],\"endpoint-moved-in\":[\"configuration\",\"package\"],\"endpoint-moved-out\":[\"configuration\",\"package\"],\"troubleshooting-activity\":[\"configuration\",\"host\",\"session\"],\"device-control\":[\"configuration\",\"host\"],\"ransomware-mitigation\":[\"threat\",\"intrusion_detection\"],\"new-incident\":[\"threat\",\"intrusion_detection\"],\"security-container-update-available\":[\"configuration\",\"package\"]}"
                ),
            )?;

            // Painless script
            // Source: def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.type = schema;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.type = schema;\n}\n"#
                ),
                cached_params!(
                    "{\"adcloud\":[\"info\"],\"aph\":[\"info\",\"access\"],\"av\":[\"info\"],\"avc\":[\"info\"],\"dp\":[\"info\"],\"exchange-malware\":[\"info\"],\"exchange-organization-info\":[\"info\"],\"exchange-user-credentials\":[\"info\"],\"fw\":[\"info\"],\"hd\":[\"info\"],\"modules\":[\"info\"],\"network-sandboxing\":[\"info\"],\"registration\":[\"info\"],\"supa-update-status\":[\"info\"],\"sva-load\":[\"info\"],\"sva\":[\"info\"],\"antiexploit\":[\"info\"],\"network-monitor\":[\"info\"],\"task-status\":[\"info\"],\"uc\":[\"info\"],\"storage-antimalware\":[\"info\"],\"install\":[\"info\"],\"uninstall\":[\"info\"],\"hwid-change\":[\"info\"],\"endpoint-moved-in\":[\"info\"],\"endpoint-moved-out\":[\"info\"],\"troubleshooting-activity\":[\"info\"],\"device-control\":[\"info\"],\"ransomware-mitigation\":[\"info\",\"denied\"],\"new-incident\":[\"info\"],\"security-container-update-available\":[\"info\"]}"
                ),
            )?;

            // Painless script
            // Source: def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.provider = schema;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def schemaId = ctx.bitdefender?.event?.module.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.provider = schema;\n}\n"#
                ),
                cached_params!(
                    "{\"adcloud\":\"Cloud AD Integration\",\"aph\":\"Antiphishing\",\"av\":\"Antimalware\",\"avc\":\"Advanced Threat Control\",\"dp\":\"Data Protection\",\"exchange-malware\":\"Exchange Malware Detection\",\"exchange-organization-info\":\"Exchange License Usage Limit Has Been Reached\",\"exchange-user-credentials\":\"Exchange User Credentials\",\"fw\":\"Firewall\",\"hd\":\"Hyper Detect Event\",\"modules\":\"Product Modules Status\",\"network-sandboxing\":\"Sandbox Analyzer Detection\",\"registration\":\"Product Registration\",\"supa-update-status\":\"Outdated Update Server\",\"sva-load\":\"Overloaded Security Server\",\"sva\":\"Security Server Status\",\"antiexploit\":\"Antiexploit\",\"network-monitor\":\"Network Attack Defense\",\"task-status\":\"Task Status\",\"uc\":\"User Control/Content Control\",\"storage-antimalware\":\"Storage Antimalware Event\",\"install\":\"Install Agent\",\"uninstall\":\"Uninstall Agent\",\"hwid-change\":\"Hardware ID Change\",\"endpoint-moved-in\":\"Endpoint Moved In\",\"endpoint-moved-out\":\"Endpoint Moved Out\",\"troubleshooting-activity\":\"Troubleshooting Activity\",\"device-control\":\"Device Control\",\"ransomware-mitigation\":\"Ransomware Activity\",\"new-incident\":\"New Incident\",\"security-container-update-available\":\"Security Container Update Available\"}"
                ),
            )?;

            let _cond = { event.get_str("bitdefender.event.severity") == Some("low") };
            if _cond {
                let v = json!(21);
                if !painless_is_empty_value(&v) {
                    event.set("event.severity", v)?;
                }
            }

            let _cond = { event.get_str("bitdefender.event.severity") == Some("medium") };
            if _cond {
                let v = json!(47);
                if !painless_is_empty_value(&v) {
                    event.set("event.severity", v)?;
                }
            }

            let _cond = { event.get_str("bitdefender.event.severity") == Some("high") };
            if _cond {
                let v = json!(73);
                if !painless_is_empty_value(&v) {
                    event.set("event.severity", v)?;
                }
            }

            let _cond = { event.get_str("bitdefender.event.severity") == Some("critical") };
            if _cond {
                let v = json!(99);
                if !painless_is_empty_value(&v) {
                    event.set("event.severity", v)?;
                }
            }

            let _cond = { !event.has_value("event.severity") };
            if _cond {
                if event.has_value("bitdefender.event.severity_score") {
                    if let Some(val) = event.get("bitdefender.event.severity_score") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.severity_score".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
            }

            let _cond = { !event.has_value("event.severity") };
            if _cond {
                if event.has_value("bitdefender.event.severityScore") {
                    if let Some(val) = event.get("bitdefender.event.severityScore") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.severityScore".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
            }

            let _cond = { !event.has_value("event.severity") };
            if _cond {
                let v = json!(0);
                if !painless_is_empty_value(&v) {
                    event.set("event.severity", v)?;
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.final_status")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.main_action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.actionTaken")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.malware.actionTaken")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.detection_action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.computer_fqdn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.computer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.endpointId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
            }

            // SKIPPED: condition not transpiled: !(ctx.bitdefender?.event?.computer_ip instanceof List) && !(ctx.bitdefender?.event?.computer_ip == ctx.bitdefender?.event?.computer_name) && !(ctx.bitdefender?.event?.computer_ip == ctx.host?.name)
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.has_value("bitdefender.event.computer_ip") {
                    if let Some(s) = event.get_string("bitdefender.event.computer_ip") {
                        let mut parts: Vec<Value> = cached_regex!(",\\ *")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("host.ip", Value::Array(parts))?;
                    }
                }
            }

            let _cond = {
                event
                    .get("bitdefender.event.computer_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "bitdefender.event.computer_ip", |event| {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                !(event
                    .get("bitdefender.event.computerIp")
                    .is_some_and(|v| v.is_array()))
            };
            if _cond {
                if event.has_value("bitdefender.event.computerIp") {
                    if let Some(val) = event.get("bitdefender.event.computerIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.computerIp".into(),
                                message,
                            }
                        })?;
                        event.set("_tmp.host_ip", converted)?;
                    }
                }
            }

            let _cond =
                { event.has_value("_tmp.host_ip") && event.get_str("_tmp.host_ip") != Some("") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("_tmp.host_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("bitdefender.event.computer_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = {
                event.has_value("bitdefender.event.computer_name")
                    && event.get_str("bitdefender.event.computer_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("bitdefender.event.computer_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.computerName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.computerName")
                    && event.get_str("bitdefender.event.computerName") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("bitdefender.event.computerName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.host_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.host_name")
                    && event.get_str("bitdefender.event.host_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("bitdefender.event.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.serverName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.serverName")
                    && event.get_str("bitdefender.event.serverName") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("bitdefender.event.serverName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.bitdefender?.event?.url != null && ctx.bitdefender?.event?.url =~ /^(?![a-zA-Z0-9]+:\/\/)/
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.has_value("bitdefender.event.url") {
                    gsub_field(
                        event,
                        "bitdefender.event.url",
                        "bitdefender.event.url",
                        cached_regex!("^"),
                        "https://",
                    )?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.url")
                    && event.get_str("bitdefender.event.url") != Some("")
            };
            if _cond {
                uri_parts(event, "bitdefender.event.url", "url", true, false)?;
            }

            if event.has_value("url.domain") {
                if let Some(domain_str) = event.get_string("url.domain") {
                    let domain = domain_str.to_string();
                    event.set("url.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("url.registered_domain", json!(registered))?;
                        }
                        event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("url.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            if event.has_value("bitdefender.event.source_ip") {
                if let Some(val) = event.get("bitdefender.event.source_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "bitdefender.event.source_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            let _cond = { !event.has_value("source.ip") };
            if _cond {
                if event.has_value("bitdefender.event.attack_source") {
                    if let Some(val) = event.get("bitdefender.event.attack_source") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.attack_source".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
            }

            let _cond = { !event.has_value("source.geo") };
            if _cond {
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
            }

            let _cond = { !event.has_value("source.as") };
            if _cond {
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
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { !event.has_value("source.nat.geo") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.nat.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.nat.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.nat.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.nat.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.nat.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.nat.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.nat.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.nat.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { !event.has_value("source.nat.as") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.nat.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.nat.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("source.nat.as.asn") {
                event.rename("source.nat.as.asn", "source.nat.as.number")?;
            }

            if event.has_value("source.nat.as.organization_name") {
                event.rename(
                    "source.nat.as.organization_name",
                    "source.nat.as.organization.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("bitdefender.event.computer_ip") {
                    if let Some(val) = event.get("bitdefender.event.computer_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "bitdefender.event.computer_ip".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("destination.geo") };
            if _cond {
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { !event.has_value("destination.as") };
            if _cond {
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            if event.has_value("bitdefender.event.victim_ip") {
                if let Some(val) = event.get("bitdefender.event.victim_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "bitdefender.event.victim_ip".into(),
                            message,
                        })?;
                    event.set("destination.nat.ip", converted)?;
                }
            }

            let _cond = { !event.has_value("destination.nat.geo") };
            if _cond {
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.nat.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.nat.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.nat.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.nat.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.nat.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.nat.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.nat.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.nat.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { !event.has_value("destination.nat.as") };
            if _cond {
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.nat.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.nat.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("destination.nat.as.asn") {
                event.rename("destination.nat.as.asn", "destination.nat.as.number")?;
            }

            if event.has_value("destination.nat.as.organization_name") {
                event.rename(
                    "destination.nat.as.organization_name",
                    "destination.nat.as.organization.name",
                )?;
            }

            if event.has_value("bitdefender.event.local_port") {
                if let Some(val) = event.get("bitdefender.event.local_port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.local_port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.userId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.user.userSid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.user.user_sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.user.sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.user.userName")
                    && event.get_str("bitdefender.event.user.userName") != Some("")
            };
            if _cond {
                if event.has_value("bitdefender.event.user.userName") {
                    if let Some(input) = event.get_string("bitdefender.event.user.userName") {
                        // Grok pattern: %{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.domain}\\\\%{GREEDYDATA:user.name}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}"
                                ),
                                cached_grok!("%{DATA:user.name}@%{GREEDYDATA:user.domain}"),
                                cached_grok!("%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("bitdefender.event.user.userName")
                    && event
                        .get_str("bitdefender.event.user.userName")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("bitdefender.event.user.userName").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.user.userName")
                    && event.get_str("bitdefender.event.user.userName") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("bitdefender.event.user.userName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("user.name")
                    && event.has_value("bitdefender.event.user.name")
                    && event.get_str("bitdefender.event.user.name") != Some("")
            };
            if _cond {
                if event.has_value("bitdefender.event.user.name") {
                    if let Some(input) = event.get_string("bitdefender.event.user.name") {
                        // Grok pattern: %{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.domain}\\\\%{GREEDYDATA:user.name}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}"
                                ),
                                cached_grok!("%{DATA:user.name}@%{GREEDYDATA:user.domain}"),
                                cached_grok!("%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("bitdefender.event.user.name")
                    && event
                        .get_str("bitdefender.event.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("bitdefender.event.user.name").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.user.name")
                    && event.get_str("bitdefender.event.user.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("bitdefender.event.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("user.name")
                    && event.has_value("bitdefender.event.username")
                    && event.get_str("bitdefender.event.username") != Some("")
            };
            if _cond {
                if event.has_value("bitdefender.event.username") {
                    if let Some(input) = event.get_string("bitdefender.event.username") {
                        // Grok pattern: %{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.domain}\\\\%{GREEDYDATA:user.name}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}"
                                ),
                                cached_grok!("%{DATA:user.name}@%{GREEDYDATA:user.domain}"),
                                cached_grok!("%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("bitdefender.event.username")
                    && event
                        .get_str("bitdefender.event.username")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("bitdefender.event.username").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.username")
                    && event.get_str("bitdefender.event.username") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("bitdefender.event.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("user.name")
                    && event.has_value("bitdefender.event.detection_username")
                    && event.get_str("bitdefender.event.detection_username") != Some("")
            };
            if _cond {
                if event.has_value("bitdefender.event.detection_username") {
                    if let Some(input) = event.get_string("bitdefender.event.detection_username") {
                        // Grok pattern: %{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.domain}\\\\%{GREEDYDATA:user.name}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}"
                                ),
                                cached_grok!("%{DATA:user.name}@%{GREEDYDATA:user.domain}"),
                                cached_grok!("%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("bitdefender.event.detection_username")
                    && event
                        .get_str("bitdefender.event.detection_username")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("bitdefender.event.detection_username").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.detection_username")
                    && event.get_str("bitdefender.event.detection_username") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("bitdefender.event.detection_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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

            if let Some(v) = event
                .get("bitdefender.event.file_hash_sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.file_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("file.hash.sha256") {
                    event.set("file.hash.sha256", v)?;
                }
            }

            let _cond = {
                event.has_value("bitdefender.event.file_hash_sha256")
                    && event.get_str("bitdefender.event.file_hash_sha256") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("bitdefender.event.file_hash_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("bitdefender.event.file_hash")
                    && event.get_str("bitdefender.event.file_hash") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("bitdefender.event.file_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("bitdefender.event.hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = {
                event.has_value("bitdefender.event.hash")
                    && event.get_str("bitdefender.event.hash") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("bitdefender.event.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("bitdefender.event.file_hash_md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = {
                event.has_value("bitdefender.event.file_hash_md5")
                    && event.get_str("bitdefender.event.file_hash_md5") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("bitdefender.event.file_hash_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("bitdefender.event.file_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.filePaths")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if event.has_value("bitdefender.event.fileSizes") {
                if let Some(val) = event.get("bitdefender.event.fileSizes") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.fileSizes".into(),
                            message,
                        }
                    })?;
                    event.set("file.size", converted)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.file_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.recipients")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.to.address", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.sender")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.sender.address", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.parent_process_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.parent_process_pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.parent_process_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.process_command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.process_info_command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.process_pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.process_info_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            let _cond = { !event.has_value("threat.software.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.detection_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            let _cond = { !event.has_value("process.executable") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.process_path")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.executable", v)?;
                }
            }

            let _cond = { !event.has_value("threat.software.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.malware_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            let _cond = { !event.has_value("threat.software.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.malwareName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            let _cond = { !event.has_value("process.executable") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.exploit_path")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.executable", v)?;
                }
            }

            let _cond = { !event.has_value("threat.software.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.exploit_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            let _cond = { event.get_str("bitdefender.event.detection_cve") != Some("cve string") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.detection_cve")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.id", v)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.detection_parentPath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if event.has_value("bitdefender.event.detection_parentPid") {
                if let Some(val) = event.get("bitdefender.event.detection_parentPid") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.detection_parentPid".into(),
                            message,
                        }
                    })?;
                    event.set("process.parent.pid", converted)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.detection_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if event.has_value("bitdefender.event.detection_pid") {
                if let Some(val) = event.get("bitdefender.event.detection_pid") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "bitdefender.event.detection_pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
            }

            let _cond = { !event.has_value("threat.software.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.detection_threatName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            let _cond = { !event.has_value("threat.software.name") };
            if _cond {
                if let Some(v) = event
                    .get("bitdefender.event.detection_exploitTechnique")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            if let Some(v) = event
                .get("bitdefender.event.attack_types")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.technique.name", v)?;
            }

            let _cond = {
                event.has_value("bitdefender.event.detection_attackTechnique")
                    && event.get_str("bitdefender.event.detection_attackTechnique") != Some("")
            };
            if _cond {
                event.append_unique(
                    "threat.technique.name",
                    json!(
                        event
                            .get("bitdefender.event.detection_attackTechnique")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("bitdefender.event.att_ck_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.technique.id", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.deviceId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if let Some(v) = event
                .get("bitdefender.event.deviceName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.ip", |event| {
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("source.nat.ip") && event.get_str("source.nat.ip") != Some("") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.nat.ip")
                    && event.get_str("destination.nat.ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("message")
                    && event.has_value("bitdefender.event.malware_name")
                    && event.get_str("bitdefender.event.malware_name") != Some("")
            };
            if _cond {
                let v = json!(format!(
                    "malware: {}",
                    event
                        .get("bitdefender.event.malware_name")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.get_str("event.module") == Some("exchange-malware")
            };
            if _cond {
                let v = json!(format!(
                    "malware: via Exchange server {}/{}",
                    event
                        .get("bitdefender.event.computer_fqdn")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.computer_ip")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.has_value("bitdefender.event.detection_threatName")
                    && event.get_str("bitdefender.event.detection_threatName") != Some("")
            };
            if _cond {
                let v = json!(format!(
                    "threat: {}",
                    event
                        .get("bitdefender.event.detection_threatName")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.has_value("bitdefender.event.detection_exploitTechnique")
                    && event.get_str("bitdefender.event.detection_exploitTechnique") != Some("")
            };
            if _cond {
                let v = json!(format!(
                    "threat: {}",
                    event
                        .get("bitdefender.event.detection_exploitTechnique")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message") && event.get_str("event.module") == Some("sva-load")
            };
            if _cond {
                let v = json!(format!(
                    "WARNING: overloaded security server: {}, Overall Usage={}%",
                    event
                        .get("host.name")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.overallUsage")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.get_str("event.module") == Some("ransomware-mitigation")
            };
            if _cond {
                let v = json!(format!(
                    "ransomware event on {}/{}",
                    event
                        .get("bitdefender.event.computer_name")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.computer_ip")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.get_str("event.module") == Some("hd")
                    && event.get_str("bitdefender.event.malware_type") == Some("file")
            };
            if _cond {
                let v = json!(format!(
                    "attack: {} via file {}",
                    event
                        .get("bitdefender.event.attack_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.file_path")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond =
                { !event.has_value("message") && event.get_str("event.module") == Some("hd") };
            if _cond {
                let v = json!(format!(
                    "attack: {}",
                    event
                        .get("bitdefender.event.attack_type")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond =
                { !event.has_value("message") && event.get_str("event.module") == Some("uc") };
            if _cond {
                let v = json!(format!(
                    "{}, block type: {}",
                    event
                        .get("bitdefender.event.status")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.block_type")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond =
                { !event.has_value("message") && event.get_str("event.module") == Some("aph") };
            if _cond {
                let v = json!(format!(
                    "{}, aph type: {}",
                    event
                        .get("bitdefender.event.status")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.aph_type")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond =
                { !event.has_value("message") && event.get_str("event.module") == Some("dp") };
            if _cond {
                let v = json!(format!(
                    "{}, blocking rule: {}",
                    event
                        .get("bitdefender.event.status")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.blocking_rule_name")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond =
                { !event.has_value("message") && event.get_str("event.module") == Some("avc") };
            if _cond {
                let v = json!(format!(
                    "{}, exploit type: {}",
                    event
                        .get("bitdefender.event.status")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.exploit_type")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond =
                { !event.has_value("message") && event.get_str("event.module") == Some("fw") };
            if _cond {
                let v = json!(format!(
                    "{}, source ip: {}",
                    event
                        .get("bitdefender.event.status")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("bitdefender.event.source_ip")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.get_str("event.module") == Some("exchange-user-credentials")
            };
            if _cond {
                let v = json!(
                    "ERROR: On-demand scan task could not start on the target Exchange server due to invalid user credentials"
                );
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.has_value("bitdefender.event.detection_name")
                    && event.get_str("bitdefender.event.detection_name") != Some("")
            };
            if _cond {
                let v = json!(
                    event
                        .get("bitdefender.event.detection_name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("message")
                    && event.has_value("bitdefender.event.threatType")
                    && event.get_str("bitdefender.event.threatType") != Some("")
            };
            if _cond {
                let v = json!(
                    event
                        .get("bitdefender.event.threatType")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'undefined') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\n// Prevent empty fields in correlated arrays from being removed.\n// The first two cases should never happen, but are included\n// defensively. The remediationActions elements may be validly\n// empty.\nctx.bitdefender?.event?.filePath?.replaceAll(e -> e == \"\" ? \"-\" : e);\nctx.bitdefender?.event?.fileSizes?.replaceAll(e -> e == \"\" ? \"-\" : e);\nctx.bitdefender?.event?.remediationActions?.replaceAll(e -> e == \"\" ? \"-\" : e);\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'undefined') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\n// Prevent empty fields in correlated arrays from being removed.\n// The first two cases should never happen, but are included\n// defensively. The remediationActions elements may be validly\n// empty.\nctx.bitdefender?.event?.filePath?.replaceAll(e -> e == \"\" ? \"-\" : e);\nctx.bitdefender?.event?.fileSizes?.replaceAll(e -> e == \"\" ? \"-\" : e);\nctx.bitdefender?.event?.remediationActions?.replaceAll(e -> e == \"\" ? \"-\" : e);\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_tmp");
                Ok(())
            })();

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("bitdefender.event.id");
                    event.remove("bitdefender.event.deviceId");
                    event.remove("bitdefender.event.deviceName");
                    event.remove("bitdefender.event.file_hash");
                    event.remove("bitdefender.event.file_name");
                    event.remove("bitdefender.event.moved_company_id");
                    event.remove("bitdefender.event.moved_company_name");
                    event.remove("bitdefender.event.parent_process_path");
                    event.remove("bitdefender.event.parent_process_pid");
                    event.remove("bitdefender.event.parent_process_id");
                    event.remove("bitdefender.event.process_command_line");
                    event.remove("bitdefender.event.process_info_command_line");
                    event.remove("bitdefender.event.process_pid");
                    event.remove("bitdefender.event.process_info_path");
                    Ok(())
                })();
            }

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
