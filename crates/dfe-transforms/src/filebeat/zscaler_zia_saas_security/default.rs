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

            let _cond = { event.get_str("input.type") == Some("http_endpoint") };
            if _cond {
                event.remove("json");
            }

            parse_json_field(event, "event.original", "zscaler_zia.saas_security")?;

            let _cond = {
                event.has_value("zscaler_zia.saas_security")
                    && event.get_bool("_conf.strict_fields") == Some(true)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.zscaler_zia.saas_security.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.saas_security.version == null ? 'null' : ctx.zscaler_zia.saas_security.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.zscaler_zia.saas_security.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.saas_security.version == null ? 'null' : ctx.zscaler_zia.saas_security.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}"#
                        ),
                        cached_params!(
                            "{\"data_stream\":\"saas_security\",\"expect\":{\"version\":\"v1\"},\"pkg_version\":\"3.19.0\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "check_template_version")?;
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

            if event.has_value("zscaler_zia.saas_security.record_id") {
                if let Some(val) = event.get("zscaler_zia.saas_security.record_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.saas_security.record_id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.saas_security.record_id", converted)?;
                }
            }

            if event.has_value("zscaler_zia.saas_security.company.id") {
                if let Some(val) = event.get("zscaler_zia.saas_security.company.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.saas_security.company.id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.saas_security.company.id", converted)?;
                }
            }

            if event.has_value("zscaler_zia.saas_security.bucket.id") {
                if let Some(val) = event.get("zscaler_zia.saas_security.bucket.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.saas_security.bucket.id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.saas_security.bucket.id", converted)?;
                }
            }

            if event.has_value("zscaler_zia.saas_security.dlp.identifier") {
                if let Some(val) = event.get("zscaler_zia.saas_security.dlp.identifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.saas_security.dlp.identifier".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.saas_security.dlp.identifier", converted)?;
                }
            }

            if event.has_value("zscaler_zia.saas_security.genai.run_id") {
                if let Some(val) = event.get("zscaler_zia.saas_security.genai.run_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.saas_security.genai.run_id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.saas_security.genai.run_id", converted)?;
                }
            }

            if event.has_value("zscaler_zia.saas_security.genai.scan_id") {
                if let Some(val) = event.get("zscaler_zia.saas_security.genai.scan_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.saas_security.genai.scan_id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.saas_security.genai.scan_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("zscaler_zia.saas_security.collab_count") {
                    if let Some(val) = event.get("zscaler_zia.saas_security.collab_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.collab_count".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security.collab_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_collab_count_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.collab_count");
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
                if event.has_value("zscaler_zia.saas_security.email.external_recipients_count") {
                    if let Some(val) =
                        event.get("zscaler_zia.saas_security.email.external_recipients_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.email.external_recipients_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zia.saas_security.email.external_recipients_count",
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
                    "convert_email_external_recipients_count_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.email.external_recipients_count");
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
                if event.has_value("zscaler_zia.saas_security.email.internal_recipients_count") {
                    if let Some(val) =
                        event.get("zscaler_zia.saas_security.email.internal_recipients_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.email.internal_recipients_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zia.saas_security.email.internal_recipients_count",
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
                    "convert_email_internal_recipients_count_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.email.internal_recipients_count");
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
                if event.has_value("zscaler_zia.saas_security.email.message_size_bytes") {
                    if let Some(val) =
                        event.get("zscaler_zia.saas_security.email.message_size_bytes")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.email.message_size_bytes".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zia.saas_security.email.message_size_bytes",
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
                    "convert_email_message_size_bytes_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.email.message_size_bytes");
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
                if event.has_value("zscaler_zia.saas_security.external_collab_count") {
                    if let Some(val) = event.get("zscaler_zia.saas_security.external_collab_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.external_collab_count".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security.external_collab_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_external_collab_count_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.external_collab_count");
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
                if event.has_value("zscaler_zia.saas_security.file.download_time_ms") {
                    if let Some(val) = event.get("zscaler_zia.saas_security.file.download_time_ms")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.file.download_time_ms".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security.file.download_time_ms", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_file_download_time_ms_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.file.download_time_ms");
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
                if event.has_value("zscaler_zia.saas_security.file.scan_time_ms") {
                    if let Some(val) = event.get("zscaler_zia.saas_security.file.scan_time_ms") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.file.scan_time_ms".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security.file.scan_time_ms", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_file_scan_time_ms_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.file.scan_time_ms");
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
                if event.has_value("zscaler_zia.saas_security.file.size") {
                    if let Some(val) = event.get("zscaler_zia.saas_security.file.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.file.size".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security.file.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_file_size_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.file.size");
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
                if event.has_value("zscaler_zia.saas_security.internal_collab_count") {
                    if let Some(val) = event.get("zscaler_zia.saas_security.internal_collab_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security.internal_collab_count".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security.internal_collab_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_internal_collab_count_to_long",
                )?;
                event.remove("zscaler_zia.saas_security.internal_collab_count");
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
                event
                    .get("zscaler_zia.saas_security.copilot_accessible")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("zscaler_zia.saas_security.copilot_accessible")
                        .is_some_and(|s| s.to_lowercase() == "yes")
            };
            if _cond {
                event.set("zscaler_zia.saas_security.copilot_accessible", json!(true))?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.copilot_accessible")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("zscaler_zia.saas_security.copilot_accessible")
                        .is_some_and(|s| s.to_lowercase() == "no")
            };
            if _cond {
                event.set("zscaler_zia.saas_security.copilot_accessible", json!(false))?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.is_incident")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("zscaler_zia.saas_security.is_incident")
                        .is_some_and(|s| s.to_lowercase() == "yes")
            };
            if _cond {
                event.set("zscaler_zia.saas_security.is_incident", json!(true))?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.is_incident")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("zscaler_zia.saas_security.is_incident")
                        .is_some_and(|s| s.to_lowercase() == "no")
            };
            if _cond {
                event.set("zscaler_zia.saas_security.is_incident", json!(false))?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.is_inbound")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("zscaler_zia.saas_security.email.is_inbound")
                        .is_some_and(|s| s.to_lowercase() == "yes")
            };
            if _cond {
                event.set("zscaler_zia.saas_security.email.is_inbound", json!(true))?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.is_inbound")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("zscaler_zia.saas_security.email.is_inbound")
                        .is_some_and(|s| s.to_lowercase() == "no")
            };
            if _cond {
                event.set("zscaler_zia.saas_security.email.is_inbound", json!(false))?;
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security.dlp.dict_counts")
                    && event.get_str("zscaler_zia.saas_security.dlp.dict_counts") == Some("None")
            };
            if _cond {
                event.remove("zscaler_zia.saas_security.dlp.dict_counts");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  if (s.length() == 0) return;\n  List out = new ArrayList();\n  int from = 0;\n  int n = s.length();\n  for (int i = 0; i < n; i++) {\n    if (s.charAt(i) == (char)'|') {\n      out.add(s.substring(from, i));\n      from = i + 1;\n    }\n  }\n  out.add(s.substring(from, n));\n  m.put(key, out);\n}\ndef ss = ctx.zscaler_zia?.saas_security;\nif (ss == null) return;\nsplitStr(ss, 'accessibility_flags');\nsplitStr(ss, 'collab_names');\nsplitStr(ss, 'collab_names_obfuscated');\nsplitStr(ss, 'external_collab_groups');\nsplitStr(ss, 'external_collab_groups_obfuscated');\nsplitStr(ss, 'external_collab_names');\nsplitStr(ss, 'external_collab_names_obfuscated');\nsplitStr(ss, 'internal_collab_groups');\nsplitStr(ss, 'internal_collab_groups_obfuscated');\nsplitStr(ss, 'internal_collab_names');\nsplitStr(ss, 'internal_collab_names_obfuscated');\nif (ss.dlp instanceof Map) {\n  splitStr(ss.dlp, 'dict_counts');\n  splitStr(ss.dlp, 'dict_names');\n  splitStr(ss.dlp, 'dict_names_obfuscated');\n  splitStr(ss.dlp, 'engine_names');\n  splitStr(ss.dlp, 'engine_names_obfuscated');\n}\nif (ss.collaboration instanceof Map) {\n  splitStr(ss.collaboration, 'external_recipients');\n  splitStr(ss.collaboration, 'external_recipients_obfuscated');\n  splitStr(ss.collaboration, 'internal_recipients');\n  splitStr(ss.collaboration, 'internal_recipients_obfuscated');\n}\nif (ss.email instanceof Map) {\n  splitStr(ss.email, 'external_recipients');\n  splitStr(ss.email, 'external_recipients_obfuscated');\n  splitStr(ss.email, 'internal_recipients');\n  splitStr(ss.email, 'internal_recipients_obfuscated');\n  if (ss.email.attachments instanceof Map) {\n    splitStr(ss.email.attachments, 'file_names');\n    splitStr(ss.email.attachments, 'file_names_obfuscated');\n    splitStr(ss.email.attachments, 'file_sizes');\n    splitStr(ss.email.attachments, 'file_types');\n    splitStr(ss.email.attachments, 'md5s');\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  if (s.length() == 0) return;\n  List out = new ArrayList();\n  int from = 0;\n  int n = s.length();\n  for (int i = 0; i < n; i++) {\n    if (s.charAt(i) == (char)'|') {\n      out.add(s.substring(from, i));\n      from = i + 1;\n    }\n  }\n  out.add(s.substring(from, n));\n  m.put(key, out);\n}\ndef ss = ctx.zscaler_zia?.saas_security;\nif (ss == null) return;\nsplitStr(ss, 'accessibility_flags');\nsplitStr(ss, 'collab_names');\nsplitStr(ss, 'collab_names_obfuscated');\nsplitStr(ss, 'external_collab_groups');\nsplitStr(ss, 'external_collab_groups_obfuscated');\nsplitStr(ss, 'external_collab_names');\nsplitStr(ss, 'external_collab_names_obfuscated');\nsplitStr(ss, 'internal_collab_groups');\nsplitStr(ss, 'internal_collab_groups_obfuscated');\nsplitStr(ss, 'internal_collab_names');\nsplitStr(ss, 'internal_collab_names_obfuscated');\nif (ss.dlp instanceof Map) {\n  splitStr(ss.dlp, 'dict_counts');\n  splitStr(ss.dlp, 'dict_names');\n  splitStr(ss.dlp, 'dict_names_obfuscated');\n  splitStr(ss.dlp, 'engine_names');\n  splitStr(ss.dlp, 'engine_names_obfuscated');\n}\nif (ss.collaboration instanceof Map) {\n  splitStr(ss.collaboration, 'external_recipients');\n  splitStr(ss.collaboration, 'external_recipients_obfuscated');\n  splitStr(ss.collaboration, 'internal_recipients');\n  splitStr(ss.collaboration, 'internal_recipients_obfuscated');\n}\nif (ss.email instanceof Map) {\n  splitStr(ss.email, 'external_recipients');\n  splitStr(ss.email, 'external_recipients_obfuscated');\n  splitStr(ss.email, 'internal_recipients');\n  splitStr(ss.email, 'internal_recipients_obfuscated');\n  if (ss.email.attachments instanceof Map) {\n    splitStr(ss.email.attachments, 'file_names');\n    splitStr(ss.email.attachments, 'file_names_obfuscated');\n    splitStr(ss.email.attachments, 'file_sizes');\n    splitStr(ss.email.attachments, 'file_types');\n    splitStr(ss.email.attachments, 'md5s');\n  }\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_split_pipe_delimited_multi_value_fields",
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

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.dlp.dict_counts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.dlp.dict_counts",
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_dlp_dict_count_to_long",
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
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.attachments.file_sizes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.attachments.file_sizes",
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_email_attachment_file_size_to_long",
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
                    },
                )?;
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security.time")
                    && event.get_str("zscaler_zia.saas_security.time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zscaler_zia.saas_security.time") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("zscaler_zia.saas_security.time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_saas_security_time",
                    )?;
                    event.remove("zscaler_zia.saas_security.time");
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
                event.has_value("zscaler_zia.saas_security.email.received_time")
                    && event.get_str("zscaler_zia.saas_security.email.received_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.saas_security.email.received_time")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("zscaler_zia.saas_security.email.received_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_email_received_time",
                    )?;
                    event.remove("zscaler_zia.saas_security.email.received_time");
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
                event.has_value("zscaler_zia.saas_security.file.last_modified_time")
                    && event.get_str("zscaler_zia.saas_security.file.last_modified_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.saas_security.file.last_modified_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.saas_security.tz"),
                            None,
                        ) {
                            event
                                .set("zscaler_zia.saas_security.file.last_modified_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_file_last_modified_time",
                    )?;
                    event.remove("zscaler_zia.saas_security.file.last_modified_time");
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
                event.has_value("zscaler_zia.saas_security.file.last_shared_on")
                    && event.get_str("zscaler_zia.saas_security.file.last_shared_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.saas_security.file.last_shared_on")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.saas_security.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.saas_security.file.last_shared_on", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_file_last_shared_on",
                    )?;
                    event.remove("zscaler_zia.saas_security.file.last_shared_on");
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
                .get("zscaler_zia.saas_security.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            if let Some(v) = event
                .get("zscaler_zia.saas_security.record_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.tz")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            event.set("event.provider", json!("Zscaler"))?;

            let _cond =
                { event.get_str("zscaler_zia.saas_security.sourcesubtype") == Some("email") };
            if _cond {
                event.append_unique("event.category", json!("email"))?;
            }

            let _cond = {
                event.get_str("zscaler_zia.saas_security.sourcesubtype") == Some("file")
                    || event.get_str("zscaler_zia.saas_security.sourcesubtype")
                        == Some("public_cloud_storage")
                    || event.get_str("zscaler_zia.saas_security.sourcesubtype")
                        == Some("repository")
            };
            if _cond {
                event.append_unique("event.category", json!("file"))?;
            }

            let _cond = {
                event.get_str("zscaler_zia.saas_security.sourcesubtype") == Some("collaboration")
            };
            if _cond {
                event.append_unique("event.category", json!("network"))?;
            }

            let _cond = { event.get_bool("zscaler_zia.saas_security.is_incident") == Some(true) };
            if _cond {
                event.append_unique("event.category", json!("intrusion_detection"))?;
            }

            let _cond = { event.get_bool("zscaler_zia.saas_security.is_incident") == Some(true) };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = {
                event.get_bool("zscaler_zia.saas_security.is_incident") == Some(false)
                    && event.get_str("zscaler_zia.saas_security.sourcesubtype")
                        == Some("collaboration")
            };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            event.append_unique("event.type", json!("info"))?;

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString severity = ctx.zscaler_zia.saas_security.severity;\nif (severity.equalsIgnoreCase(\"information\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString severity = ctx.zscaler_zia.saas_security.severity;\nif (severity.equalsIgnoreCase(\"information\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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
                event
                    .get("zscaler_zia.saas_security.collaboration.external_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.collaboration.external_recipients",
                    |event| {
                        event.append_unique(
                            "destination.user.email",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.collaboration.internal_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.collaboration.internal_recipients",
                    |event| {
                        event.append_unique(
                            "destination.user.email",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.external_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.external_recipients",
                    |event| {
                        event.append_unique(
                            "destination.user.email",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.internal_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.internal_recipients",
                    |event| {
                        event.append_unique(
                            "destination.user.email",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond =
                { event.get_str("zscaler_zia.saas_security.sourcesubtype") == Some("email") };
            if _cond {
                if let Some(v) = event
                    .get("zscaler_zia.saas_security.message_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.message_id", v)?;
                }
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.email.received_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.delivery_timestamp", v)?;
            }

            let _cond =
                { event.get_bool("zscaler_zia.saas_security.email.is_inbound") == Some(true) };
            if _cond {
                event.set("email.direction", json!("inbound"))?;
            }

            let _cond =
                { event.get_bool("zscaler_zia.saas_security.email.is_inbound") == Some(false) };
            if _cond {
                event.set("email.direction", json!("outbound"))?;
            }

            let _cond = {
                event.get_str("zscaler_zia.saas_security.sourcesubtype") == Some("email")
                    && event.has_value("zscaler_zia.saas_security.user_name")
            };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.external_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.external_recipients",
                    |event| {
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.internal_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.internal_recipients",
                    |event| {
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.attachments")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def att = ctx.zscaler_zia.saas_security.email.attachments;\ndef names = att.file_names instanceof List ? att.file_names : null;\ndef sizes = att.file_sizes instanceof List ? att.file_sizes : null;\ndef md5s = att.md5s instanceof List ? att.md5s : null;\nint n = 0;\nif (names != null && names.size() > n) n = names.size();\nif (sizes != null && sizes.size() > n) n = sizes.size();\nif (md5s != null && md5s.size() > n) n = md5s.size();\nif (n == 0) return;\nif (ctx.email == null) ctx.email = [:];\nif (ctx.email.attachments == null) ctx.email.attachments = new ArrayList();\nfor (int i = 0; i < n; i++) {\n  def file = new HashMap();\n  if (names != null && i < names.size()) file.put('name', names.get(i));\n  if (sizes != null && i < sizes.size()) file.put('size', sizes.get(i));\n  if (md5s != null && i < md5s.size()) {\n    def hash = new HashMap();\n    hash.put('md5', md5s.get(i));\n    file.put('hash', hash);\n  }\n  def item = new HashMap();\n  item.put('file', file);\n  ctx.email.attachments.add(item);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def att = ctx.zscaler_zia.saas_security.email.attachments;\ndef names = att.file_names instanceof List ? att.file_names : null;\ndef sizes = att.file_sizes instanceof List ? att.file_sizes : null;\ndef md5s = att.md5s instanceof List ? att.md5s : null;\nint n = 0;\nif (names != null && names.size() > n) n = names.size();\nif (sizes != null && sizes.size() > n) n = sizes.size();\nif (md5s != null && md5s.size() > n) n = md5s.size();\nif (n == 0) return;\nif (ctx.email == null) ctx.email = [:];\nif (ctx.email.attachments == null) ctx.email.attachments = new ArrayList();\nfor (int i = 0; i < n; i++) {\n  def file = new HashMap();\n  if (names != null && i < names.size()) file.put('name', names.get(i));\n  if (sizes != null && i < sizes.size()) file.put('size', sizes.get(i));\n  if (md5s != null && i < md5s.size()) {\n    def hash = new HashMap();\n    hash.put('md5', md5s.get(i));\n    file.put('hash', hash);\n  }\n  def item = new HashMap();\n  item.put('file', file);\n  ctx.email.attachments.add(item);\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_zip_email_attachments_into_ecs",
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

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.type_category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.type", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.extension")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.extension", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.directory")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.directory", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.owner")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.owner", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.file.last_modified_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mtime", v)?;
            }

            event.set("observer.vendor", json!("Zscaler"))?;

            event.set("observer.product", json!("Zscaler ZIA"))?;

            if let Some(v) = event
                .get("zscaler_zia.saas_security.datacenter.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.datacenter.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.datacenter.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.datacenter.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.country_iso_code", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.company.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.company.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.rule.label")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.rule.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.collaboration.sender")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.threat.indicator.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if event.has_value("zscaler_zia.saas_security.file.full_url") {
                uri_parts(
                    event,
                    "zscaler_zia.saas_security.file.full_url",
                    "url",
                    true,
                    false,
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event.get("user.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(v) = event
                    .get("user.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.internal_user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.internal_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.external_user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.external_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.file.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.file.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.file.last_share_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.file.last_share_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.collaboration.sender") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.collaboration.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.collab_names")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "zscaler_zia.saas_security.collab_names", |event| {
                    event.append_unique(
                        "related.user",
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
                event
                    .get("zscaler_zia.saas_security.external_collab_names")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.external_collab_names",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.internal_collab_names")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.internal_collab_names",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.collaboration.external_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.collaboration.external_recipients",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.collaboration.internal_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.collaboration.internal_recipients",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.external_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.external_recipients",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.internal_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.internal_recipients",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security.email.attachments.md5s")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.saas_security.email.attachments.md5s",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("zscaler_zia.saas_security.collaboration.channel.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("zscaler_zia.saas_security.collaboration.channel.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.remove("zscaler_zia.saas_security.collaboration.external_recipients");
                event.remove("zscaler_zia.saas_security.collaboration.internal_recipients");
                event.remove("zscaler_zia.saas_security.collaboration.sender");
                event.remove("zscaler_zia.saas_security.company.id");
                event.remove("zscaler_zia.saas_security.company.name");
                event.remove("zscaler_zia.saas_security.datacenter.city");
                event.remove("zscaler_zia.saas_security.datacenter.country");
                event.remove("zscaler_zia.saas_security.datacenter.name");
                event.remove("zscaler_zia.saas_security.email.attachments.file_names");
                event.remove("zscaler_zia.saas_security.email.attachments.file_sizes");
                event.remove("zscaler_zia.saas_security.email.attachments.md5s");
                event.remove("zscaler_zia.saas_security.email.external_recipients");
                event.remove("zscaler_zia.saas_security.email.internal_recipients");
                event.remove("zscaler_zia.saas_security.email.received_time");
                event.remove("zscaler_zia.saas_security.file.directory");
                event.remove("zscaler_zia.saas_security.file.extension");
                event.remove("zscaler_zia.saas_security.file.full_url");
                event.remove("zscaler_zia.saas_security.file.hash.md5");
                event.remove("zscaler_zia.saas_security.file.hash.sha256");
                event.remove("zscaler_zia.saas_security.file.last_modified_time");
                event.remove("zscaler_zia.saas_security.file.name");
                event.remove("zscaler_zia.saas_security.file.owner");
                event.remove("zscaler_zia.saas_security.file.path");
                event.remove("zscaler_zia.saas_security.file.size");
                event.remove("zscaler_zia.saas_security.file.type_category");
                event.remove("zscaler_zia.saas_security.record_id");
                event.remove("zscaler_zia.saas_security.rule.label");
                event.remove("zscaler_zia.saas_security.rule.type");
                event.remove("zscaler_zia.saas_security.threat.indicator.name");
                event.remove("zscaler_zia.saas_security.time");
                event.remove("zscaler_zia.saas_security.tz");
                event.remove("zscaler_zia.saas_security.user_name");
            }

            event.remove("_conf");

            // Painless script
            // Source: boolean dropScalar(Object v) {\n  return v == null || v == '' || v == 'N/A'\n    || v == 'None' || v == 'Unknown' || v == 'Unknown Host' || v == 'Unknown URL';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropScalar(Object v) {\n  return v == null || v == '' || v == 'N/A'\n    || v == 'None' || v == 'Unknown' || v == 'Unknown Host' || v == 'Unknown URL';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

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
