// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_process` pipeline.
pub struct PipelineProcess;

impl Transform for PipelineProcess {
    fn name(&self) -> &str {
        "pipeline_process"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = {
            event
                .get("beyondtrust_epm.event.process.HostedFile.owner")
                .is_some_and(|v| v.is_string())
        };
        if _cond {
            event.rename(
                "beyondtrust_epm.event.process.HostedFile.owner",
                "beyondtrust_epm.event.process.HostedFile.OwnerKeyword",
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.ElevationRequired") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.ElevationRequired") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.ElevationRequired".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.ElevationRequired", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_ElevationRequired_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.ElevationRequired");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.accessed")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.accessed") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.HostedFile.accessed")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.process.HostedFile.accessed", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.accessed".into(),
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
                    "date_process_HostedFile_accessed",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.accessed");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.HostedFile.code_signature.exists") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.HostedFile.code_signature.exists")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.code_signature.exists"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.HostedFile.code_signature.exists",
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
                "convert_process_HostedFile_code_signature_exists_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.code_signature.exists");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.code_signature.timestamp")
                && event
                    .get_str("beyondtrust_epm.event.process.HostedFile.code_signature.timestamp")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string(
                    "beyondtrust_epm.event.process.HostedFile.code_signature.timestamp",
                ) {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.process.HostedFile.code_signature.timestamp",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.code_signature.timestamp".into(),
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
                    "date_process_HostedFile_code_signature_timestamp",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.code_signature.timestamp");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.HostedFile.code_signature.trusted") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.HostedFile.code_signature.trusted")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.code_signature.trusted"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.HostedFile.code_signature.trusted",
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
                "convert_process_HostedFile_code_signature_trusted_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.code_signature.trusted");
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
            if event.has_value("beyondtrust_epm.event.process.HostedFile.code_signature.valid") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.HostedFile.code_signature.valid")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.code_signature.valid"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.HostedFile.code_signature.valid",
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
                "convert_process_HostedFile_code_signature_valid_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.code_signature.valid");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.created")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.created") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.HostedFile.created")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.process.HostedFile.created", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.created".into(),
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
                    "date_process_HostedFile_created",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.created");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.ctime")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.ctime") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.HostedFile.ctime")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.process.HostedFile.ctime", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.ctime".into(),
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
                    "date_process_HostedFile_ctime",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.ctime");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.elf.creation_date")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.elf.creation_date")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event
                    .get_as_string("beyondtrust_epm.event.process.HostedFile.elf.creation_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.process.HostedFile.elf.creation_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.elf.creation_date"
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
                    "date_process_HostedFile_elf_creation_date",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.elf.creation_date");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.HostedFile.elf.header.entrypoint") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.HostedFile.elf.header.entrypoint")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.elf.header.entrypoint"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.HostedFile.elf.header.entrypoint",
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
                "convert_process_HostedFile_elf_header_entrypoint_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.elf.header.entrypoint");
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
                .get("beyondtrust_epm.event.process.HostedFile.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.HostedFile.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.chi2") {
                            if let Some(val) = event.get("_ingest._value.chi2") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.chi2".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.chi2", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_HostedFile_elf_sections_chi2_to_long",
                        )?;
                        event.remove("_ingest._value.chi2");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.HostedFile.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.HostedFile.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.entropy") {
                            if let Some(val) = event.get("_ingest._value.entropy") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.entropy".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.entropy", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_HostedFile_elf_sections_entropy_to_long",
                        )?;
                        event.remove("_ingest._value.entropy");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.HostedFile.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.HostedFile.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.physical_size") {
                            if let Some(val) = event.get("_ingest._value.physical_size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.physical_size".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.physical_size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_HostedFile_elf_sections_physical_size_to_long",
                        )?;
                        event.remove("_ingest._value.physical_size");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.HostedFile.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.HostedFile.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.virtual_address") {
                            if let Some(val) = event.get("_ingest._value.virtual_address") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.virtual_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.virtual_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_HostedFile_elf_sections_virtual_address_to_long",
                        )?;
                        event.remove("_ingest._value.virtual_address");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.HostedFile.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.HostedFile.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.virtual_size") {
                            if let Some(val) = event.get("_ingest._value.virtual_size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.virtual_size".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.virtual_size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_HostedFile_elf_sections_virtual_size_to_long",
                        )?;
                        event.remove("_ingest._value.virtual_size");
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
                },
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.elf.telfhash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.elf.telfhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.md5") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.sha1") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.sha256") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.sha384") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.sha384")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.sha512") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.sha512")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.ssdeep") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.ssdeep")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.hash.tlsh") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.hash.tlsh")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.HostedFile.mtime")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.mtime") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.HostedFile.mtime")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.process.HostedFile.mtime", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.mtime".into(),
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
                    "date_process_HostedFile_mtime",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.mtime");
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

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.pe.imphash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.pe.imphash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.HostedFile.pe.pehash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.HostedFile.pe.pehash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.HostedFile.size") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.HostedFile.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.size".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.HostedFile.size", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_HostedFile_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.size");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.x509.not_after")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.x509.not_after")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.HostedFile.x509.not_after")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.process.HostedFile.x509.not_after",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.x509.not_after"
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
                    "date_process_HostedFile_x509_not_after",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.x509.not_after");
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
            event.has_value("beyondtrust_epm.event.process.HostedFile.x509.not_before")
                && event.get_str("beyondtrust_epm.event.process.HostedFile.x509.not_before")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.HostedFile.x509.not_before")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.process.HostedFile.x509.not_before",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.HostedFile.x509.not_before"
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
                    "date_process_HostedFile_x509_not_before",
                )?;
                event.remove("beyondtrust_epm.event.process.HostedFile.x509.not_before");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.HostedFile.x509.public_key_exponent")
            {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.HostedFile.x509.public_key_exponent")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.process.HostedFile.x509.public_key_exponent"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.HostedFile.x509.public_key_exponent",
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
                "convert_process_HostedFile_x509_public_key_exponent_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.x509.public_key_exponent");
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
            if event.has_value("beyondtrust_epm.event.process.HostedFile.x509.public_key_size") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.HostedFile.x509.public_key_size")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.HostedFile.x509.public_key_size"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.HostedFile.x509.public_key_size",
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
                "convert_process_HostedFile_x509_public_key_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.HostedFile.x509.public_key_size");
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
                .get("beyondtrust_epm.event.process.args")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.process.args", |event| {
                event.append(
                    "process.args",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.args_count") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.args_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.args_count".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.args_count", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_args_count_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.args_count");
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
            .get("beyondtrust_epm.event.process.args_count")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.args_count", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.code_signature.digest_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.digest_algorithm", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.code_signature.exists") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.code_signature.exists")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.code_signature.exists".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.code_signature.exists",
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
                "convert_process_code_signature_exists_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.code_signature.exists");
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
            .get("beyondtrust_epm.event.process.code_signature.exists")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.exists", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.code_signature.signing_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.signing_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.code_signature.status")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.status", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.code_signature.subject_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.subject_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.code_signature.team_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.team_id", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.code_signature.timestamp")
                && event.get_str("beyondtrust_epm.event.process.code_signature.timestamp")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.code_signature.timestamp")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.process.code_signature.timestamp",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.code_signature.timestamp"
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
                    "date_process_code_signature_timestamp",
                )?;
                event.remove("beyondtrust_epm.event.process.code_signature.timestamp");
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
            .get("beyondtrust_epm.event.process.code_signature.timestamp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.timestamp", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.code_signature.trusted") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.code_signature.trusted")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.code_signature.trusted".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.code_signature.trusted",
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
                "convert_process_code_signature_trusted_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.code_signature.trusted");
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
            .get("beyondtrust_epm.event.process.code_signature.trusted")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.trusted", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.code_signature.valid") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.code_signature.valid") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.code_signature.valid".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.code_signature.valid",
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
                "convert_process_code_signature_valid_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.code_signature.valid");
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
            .get("beyondtrust_epm.event.process.code_signature.valid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.code_signature.valid", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.command_line")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.command_line", v)?;
        }

        let _cond = { !event.has_value("process.command_line") };
        if _cond {
            if let Some(v) = event
                .get("beyondtrust_epm.event.EPMWinMac.RemotePowerShell.Command")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.byte_order")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.byte_order", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.cpu_type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.cpu_type", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.elf.creation_date")
                && event.get_str("beyondtrust_epm.event.process.elf.creation_date") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.process.elf.creation_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.process.elf.creation_date", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.elf.creation_date".into(),
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
                    "date_process_elf_creation_date",
                )?;
                event.remove("beyondtrust_epm.event.process.elf.creation_date");
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
            .get("beyondtrust_epm.event.process.elf.creation_date")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.creation_date", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.abi_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.abi_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.class")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.class", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.data")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.data", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.elf.header.entrypoint") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.elf.header.entrypoint")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.elf.header.entrypoint".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.elf.header.entrypoint",
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
                "convert_process_elf_header_entrypoint_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.elf.header.entrypoint");
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
            .get("beyondtrust_epm.event.process.elf.header.entrypoint")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.entrypoint", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.object_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.object_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.os_abi")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.os_abi", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.header.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.header.version", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.chi2") {
                            if let Some(val) = event.get("_ingest._value.chi2") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.chi2".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.chi2", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_elf_sections_chi2_to_long",
                        )?;
                        event.remove("_ingest._value.chi2");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.entropy") {
                            if let Some(val) = event.get("_ingest._value.entropy") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.entropy".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.entropy", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_elf_sections_entropy_to_long",
                        )?;
                        event.remove("_ingest._value.entropy");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.physical_size") {
                            if let Some(val) = event.get("_ingest._value.physical_size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.physical_size".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.physical_size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_elf_sections_physical_size_to_long",
                        )?;
                        event.remove("_ingest._value.physical_size");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.virtual_address") {
                            if let Some(val) = event.get("_ingest._value.virtual_address") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.virtual_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.virtual_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_elf_sections_virtual_address_to_long",
                        )?;
                        event.remove("_ingest._value.virtual_address");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.virtual_size") {
                            if let Some(val) = event.get("_ingest._value.virtual_size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.virtual_size".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.virtual_size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_elf_sections_virtual_size_to_long",
                        )?;
                        event.remove("_ingest._value.virtual_size");
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.chi2",
                        json!(
                            event
                                .get("_ingest._value.chi2")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.entropy",
                        json!(
                            event
                                .get("_ingest._value.entropy")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.flags",
                        json!(
                            event
                                .get("_ingest._value.flags")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.physical_offset",
                        json!(
                            event
                                .get("_ingest._value.physical_offset")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.physical_size",
                        json!(
                            event
                                .get("_ingest._value.physical_size")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.type",
                        json!(
                            event
                                .get("_ingest._value.type")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.virtual_address",
                        json!(
                            event
                                .get("_ingest._value.virtual_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.append_unique(
                        "process.elf.sections.virtual_size",
                        json!(
                            event
                                .get("_ingest._value.virtual_size")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("process.elf.sections.chi2")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "process.elf.sections.chi2", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
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
                        "convert_ecs_process_elf_sections_chi2_elements_to_long",
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
            event
                .get("process.elf.sections.entropy")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "process.elf.sections.entropy", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
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
                        "convert_ecs_process_elf_sections_entropy_elements_to_long",
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
            event
                .get("process.elf.sections.physical_size")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "process.elf.sections.physical_size", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
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
                        "convert_ecs_process_elf_sections_physical_size_elements_to_long",
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
            event
                .get("process.elf.sections.virtual_address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "process.elf.sections.virtual_address", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
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
                        "convert_ecs_process_elf_sections_virtual_address_elements_to_long",
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
            event
                .get("process.elf.sections.virtual_size")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "process.elf.sections.virtual_size", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
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
                        "convert_ecs_process_elf_sections_virtual_size_elements_to_long",
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
            event
                .get("beyondtrust_epm.event.process.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.segments",
                |event| {
                    event.append_unique(
                        "process.elf.segments.sections",
                        json!(
                            event
                                .get("_ingest._value.sections")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.segments",
                |event| {
                    event.append_unique(
                        "process.elf.segments.type",
                        json!(
                            event
                                .get("_ingest._value.type")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.shared_libraries")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.shared_libraries",
                |event| {
                    event.append_unique(
                        "process.elf.shared_libraries",
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.elf.telfhash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.elf.telfhash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.elf.telfhash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.elf.telfhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.end")
                && event.get_str("beyondtrust_epm.event.process.end") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.process.end") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("beyondtrust_epm.event.process.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.end".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_process_end")?;
                event.remove("beyondtrust_epm.event.process.end");
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
            .get("beyondtrust_epm.event.process.end")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.end", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.entity_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.entity_id", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.as.number") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.as.number")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.entry_meta.source.as.number"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.as.number",
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
                "convert_process_entry_meta_source_as_number_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.as.number");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.bytes") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.bytes")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.entry_meta.source.bytes".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.bytes",
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
                "convert_process_entry_meta_source_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.bytes");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.geo.TimezoneOffset")
            {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.geo.TimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.process.entry_meta.source.geo.TimezoneOffset"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.geo.TimezoneOffset",
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
                "convert_process_entry_meta_source_geo_TimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.geo.TimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.geo.location.lat") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.geo.location.lat")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.process.entry_meta.source.geo.location.lat"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.geo.location.lat",
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
                "convert_process_entry_meta_source_geo_location_lat_to_double",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.geo.location.lat");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.geo.location.lon") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.geo.location.lon")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.process.entry_meta.source.geo.location.lon"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.geo.location.lon",
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
                "convert_process_entry_meta_source_geo_location_lon_to_double",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.geo.location.lon");
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
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.geo.location.lat")
                && event
                    .has_value("beyondtrust_epm.event.process.entry_meta.source.geo.location.lon")
        };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.process.entry_meta.source.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.process.entry_meta.source.geo.location.lon);\nctx.beyondtrust_epm.event.process.entry_meta.source.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.process.entry_meta.source.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.process.entry_meta.source.geo.location.lon);\nctx.beyondtrust_epm.event.process.entry_meta.source.geo.location = location;"#
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.entry_meta.source.ip") };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.event.process.entry_meta.source.ip") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.event.process.entry_meta.source.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.entry_meta.source.ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.event.process.entry_meta.source.ip",
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
                    "convert_process_entry_meta_source_ip_to_ip",
                )?;
                event.remove("beyondtrust_epm.event.process.entry_meta.source.ip");
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

        let _cond = { event.has_value("beyondtrust_epm.event.process.entry_meta.source.ip") };
        if _cond {
            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.entry_meta.source.nat.ip") };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.event.process.entry_meta.source.nat.ip") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.event.process.entry_meta.source.nat.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.entry_meta.source.nat.ip"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.event.process.entry_meta.source.nat.ip",
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
                    "convert_process_entry_meta_source_nat_ip_to_ip",
                )?;
                event.remove("beyondtrust_epm.event.process.entry_meta.source.nat.ip");
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

        let _cond = { event.has_value("beyondtrust_epm.event.process.entry_meta.source.nat.ip") };
        if _cond {
            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.nat.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.nat.port") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.nat.port")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.entry_meta.source.nat.port".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.nat.port",
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
                "convert_process_entry_meta_source_nat_port_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.nat.port");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.packets") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.entry_meta.source.packets")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.entry_meta.source.packets".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.packets",
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
                "convert_process_entry_meta_source_packets_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.packets");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.port") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.entry_meta.source.port")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.entry_meta.source.port".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.port",
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
                "convert_process_entry_meta_source_port_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.port");
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
            if event.has_value(
                "beyondtrust_epm.event.process.entry_meta.source.user.DefaultTimezoneOffset",
            ) {
                if let Some(val) = event.get(
                    "beyondtrust_epm.event.process.entry_meta.source.user.DefaultTimezoneOffset",
                ) {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                    event.set("beyondtrust_epm.event.process.entry_meta.source.user.DefaultTimezoneOffset", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_DefaultTimezoneOffset_to_long",
            )?;
            event.remove(
                "beyondtrust_epm.event.process.entry_meta.source.user.DefaultTimezoneOffset",
            );
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
            if event
                .has_value("beyondtrust_epm.event.process.entry_meta.source.user.LocalIdentifier")
            {
                if let Some(val) = event
                    .get("beyondtrust_epm.event.process.entry_meta.source.user.LocalIdentifier")
                {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.LocalIdentifier".into(),
                        message,
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.entry_meta.source.user.LocalIdentifier",
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
                "convert_process_entry_meta_source_user_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.user.LocalIdentifier");
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
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.changes.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.process.entry_meta.source.user.changes.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.changes.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.process.entry_meta.source.user.changes.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_changes_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.user.changes.DefaultTimezoneOffset");
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
            if event.has_value(
                "beyondtrust_epm.event.process.entry_meta.source.user.changes.LocalIdentifier",
            ) {
                if let Some(val) = event.get(
                    "beyondtrust_epm.event.process.entry_meta.source.user.changes.LocalIdentifier",
                ) {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.changes.LocalIdentifier".into(),
                        message,
                    })?;
                    event.set("beyondtrust_epm.event.process.entry_meta.source.user.changes.LocalIdentifier", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_changes_LocalIdentifier_to_long",
            )?;
            event.remove(
                "beyondtrust_epm.event.process.entry_meta.source.user.changes.LocalIdentifier",
            );
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
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.changes.email")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.changes.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event
                .has_value("beyondtrust_epm.event.process.entry_meta.source.user.changes.full_name")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get(
                            "beyondtrust_epm.event.process.entry_meta.source.user.changes.full_name"
                        )
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.changes.hash")
        };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.changes.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.changes.name")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.changes.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.effective.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.process.entry_meta.source.user.effective.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.effective.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.process.entry_meta.source.user.effective.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_effective_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.entry_meta.source.user.effective.DefaultTimezoneOffset");
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
            if event.has_value(
                "beyondtrust_epm.event.process.entry_meta.source.user.effective.LocalIdentifier",
            ) {
                if let Some(val) = event.get("beyondtrust_epm.event.process.entry_meta.source.user.effective.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.effective.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.process.entry_meta.source.user.effective.LocalIdentifier", converted)?;
            }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_effective_LocalIdentifier_to_long",
            )?;
            event.remove(
                "beyondtrust_epm.event.process.entry_meta.source.user.effective.LocalIdentifier",
            );
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
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.effective.email")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.effective.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value(
                "beyondtrust_epm.event.process.entry_meta.source.user.effective.full_name",
            )
        };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.process.entry_meta.source.user.effective.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.effective.hash")
        };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.effective.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.effective.id")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.effective.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.id") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value(
                "beyondtrust_epm.event.process.entry_meta.source.user.target.DefaultTimezoneOffset",
            ) {
                if let Some(val) = event.get("beyondtrust_epm.event.process.entry_meta.source.user.target.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.target.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.process.entry_meta.source.user.target.DefaultTimezoneOffset", converted)?;
            }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_target_DefaultTimezoneOffset_to_long",
            )?;
            event.remove(
                "beyondtrust_epm.event.process.entry_meta.source.user.target.DefaultTimezoneOffset",
            );
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
            if event.has_value(
                "beyondtrust_epm.event.process.entry_meta.source.user.target.LocalIdentifier",
            ) {
                if let Some(val) = event.get(
                    "beyondtrust_epm.event.process.entry_meta.source.user.target.LocalIdentifier",
                ) {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.process.entry_meta.source.user.target.LocalIdentifier".into(),
                        message,
                    })?;
                    event.set("beyondtrust_epm.event.process.entry_meta.source.user.target.LocalIdentifier", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_entry_meta_source_user_target_LocalIdentifier_to_long",
            )?;
            event.remove(
                "beyondtrust_epm.event.process.entry_meta.source.user.target.LocalIdentifier",
            );
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
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.target.email")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.target.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.target.full_name")
        };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get(
                            "beyondtrust_epm.event.process.entry_meta.source.user.target.full_name"
                        )
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.target.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.target.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.target.id") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.target.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond =
            { event.has_value("beyondtrust_epm.event.process.entry_meta.source.user.target.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.entry_meta.source.user.target.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.env_vars") };
        if _cond {
            event.append_unique(
                "process.env_vars",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.env_vars")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.executable")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.executable", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.exit_code") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.exit_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.exit_code".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.exit_code", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_exit_code_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.exit_code");
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
            .get("beyondtrust_epm.event.process.exit_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.exit_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.group.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.group.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.group.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.md5")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.md5", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.md5") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.sha1")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.sha1", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.sha1") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.sha256")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.sha256", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.sha256") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.sha384")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.sha384", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.sha384") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.sha384")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.sha512")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.sha512", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.sha512") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.sha512")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.ssdeep")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.ssdeep", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.ssdeep") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.ssdeep")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.hash.tlsh")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.hash.tlsh", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.hash.tlsh") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.hash.tlsh")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.interactive") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.interactive") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.interactive".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.interactive", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_interactive_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.interactive");
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
            .get("beyondtrust_epm.event.process.interactive")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.interactive", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.company")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.company", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.file_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.file_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.imphash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.imphash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.pe.imphash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.pe.imphash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.original_file_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.original_file_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.pehash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.pehash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.pe.pehash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.pe.pehash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.pe.product")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pe.product", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.pgid") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.pgid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.pgid".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.pgid", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_pgid_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.pgid");
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
            if event.has_value("beyondtrust_epm.event.process.pid") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.pid".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.pid", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_pid_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.pid");
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
            .get("beyondtrust_epm.event.process.pid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.pid", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.same_as_process") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.same_as_process") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.same_as_process".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.same_as_process", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_same_as_process_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.process.same_as_process");
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
            event.has_value("beyondtrust_epm.event.process.start")
                && event.get_str("beyondtrust_epm.event.process.start") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.process.start") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("beyondtrust_epm.event.process.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.process.start".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_process_start")?;
                event.remove("beyondtrust_epm.event.process.start");
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
            .get("beyondtrust_epm.event.process.start")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.start", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.thread.id") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.thread.id") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.thread.id".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.thread.id", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_thread_id_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.thread.id");
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
            .get("beyondtrust_epm.event.process.thread.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.thread.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.thread.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.thread.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.title")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.title", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.tty")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.tty", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.uptime") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.uptime") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.uptime".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.process.uptime", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_process_uptime_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.uptime");
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
            .get("beyondtrust_epm.event.process.uptime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.uptime", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.user.DefaultTimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.user.DefaultTimezoneOffset".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.DefaultTimezoneOffset",
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
                "convert_process_user_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.process.user.LocalIdentifier") {
                if let Some(val) = event.get("beyondtrust_epm.event.process.user.LocalIdentifier") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.user.LocalIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.LocalIdentifier",
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
                "convert_process_user_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.LocalIdentifier");
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
            if event.has_value("beyondtrust_epm.event.process.user.changes.DefaultTimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.changes.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.process.user.changes.DefaultTimezoneOffset"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.changes.DefaultTimezoneOffset",
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
                "convert_process_user_changes_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.changes.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.process.user.changes.LocalIdentifier") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.changes.LocalIdentifier")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.user.changes.LocalIdentifier"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.changes.LocalIdentifier",
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
                "convert_process_user_changes_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.changes.LocalIdentifier");
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

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.changes.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.changes.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.changes.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.changes.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.changes.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.changes.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.changes.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.changes.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.user.effective.DefaultTimezoneOffset")
            {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.effective.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.process.user.effective.DefaultTimezoneOffset"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.effective.DefaultTimezoneOffset",
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
                "convert_process_user_effective_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.effective.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.process.user.effective.LocalIdentifier") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.effective.LocalIdentifier")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.user.effective.LocalIdentifier"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.effective.LocalIdentifier",
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
                "convert_process_user_effective_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.effective.LocalIdentifier");
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

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.effective.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.effective.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.effective.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.effective.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.effective.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.effective.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.effective.id") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.effective.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.user.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.user.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.id") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.user.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.user.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.process.user.target.DefaultTimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.target.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.user.target.DefaultTimezoneOffset"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.target.DefaultTimezoneOffset",
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
                "convert_process_user_target_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.target.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.process.user.target.LocalIdentifier") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.process.user.target.LocalIdentifier")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.process.user.target.LocalIdentifier"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.process.user.target.LocalIdentifier",
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
                "convert_process_user_target_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.process.user.target.LocalIdentifier");
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

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.target.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.target.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.target.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.target.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.target.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.target.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.target.id") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.target.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.process.user.target.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.process.user.target.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.working_directory")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("process.working_directory", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.code_signature.exists")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.code_signature.exists") {
                event.set("process.code_signature.exists", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.code_signature.subject_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.code_signature.subject_name") {
                event.set("process.code_signature.subject_name", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.code_signature.valid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.code_signature.valid") {
                event.set("process.code_signature.valid", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.hash.md5")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.hash.md5") {
                event.set("process.hash.md5", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.hash.sha1")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.hash.sha1") {
                event.set("process.hash.sha1", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.hash.sha256")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.hash.sha256") {
                event.set("process.hash.sha256", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.name") {
                event.set("process.name", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.executable") {
                event.set("process.executable", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.pe.company")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.pe.company") {
                event.set("process.pe.company", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.pe.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.pe.description") {
                event.set("process.pe.description", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.process.HostedFile.pe.original_file_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("process.pe.original_file_name") {
                event.set("process.pe.original_file_name", v)?;
            }
        }

        let _cond = {
            event.has_value("process")
                && (!event.has_value("event.category")
                    || !(event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("process"))
                        }
                        serde_json::Value::String(s) => s.contains("process"),
                        _ => false,
                    })))
        };
        if _cond {
            event.append_unique("event.category", json!("process"))?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.sections",
                |event| {
                    event.remove("_ingest._value.chi2");
                    event.remove("_ingest._value.entropy");
                    event.remove("_ingest._value.flags");
                    event.remove("_ingest._value.name");
                    event.remove("_ingest._value.physical_offset");
                    event.remove("_ingest._value.physical_size");
                    event.remove("_ingest._value.type");
                    event.remove("_ingest._value.virtual_address");
                    event.remove("_ingest._value.virtual_size");
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.process.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.process.elf.segments",
                |event| {
                    event.remove("_ingest._value.sections");
                    event.remove("_ingest._value.type");
                    Ok(())
                },
            )?;
        }

        event.remove("beyondtrust_epm.event.EPMWinMac.RemotePowerShell.Command");
        event.remove("beyondtrust_epm.event.process.args");
        event.remove("beyondtrust_epm.event.process.args_count");
        event.remove("beyondtrust_epm.event.process.code_signature.digest_algorithm");
        event.remove("beyondtrust_epm.event.process.code_signature.exists");
        event.remove("beyondtrust_epm.event.process.code_signature.signing_id");
        event.remove("beyondtrust_epm.event.process.code_signature.status");
        event.remove("beyondtrust_epm.event.process.code_signature.subject_name");
        event.remove("beyondtrust_epm.event.process.code_signature.team_id");
        event.remove("beyondtrust_epm.event.process.code_signature.timestamp");
        event.remove("beyondtrust_epm.event.process.code_signature.trusted");
        event.remove("beyondtrust_epm.event.process.code_signature.valid");
        event.remove("beyondtrust_epm.event.process.command_line");
        event.remove("beyondtrust_epm.event.process.elf.architecture");
        event.remove("beyondtrust_epm.event.process.elf.byte_order");
        event.remove("beyondtrust_epm.event.process.elf.cpu_type");
        event.remove("beyondtrust_epm.event.process.elf.creation_date");
        event.remove("beyondtrust_epm.event.process.elf.header.abi_version");
        event.remove("beyondtrust_epm.event.process.elf.header.class");
        event.remove("beyondtrust_epm.event.process.elf.header.data");
        event.remove("beyondtrust_epm.event.process.elf.header.entrypoint");
        event.remove("beyondtrust_epm.event.process.elf.header.object_version");
        event.remove("beyondtrust_epm.event.process.elf.header.os_abi");
        event.remove("beyondtrust_epm.event.process.elf.header.type");
        event.remove("beyondtrust_epm.event.process.elf.header.version");
        event.remove("beyondtrust_epm.event.process.elf.shared_libraries");
        event.remove("beyondtrust_epm.event.process.elf.telfhash");
        event.remove("beyondtrust_epm.event.process.end");
        event.remove("beyondtrust_epm.event.process.entity_id");
        event.remove("beyondtrust_epm.event.process.env_vars");
        event.remove("beyondtrust_epm.event.process.executable");
        event.remove("beyondtrust_epm.event.process.exit_code");
        event.remove("beyondtrust_epm.event.process.group.id");
        event.remove("beyondtrust_epm.event.process.group.name");
        event.remove("beyondtrust_epm.event.process.hash.md5");
        event.remove("beyondtrust_epm.event.process.hash.sha1");
        event.remove("beyondtrust_epm.event.process.hash.sha256");
        event.remove("beyondtrust_epm.event.process.hash.sha384");
        event.remove("beyondtrust_epm.event.process.hash.sha512");
        event.remove("beyondtrust_epm.event.process.hash.ssdeep");
        event.remove("beyondtrust_epm.event.process.hash.tlsh");
        event.remove("beyondtrust_epm.event.process.interactive");
        event.remove("beyondtrust_epm.event.process.name");
        event.remove("beyondtrust_epm.event.process.pe.architecture");
        event.remove("beyondtrust_epm.event.process.pe.company");
        event.remove("beyondtrust_epm.event.process.pe.description");
        event.remove("beyondtrust_epm.event.process.pe.file_version");
        event.remove("beyondtrust_epm.event.process.pe.imphash");
        event.remove("beyondtrust_epm.event.process.pe.original_file_name");
        event.remove("beyondtrust_epm.event.process.pe.pehash");
        event.remove("beyondtrust_epm.event.process.pe.product");
        event.remove("beyondtrust_epm.event.process.pid");
        event.remove("beyondtrust_epm.event.process.start");
        event.remove("beyondtrust_epm.event.process.thread.id");
        event.remove("beyondtrust_epm.event.process.thread.name");
        event.remove("beyondtrust_epm.event.process.title");
        event.remove("beyondtrust_epm.event.process.uptime");
        event.remove("beyondtrust_epm.event.process.user.id");
        event.remove("beyondtrust_epm.event.process.user.name");
        event.remove("beyondtrust_epm.event.process.working_directory");

        Ok(TransformResult::Continue)
    }
}
