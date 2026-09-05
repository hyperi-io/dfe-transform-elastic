// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_identity_user` pipeline.
pub struct PipelineIdentityUser;

impl Transform for PipelineIdentityUser {
    fn name(&self) -> &str {
        "pipeline_identity_user"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if let Some(v) = event
            .get("beyondtrust_epm.event.EPMWinMac.PrivilegedGroup.Name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("group.name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.extension",
                    json!(
                        event
                            .get("_ingest._value.file.extension")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.md5",
                    json!(
                        event
                            .get("_ingest._value.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.sha1",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.sha256",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.sha384",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha384")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha384")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.sha512",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha512")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.sha512")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.ssdeep",
                    json!(
                        event
                            .get("_ingest._value.file.hash.ssdeep")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.ssdeep")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.hash.tlsh",
                    json!(
                        event
                            .get("_ingest._value.file.hash.tlsh")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_ingest._value.file.hash.tlsh")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.mime_type",
                    json!(
                        event
                            .get("_ingest._value.file.mime_type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.name",
                    json!(
                        event
                            .get("_ingest._value.file.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.file.size") {
                        if let Some(val) = event.get("_ingest._value.file.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.file.size".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.file.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_attachments_file_size_to_long",
                    )?;
                    event.remove("_ingest._value.file.size");
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
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.append_unique(
                    "email.attachments.file.size",
                    json!(
                        event
                            .get("_ingest._value.file.size")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.bcc.address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.bcc.address", |event| {
                event.append_unique(
                    "email.bcc.address",
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
                .get("beyondtrust_epm.event.email.cc.address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.cc.address", |event| {
                event.append_unique(
                    "email.cc.address",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.email.content_type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.content_type", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.email.delivery_timestamp")
                && event.get_str("beyondtrust_epm.event.email.delivery_timestamp") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.email.delivery_timestamp")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.email.delivery_timestamp", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.email.delivery_timestamp".into(),
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
                    "date_email_delivery_timestamp",
                )?;
                event.remove("beyondtrust_epm.event.email.delivery_timestamp");
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
            .get("beyondtrust_epm.event.email.delivery_timestamp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.delivery_timestamp", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.email.direction")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.direction", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.from.address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.from.address", |event| {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.email.local_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.local_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.email.message_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.message_id", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.email.origination_timestamp")
                && event.get_str("beyondtrust_epm.event.email.origination_timestamp") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.email.origination_timestamp")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.email.origination_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.email.origination_timestamp".into(),
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
                    "date_email_origination_timestamp",
                )?;
                event.remove("beyondtrust_epm.event.email.origination_timestamp");
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
            .get("beyondtrust_epm.event.email.origination_timestamp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.origination_timestamp", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.reply_to.address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.email.reply_to.address",
                |event| {
                    event.append_unique(
                        "email.reply_to.address",
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
            .get("beyondtrust_epm.event.email.sender.address")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.sender.address", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.email.subject")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.subject", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.to.address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.to.address", |event| {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.email.x_mailer")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("email.x_mailer", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.group.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("group.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("group.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.group.name") };
        if _cond {
            event.append_unique(
                "group.name",
                json!(
                    event
                        .get("beyondtrust_epm.event.group.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.user.DefaultTimezoneOffset") {
                if let Some(val) = event.get("beyondtrust_epm.event.user.DefaultTimezoneOffset") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.DefaultTimezoneOffset".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.DefaultTimezoneOffset",
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
                "convert_user_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.user.LocalIdentifier") {
                if let Some(val) = event.get("beyondtrust_epm.event.user.LocalIdentifier") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.LocalIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.user.LocalIdentifier", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_user_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.LocalIdentifier");
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
            if event.has_value("beyondtrust_epm.event.user.changes.DefaultTimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.user.changes.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.changes.DefaultTimezoneOffset".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.changes.DefaultTimezoneOffset",
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
                "convert_user_changes_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.changes.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.user.changes.LocalIdentifier") {
                if let Some(val) = event.get("beyondtrust_epm.event.user.changes.LocalIdentifier") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.changes.LocalIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.changes.LocalIdentifier",
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
                "convert_user_changes_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.changes.LocalIdentifier");
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
            .get("beyondtrust_epm.event.user.changes.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.email")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.changes.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.changes.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.full_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.changes.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.changes.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.group.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.group.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.group.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.group.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.group.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.hash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.changes.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.changes.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.changes.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.changes.name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.user.changes.roles")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.user.changes.roles", |event| {
                event.append_unique(
                    "user.changes.roles",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.user.effective.DefaultTimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.user.effective.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.effective.DefaultTimezoneOffset"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.effective.DefaultTimezoneOffset",
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
                "convert_user_effective_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.effective.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.user.effective.LocalIdentifier") {
                if let Some(val) = event.get("beyondtrust_epm.event.user.effective.LocalIdentifier")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.effective.LocalIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.effective.LocalIdentifier",
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
                "convert_user_effective_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.effective.LocalIdentifier");
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
            .get("beyondtrust_epm.event.user.effective.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.email")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.effective.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.effective.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.full_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.effective.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.effective.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.group.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.group.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.group.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.group.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.group.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.hash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.effective.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.effective.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.effective.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.effective.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.effective.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.effective.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.user.effective.roles")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.user.effective.roles",
                |event| {
                    event.append_unique(
                        "user.effective.roles",
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
            .get("beyondtrust_epm.event.user.email")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.full_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.group.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.group.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.group.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.group.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.group.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.hash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.user.roles")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.user.roles", |event| {
                event.append_unique(
                    "user.roles",
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
            if event.has_value("beyondtrust_epm.event.user.target.DefaultTimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.user.target.DefaultTimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.target.DefaultTimezoneOffset".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.target.DefaultTimezoneOffset",
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
                "convert_user_target_DefaultTimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.target.DefaultTimezoneOffset");
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
            if event.has_value("beyondtrust_epm.event.user.target.LocalIdentifier") {
                if let Some(val) = event.get("beyondtrust_epm.event.user.target.LocalIdentifier") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.user.target.LocalIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.user.target.LocalIdentifier",
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
                "convert_user_target_LocalIdentifier_to_long",
            )?;
            event.remove("beyondtrust_epm.event.user.target.LocalIdentifier");
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
            .get("beyondtrust_epm.event.user.target.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.email")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.target.email") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.target.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.full_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.target.full_name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.target.full_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.group.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.group.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.group.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.group.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.group.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.hash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.target.hash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.target.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user.target.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.target.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.user.target.name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.user.target.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.device.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.device.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.original")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.original", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.family")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.family", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.full")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.full", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.kernel")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.kernel", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.platform")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.platform", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.os.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.os.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.user_agent.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user_agent.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.client.user.DomainIdentifier")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("user.id") {
                event.set("user.id", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.client.user.DomainNetBIOSName")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("user.domain") {
                event.set("user.domain", v)?;
            }
        }

        let _cond = {
            event.get("email.attachments").is_some_and(|v| v.is_array()) && event.has_value("email.attachments") && event.get("email.attachments").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
        };
        if _cond {
            foreach_array(event, "email.attachments.file.size", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                })();
                Ok(())
            })?;
        }

        let _cond = {
            event.has_value("email")
                && (!event.has_value("event.category")
                    || !(event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("email"))
                        }
                        serde_json::Value::String(s) => s.contains("email"),
                        _ => false,
                    })))
        };
        if _cond {
            event.append_unique("event.category", json!("email"))?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.email.attachments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.email.attachments", |event| {
                event.remove("_ingest._value.file.extension");
                event.remove("_ingest._value.file.hash.md5");
                event.remove("_ingest._value.file.hash.sha1");
                event.remove("_ingest._value.file.hash.sha256");
                event.remove("_ingest._value.file.hash.sha384");
                event.remove("_ingest._value.file.hash.sha512");
                event.remove("_ingest._value.file.hash.ssdeep");
                event.remove("_ingest._value.file.hash.tlsh");
                event.remove("_ingest._value.file.mime_type");
                event.remove("_ingest._value.file.name");
                event.remove("_ingest._value.file.size");
                Ok(())
            })?;
        }

        event.remove("beyondtrust_epm.event.EPMWinMac.PrivilegedGroup.Name");
        event.remove("beyondtrust_epm.event.email.bcc.address");
        event.remove("beyondtrust_epm.event.email.cc.address");
        event.remove("beyondtrust_epm.event.email.content_type");
        event.remove("beyondtrust_epm.event.email.delivery_timestamp");
        event.remove("beyondtrust_epm.event.email.direction");
        event.remove("beyondtrust_epm.event.email.from.address");
        event.remove("beyondtrust_epm.event.email.local_id");
        event.remove("beyondtrust_epm.event.email.message_id");
        event.remove("beyondtrust_epm.event.email.origination_timestamp");
        event.remove("beyondtrust_epm.event.email.reply_to.address");
        event.remove("beyondtrust_epm.event.email.sender.address");
        event.remove("beyondtrust_epm.event.email.subject");
        event.remove("beyondtrust_epm.event.email.to.address");
        event.remove("beyondtrust_epm.event.email.x_mailer");
        event.remove("beyondtrust_epm.event.group.domain");
        event.remove("beyondtrust_epm.event.group.id");
        event.remove("beyondtrust_epm.event.group.name");
        event.remove("beyondtrust_epm.event.user.changes.domain");
        event.remove("beyondtrust_epm.event.user.changes.email");
        event.remove("beyondtrust_epm.event.user.changes.full_name");
        event.remove("beyondtrust_epm.event.user.changes.group.domain");
        event.remove("beyondtrust_epm.event.user.changes.group.id");
        event.remove("beyondtrust_epm.event.user.changes.group.name");
        event.remove("beyondtrust_epm.event.user.changes.hash");
        event.remove("beyondtrust_epm.event.user.changes.id");
        event.remove("beyondtrust_epm.event.user.changes.name");
        event.remove("beyondtrust_epm.event.user.changes.roles");
        event.remove("beyondtrust_epm.event.user.domain");
        event.remove("beyondtrust_epm.event.user.effective.domain");
        event.remove("beyondtrust_epm.event.user.effective.email");
        event.remove("beyondtrust_epm.event.user.effective.full_name");
        event.remove("beyondtrust_epm.event.user.effective.group.domain");
        event.remove("beyondtrust_epm.event.user.effective.group.id");
        event.remove("beyondtrust_epm.event.user.effective.group.name");
        event.remove("beyondtrust_epm.event.user.effective.hash");
        event.remove("beyondtrust_epm.event.user.effective.id");
        event.remove("beyondtrust_epm.event.user.effective.name");
        event.remove("beyondtrust_epm.event.user.effective.roles");
        event.remove("beyondtrust_epm.event.user.email");
        event.remove("beyondtrust_epm.event.user.full_name");
        event.remove("beyondtrust_epm.event.user.group.domain");
        event.remove("beyondtrust_epm.event.user.group.id");
        event.remove("beyondtrust_epm.event.user.group.name");
        event.remove("beyondtrust_epm.event.user.hash");
        event.remove("beyondtrust_epm.event.user.id");
        event.remove("beyondtrust_epm.event.user.name");
        event.remove("beyondtrust_epm.event.user.roles");
        event.remove("beyondtrust_epm.event.user.target.domain");
        event.remove("beyondtrust_epm.event.user.target.email");
        event.remove("beyondtrust_epm.event.user.target.full_name");
        event.remove("beyondtrust_epm.event.user.target.group.domain");
        event.remove("beyondtrust_epm.event.user.target.group.id");
        event.remove("beyondtrust_epm.event.user.target.group.name");
        event.remove("beyondtrust_epm.event.user.target.hash");
        event.remove("beyondtrust_epm.event.user.target.id");
        event.remove("beyondtrust_epm.event.user.target.name");
        event.remove("beyondtrust_epm.event.user_agent.device.name");
        event.remove("beyondtrust_epm.event.user_agent.name");
        event.remove("beyondtrust_epm.event.user_agent.original");
        event.remove("beyondtrust_epm.event.user_agent.os.family");
        event.remove("beyondtrust_epm.event.user_agent.os.full");
        event.remove("beyondtrust_epm.event.user_agent.os.kernel");
        event.remove("beyondtrust_epm.event.user_agent.os.name");
        event.remove("beyondtrust_epm.event.user_agent.os.platform");
        event.remove("beyondtrust_epm.event.user_agent.os.type");
        event.remove("beyondtrust_epm.event.user_agent.os.version");
        event.remove("beyondtrust_epm.event.user_agent.version");

        Ok(TransformResult::Continue)
    }
}
