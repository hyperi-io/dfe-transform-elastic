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
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("observer.vendor", json!("Palo Alto Networks"))?;

            event.set("observer.product", json!("Cortex XDR"))?;

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

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

            if event.has_value("json._id") {
                event.rename("json._id", "panw_cortex.xdr.event.id")?;
            }

            if event.has_value("json.uuid") {
                event.rename("json.uuid", "panw_cortex.xdr.event.uuid")?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entity_id", v)?;
            }

            let _cond = { !event.has_value("process.entity_id") };
            if _cond {
                if let Some(v) = event
                    .get("panw_cortex.xdr.event.uuid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.entity_id", v)?;
                }
            }

            let _cond = {
                event.has_value("json.action_boot_time")
                    && event.get_str("json.action_boot_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action_boot_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("panw_cortex.xdr.event.action.boot_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_boot_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_action_boot_time")?;
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
                event.has_value("json.action_file_access_time")
                    && event.get_str("json.action_file_access_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action_file_access_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("panw_cortex.xdr.event.action.file.access_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_file_access_time".into(),
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
                        "date_action_file_access_time",
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
                .get("panw_cortex.xdr.event.action.file.access_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.accessed", v)?;
            }

            if event.has_value("json.action_file_attributes") {
                event.rename(
                    "json.action_file_attributes",
                    "panw_cortex.xdr.event.action.file.attributes",
                )?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.event.action.file.attributes") };
            if _cond {
                event.append_unique(
                    "file.attributes",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.file.attributes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.action_file_create_time")
                    && event.get_str("json.action_file_create_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action_file_create_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("panw_cortex.xdr.event.action.file.create_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_file_create_time".into(),
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
                        "date_action_file_create_time",
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
                .get("panw_cortex.xdr.event.action.file.create_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.created", v)?;
            }

            if event.has_value("json.action_file_device_type") {
                event.rename(
                    "json.action_file_device_type",
                    "panw_cortex.xdr.event.action.file.device_type",
                )?;
            }

            if event.has_value("json.action_file_group") {
                event.rename(
                    "json.action_file_group",
                    "panw_cortex.xdr.event.action.file.group",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.group")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.gid", v)?;
            }

            if event.has_value("json.action_file_group_name") {
                event.rename(
                    "json.action_file_group_name",
                    "panw_cortex.xdr.event.action.file.group_name",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.group_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.group", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_file_is_anonymous") {
                    if let Some(val) = event.get("json.action_file_is_anonymous") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_file_is_anonymous".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.file.is_anonymous", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_file_is_anonymous_to_boolean",
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

            if event.has_value("json.action_file_last_writer_actor") {
                event.rename(
                    "json.action_file_last_writer_actor",
                    "panw_cortex.xdr.event.action.file.last_writer_actor",
                )?;
            }

            if event.has_value("json.action_file_md5") {
                event.rename(
                    "json.action_file_md5",
                    "panw_cortex.xdr.event.action.file.md5",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.event.action.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.action_file_mod_time")
                    && event.get_str("json.action_file_mod_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action_file_mod_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("panw_cortex.xdr.event.action.file.mod_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_file_mod_time".into(),
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
                        "date_action_file_mod_time",
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
                .get("panw_cortex.xdr.event.action.file.mod_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mtime", v)?;
            }

            if event.has_value("json.action_file_mode") {
                event.rename(
                    "json.action_file_mode",
                    "panw_cortex.xdr.event.action.file.mode",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.mode")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mode", v)?;
            }

            let _cond = {
                event
                    .get("json.action_file_name")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_file_name", |event| {
                    event.append_unique(
                        "file.name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.action_file_name") {
                event.rename(
                    "json.action_file_name",
                    "panw_cortex.xdr.event.action.file.name",
                )?;
            }

            if event.has_value("json.action_file_operation_flags") {
                event.rename(
                    "json.action_file_operation_flags",
                    "panw_cortex.xdr.event.action.file.operation_flags",
                )?;
            }

            if event.has_value("json.action_file_owner") {
                event.rename(
                    "json.action_file_owner",
                    "panw_cortex.xdr.event.action.file.owner",
                )?;
            }

            if event.has_value("json.action_file_owner_name") {
                event.rename(
                    "json.action_file_owner_name",
                    "panw_cortex.xdr.event.action.file.owner_name",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.owner_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.owner", v)?;
            }

            let _cond = {
                event
                    .get("json.action_file_path")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_file_path", |event| {
                    event.append_unique(
                        "file.path",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.action_file_path") {
                event.rename(
                    "json.action_file_path",
                    "panw_cortex.xdr.event.action.file.path",
                )?;
            }

            if event.has_value("json.action_file_pe_info") {
                event.rename(
                    "json.action_file_pe_info",
                    "panw_cortex.xdr.event.action.file.pe_info",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.pe_info")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.pe.description", v)?;
            }

            if event.has_value("json.action_file_prev_type") {
                event.rename(
                    "json.action_file_prev_type",
                    "panw_cortex.xdr.event.action.file.prev_type",
                )?;
            }

            if event.has_value("json.action_file_previous_file_name") {
                event.rename(
                    "json.action_file_previous_file_name",
                    "panw_cortex.xdr.event.action.file.previous_file_name",
                )?;
            }

            if event.has_value("json.action_file_previous_file_path") {
                event.rename(
                    "json.action_file_previous_file_path",
                    "panw_cortex.xdr.event.action.file.previous_file_path",
                )?;
            }

            if event.has_value("json.action_file_sha256") {
                event.rename(
                    "json.action_file_sha256",
                    "panw_cortex.xdr.event.action.file.sha256",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.action.file.sha256") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_file_signature_is_embedded") {
                    if let Some(val) = event.get("json.action_file_signature_is_embedded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_file_signature_is_embedded".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.file.signature_is_embedded",
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
                    "convert_action_file_signature_is_embedded_to_boolean",
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

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.signature_is_embedded")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.code_signature.exists", v)?;
            }

            if event.has_value("json.action_file_signature_product") {
                event.rename(
                    "json.action_file_signature_product",
                    "panw_cortex.xdr.event.action.file.signature_product",
                )?;
            }

            if event.has_value("json.action_file_signature_status") {
                event.rename(
                    "json.action_file_signature_status",
                    "panw_cortex.xdr.event.action.file.signature_status",
                )?;
            }

            if event.has_value("json.action_file_signature_vendor") {
                event.rename(
                    "json.action_file_signature_vendor",
                    "panw_cortex.xdr.event.action.file.signature_vendor",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_file_size") {
                    if let Some(val) = event.get("json.action_file_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_file_size".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.file.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_file_size_to_long",
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

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if event.has_value("json.action_file_type") {
                event.rename(
                    "json.action_file_type",
                    "panw_cortex.xdr.event.action.file.type",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.file.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.type", v)?;
            }

            let _cond = {
                event
                    .get("json.action_local_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.action_local_ip").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    "convert_action_local_ip_to_ip",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "json.action_local_ip",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("json.action_local_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_local_ip", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "source.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                    .get("json.action_local_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_local_ip", |event| {
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

            if event.has_value("json.action_local_ip") {
                event.rename(
                    "json.action_local_ip",
                    "panw_cortex.xdr.event.action.local_ip",
                )?;
            }

            let _cond = {
                event
                    .get("json.action_local_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_local_port", |event| {
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
                            "convert_action_local_port_to_long",
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
                    .get("json.action_local_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_local_port", |event| {
                    event.append_unique(
                        "source.port",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.action_local_port") {
                event.rename(
                    "json.action_local_port",
                    "panw_cortex.xdr.event.action.local_port",
                )?;
            }

            if event.has_value("json.action_module_base_address") {
                event.rename(
                    "json.action_module_base_address",
                    "panw_cortex.xdr.event.action.module.base_address",
                )?;
            }

            if event.has_value("json.action_module_boot_code_integrity") {
                event.rename(
                    "json.action_module_boot_code_integrity",
                    "panw_cortex.xdr.event.action.module.boot_code_integrity",
                )?;
            }

            if event.has_value("json.action_module_code_integrity") {
                event.rename(
                    "json.action_module_code_integrity",
                    "panw_cortex.xdr.event.action.module.code_integrity",
                )?;
            }

            if event.has_value("json.action_module_file_info") {
                event.rename(
                    "json.action_module_file_info",
                    "panw_cortex.xdr.event.action.module.file_info",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_module_image_size") {
                    if let Some(val) = event.get("json.action_module_image_size") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_module_image_size".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.module.image_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_module_image_size_to_double",
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

            if event.has_value("json.action_module_last_writer_actor") {
                event.rename(
                    "json.action_module_last_writer_actor",
                    "panw_cortex.xdr.event.action.module.last_writer_actor",
                )?;
            }

            if event.has_value("json.action_module_md5") {
                event.rename(
                    "json.action_module_md5",
                    "panw_cortex.xdr.event.action.module.md5",
                )?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.action.module.md5") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.module.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_module_other_load_location") {
                event.rename(
                    "json.action_module_other_load_location",
                    "panw_cortex.xdr.event.action.module.other_load_location",
                )?;
            }

            if event.has_value("json.action_module_page_protection") {
                event.rename(
                    "json.action_module_page_protection",
                    "panw_cortex.xdr.event.action.module.page_protection",
                )?;
            }

            if event.has_value("json.action_module_path") {
                event.rename(
                    "json.action_module_path",
                    "panw_cortex.xdr.event.action.module.path",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.module.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.path", v)?;
            }

            if event.has_value("json.action_module_sha256") {
                event.rename(
                    "json.action_module_sha256",
                    "panw_cortex.xdr.event.action.module.sha256",
                )?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.action.module.sha256") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.module.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_module_signature_is_embedded") {
                    if let Some(val) = event.get("json.action_module_signature_is_embedded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_module_signature_is_embedded".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.module.signature_is_embedded",
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
                    "convert_action_module_signature_is_embedded_to_boolean",
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

            if event.has_value("json.action_module_signature_product") {
                event.rename(
                    "json.action_module_signature_product",
                    "panw_cortex.xdr.event.action.module.signature_product",
                )?;
            }

            if event.has_value("json.action_module_signature_status") {
                event.rename(
                    "json.action_module_signature_status",
                    "panw_cortex.xdr.event.action.module.signature_status",
                )?;
            }

            if event.has_value("json.action_module_signature_vendor") {
                event.rename(
                    "json.action_module_signature_vendor",
                    "panw_cortex.xdr.event.action.module.signature_vendor",
                )?;
            }

            if event.has_value("json.action_module_system_properties") {
                event.rename(
                    "json.action_module_system_properties",
                    "panw_cortex.xdr.event.action.module.system_properties",
                )?;
            }

            if event.has_value("json.action_network_connection_id") {
                event.rename(
                    "json.action_network_connection_id",
                    "panw_cortex.xdr.event.action.network.connection_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_network_is_server") {
                    if let Some(val) = event.get("json.action_network_is_server") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_network_is_server".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.network.is_server", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_network_is_server_to_boolean",
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

            if event.has_value("json.action_network_protocol") {
                event.rename(
                    "json.action_network_protocol",
                    "panw_cortex.xdr.event.action.network.protocol",
                )?;
            }

            let _cond = {
                event.has_value("json.action_network_creation_time")
                    && event.get_str("json.action_network_creation_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action_network_creation_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "panw_cortex.xdr.event.action.network.creation_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_network_creation_time".into(),
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
                        "date_action_network_creation_time",
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
                .get("panw_cortex.xdr.event.action.network.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_network_stats_is_last") {
                    if let Some(val) = event.get("json.action_network_stats_is_last") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_network_stats_is_last".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.network.stats_is_last",
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
                    "convert_action_network_stats_is_last_to_boolean",
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

            if event.has_value("json.action_powered_off") {
                event.rename(
                    "json.action_powered_off",
                    "panw_cortex.xdr.event.action.powered_off",
                )?;
            }

            if event.has_value("json.action_process_fds") {
                event.rename(
                    "json.action_process_fds",
                    "panw_cortex.xdr.event.action.process.fds",
                )?;
            }

            if event.has_value("json.action_process_image_command_line") {
                event.rename(
                    "json.action_process_image_command_line",
                    "panw_cortex.xdr.event.action.process.image_command_line",
                )?;
            }

            if event.has_value("json.action_process_image_extension") {
                event.rename(
                    "json.action_process_image_extension",
                    "panw_cortex.xdr.event.action.process.image_extension",
                )?;
            }

            if event.has_value("json.action_process_image_md5") {
                event.rename(
                    "json.action_process_image_md5",
                    "panw_cortex.xdr.event.action.process.image_md5",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.image_md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.md5", v)?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.action.process.image_md5") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.process.image_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_process_image_name") {
                event.rename(
                    "json.action_process_image_name",
                    "panw_cortex.xdr.event.action.process.image_name",
                )?;
            }

            if event.has_value("json.action_process_image_path") {
                event.rename(
                    "json.action_process_image_path",
                    "panw_cortex.xdr.event.action.process.image_path",
                )?;
            }

            if event.has_value("json.action_process_image_sha256") {
                event.rename(
                    "json.action_process_image_sha256",
                    "panw_cortex.xdr.event.action.process.image_sha256",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.image_sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.action.process.image_sha256") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.process.image_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_process_in_txn") {
                    if let Some(val) = event.get("json.action_process_in_txn") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_process_in_txn".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.process.in_txn", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_process_in_txn_to_boolean",
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
                event.has_value("json.action_process_instance_execution_time")
                    && event.get_str("json.action_process_instance_execution_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.action_process_instance_execution_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "panw_cortex.xdr.event.action.process.instance_execution_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_process_instance_execution_time".into(),
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
                        "date_action_process_instance_execution_time",
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

            if event.has_value("json.action_process_instance_id") {
                event.rename(
                    "json.action_process_instance_id",
                    "panw_cortex.xdr.event.action.process.instance_id",
                )?;
            }

            if event.has_value("json.action_process_integrity_level") {
                event.rename(
                    "json.action_process_integrity_level",
                    "panw_cortex.xdr.event.action.process.integrity_level",
                )?;
            }

            if event.has_value("json.action_process_last_writer_actor") {
                event.rename(
                    "json.action_process_last_writer_actor",
                    "panw_cortex.xdr.event.action.process.last_writer_actor",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_process_os_pid") {
                    if let Some(val) = event.get("json.action_process_os_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_process_os_pid".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.process.os_pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_process_os_pid_to_long",
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

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.os_pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if event.has_value("json.action_process_pe_load_info") {
                event.rename(
                    "json.action_process_pe_load_info",
                    "panw_cortex.xdr.event.action.process.pe_load_info",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.pe_load_info")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pe.description", v)?;
            }

            if event.has_value("json.action_process_peb") {
                event.rename(
                    "json.action_process_peb",
                    "panw_cortex.xdr.event.action.process.peb",
                )?;
            }

            if event.has_value("json.action_process_peb32") {
                event.rename(
                    "json.action_process_peb32",
                    "panw_cortex.xdr.event.action.process.peb32",
                )?;
            }

            if event.has_value("json.action_process_privileges") {
                event.rename(
                    "json.action_process_privileges",
                    "panw_cortex.xdr.event.action.process.privileges",
                )?;
            }

            if event.has_value("json.action_process_scheduled_task_name") {
                event.rename(
                    "json.action_process_scheduled_task_name",
                    "panw_cortex.xdr.event.action.process.scheduled_task_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_process_signature_is_embedded") {
                    if let Some(val) = event.get("json.action_process_signature_is_embedded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_process_signature_is_embedded".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.process.signature_is_embedded",
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
                    "convert_action_process_signature_is_embedded_to_boolean",
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

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.signature_is_embedded")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.code_signature.exists", v)?;
            }

            if event.has_value("json.action_process_signature_product") {
                event.rename(
                    "json.action_process_signature_product",
                    "panw_cortex.xdr.event.action.process.signature_product",
                )?;
            }

            let _cond = {
                event
                    .get("json.action_process_signature_status")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_process_signature_status", |event| {
                    event.append_unique(
                        "process.code_signature.status",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.action_process_signature_status") {
                event.rename(
                    "json.action_process_signature_status",
                    "panw_cortex.xdr.event.action.process.signature_status",
                )?;
            }

            if event.has_value("json.action_process_signature_vendor") {
                event.rename(
                    "json.action_process_signature_vendor",
                    "panw_cortex.xdr.event.action.process.signature_vendor",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.signature_vendor")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.code_signature.subject_name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_process_termination_code") {
                    if let Some(val) = event.get("json.action_process_termination_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_process_termination_code".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.process.termination_code",
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
                    "convert_action_process_termination_code_to_long",
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

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.termination_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.exit_code", v)?;
            }

            let _cond = {
                event.has_value("json.action_process_termination_date")
                    && event.get_str("json.action_process_termination_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.action_process_termination_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "panw_cortex.xdr.event.action.process.termination_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action_process_termination_date".into(),
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
                        "date_action_process_termination_date",
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
                .get("panw_cortex.xdr.event.action.process.termination_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.end", v)?;
            }

            if event.has_value("json.action_process_token") {
                event.rename(
                    "json.action_process_token",
                    "panw_cortex.xdr.event.action.process.token",
                )?;
            }

            if event.has_value("json.action_process_user_sid") {
                event.rename(
                    "json.action_process_user_sid",
                    "panw_cortex.xdr.event.action.process.user_sid",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.user_sid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entry_leader.attested_user.id", v)?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.event.action.process.user_sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.process.user_sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_process_username") {
                event.rename(
                    "json.action_process_username",
                    "panw_cortex.xdr.event.action.process.username",
                )?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.event.action.process.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.process.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.process.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entry_leader.attested_user.name", v)?;
            }

            if event.has_value("json.action_registry_data") {
                event.rename(
                    "json.action_registry_data",
                    "panw_cortex.xdr.event.action.registry.data",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.registry.data")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.data.bytes", v)?;
            }

            if event.has_value("json.action_registry_file_path") {
                event.rename(
                    "json.action_registry_file_path",
                    "panw_cortex.xdr.event.action.registry.file_path",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.registry.file_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.path", v)?;
            }

            if event.has_value("json.action_registry_key_name") {
                event.rename(
                    "json.action_registry_key_name",
                    "panw_cortex.xdr.event.action.registry.key_name",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.registry.key_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.key", v)?;
            }

            if event.has_value("json.action_registry_old_key_name") {
                event.rename(
                    "json.action_registry_old_key_name",
                    "panw_cortex.xdr.event.action.registry.old_key_name",
                )?;
            }

            if event.has_value("json.action_registry_return_val") {
                event.rename(
                    "json.action_registry_return_val",
                    "panw_cortex.xdr.event.action.registry.return_val",
                )?;
            }

            if event.has_value("json.action_registry_value_name") {
                event.rename(
                    "json.action_registry_value_name",
                    "panw_cortex.xdr.event.action.registry.value_name",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.registry.value_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.value", v)?;
            }

            if event.has_value("json.action_registry_value_type") {
                event.rename(
                    "json.action_registry_value_type",
                    "panw_cortex.xdr.event.action.registry.value_type",
                )?;
            }

            let _cond = {
                event
                    .get("json.action_remote_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_remote_ip", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
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
                            "convert_action_remote_ip_to_ip",
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
                    .get("json.action_remote_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_remote_ip", |event| {
                    event.append_unique(
                        "destination.ip",
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
                    .get("json.action_remote_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_remote_ip", |event| {
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

            if event.has_value("json.action_remote_ip") {
                event.rename(
                    "json.action_remote_ip",
                    "panw_cortex.xdr.event.action.remote_ip",
                )?;
            }

            let _cond = {
                event
                    .get("json.action_remote_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_remote_port", |event| {
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
                            "convert_action_remote_port_to_long",
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
                    .get("json.action_remote_port")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.action_remote_port", |event| {
                    event.append_unique(
                        "destination.port",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.action_remote_port") {
                event.rename(
                    "json.action_remote_port",
                    "panw_cortex.xdr.event.action.remote_port",
                )?;
            }

            if event.has_value("json.action_remote_process_image_command_line") {
                event.rename(
                    "json.action_remote_process_image_command_line",
                    "panw_cortex.xdr.event.action.remote_process.image_command_line",
                )?;
            }

            if event.has_value("json.action_remote_process_image_md5") {
                event.rename(
                    "json.action_remote_process_image_md5",
                    "panw_cortex.xdr.event.action.remote_process.image_md5",
                )?;
            }

            let _cond =
                { event.has_value("panw_cortex.xdr.event.action.remote_process.image_md5") };
            if _cond {
                event.append_unique(
                    "process.hash.md5",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.remote_process.image_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.action.remote_process.image_md5") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.remote_process.image_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_remote_process_image_name") {
                event.rename(
                    "json.action_remote_process_image_name",
                    "panw_cortex.xdr.event.action.remote_process.image_name",
                )?;
            }

            if event.has_value("json.action_remote_process_image_path") {
                event.rename(
                    "json.action_remote_process_image_path",
                    "panw_cortex.xdr.event.action.remote_process.image_path",
                )?;
            }

            if event.has_value("json.action_remote_process_image_sha256") {
                event.rename(
                    "json.action_remote_process_image_sha256",
                    "panw_cortex.xdr.event.action.remote_process.image_sha256",
                )?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.action.remote_process.image_sha256")
                    != Some("")
            };
            if _cond {
                event.append_unique(
                    "process.hash.sha256",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.remote_process.image_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.action.remote_process.image_sha256")
                    != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.remote_process.image_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_remote_process_instance_id") {
                event.rename(
                    "json.action_remote_process_instance_id",
                    "panw_cortex.xdr.event.action.remote_process.instance_id",
                )?;
            }

            if event.has_value("json.action_remote_process_integrity_level") {
                event.rename(
                    "json.action_remote_process_integrity_level",
                    "panw_cortex.xdr.event.action.remote_process.integrity_level",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_remote_process_os_pid") {
                    if let Some(val) = event.get("json.action_remote_process_os_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_remote_process_os_pid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.remote_process.os_pid",
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
                    "convert_action_remote_process_os_pid_to_long",
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

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.remote_process.os_pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entry_leader.pid", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_remote_process_signature_is_embedded") {
                    if let Some(val) = event.get("json.action_remote_process_signature_is_embedded")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_remote_process_signature_is_embedded".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.remote_process.signature_is_embedded",
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
                    "convert_action_remote_process_signature_is_embedded_to_boolean",
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

            if event.has_value("json.action_remote_process_signature_product") {
                event.rename(
                    "json.action_remote_process_signature_product",
                    "panw_cortex.xdr.event.action.remote_process.signature_product",
                )?;
            }

            if event.has_value("json.action_remote_process_signature_status") {
                event.rename(
                    "json.action_remote_process_signature_status",
                    "panw_cortex.xdr.event.action.remote_process.signature_status",
                )?;
            }

            if event.has_value("json.action_remote_process_signature_vendor") {
                event.rename(
                    "json.action_remote_process_signature_vendor",
                    "panw_cortex.xdr.event.action.remote_process.signature_vendor",
                )?;
            }

            if event.has_value("json.action_remote_process_thread_id") {
                event.rename(
                    "json.action_remote_process_thread_id",
                    "panw_cortex.xdr.event.action.remote_process.thread_id",
                )?;
            }

            if event.has_value("json.action_remote_process_thread_start_address") {
                event.rename(
                    "json.action_remote_process_thread_start_address",
                    "panw_cortex.xdr.event.action.remote_process.thread_start_address",
                )?;
            }

            if event.has_value("json.action_remote_process_user_sid") {
                event.rename(
                    "json.action_remote_process_user_sid",
                    "panw_cortex.xdr.event.action.remote_process.user_sid",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.remote_process.user_sid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entry_leader.user.id", v)?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.action.remote_process.user_sid") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.remote_process.user_sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_remote_process_username") {
                event.rename(
                    "json.action_remote_process_username",
                    "panw_cortex.xdr.event.action.remote_process.username",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.remote_process.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entry_leader.user.name", v)?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.action.remote_process.username") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.remote_process.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_socket_type") {
                event.rename(
                    "json.action_socket_type",
                    "panw_cortex.xdr.event.action.socket_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action_total_download") {
                    if let Some(val) = event.get("json.action_total_download") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_total_download".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.total_download", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_total_download_to_long",
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
                if event.has_value("json.action_total_upload") {
                    if let Some(val) = event.get("json.action_total_upload") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_total_upload".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.action.total_upload", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_total_upload_to_long",
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
                if event.has_value("json.action_user_is_local_session") {
                    if let Some(val) = event.get("json.action_user_is_local_session") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action_user_is_local_session".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "panw_cortex.xdr.event.action.user_is_local_session",
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
                    "convert_action_user_is_local_session_to_boolean",
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

            if event.has_value("json.action_user_session_id") {
                event.rename(
                    "json.action_user_session_id",
                    "panw_cortex.xdr.event.action.user_session_id",
                )?;
            }

            if event.has_value("json.action_user_status") {
                event.rename(
                    "json.action_user_status",
                    "panw_cortex.xdr.event.action.user_status",
                )?;
            }

            if event.has_value("json.action_user_status_sid") {
                event.rename(
                    "json.action_user_status_sid",
                    "panw_cortex.xdr.event.action.user_status_sid",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.user_status_sid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.action.user_status_sid") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.user_status_sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action_username") {
                event.rename(
                    "json.action_username",
                    "panw_cortex.xdr.event.action.username",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.action.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.action.username") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.action.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.address_mapping") {
                event.rename(
                    "json.address_mapping",
                    "panw_cortex.xdr.event.address_mapping",
                )?;
            }

            if event.has_value("json.agent_content_version") {
                event.rename(
                    "json.agent_content_version",
                    "panw_cortex.xdr.event.agent.content_version",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.agent.content_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("agent.version", v)?;
            }

            if event.has_value("json.agent_hostname") {
                event.rename(
                    "json.agent_hostname",
                    "panw_cortex.xdr.event.agent.hostname",
                )?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.agent.hostname") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.agent.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.agent_id") {
                event.rename("json.agent_id", "panw_cortex.xdr.event.agent.id")?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.agent.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("agent.id", v)?;
            }

            if event.has_value("json.agent_interface_map") {
                event.rename(
                    "json.agent_interface_map",
                    "panw_cortex.xdr.event.agent.interface_map",
                )?;
            }

            let _cond = {
                event
                    .get("json.agent_ip_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.agent_ip_addresses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
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
                            "convert_agent_ip_addresses_to_ip",
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
                    .get("json.agent_ip_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.agent_ip_addresses", |event| {
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

            if event.has_value("json.agent_ip_addresses") {
                event.rename(
                    "json.agent_ip_addresses",
                    "panw_cortex.xdr.event.agent.ip_addresses",
                )?;
            }

            let _cond = {
                event
                    .get("json.agent_ip_addresses_v6")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.agent_ip_addresses_v6", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
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
                            "convert_agent_ip_addresses_v6_to_ip",
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
                    .get("json.agent_ip_addresses_v6")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.agent_ip_addresses_v6", |event| {
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

            if event.has_value("json.agent_ip_addresses_v6") {
                event.rename(
                    "json.agent_ip_addresses_v6",
                    "panw_cortex.xdr.event.agent.ip_addresses_v6",
                )?;
            }

            if event.has_value("json.agent_os_sub_type") {
                event.rename(
                    "json.agent_os_sub_type",
                    "panw_cortex.xdr.event.agent.os_sub_type",
                )?;
            }

            if event.has_value("json.agent_os_type") {
                event.rename("json.agent_os_type", "panw_cortex.xdr.event.agent.os_type")?;
            }

            if event.has_value("json.agent_version") {
                event.rename("json.agent_version", "panw_cortex.xdr.event.agent.version")?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.agent.version") != Some("") };
            if _cond {
                event.append_unique(
                    "agent.version",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.agent.version")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event_id") {
                event.rename("json.event_id", "panw_cortex.xdr.event.event_id")?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.event_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.event_rpc_func_opnum") {
                event.rename(
                    "json.event_rpc_func_opnum",
                    "panw_cortex.xdr.event.event_rpc_func_opnum",
                )?;
            }

            if event.has_value("json.event_rpc_interface_uuid") {
                event.rename(
                    "json.event_rpc_interface_uuid",
                    "panw_cortex.xdr.event.event_rpc_interface_uuid",
                )?;
            }

            if event.has_value("json.event_sub_type") {
                event.rename(
                    "json.event_sub_type",
                    "panw_cortex.xdr.event.event_sub_type",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.event_sub_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond = { event.has_value("event.action") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
                    .get("json.event_timestamp")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.event_timestamp").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value") {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event.set("@timestamp", parsed)?,
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
                                    "date_event_timestamp",
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
                            "json.event_timestamp",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if let Some(v) = event
                .get("json.event_timestamp.0")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.event_timestamp") {
                event.rename(
                    "json.event_timestamp",
                    "panw_cortex.xdr.event.event_timestamp",
                )?;
            }

            if event.has_value("json.event_type") {
                event.rename("json.event_type", "panw_cortex.xdr.event.event_type")?;
            }

            if event.has_value("json.event_version") {
                event.rename("json.event_version", "panw_cortex.xdr.event.event_version")?;
            }

            if event.has_value("json.host_metadata_domain") {
                event.rename(
                    "json.host_metadata_domain",
                    "panw_cortex.xdr.event.host_metadata.domain",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.host_metadata.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.event.host_metadata.domain") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.host_metadata.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.host_metadata_hostname") {
                event.rename(
                    "json.host_metadata_hostname",
                    "panw_cortex.xdr.event.host_metadata.hostname",
                )?;
            }

            if let Some(v) = event
                .get("panw_cortex.xdr.event.host_metadata.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("host.hostname") {
                map_strings(event, "host.hostname", "host.hostname", str::to_lowercase)?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.host_metadata.hostname") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.host_metadata.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.hostname") && event.has_value("host.domain") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.host = ctx.host ?: [:];\nString hostname = ctx.host.hostname.toLowerCase() + '.' + ctx.host.domain.toLowerCase();\nctx.host.put(\"name\", hostname);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.host = ctx.host ?: [:];\nString hostname = ctx.host.hostname.toLowerCase() + '.' + ctx.host.domain.toLowerCase();\nctx.host.put(\"name\", hostname);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "create_host_name_from_host_hostname_and_host_domain",
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

            let _cond = { event.get_str("host.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.host_metadata_interface_map") {
                event.rename(
                    "json.host_metadata_interface_map",
                    "panw_cortex.xdr.event.host_metadata.interface_map",
                )?;
            }

            let _cond = { event.get_str("json.os_actor_local_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.os_actor_local_ip") {
                        if let Some(val) = event.get("json.os_actor_local_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.os_actor_local_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.event.os_actor.local_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_os_actor_local_ip_to_ip",
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

            let _cond = { event.has_value("panw_cortex.xdr.event.os_actor.local_ip") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.local_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.local_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.os_actor_local_port") {
                    if let Some(val) = event.get("json.os_actor_local_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.os_actor_local_port".into(),
                                message,
                            }
                        })?;
                        event.set("panw_cortex.xdr.event.os_actor.local_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_os_actor_local_port_to_long",
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

            let _cond = { event.has_value("panw_cortex.xdr.event.os_actor.local_port") };
            if _cond {
                event.append_unique(
                    "source.port",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.local_port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_actor_primary_user_sid") {
                event.rename(
                    "json.os_actor_primary_user_sid",
                    "panw_cortex.xdr.event.os_actor.primary_user_sid",
                )?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.os_actor.primary_user_sid") != Some("") };
            if _cond {
                event.append_unique(
                    "user.id",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.primary_user_sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.os_actor.primary_user_sid") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.primary_user_sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_actor_primary_username") {
                event.rename(
                    "json.os_actor_primary_username",
                    "panw_cortex.xdr.event.os_actor.primary_username",
                )?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.os_actor.primary_username") != Some("") };
            if _cond {
                event.append_unique(
                    "user.name",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.primary_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.os_actor.primary_username") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.primary_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_actor_process_command_line") {
                event.rename(
                    "json.os_actor_process_command_line",
                    "panw_cortex.xdr.event.os_actor.process_command_line",
                )?;
            }

            if event.has_value("json.os_actor_process_image_md5") {
                event.rename(
                    "json.os_actor_process_image_md5",
                    "panw_cortex.xdr.event.os_actor.process_image_md5",
                )?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.os_actor.process_image_md5") != Some("") };
            if _cond {
                event.append_unique(
                    "process.hash.md5",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.process_image_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("panw_cortex.xdr.event.os_actor.process_image_md5") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.process_image_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_actor_process_image_name") {
                event.rename(
                    "json.os_actor_process_image_name",
                    "panw_cortex.xdr.event.os_actor.process_image_name",
                )?;
            }

            if event.has_value("json.os_actor_process_image_path") {
                event.rename(
                    "json.os_actor_process_image_path",
                    "panw_cortex.xdr.event.os_actor.process_image_path",
                )?;
            }

            if event.has_value("json.os_actor_process_image_sha256") {
                event.rename(
                    "json.os_actor_process_image_sha256",
                    "panw_cortex.xdr.event.os_actor.process_image_sha256",
                )?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.os_actor.process_image_sha256") != Some("")
            };
            if _cond {
                event.append_unique(
                    "process.hash.sha256",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.process_image_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("panw_cortex.xdr.event.os_actor.process_image_sha256") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.process_image_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_actor_process_instance_id") {
                event.rename(
                    "json.os_actor_process_instance_id",
                    "panw_cortex.xdr.event.os_actor.process_instance_id",
                )?;
            }

            if event.has_value("json.os_actor_process_logon_id") {
                event.rename(
                    "json.os_actor_process_logon_id",
                    "panw_cortex.xdr.event.os_actor.process_logon_id",
                )?;
            }

            if event.has_value("json.os_actor_process_os_pid") {
                event.rename(
                    "json.os_actor_process_os_pid",
                    "panw_cortex.xdr.event.os_actor.process_os_pid",
                )?;
            }

            if event.has_value("json.os_actor_process_signature_status") {
                event.rename(
                    "json.os_actor_process_signature_status",
                    "panw_cortex.xdr.event.os_actor.process_signature_status",
                )?;
            }

            let _cond = { event.get_str("json.os_actor_remote_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.os_actor_remote_ip") {
                        if let Some(val) = event.get("json.os_actor_remote_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.os_actor_remote_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.event.os_actor.remote_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_os_actor_remote_ip_to_ip",
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

            let _cond = { event.has_value("panw_cortex.xdr.event.os_actor.remote_ip") };
            if _cond {
                event.append_unique(
                    "destination.ip",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.remote_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.event.os_actor.remote_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("panw_cortex.xdr.event.os_actor.remote_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_actor_thread_thread_id") {
                event.rename(
                    "json.os_actor_thread_thread_id",
                    "panw_cortex.xdr.event.os_actor.thread_thread_id",
                )?;
            }

            let _cond = { event.get("source.port").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("source.port").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    "convert_source_port_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "source.port",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("destination.port").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("destination.port").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    "convert_destination_port_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "destination.port",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
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
                event.remove("panw_cortex.xdr.event.action.file.access_time");
                event.remove("panw_cortex.xdr.event.action.file.attributes");
                event.remove("panw_cortex.xdr.event.action.file.create_time");
                event.remove("panw_cortex.xdr.event.action.file.group");
                event.remove("panw_cortex.xdr.event.action.file.group_name");
                event.remove("panw_cortex.xdr.event.action.file.md5");
                event.remove("panw_cortex.xdr.event.action.file.mod_time");
                event.remove("panw_cortex.xdr.event.action.file.mode");
                event.remove("panw_cortex.xdr.event.action.file.owner_name");
                event.remove("panw_cortex.xdr.event.action.file.pe_info");
                event.remove("panw_cortex.xdr.event.action.file.sha256");
                event.remove("panw_cortex.xdr.event.action.file.signature_is_embedded");
                event.remove("panw_cortex.xdr.event.action.file.size");
                event.remove("panw_cortex.xdr.event.action.file.type");
                event.remove("panw_cortex.xdr.event.action.module.path");
                event.remove("panw_cortex.xdr.event.action.network.protocol");
                event.remove("panw_cortex.xdr.event.action.process.image_md5");
                event.remove("panw_cortex.xdr.event.action.process.image_sha256");
                event.remove("panw_cortex.xdr.event.action.process.os_pid");
                event.remove("panw_cortex.xdr.event.action.process.pe_load_info");
                event.remove("panw_cortex.xdr.event.action.process.signature_is_embedded");
                event.remove("panw_cortex.xdr.event.action.process.signature_vendor");
                event.remove("panw_cortex.xdr.event.action.process.termination_code");
                event.remove("panw_cortex.xdr.event.action.process.termination_date");
                event.remove("panw_cortex.xdr.event.action.process.user_sid");
                event.remove("panw_cortex.xdr.event.action.process.username");
                event.remove("panw_cortex.xdr.event.action.registry.data");
                event.remove("panw_cortex.xdr.event.action.registry.file_path");
                event.remove("panw_cortex.xdr.event.action.registry.key_name");
                event.remove("panw_cortex.xdr.event.action.registry.value_name");
                event.remove("panw_cortex.xdr.event.action.remote_process.image_md5");
                event.remove("panw_cortex.xdr.event.action.remote_process.image_sha256");
                event.remove("panw_cortex.xdr.event.action.user_status_sid");
                event.remove("panw_cortex.xdr.event.action.username");
                event.remove("panw_cortex.xdr.event.agent.content_version");
                event.remove("panw_cortex.xdr.event.agent.id");
                event.remove("panw_cortex.xdr.event.agent.version");
                event.remove("panw_cortex.xdr.event.action.file.name");
                event.remove("panw_cortex.xdr.event.action.file.path");
                event.remove("panw_cortex.xdr.event.action.local_ip");
                event.remove("panw_cortex.xdr.event.action.local_port");
                event.remove("panw_cortex.xdr.event.action.process.signature_status");
                event.remove("panw_cortex.xdr.event.action.remote_ip");
                event.remove("panw_cortex.xdr.event.action.remote_port");
                event.remove("panw_cortex.xdr.event.event_id");
                event.remove("panw_cortex.xdr.event.event_sub_type");
                event.remove("panw_cortex.xdr.event.event_timestamp");
                event.remove("panw_cortex.xdr.event.host_metadata.domain");
                event.remove("panw_cortex.xdr.event.host_metadata.hostname");
                event.remove("panw_cortex.xdr.event.id");
                event.remove("panw_cortex.xdr.event.os_actor.local_ip");
                event.remove("panw_cortex.xdr.event.os_actor.local_port");
                event.remove("panw_cortex.xdr.event.os_actor.primary_user_sid");
                event.remove("panw_cortex.xdr.event.os_actor.primary_username");
                event.remove("panw_cortex.xdr.event.os_actor.process_image_md5");
                event.remove("panw_cortex.xdr.event.os_actor.process_image_sha256");
                event.remove("panw_cortex.xdr.event.os_actor.remote_ip");
                event.remove("panw_cortex.xdr.event.action.remote_process.user_sid");
                event.remove("panw_cortex.xdr.event.action.remote_process.username");
                event.remove("panw_cortex.xdr.event.action.remote_process.os_pid");
                event.remove("panw_cortex.xdr.event.uuid");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
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
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
