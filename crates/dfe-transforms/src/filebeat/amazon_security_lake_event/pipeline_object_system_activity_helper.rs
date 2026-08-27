// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_system_activity_helper` pipeline.
pub struct PipelineObjectSystemActivityHelper;

impl Transform for PipelineObjectSystemActivityHelper {
    fn name(&self) -> &str {
        "pipeline_object_system_activity_helper"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("ocsf.class_uid") && ["1002", "1005", "1006", "1007"].contains(&event.get_str("ocsf.class_uid").unwrap_or("")) };
            if _cond {
                event.remove("file.accessed");
                event.remove("file.created");
                event.remove("file.x509.serial_number");
                event.remove("file.x509.not_after");
                event.remove("file.x509.issuer.distinguished_name");
                event.remove("file.x509.subject.distinguished_name");
                event.remove("file.x509.version_number");
                event.remove("file.hash.*");
                event.remove("file.mime_type");
                event.remove("file.mtime");
                event.remove("file.name");
                event.remove("file.owner");
                event.remove("file.uid");
                event.remove("file.directory");
                event.remove("file.path");
                event.remove("file.size");
                event.remove("file.type");
                event.remove("file.inode");
            }

            let _cond = { event.has_value("ocsf.file_result.accessed_time_dt") && event.get_str("ocsf.file_result.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_accessed_time_dt")?;
                        event.remove("ocsf.file_result.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.accessed_time") && event.get_str("ocsf.file_result.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_accessed_time")?;
                        event.remove("ocsf.file_result.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.created_time_dt") && event.get_str("ocsf.file_result.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_created_time_dt")?;
                        event.remove("ocsf.file_result.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.created_time") && event.get_str("ocsf.file_result.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_created_time")?;
                        event.remove("ocsf.file_result.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.modified_time_dt") && event.get_str("ocsf.file_result.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_modified_time_dt")?;
                        event.remove("ocsf.file_result.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.modified_time") && event.get_str("ocsf.file_result.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_modified_time")?;
                        event.remove("ocsf.file_result.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            if event.has_value("ocsf.file_result.size") {
                if let Some(val) = event.get("ocsf.file_result.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.size".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_size_to_long")?;
                        event.remove("ocsf.file_result.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.file_result.signature.certificate.expiration_time_dt") && event.get_str("ocsf.file_result.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.file_result.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.signature.certificate.expiration_time") && event.get_str("ocsf.file_result.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_certificate_expiration_time")?;
                        event.remove("ocsf.file_result.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.file_result.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.file_result.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.accessor.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file_result.accessor.type_id") {
                if let Some(val) = event.get("ocsf.file_result.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.file_result.attributes") {
                if let Some(val) = event.get("ocsf.file_result.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_attributes_to_long")?;
                        event.remove("ocsf.file_result.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.file_result.confidentiality_id") {
                if let Some(val) = event.get("ocsf.file_result.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.file_result.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.file_result.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file_result.creator.type_id") {
                if let Some(val) = event.get("ocsf.file_result.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.file_result.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.file_result.signature.certificate.fingerprints", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.file_result.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.file_result.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.file_result.signature.certificate.created_time_dt") && event.get_str("ocsf.file_result.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.file_result.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.signature.certificate.created_time") && event.get_str("ocsf.file_result.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_certificate_created_time")?;
                        event.remove("ocsf.file_result.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.signature.created_time_dt") && event.get_str("ocsf.file_result.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_created_time_dt")?;
                        event.remove("ocsf.file_result.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file_result.signature.created_time") && event.get_str("ocsf.file_result.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file_result.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file_result.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file_result.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_result_signature_created_time")?;
                        event.remove("ocsf.file_result.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.file_result.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.file_result.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.file_result.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.file_result.hashes", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.file_result.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.file_result.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file_result.modifier.type_id") {
                if let Some(val) = event.get("ocsf.file_result.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file_result.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.file_result.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file_result.owner.type_id") {
                if let Some(val) = event.get("ocsf.file_result.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.file_result.is_system") {
                if let Some(val) = event.get("ocsf.file_result.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_result_is_system_to_boolean")?;
                        event.remove("ocsf.file_result.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.file_result.type_id") {
                if let Some(val) = event.get("ocsf.file_result.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file_result.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file_result.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.driver.file.accessed_time_dt") && event.get_str("ocsf.driver.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_accessed_time_dt")?;
                        event.remove("ocsf.driver.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.accessed_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.accessed_time") && event.get_str("ocsf.driver.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_accessed_time")?;
                        event.remove("ocsf.driver.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.accessed_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.created_time_dt") && event.get_str("ocsf.driver.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_created_time_dt")?;
                        event.remove("ocsf.driver.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.created_time") && event.get_str("ocsf.driver.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_created_time")?;
                        event.remove("ocsf.driver.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.parent_folder").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.directory", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.hashes") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.driver.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.driver.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-512\":\"sha512\",\"CTPH\":\"ssdeep\",\"TLSH\":\"tlsh\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_file_hash_*")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.driver.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.driver.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.driver.file.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.inode", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mime_type", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modified_time_dt") && event.get_str("ocsf.driver.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_driver_file_modified_time_dt")?;
                        event.remove("ocsf.driver.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.modified_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modified_time") && event.get_str("ocsf.driver.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_modified_time")?;
                        event.remove("ocsf.driver.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.modified_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.owner.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.owner", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.driver.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.driver.file.size") {
                if let Some(val) = event.get("ocsf.driver.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size_to_long")?;
                        event.remove("ocsf.driver.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.driver.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.owner.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.uid", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.driver.file.signature.certificate.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.issuer.distinguished_name", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.driver.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.driver.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.signature.certificate.expiration_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.certificate.expiration_time") && event.get_str("ocsf.driver.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.driver.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.driver.file.signature.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.signature.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.signature.certificate.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("ocsf.driver.file.signature.certificate.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.version_number", v)?;
            }

            if event.has_value("ocsf.driver.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.driver.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.driver.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.driver.file.attributes") {
                if let Some(val) = event.get("ocsf.driver.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_attributes_to_long")?;
                        event.remove("ocsf.driver.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.driver.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.driver.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.driver.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.driver.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.driver.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.driver.file.signature.certificate.fingerprints", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.driver.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.driver.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.driver.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.driver.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.driver.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.certificate.created_time_dt") && event.get_str("ocsf.driver.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.driver.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.certificate.created_time") && event.get_str("ocsf.driver.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_created_time")?;
                        event.remove("ocsf.driver.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.created_time_dt") && event.get_str("ocsf.driver.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_created_time_dt")?;
                        event.remove("ocsf.driver.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.created_time") && event.get_str("ocsf.driver.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.driver.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.driver.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.driver.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_created_time")?;
                        event.remove("ocsf.driver.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.driver.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.driver.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.driver.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.driver.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.driver.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.driver.file.hashes", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.driver.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.driver.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.driver.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.driver.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.driver.file.is_system") {
                if let Some(val) = event.get("ocsf.driver.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_is_system_to_boolean")?;
                        event.remove("ocsf.driver.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.driver.file.type_id") {
                if let Some(val) = event.get("ocsf.driver.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.driver.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.driver.file.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.driver.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.driver.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.driver.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.accessed_time_dt") && event.get_str("ocsf.module.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_accessed_time_dt")?;
                        event.remove("ocsf.module.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.accessed_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.accessed_time") && event.get_str("ocsf.module.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_accessed_time")?;
                        event.remove("ocsf.module.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.accessed_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.created_time_dt") && event.get_str("ocsf.module.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_created_time_dt")?;
                        event.remove("ocsf.module.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.created_time") && event.get_str("ocsf.module.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_created_time")?;
                        event.remove("ocsf.module.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.parent_folder").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.directory", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.hashes") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.module.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.module.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-512\":\"sha512\",\"CTPH\":\"ssdeep\",\"TLSH\":\"tlsh\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_module_file_hash_*")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.module.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.module.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.module.file.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.inode", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mime_type", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.modified_time_dt") && event.get_str("ocsf.module.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_modified_time_dt")?;
                        event.remove("ocsf.module.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.modified_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.modified_time") && event.get_str("ocsf.module.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_modified_time")?;
                        event.remove("ocsf.module.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.modified_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.owner.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.owner", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.module.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.module.file.size") {
                if let Some(val) = event.get("ocsf.module.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_module_file_size_to_long")?;
                        event.remove("ocsf.module.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.module.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.owner.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.uid", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.module.file.signature.certificate.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.issuer.distinguished_name", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.module.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.module.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.signature.certificate.expiration_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            let _cond = { event.has_value("ocsf.module.file.signature.certificate.expiration_time") && event.get_str("ocsf.module.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.module.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.module.file.signature.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.signature.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.signature.certificate.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("ocsf.module.file.signature.certificate.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.version_number", v)?;
            }

            if event.has_value("ocsf.module.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.module.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.module.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.module.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.module.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.module.file.attributes") {
                if let Some(val) = event.get("ocsf.module.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_module_file_attributes_to_long")?;
                        event.remove("ocsf.module.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.module.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.module.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.module.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.module.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.module.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.module.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.module.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.module.file.signature.certificate.fingerprints", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.module.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.module.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.module.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.module.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.module.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.module.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.signature.certificate.created_time_dt") && event.get_str("ocsf.module.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.module.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.module.file.signature.certificate.created_time") && event.get_str("ocsf.module.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_signature_certificate_created_time")?;
                        event.remove("ocsf.module.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.module.file.signature.created_time_dt") && event.get_str("ocsf.module.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_signature_created_time_dt")?;
                        event.remove("ocsf.module.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.module.file.signature.created_time") && event.get_str("ocsf.module.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.module.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.module.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.module.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_module_file_signature_created_time")?;
                        event.remove("ocsf.module.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.module.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.module.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.module.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.module.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.module.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.module.file.hashes", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.module.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.module.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.module.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.module.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.module.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.module.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.module.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.module.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.module.file.is_system") {
                if let Some(val) = event.get("ocsf.module.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_module_file_is_system_to_boolean")?;
                        event.remove("ocsf.module.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.module.file.type_id") {
                if let Some(val) = event.get("ocsf.module.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.module.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.module.file.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.module.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.module.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.module.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.accessed_time_dt") && event.get_str("ocsf.job.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_accessed_time_dt")?;
                        event.remove("ocsf.job.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.accessed_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.accessed_time") && event.get_str("ocsf.job.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_accessed_time")?;
                        event.remove("ocsf.job.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.accessed_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.created_time_dt") && event.get_str("ocsf.job.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_created_time_dt")?;
                        event.remove("ocsf.job.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.created_time") && event.get_str("ocsf.job.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_created_time")?;
                        event.remove("ocsf.job.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.parent_folder").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.directory", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.hashes") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.job.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.job.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-512\":\"sha512\",\"CTPH\":\"ssdeep\",\"TLSH\":\"tlsh\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_job_file_hash_*")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.job.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.job.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.job.file.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.inode", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mime_type", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.modified_time_dt") && event.get_str("ocsf.job.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_modified_time_dt")?;
                        event.remove("ocsf.job.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.modified_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.modified_time") && event.get_str("ocsf.job.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_modified_time")?;
                        event.remove("ocsf.job.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.modified_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.owner.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.owner", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.job.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.job.file.size") {
                if let Some(val) = event.get("ocsf.job.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_job_file_size_to_long")?;
                        event.remove("ocsf.job.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.job.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.owner.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.uid", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.job.file.signature.certificate.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.issuer.distinguished_name", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.job.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.job.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.signature.certificate.expiration_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            let _cond = { event.has_value("ocsf.job.file.signature.certificate.expiration_time") && event.get_str("ocsf.job.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.job.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.job.file.signature.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.signature.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.signature.certificate.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("ocsf.job.file.signature.certificate.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.version_number", v)?;
            }

            if event.has_value("ocsf.job.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.job.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.job.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.job.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.job.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.job.file.attributes") {
                if let Some(val) = event.get("ocsf.job.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_job_file_attributes_to_long")?;
                        event.remove("ocsf.job.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.job.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.job.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.job.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.job.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.job.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.job.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.job.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.job.file.signature.certificate.fingerprints", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.job.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.job.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.job.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.job.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.job.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.job.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.signature.certificate.created_time_dt") && event.get_str("ocsf.job.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.job.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.job.file.signature.certificate.created_time") && event.get_str("ocsf.job.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_signature_certificate_created_time")?;
                        event.remove("ocsf.job.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.job.file.signature.created_time_dt") && event.get_str("ocsf.job.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_signature_created_time_dt")?;
                        event.remove("ocsf.job.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.job.file.signature.created_time") && event.get_str("ocsf.job.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.job.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.job.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.job.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_job_file_signature_created_time")?;
                        event.remove("ocsf.job.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.job.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.job.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.job.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.job.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.job.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.job.file.hashes", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.job.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.job.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.job.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.job.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.job.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.job.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.job.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.job.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.job.file.is_system") {
                if let Some(val) = event.get("ocsf.job.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_job_file_is_system_to_boolean")?;
                        event.remove("ocsf.job.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.job.file.type_id") {
                if let Some(val) = event.get("ocsf.job.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.job.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.job.file.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.job.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.job.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.job.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
