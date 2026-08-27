// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_category_identity_and_access_management` pipeline.
pub struct PipelineCategoryIdentityAndAccessManagement;

impl Transform for PipelineCategoryIdentityAndAccessManagement {
    fn name(&self) -> &str {
        "pipeline_category_identity_and_access_management"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("ocsf.user_result.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.changes.domain", v)?;
            }

            if let Some(v) = event.get("ocsf.user_result.email_addr").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.changes.email", v)?;
            }

            let _cond = { event.has_value("ocsf.user_result.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user_result.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.user_result.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.changes.full_name", v)?;
            }

            let _cond = { event.has_value("ocsf.user_result.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user_result.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.user_result.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.user_result.groups", |event| {
                    event.append_unique("user.changes.group.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.user_result.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.user_result.groups", |event| {
                    event.append_unique("user.changes.group.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.user_result.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.changes.id", v)?;
            }

            let _cond = { event.has_value("ocsf.user_result.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user_result.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.user_result.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.changes.name", v)?;
            }

            let _cond = { event.has_value("ocsf.user_result.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user_result.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.user_result.account.type_id") {
                if let Some(val) = event.get("ocsf.user_result.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.user_result.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.user_result.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.user_result.type_id") {
                if let Some(val) = event.get("ocsf.user_result.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.user_result.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.user_result.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.user_result.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user_result.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.auth_protocol_id") {
                if let Some(val) = event.get("ocsf.auth_protocol_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.auth_protocol_id".into(),
                            message,
                        })?;
                    event.set("ocsf.auth_protocol_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.certificate.created_time_dt") && event.get_str("ocsf.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_certificate_created_time_dt")?;
                        event.remove("ocsf.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.certificate.created_time") && event.get_str("ocsf.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_certificate_created_time")?;
                        event.remove("ocsf.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.certificate.expiration_time_dt") && event.get_str("ocsf.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_certificate_expiration_time_dt")?;
                        event.remove("ocsf.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.certificate.expiration_time") && event.get_str("ocsf.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_certificate_expiration_time")?;
                        event.remove("ocsf.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.is_cleartext") {
                if let Some(val) = event.get("ocsf.is_cleartext") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.is_cleartext".into(),
                            message,
                        })?;
                    event.set("ocsf.is_cleartext", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_is_cleartext_to_boolean")?;
                        event.remove("ocsf.is_cleartext");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.container.hash.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.logon_process.container.hash.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessed_time_dt") && event.get_str("ocsf.logon_process.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_accessed_time_dt")?;
                        event.remove("ocsf.logon_process.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessed_time") && event.get_str("ocsf.logon_process.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_accessed_time")?;
                        event.remove("ocsf.logon_process.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.created_time_dt") && event.get_str("ocsf.logon_process.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_created_time_dt")?;
                        event.remove("ocsf.logon_process.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.created_time") && event.get_str("ocsf.logon_process.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_created_time")?;
                        event.remove("ocsf.logon_process.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.logon_process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modified_time_dt") && event.get_str("ocsf.logon_process.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_modified_time_dt")?;
                        event.remove("ocsf.logon_process.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modified_time") && event.get_str("ocsf.logon_process.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_modified_time")?;
                        event.remove("ocsf.logon_process.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.file.size") {
                if let Some(val) = event.get("ocsf.logon_process.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_file_size_to_long")?;
                        event.remove("ocsf.logon_process.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.logon_process.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.logon_process.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.certificate.expiration_time") && event.get_str("ocsf.logon_process.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.logon_process.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.terminated_time_dt") && event.get_str("ocsf.logon_process.terminated_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.terminated_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.terminated_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.terminated_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_terminated_time_dt")?;
                        event.remove("ocsf.logon_process.terminated_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.terminated_time") && event.get_str("ocsf.logon_process.terminated_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.terminated_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.terminated_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.terminated_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_terminated_time")?;
                        event.remove("ocsf.logon_process.terminated_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.logon_process.egid") {
                if let Some(val) = event.get("ocsf.logon_process.egid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.egid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.egid", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.pid") {
                if let Some(val) = event.get("ocsf.logon_process.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.pid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_pid_to_long")?;
                        event.remove("ocsf.logon_process.pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.created_time_dt") && event.get_str("ocsf.logon_process.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_created_time_dt")?;
                        event.remove("ocsf.logon_process.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.created_time") && event.get_str("ocsf.logon_process.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_created_time")?;
                        event.remove("ocsf.logon_process.created_time");
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
            if event.has_value("ocsf.logon_process.tid") {
                if let Some(val) = event.get("ocsf.logon_process.tid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.tid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.tid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_tid_to_long")?;
                        event.remove("ocsf.logon_process.tid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.user.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.user.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.user.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.user.full_name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_process.euid") {
                if let Some(val) = event.get("ocsf.logon_process.euid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.euid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.euid", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.euid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.euid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.user.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.user.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_process.container.hash.algorithm_id") {
                if let Some(val) = event.get("ocsf.logon_process.container.hash.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.container.hash.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.container.hash.algorithm_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.auid") {
                if let Some(val) = event.get("ocsf.logon_process.auid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.auid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.auid", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.container.size") {
                if let Some(val) = event.get("ocsf.logon_process.container.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.container.size".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.container.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_container_size_to_long")?;
                        event.remove("ocsf.logon_process.container.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_process.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.file.attributes") {
                if let Some(val) = event.get("ocsf.logon_process.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_file_attributes_to_long")?;
                        event.remove("ocsf.logon_process.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.logon_process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.file.signature.certificate.fingerprints", |event| {
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

            let _cond = { event.get("ocsf.logon_process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.logon_process.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.logon_process.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.certificate.created_time_dt") && event.get_str("ocsf.logon_process.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.logon_process.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.certificate.created_time") && event.get_str("ocsf.logon_process.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_signature_certificate_created_time")?;
                        event.remove("ocsf.logon_process.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.created_time_dt") && event.get_str("ocsf.logon_process.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_signature_created_time_dt")?;
                        event.remove("ocsf.logon_process.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.created_time") && event.get_str("ocsf.logon_process.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_file_signature_created_time")?;
                        event.remove("ocsf.logon_process.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.logon_process.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.logon_process.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.logon_process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.file.hashes", |event| {
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

            if event.has_value("ocsf.logon_process.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.file.is_system") {
                if let Some(val) = event.get("ocsf.logon_process.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_file_is_system_to_boolean")?;
                        event.remove("ocsf.logon_process.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.file.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.file.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.integrity_id") {
                if let Some(val) = event.get("ocsf.logon_process.integrity_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.integrity_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.integrity_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.namespace_pid") {
                if let Some(val) = event.get("ocsf.logon_process.namespace_pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.namespace_pid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.namespace_pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_namespace_pid_to_long")?;
                        event.remove("ocsf.logon_process.namespace_pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def process = ctx.ocsf.logon_process.parent_process;\ndef count = 0;\nwhile (true) {\n  if (process != null && process.parent_process != null) {\n    count += 1;\n    process = process.parent_process;\n  } else {\n    break;\n  }\n}\nif (count >= 15) {\n  ctx.ocsf.logon_process.parent_process.put(\"parent_process_keyword\", ctx.ocsf.logon_process.parent_process.parent_process.toString());\n  ctx.ocsf.logon_process.parent_process.remove(\"parent_process\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def process = ctx.ocsf.logon_process.parent_process;\ndef count = 0;\nwhile (true) {\n  if (process != null && process.parent_process != null) {\n    count += 1;\n    process = process.parent_process;\n  } else {\n    break;\n  }\n}\nif (count >= 15) {\n  ctx.ocsf.logon_process.parent_process.put(\"parent_process_keyword\", ctx.ocsf.logon_process.parent_process.parent_process.toString());\n  ctx.ocsf.logon_process.parent_process.remove(\"parent_process\");\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_logon_process_parent_process_stringify")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.container.hash.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.logon_process.parent_process.container.hash.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessed_time_dt") && event.get_str("ocsf.logon_process.parent_process.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_accessed_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessed_time") && event.get_str("ocsf.logon_process.parent_process.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_accessed_time")?;
                        event.remove("ocsf.logon_process.parent_process.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.created_time_dt") && event.get_str("ocsf.logon_process.parent_process.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_created_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.created_time") && event.get_str("ocsf.logon_process.parent_process.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_created_time")?;
                        event.remove("ocsf.logon_process.parent_process.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.logon_process.parent_process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.parent_process.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modified_time_dt") && event.get_str("ocsf.logon_process.parent_process.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_modified_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modified_time") && event.get_str("ocsf.logon_process.parent_process.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_modified_time")?;
                        event.remove("ocsf.logon_process.parent_process.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.file.size") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_file_size_to_long")?;
                        event.remove("ocsf.logon_process.parent_process.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time") && event.get_str("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.logon_process.parent_process.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.terminated_time_dt") && event.get_str("ocsf.logon_process.parent_process.terminated_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.terminated_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.terminated_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.terminated_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_terminated_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.terminated_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.terminated_time") && event.get_str("ocsf.logon_process.parent_process.terminated_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.terminated_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.terminated_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.terminated_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_terminated_time")?;
                        event.remove("ocsf.logon_process.parent_process.terminated_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.logon_process.parent_process.egid") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.egid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.egid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.egid", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.pid") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.pid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_pid_to_long")?;
                        event.remove("ocsf.logon_process.parent_process.pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.created_time_dt") && event.get_str("ocsf.logon_process.parent_process.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_created_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.created_time") && event.get_str("ocsf.logon_process.parent_process.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_created_time")?;
                        event.remove("ocsf.logon_process.parent_process.created_time");
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
            if event.has_value("ocsf.logon_process.parent_process.tid") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.tid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.tid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.tid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_tid_to_long")?;
                        event.remove("ocsf.logon_process.parent_process.tid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.user.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.user.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.user.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.user.full_name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_process.parent_process.euid") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.euid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.euid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.euid", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.euid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.euid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.user.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.user.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_process.parent_process.container.hash.algorithm_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.container.hash.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.container.hash.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.container.hash.algorithm_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.auid") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.auid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.auid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.auid", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.container.size") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.container.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.container.size".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.container.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_container_size_to_long")?;
                        event.remove("ocsf.logon_process.parent_process.container.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_process.parent_process.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.file.attributes") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_file_attributes_to_long")?;
                        event.remove("ocsf.logon_process.parent_process.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.logon_process.parent_process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.parent_process.file.signature.certificate.fingerprints", |event| {
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

            let _cond = { event.get("ocsf.logon_process.parent_process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.parent_process.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.logon_process.parent_process.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.logon_process.parent_process.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.certificate.created_time_dt") && event.get_str("ocsf.logon_process.parent_process.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.certificate.created_time") && event.get_str("ocsf.logon_process.parent_process.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_signature_certificate_created_time")?;
                        event.remove("ocsf.logon_process.parent_process.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.created_time_dt") && event.get_str("ocsf.logon_process.parent_process.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_signature_created_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.created_time") && event.get_str("ocsf.logon_process.parent_process.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_file_signature_created_time")?;
                        event.remove("ocsf.logon_process.parent_process.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.logon_process.parent_process.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.logon_process.parent_process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.logon_process.parent_process.file.hashes", |event| {
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

            if event.has_value("ocsf.logon_process.parent_process.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.file.is_system") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_file_is_system_to_boolean")?;
                        event.remove("ocsf.logon_process.parent_process.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.file.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.file.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.integrity_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.integrity_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.integrity_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.integrity_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.namespace_pid") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.namespace_pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.namespace_pid".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.namespace_pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_namespace_pid_to_long")?;
                        event.remove("ocsf.logon_process.parent_process.namespace_pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.session.created_time_dt") && event.get_str("ocsf.logon_process.parent_process.session.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.session.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.session.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.session.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_session_created_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.session.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.session.created_time") && event.get_str("ocsf.logon_process.parent_process.session.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.session.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.session.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.session.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_session_created_time")?;
                        event.remove("ocsf.logon_process.parent_process.session.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.session.expiration_time_dt") && event.get_str("ocsf.logon_process.parent_process.session.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.session.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.session.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.session.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_session_expiration_time_dt")?;
                        event.remove("ocsf.logon_process.parent_process.session.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.session.expiration_time") && event.get_str("ocsf.logon_process.parent_process.session.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.parent_process.session.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.parent_process.session.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.parent_process.session.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_parent_process_session_expiration_time")?;
                        event.remove("ocsf.logon_process.parent_process.session.expiration_time");
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
            if event.has_value("ocsf.logon_process.parent_process.session.mfa") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.session.mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.session.mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.session.mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_session_mfa_to_boolean")?;
                        event.remove("ocsf.logon_process.parent_process.session.mfa");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.parent_process.session.is_remote") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.session.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.session.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.session.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_parent_process_session_is_remote_to_boolean")?;
                        event.remove("ocsf.logon_process.parent_process.session.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.user.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.user.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.user.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.user.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.parent_process.user.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.parent_process.user.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.parent_process.user.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.parent_process.user.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.parent_process.user.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.parent_process.user.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.session.created_time_dt") && event.get_str("ocsf.logon_process.session.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.session.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.session.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.session.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_session_created_time_dt")?;
                        event.remove("ocsf.logon_process.session.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.session.created_time") && event.get_str("ocsf.logon_process.session.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.session.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.session.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.session.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_session_created_time")?;
                        event.remove("ocsf.logon_process.session.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.session.expiration_time_dt") && event.get_str("ocsf.logon_process.session.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.session.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.session.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.session.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_session_expiration_time_dt")?;
                        event.remove("ocsf.logon_process.session.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.logon_process.session.expiration_time") && event.get_str("ocsf.logon_process.session.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.logon_process.session.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.logon_process.session.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.logon_process.session.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_logon_process_session_expiration_time")?;
                        event.remove("ocsf.logon_process.session.expiration_time");
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
            if event.has_value("ocsf.logon_process.session.mfa") {
                if let Some(val) = event.get("ocsf.logon_process.session.mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.session.mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.session.mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_session_mfa_to_boolean")?;
                        event.remove("ocsf.logon_process.session.mfa");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.logon_process.session.is_remote") {
                if let Some(val) = event.get("ocsf.logon_process.session.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.session.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.session.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_logon_process_session_is_remote_to_boolean")?;
                        event.remove("ocsf.logon_process.session.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.logon_process.user.account.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.user.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.user.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.user.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.logon_process.user.type_id") {
                if let Some(val) = event.get("ocsf.logon_process.user.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_process.user.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_process.user.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.logon_process.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.logon_process.user.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.logon_process.user.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.logon_type_id") {
                if let Some(val) = event.get("ocsf.logon_type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.logon_type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.logon_type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.is_mfa") {
                if let Some(val) = event.get("ocsf.is_mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.is_mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.is_mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_is_mfa_to_boolean")?;
                        event.remove("ocsf.is_mfa");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.is_new_logon") {
                if let Some(val) = event.get("ocsf.is_new_logon") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.is_new_logon".into(),
                            message,
                        })?;
                    event.set("ocsf.is_new_logon", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_is_new_logon_to_boolean")?;
                        event.remove("ocsf.is_new_logon");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.is_remote") {
                if let Some(val) = event.get("ocsf.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_is_remote_to_boolean")?;
                        event.remove("ocsf.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.service.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("service.name", v)?;
            }

            if let Some(v) = event.get("ocsf.service.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("service.id", v)?;
            }

            if let Some(v) = event.get("ocsf.service.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("service.version", v)?;
            }

            let _cond = { event.has_value("ocsf.session.created_time_dt") && event.get_str("ocsf.session.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.session.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.session.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.session.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_session_created_time_dt")?;
                        event.remove("ocsf.session.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.session.created_time") && event.get_str("ocsf.session.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.session.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.session.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.session.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_session_created_time")?;
                        event.remove("ocsf.session.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.session.expiration_time_dt") && event.get_str("ocsf.session.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.session.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.session.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.session.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_session_expiration_time_dt")?;
                        event.remove("ocsf.session.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.session.expiration_time") && event.get_str("ocsf.session.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.session.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.session.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.session.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_session_expiration_time")?;
                        event.remove("ocsf.session.expiration_time");
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
            if event.has_value("ocsf.session.is_remote") {
                if let Some(val) = event.get("ocsf.session.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.session.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.session.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_is_remote_to_boolean")?;
                        event.remove("ocsf.session.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.session.mfa") {
                if let Some(val) = event.get("ocsf.session.mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.session.mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.session.mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_mfa_to_boolean")?;
                        event.remove("ocsf.session.mfa");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("group.name", v)?;
            }

            if let Some(v) = event.get("ocsf.group.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("group.id", v)?;
            }

            if event.has_value("ocsf.resource.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.resource.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.resource.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.resource.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.resource.owner.type_id") {
                if let Some(val) = event.get("ocsf.resource.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.resource.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.resource.owner.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.resource.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.resource.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.resource.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.resource.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.resource.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.resource.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.resource.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.resource.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.resource.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.resource.owner.uid").map_or_else(String::new, template_to_string)))?;
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
