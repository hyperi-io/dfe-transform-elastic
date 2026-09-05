// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_cross_process` pipeline.
pub struct PipelineCrossProcess;

impl Transform for PipelineCrossProcess {
    fn name(&self) -> &str {
        "pipeline_cross_process"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("process")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if event.has_value("json.k8sCluster.containerImage.sha256") {
                event.rename(
                    "json.k8sCluster.containerImage.sha256",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                )?;
            }

            let _cond = {
                event
                    .has_value("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256")
            };
            if _cond {
                event.append_unique(
                    "container.image.hash.all",
                    json!(
                        event
                            .get(
                                "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .has_value("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get(
                                "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.relation") {
                event.rename(
                    "json.tgt.process.relation",
                    "sentinel_one_cloud_funnel.event.tgt.process.relation",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.accessRights") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.accessRights") {
                        if let Some(val) = event.get("json.tgt.process.accessRights") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.accessRights".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.access_rights",
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
                        "convert_json_tgt_process_accessRights",
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
            }

            if event.has_value("json.tgt.process.activeContent.hash") {
                event.rename(
                    "json.tgt.process.activeContent.hash",
                    "sentinel_one_cloud_funnel.event.tgt.process.active_content.hash",
                )?;
            }

            if event.has_value("json.tgt.process.activeContent.id") {
                event.rename(
                    "json.tgt.process.activeContent.id",
                    "sentinel_one_cloud_funnel.event.tgt.process.active_content.id",
                )?;
            }

            if event.has_value("json.tgt.process.activeContent.path") {
                event.rename(
                    "json.tgt.process.activeContent.path",
                    "sentinel_one_cloud_funnel.event.tgt.process.active_content.path",
                )?;
            }

            if event.has_value("json.tgt.process.activeContent.signedStatus") {
                event.rename(
                    "json.tgt.process.activeContent.signedStatus",
                    "sentinel_one_cloud_funnel.event.tgt.process.active_content.signed_status",
                )?;
            }

            if event.has_value("json.tgt.process.activeContentType") {
                event.rename(
                    "json.tgt.process.activeContentType",
                    "sentinel_one_cloud_funnel.event.tgt.process.active_content.type",
                )?;
            }

            if event.has_value("json.tgt.process.cmdline") {
                event.rename(
                    "json.tgt.process.cmdline",
                    "sentinel_one_cloud_funnel.event.tgt.process.cmd_line",
                )?;
            }

            if event.has_value("json.tgt.process.displayName") {
                event.rename(
                    "json.tgt.process.displayName",
                    "sentinel_one_cloud_funnel.event.tgt.process.display_name",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.image.binaryIsExecutable") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.image.binaryIsExecutable") {
                        if let Some(val) = event.get("json.tgt.process.image.binaryIsExecutable") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.image.binaryIsExecutable".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.tgt.process.image.binary_is_executable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_tgt_process_image_binaryIsExecutable",
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
            }

            if event.has_value("json.tgt.process.image.md5") {
                event.rename(
                    "json.tgt.process.image.md5",
                    "sentinel_one_cloud_funnel.event.tgt.process.image.md5",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.image.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.image.path") {
                event.rename(
                    "json.tgt.process.image.path",
                    "sentinel_one_cloud_funnel.event.tgt.process.image.path",
                )?;
            }

            if event.has_value("json.tgt.process.image.sha1") {
                event.rename(
                    "json.tgt.process.image.sha1",
                    "sentinel_one_cloud_funnel.event.tgt.process.image.sha1",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.image.sha256") {
                event.rename(
                    "json.tgt.process.image.sha256",
                    "sentinel_one_cloud_funnel.event.tgt.process.image.sha256",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.integrityLevel") {
                event.rename(
                    "json.tgt.process.integrityLevel",
                    "sentinel_one_cloud_funnel.event.tgt.process.integrity_level",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.isNative64Bit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.isNative64Bit") {
                        if let Some(val) = event.get("json.tgt.process.isNative64Bit") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.isNative64Bit".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.is_native_64_bit",
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
                        "convert_json_tgt_process_isNative64Bit",
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
            }

            let _cond = { event.get_str("json.tgt.process.isRedirectCmdProcessor") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.isRedirectCmdProcessor") {
                        if let Some(val) = event.get("json.tgt.process.isRedirectCmdProcessor") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.isRedirectCmdProcessor".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.tgt.process.is_redirect_cmd_processor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_tgt_process_isRedirectCmdProcessor",
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
            }

            let _cond = { event.get_str("json.tgt.process.isStorylineRoot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.isStorylineRoot") {
                        if let Some(val) = event.get("json.tgt.process.isStorylineRoot") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.isStorylineRoot".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.is_storyline_root",
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
                        "convert_json_tgt_process_isStorylineRoot",
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
            }

            if event.has_value("json.tgt.process.name") {
                event.rename(
                    "json.tgt.process.name",
                    "sentinel_one_cloud_funnel.event.tgt.process.name",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.pid") {
                        if let Some(val) = event.get("json.tgt.process.pid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.pid",
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
                        "convert_json_tgt_process_pid",
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
            }

            if event.has_value("json.tgt.process.publisher") {
                event.rename(
                    "json.tgt.process.publisher",
                    "sentinel_one_cloud_funnel.event.tgt.process.publisher",
                )?;
            }

            if event.has_value("json.tgt.process.reasonSignatureInvalid") {
                event.rename(
                    "json.tgt.process.reasonSignatureInvalid",
                    "sentinel_one_cloud_funnel.event.tgt.process.reason_signature_invalid",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.sessionId") {
                        if let Some(val) = event.get("json.tgt.process.sessionId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.sessionId".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.session_id",
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
                        "convert_json_tgt_process_sessionId",
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
            }

            if event.has_value("json.tgt.process.signedStatus") {
                event.rename(
                    "json.tgt.process.signedStatus",
                    "sentinel_one_cloud_funnel.event.tgt.process.signed_status",
                )?;
            }

            let _cond = {
                event.has_value("json.tgt.process.startTime")
                    && event.get_str("json.tgt.process.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.tgt.process.startTime") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.tgt.process.startTime".into(),
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
                        "date_json_tgt_process_startTime",
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
            }

            if event.has_value("json.tgt.process.storyline.id") {
                event.rename(
                    "json.tgt.process.storyline.id",
                    "sentinel_one_cloud_funnel.event.tgt.process.storyline_id",
                )?;
            }

            if event.has_value("json.tgt.process.subsystem") {
                event.rename(
                    "json.tgt.process.subsystem",
                    "sentinel_one_cloud_funnel.event.tgt.process.subsystem",
                )?;
            }

            if event.has_value("json.tgt.process.uid") {
                event.rename(
                    "json.tgt.process.uid",
                    "sentinel_one_cloud_funnel.event.tgt.process.uid",
                )?;
            }

            if event.has_value("json.tgt.process.user") {
                event.rename(
                    "json.tgt.process.user",
                    "sentinel_one_cloud_funnel.event.tgt.process.user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.name") };
            if _cond {
                event.append_unique(
                    "process.user.name",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.verifiedStatus") {
                event.rename(
                    "json.tgt.process.verifiedStatus",
                    "sentinel_one_cloud_funnel.event.tgt.process.verified_status",
                )?;
            }

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
