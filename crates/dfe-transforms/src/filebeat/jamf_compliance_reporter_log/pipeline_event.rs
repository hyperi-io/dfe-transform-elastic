// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event` pipeline.
pub struct PipelineEvent;

impl Transform for PipelineEvent {
    fn name(&self) -> &str {
        "pipeline_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.set("jamf_compliance_reporter.log.dataset", json!("event"))?;

                event.append("event.category", json!("process"))?;

            event.set("host.os.type", json!("macos"))?;

            let _cond = { !(["UNIFIED_LOG_EVENT", "XPROTECT_EVENT_LOG"].contains(&event.get_str("json.header.event_name").unwrap_or(""))) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json._event_score") {
                if let Some(val) = event.get("json._event_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json._event_score".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.header.event_name") {
                    event.rename("json.header.event_name", "event.action")?;
                }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_i64("json.header.time_seconds_epoch") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.header.time_seconds_epoch") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.header.time_seconds_epoch".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.host_info.host_name") {
                    event.rename("json.host_info.host_name", "host.hostname")?;
                }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.host_info.host_uuid") {
                    event.rename("json.host_info.host_uuid", "jamf_compliance_reporter.log.host_info.host.uuid")?;
                }

                if event.has_value("json.host_info.osversion") {
                    event.rename("json.host_info.osversion", "host.os.version")?;
                }

            let _cond = { event.has_value("json.host_info.primary_mac_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("host.mac", json!(event.get("json.host_info.primary_mac_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

                if event.has_value("json.host_info.serial_number") {
                    event.rename("json.host_info.serial_number", "host.id")?;
                }

            let _cond = { event.get_str("event.action") == Some("audio_video_device_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_audio_video_device_event"
                if event.has_value("json.audio_video_device_info.audio_device_creator") {
                event.rename("json.audio_video_device_info.audio_device_creator", "jamf_compliance_reporter.log.audio_video_device_info.audio_device.creator")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.audio_video_device_info.audio_device_hog_mode") {
                if let Some(val) = event.get("json.audio_video_device_info.audio_device_hog_mode") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.audio_video_device_info.audio_device_hog_mode".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.audio_video_device_info.audio_device.hog_mode", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.audio_video_device_info.audio_device_id") {
                if let Some(val) = event.get("json.audio_video_device_info.audio_device_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.audio_video_device_info.audio_device_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.audio_video_device_info.audio_device.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.audio_video_device_info.audio_device_manufacturer") {
                event.rename("json.audio_video_device_info.audio_device_manufacturer", "jamf_compliance_reporter.log.audio_video_device_info.audio_device.manufacturer")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.audio_video_device_info.audio_device_running") {
                if let Some(val) = event.get("json.audio_video_device_info.audio_device_running") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.audio_video_device_info.audio_device_running".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.audio_video_device_info.audio_device.running", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.audio_video_device_info.audio_device_uuid") {
                event.rename("json.audio_video_device_info.audio_device_uuid", "jamf_compliance_reporter.log.audio_video_device_info.audio_device.uuid")?;
                }
                if event.has_value("json.audio_video_device_info.device_status") {
                event.rename("json.audio_video_device_info.device_status", "jamf_compliance_reporter.log.audio_video_device_info.device_status")?;
                }
                // End nested pipeline: "pipeline_audio_video_device_event"
            }

            let _cond = { event.get_str("event.action") == Some("audit_class_verification_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_audit_class_verification_event"
                if event.has_value("json.audit_class_verification_info.contents") {
                event.rename("json.audit_class_verification_info.contents", "jamf_compliance_reporter.log.audit_class_verification_info.contents")?;
                }
                if event.has_value("json.audit_class_verification_info.osversion") {
                event.rename("json.audit_class_verification_info.osversion", "jamf_compliance_reporter.log.audit_class_verification_info.os.version")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.audit_class_verification_info.restored_default") {
                if let Some(val) = event.get("json.audit_class_verification_info.restored_default") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.audit_class_verification_info.restored_default".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.audit_class_verification_info.restored_default", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.audit_class_verification_info.status") {
                if let Some(val) = event.get("json.audit_class_verification_info.status") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.audit_class_verification_info.status".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.audit_class_verification_info.status", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.audit_class_verification_info.status_str") {
                event.rename("json.audit_class_verification_info.status_str", "jamf_compliance_reporter.log.audit_class_verification_info.status_str")?;
                }
                // End nested pipeline: "pipeline_audit_class_verification_event"
            }

            let _cond = { ["compliance_reporter_tamper_event", "file_event"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                // Begin nested pipeline: "pipeline_compliance_reporter_tamper_event_and_file_event_info"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.eventid_wrapped") {
                if let Some(val) = event.get("json.file_event_info.eventid_wrapped") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.eventid_wrapped".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.eventid_wrapped", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.file_event_info.hash") {
                event.rename("json.file_event_info.hash", "file.hash.sha1")?;
                }
                let _cond = { event.has_value("file.hash.sha1") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("file.hash.sha1").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.history_done") {
                if let Some(val) = event.get("json.file_event_info.history_done") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.history_done".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.history_done", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_change_owner") {
                if let Some(val) = event.get("json.file_event_info.item_change_owner") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_change_owner".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.change_owner", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_cloned") {
                if let Some(val) = event.get("json.file_event_info.item_cloned") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_cloned".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.cloned", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_created") {
                if let Some(val) = event.get("json.file_event_info.item_created") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_created".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.created", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_extended_attribute_modified") {
                if let Some(val) = event.get("json.file_event_info.item_extended_attribute_modified") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_extended_attribute_modified".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.extended_attribute_modified", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_finder_info_modified") {
                if let Some(val) = event.get("json.file_event_info.item_finder_info_modified") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_finder_info_modified".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.finder_info_modified", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_inode_metadata_modified") {
                if let Some(val) = event.get("json.file_event_info.item_inode_metadata_modified") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_inode_metadata_modified".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.inode_metadata_modified", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_is_directory") {
                if let Some(val) = event.get("json.file_event_info.item_is_directory") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_is_directory".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.is_directory", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_is_file") {
                if let Some(val) = event.get("json.file_event_info.item_is_file") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_is_file".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.is_file", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_is_hard_link") {
                if let Some(val) = event.get("json.file_event_info.item_is_hard_link") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_is_hard_link".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.is_hard_link", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_is_last_hard_link") {
                if let Some(val) = event.get("json.file_event_info.item_is_last_hard_link") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_is_last_hard_link".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.is_last_hard_link", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_is_sym_link") {
                if let Some(val) = event.get("json.file_event_info.item_is_sym_link") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_is_sym_link".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.is_sym_link", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_removed") {
                if let Some(val) = event.get("json.file_event_info.item_removed") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_removed".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.removed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_renamed") {
                if let Some(val) = event.get("json.file_event_info.item_renamed") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_renamed".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.renamed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.item_updated") {
                if let Some(val) = event.get("json.file_event_info.item_updated") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.item_updated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.item.updated", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.kernel_dropped") {
                if let Some(val) = event.get("json.file_event_info.kernel_dropped") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.kernel_dropped".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.kernel_dropped", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.mount") {
                if let Some(val) = event.get("json.file_event_info.mount") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.mount".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.mount", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.must_scan_sub_dir") {
                if let Some(val) = event.get("json.file_event_info.must_scan_sub_dir") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.must_scan_sub_dir".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.must_scan_sub_dir", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.none") {
                if let Some(val) = event.get("json.file_event_info.none") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.none".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.none", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.own_event") {
                if let Some(val) = event.get("json.file_event_info.own_event") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.own_event".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.own_event", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.file_event_info.path") {
                event.rename("json.file_event_info.path", "file.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.root_changed") {
                if let Some(val) = event.get("json.file_event_info.root_changed") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.root_changed".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.root_changed", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.unmount") {
                if let Some(val) = event.get("json.file_event_info.unmount") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.unmount".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.unmount", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.file_event_info.user_dropped") {
                if let Some(val) = event.get("json.file_event_info.user_dropped") {
                let converted = convert_value(val, "boolean")
                .map_err(|message| TransformError::ParseError {
                path: "json.file_event_info.user_dropped".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.file_event_info.user_dropped", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // End nested pipeline: "pipeline_compliance_reporter_tamper_event_and_file_event_info"
            }

            let _cond = { event.get_str("event.action") == Some("gatekeeper_info_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_gatekeeper_info_event"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.assessments_enabled") {
                if let Some(val) = event.get("json.event_attributes.assessments_enabled") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.assessments_enabled".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.assessments_enabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.dev_id_enabled") {
                if let Some(val) = event.get("json.event_attributes.dev_id_enabled") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.dev_id_enabled".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.dev_id_enabled", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.event_attributes.opaque_version") {
                event.rename("json.event_attributes.opaque_version", "jamf_compliance_reporter.log.event_attributes.opaque_version")?;
                }
                if event.has_value("json.event_attributes.version") {
                event.rename("json.event_attributes.version", "jamf_compliance_reporter.log.event_attributes.version")?;
                }
                // End nested pipeline: "pipeline_gatekeeper_info_event"
            }

            let _cond = { event.get_str("event.action") == Some("gatekeeper_manual_overrides") };
            if _cond {
                // Begin nested pipeline: "pipeline_gatekeeper_manual_overrides"
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("json.event_attributes.attributes").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.ctime") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("_ingest._value.ctime", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.ctime".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.remove("_ingest._value.ctime");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("json.event_attributes.attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("json.event_attributes.attributes").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.mtime") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("_ingest._value.mtime", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.mtime".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.remove("_ingest._value.mtime");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("json.event_attributes.attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                if event.has_value("json.event_attributes.attributes") {
                event.rename("json.event_attributes.attributes", "jamf_compliance_reporter.log.event_attributes.attributes")?;
                }
                if event.has_value("json.event_attributes.path") {
                event.rename("json.event_attributes.path", "jamf_compliance_reporter.log.event_attributes.path")?;
                }
                // End nested pipeline: "pipeline_gatekeeper_manual_overrides"
            }

            let _cond = { event.get_str("event.action") == Some("gatekeeper_quarantine_log") };
            if _cond {
                // Begin nested pipeline: "pipeline_gatekeeper_quarantine_log"
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("_ingest._value.QuarantineAgentBundleIdentifier", "_ingest._value.quarantine.agent_bundle_identifier")?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("_ingest._value.QuarantineAgentName", "_ingest._value.quarantine.agent_name")?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("_ingest._value.QuarantineDataURLString", "_ingest._value.quarantine.data_url_string")?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("_ingest._value.QuarantineEventIdentifier", "_ingest._value.quarantine.event_identifier")?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("_ingest._value.QuarantineOriginURLString", "_ingest._value.quarantine.origin_url_string")?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("json.event_attributes.attributes").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.QuarantineTimeStamp") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("_ingest._value.quarantine.timestamp", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.QuarantineTimeStamp".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.remove("_ingest._value.QuarantineTimeStamp");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("json.event_attributes.attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("json.event_attributes.attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("json.event_attributes.attributes").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.remove("_ingest._value.QuarantineTimeStamp").is_none() {
                return Err(TransformError::FieldNotFound { path: "_ingest._value.QuarantineTimeStamp".into() });
                }
                Ok(())
                })();
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("json.event_attributes.attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                if event.has_value("json.event_attributes.attributes") {
                event.rename("json.event_attributes.attributes", "jamf_compliance_reporter.log.event_attributes.attributes")?;
                }
                if event.has_value("json.event_attributes.path") {
                event.rename("json.event_attributes.path", "jamf_compliance_reporter.log.event_attributes.path")?;
                }
                // End nested pipeline: "pipeline_gatekeeper_quarantine_log"
            }

            let _cond = { event.get_str("event.action") == Some("hardware_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_hardware_event"
                if event.has_value("json.hardware_event_info.device_attributes.IOCFPlugInTypes") {
                event.rename("json.hardware_event_info.device_attributes.IOCFPlugInTypes", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.cf_plugin_types")?;
                }
                if event.has_value("json.hardware_event_info.device_attributes.IOClassNameOverride") {
                event.rename("json.hardware_event_info.device_attributes.IOClassNameOverride", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.class_name_override")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.capability_flags", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.current_power_state", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.device_power_state", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.driver_power_state", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.max_power_state", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.hardware_event_info.device_attributes.Removable") {
                event.rename("json.hardware_event_info.device_attributes.Removable", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.removable")?;
                }
                if event.has_value("json.hardware_event_info.device_attributes.USB Product Name") {
                event.rename("json.hardware_event_info.device_attributes.USB Product Name", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.usb.product_name")?;
                }
                if event.has_value("json.hardware_event_info.device_attributes.USB Vendor Name") {
                event.rename("json.hardware_event_info.device_attributes.USB Vendor Name", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.usb.vendor_name")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.hardware_event_info.device_attributes.iSerialNumber") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.iSerialNumber") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.iSerialNumber".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.iserial_number", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.hardware_event_info.device_class") {
                event.rename("json.hardware_event_info.device_class", "jamf_compliance_reporter.log.hardware_event_info.device.class")?;
                }
                if event.has_value("json.hardware_event_info.device_name") {
                event.rename("json.hardware_event_info.device_name", "jamf_compliance_reporter.log.hardware_event_info.device.name")?;
                }
                if event.has_value("json.hardware_event_info.device_status") {
                event.rename("json.hardware_event_info.device_status", "jamf_compliance_reporter.log.hardware_event_info.device.status")?;
                }
                // End nested pipeline: "pipeline_hardware_event"
            }

            let _cond = { event.get_str("event.action") == Some("license_info_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_license_info_event"
                if event.has_value("json.ComplianceReporter_license_info.email") {
                event.rename("json.ComplianceReporter_license_info.email", "user.email")?;
                }
                let _cond = { event.has_value("user.email") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.email").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.ComplianceReporter_license_info.expiration_date") && event.get_i64("json.ComplianceReporter_license_info.expiration_date") != Some(0) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.ComplianceReporter_license_info.expiration_date") {
                match parse_date_out(&date_str, &["dd/MM/yyyy"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.compliancereporter_license_info.expiration_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.ComplianceReporter_license_info.expiration_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if event.has_value("json.ComplianceReporter_license_info.status") {
                event.rename("json.ComplianceReporter_license_info.status", "jamf_compliance_reporter.log.compliancereporter_license_info.status")?;
                }
                let _cond = { event.has_value("json.ComplianceReporter_license_info.time_seconds_epoch") && event.get_str("json.ComplianceReporter_license_info.time_seconds_epoch") != Some("0") };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.ComplianceReporter_license_info.time_seconds_epoch") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.compliancereporter_license_info.time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.ComplianceReporter_license_info.time_seconds_epoch".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if event.has_value("json.ComplianceReporter_license_info.type") {
                event.rename("json.ComplianceReporter_license_info.type", "jamf_compliance_reporter.log.compliancereporter_license_info.type")?;
                }
                if event.has_value("json.ComplianceReporter_license_info.version") {
                event.rename("json.ComplianceReporter_license_info.version", "jamf_compliance_reporter.log.compliancereporter_license_info.version")?;
                }
                // End nested pipeline: "pipeline_license_info_event"
            }

            let _cond = { event.get_str("event.action") == Some("preference_list_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_preference_list_event"
                if event.has_value("json.event_attributes.AuditEventExcludedProcesses") {
                event.rename("json.event_attributes.AuditEventExcludedProcesses", "jamf_compliance_reporter.log.event_attributes.audit_event.excluded_processes")?;
                }
                if event.has_value("json.event_attributes.AuditEventExcludedUsers") {
                event.rename("json.event_attributes.AuditEventExcludedUsers", "jamf_compliance_reporter.log.event_attributes.audit_event.excluded_users")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.AuditEventLogVerboseMessages") {
                if let Some(val) = event.get("json.event_attributes.AuditEventLogVerboseMessages") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.AuditEventLogVerboseMessages".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.audit_event_log_verbose_messages", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.AuditLevel") {
                if let Some(val) = event.get("json.event_attributes.AuditLevel") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.AuditLevel".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.audit_level", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.event_attributes.FileEventExclusionPaths") {
                event.rename("json.event_attributes.FileEventExclusionPaths", "jamf_compliance_reporter.log.event_attributes.file_event.exclusion_paths")?;
                }
                if event.has_value("json.event_attributes.FileEventInclusionPaths") {
                event.rename("json.event_attributes.FileEventInclusionPaths", "jamf_compliance_reporter.log.event_attributes.file_event.inclusion_paths")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.FileEventUseFuzzyMatch") {
                if let Some(val) = event.get("json.event_attributes.FileEventUseFuzzyMatch") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.FileEventUseFuzzyMatch".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.file_event.use_fuzzy_match", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseEmail") {
                event.rename("json.event_attributes.FileLicenseInfo.LicenseEmail", "user.email")?;
                }
                let _cond = { event.has_value("user.email") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.email").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.get_str("json.event_attributes.FileLicenseInfo.LicenseExpirationDate") != Some("0") };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.FileLicenseInfo.LicenseExpirationDate") {
                match parse_date_out(&date_str, &["dd/MM/yyyy"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.file_license_info.license_expiration_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.FileLicenseInfo.LicenseExpirationDate".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseKey") {
                event.rename("json.event_attributes.FileLicenseInfo.LicenseKey", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_key")?;
                }
                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseType") {
                event.rename("json.event_attributes.FileLicenseInfo.LicenseType", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_type")?;
                }
                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseVersion") {
                event.rename("json.event_attributes.FileLicenseInfo.LicenseVersion", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_version")?;
                }
                if event.has_value("json.event_attributes.LogFileLocation") {
                event.rename("json.event_attributes.LogFileLocation", "jamf_compliance_reporter.log.event_attributes.log.file.location")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.LogFileMaxNumberBackups") {
                if let Some(val) = event.get("json.event_attributes.LogFileMaxNumberBackups") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.LogFileMaxNumberBackups".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.log.file.max_number_backups", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.LogFileMaxSizeMegaBytes") {
                if let Some(val) = event.get("json.event_attributes.LogFileMaxSizeMegaBytes") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.LogFileMaxSizeMegaBytes".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.log.file.max_size_mega_bytes", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.event_attributes.LogFileOwnership") {
                event.rename("json.event_attributes.LogFileOwnership", "jamf_compliance_reporter.log.event_attributes.log.file.ownership")?;
                }
                if event.has_value("json.event_attributes.LogFilePermission") {
                event.rename("json.event_attributes.LogFilePermission", "jamf_compliance_reporter.log.event_attributes.log.file.permission")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointEnabled") {
                event.rename("json.event_attributes.LogRemoteEndpointEnabled", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_enabled")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointType") {
                event.rename("json.event_attributes.LogRemoteEndpointType", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.AccessKeyId") {
                event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.AccessKeyId", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.access_key_id")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.Region") {
                event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.Region", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.region")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.SecretKey") {
                event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.SecretKey", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.secret_key")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.StreamName") {
                event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.StreamName", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.stream_name")?;
                }
                if event.has_value("json.event_attributes.LogRemoteEndpointURL") {
                event.rename("json.event_attributes.LogRemoteEndpointURL", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_url")?;
                }
                if event.has_value("json.event_attributes.UnifiedLogPredicates") {
                event.rename("json.event_attributes.UnifiedLogPredicates", "jamf_compliance_reporter.log.event_attributes.unified_log_predicates")?;
                }
                if event.has_value("json.event_attributes.Version") {
                event.rename("json.event_attributes.Version", "jamf_compliance_reporter.log.event_attributes.version")?;
                }
                // End nested pipeline: "pipeline_preference_list_event"
            }

            let _cond = { event.get_str("event.action") == Some("print_event_information") };
            if _cond {
                // Begin nested pipeline: "pipeline_print_event_information"
                let _cond = { event.has_value("json.event_attributes.job_completed_time") && event.get_i64("json.event_attributes.job_completed_time") != Some(0) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.job_completed_time") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.completed_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.job_completed_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                let _cond = { event.has_value("json.event_attributes.job_creation_time") && event.get_i64("json.event_attributes.job_creation_time") != Some(0) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.job_creation_time") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.creation_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.job_creation_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if event.has_value("json.event_attributes.job_destination") {
                event.rename("json.event_attributes.job_destination", "jamf_compliance_reporter.log.event_attributes.job.destination")?;
                }
                if event.has_value("json.event_attributes.job_format") {
                event.rename("json.event_attributes.job_format", "jamf_compliance_reporter.log.event_attributes.job.format")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.job_id") {
                if let Some(val) = event.get("json.event_attributes.job_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.job_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.job.id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.event_attributes.job_processing_time") && event.get_i64("json.event_attributes.job_processing_time") != Some(0) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.job_processing_time") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.processing_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.job_processing_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if event.has_value("json.event_attributes.job_size") {
                event.rename("json.event_attributes.job_size", "jamf_compliance_reporter.log.event_attributes.job.size")?;
                }
                if event.has_value("json.event_attributes.job_state") {
                event.rename("json.event_attributes.job_state", "jamf_compliance_reporter.log.event_attributes.job.state")?;
                }
                if event.has_value("json.event_attributes.job_title") {
                event.rename("json.event_attributes.job_title", "jamf_compliance_reporter.log.event_attributes.job.title")?;
                }
                if event.has_value("json.event_attributes.job_user") {
                event.rename("json.event_attributes.job_user", "jamf_compliance_reporter.log.event_attributes.job.user")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.event_attributes.job.user") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.event_attributes.job.user").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // End nested pipeline: "pipeline_print_event_information"
            }

            let _cond = { event.get_str("event.action") == Some("prohibited_app_blocked") };
            if _cond {
                // Begin nested pipeline: "pipeline_prohibited_app_blocked"
                if event.has_value("json.header.action") {
                event.rename("json.header.action", "jamf_compliance_reporter.log.header.action")?;
                }
                if event.has_value("json.exec_args.args") {
                event.rename("json.exec_args.args", "json.args")?;
                }
                if event.has_value("json.exec_args.args_compiled") {
                event.rename("json.exec_args.args_compiled", "jamf_compliance_reporter.log.exec_args.args_compiled")?;
                }
                if event.has_value("json.exec_env.env.PATH") {
                event.rename("json.exec_env.env.PATH", "jamf_compliance_reporter.log.exec_env.env.path")?;
                }
                if event.has_value("json.exec_env.env.SHELL") {
                event.rename("json.exec_env.env.SHELL", "jamf_compliance_reporter.log.exec_env.env.shell")?;
                }
                if event.has_value("json.exec_env.env.SSH_AUTH_SOCK") {
                event.rename("json.exec_env.env.SSH_AUTH_SOCK", "jamf_compliance_reporter.log.exec_env.env.ssh_auth_sock")?;
                }
                if event.has_value("json.exec_env.env.TMPDIR") {
                event.rename("json.exec_env.env.TMPDIR", "jamf_compliance_reporter.log.exec_env.env.tmpdir")?;
                }
                let _cond = { event.has_value("json.exec_env.env.USER") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.exec_env.env.USER").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.exec_env.env.USER") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.exec_env.env.USER").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.exec_env.env.XPC_FLAGS") {
                event.rename("json.exec_env.env.XPC_FLAGS", "jamf_compliance_reporter.log.exec_env.env.xpc.flags")?;
                }
                if event.has_value("json.exec_env.env.XPC_SERVICE_NAME") {
                event.rename("json.exec_env.env.XPC_SERVICE_NAME", "jamf_compliance_reporter.log.exec_env.env.xpc.service_name")?;
                }
                if event.has_value("json.exec_env.env_compiled") {
                event.rename("json.exec_env.env_compiled", "jamf_compliance_reporter.log.exec_env.env_compiled")?;
                }
                if event.has_value("json.identity.cd_hash") {
                event.rename("json.identity.cd_hash", "jamf_compliance_reporter.log.identity.cd_hash")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("jamf_compliance_reporter.log.identity.cd_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_id") {
                if let Some(val) = event.get("json.identity.signer_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_id_truncated") {
                if let Some(val) = event.get("json.identity.signer_id_truncated") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_id_truncated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.id_truncated", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_type") {
                if let Some(val) = event.get("json.identity.signer_type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.type", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.team_id") {
                if let Some(val) = event.get("json.identity.team_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.team_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.team.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.team_id_truncated") {
                if let Some(val) = event.get("json.identity.team_id_truncated") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.team_id_truncated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.team.id_truncated", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.audit_id") {
                if let Some(val) = event.get("json.subject.audit_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.audit_id".into(),
                message,
                })?;
                event.set("process.real_user.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.subject.audit_user_name") {
                event.rename("json.subject.audit_user_name", "process.real_user.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.effective_group_id") {
                if let Some(val) = event.get("json.subject.effective_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.effective_group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.subject.effective.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.subject.effective_group_name") {
                event.rename("json.subject.effective_group_name", "jamf_compliance_reporter.log.subject.effective.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.effective_user_id") {
                if let Some(val) = event.get("json.subject.effective_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.effective_user_id".into(),
                message,
                })?;
                event.set("user.effective.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if let Some(v) = event.get("user.effective.id").cloned() {
                event.set("jamf_compliance_reporter.log.subject.effective.user.id", v)?;
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.subject.effective_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.subject.effective_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.subject.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.subject.effective_user_name") {
                event.rename("json.subject.effective_user_name", "user.effective.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if let Some(v) = event.get("user.effective.name").cloned() {
                event.set("jamf_compliance_reporter.log.subject.effective.user.name", v)?;
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.group_id") {
                if let Some(val) = event.get("json.subject.group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.group_id".into(),
                message,
                })?;
                event.set("user.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.subject.group_name") {
                event.rename("json.subject.group_name", "user.group.name")?;
                }
                if event.has_value("json.subject.process_hash") {
                event.rename("json.subject.process_hash", "process.hash.sha1")?;
                }
                let _cond = { event.has_value("process.hash.sha1") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("process.hash.sha1").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.subject.process_id") {
                if let Some(val) = event.get("json.subject.process_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.process_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.subject.process.pid", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.subject.process_information") {
                event.rename("json.subject.process_information", "jamf_compliance_reporter.log.subject.process.information")?;
                }
                if event.has_value("json.subject.process_name") {
                event.rename("json.subject.process_name", "process.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.responsible_process_id") {
                if let Some(val) = event.get("json.subject.responsible_process_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.responsible_process_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.subject.responsible.process.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.subject.responsible_process_name") {
                event.rename("json.subject.responsible_process_name", "jamf_compliance_reporter.log.subject.responsible.process.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.session_id") {
                if let Some(val) = event.get("json.subject.session_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.session_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.subject.session.id", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.subject.terminal_id.ip_address") {
                if let Some(val) = event.get("json.subject.terminal_id.ip_address") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.terminal_id.ip_address".into(),
                message,
                })?;
                event.set("json.subject.terminal_id.ip_address", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("json.subject.terminal_id.ip_address");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("host.ip", json!(event.get("json.subject.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("json.subject.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.subject.terminal_id.port") {
                if let Some(val) = event.get("json.subject.terminal_id.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.terminal_id.port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.subject.terminal_id.port", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.terminal_id.type") {
                if let Some(val) = event.get("json.subject.terminal_id.type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.terminal_id.type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.subject.terminal_id.type", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.subject.user_id") {
                if let Some(val) = event.get("json.subject.user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.subject.user_id".into(),
                message,
                })?;
                event.set("user.id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.subject.user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.subject.user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.subject.user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.texts") {
                event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }
                // Painless script
                // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#))?;
                // End nested pipeline: "pipeline_prohibited_app_blocked"
            }

            let _cond = { event.get_str("event.action") == Some("signal_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_signal_event"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.signal_event_info.signal") {
                if let Some(val) = event.get("json.signal_event_info.signal") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.signal_event_info.signal".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.signal_event_info.signal", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                // End nested pipeline: "pipeline_signal_event"
            }

            let _cond = { event.get_str("event.action") == Some("unified_log_event") };
            if _cond {
                // Begin nested pipeline: "pipeline_unified_log_event"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.activityIdentifier") {
                if let Some(val) = event.get("json.event_attributes.activityIdentifier") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.activityIdentifier".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.activity_identifier", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.get("json.event_attributes.backtrace.frames").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.backtrace.frames", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("_ingest._value.imageOffset") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.imageOffset".into(),
                message,
                })?;
                event.set("_ingest._value.image_offset", converted)?;
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("_ingest._value.imageOffset");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                let _cond = { event.get("json.event_attributes.backtrace.frames").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("json.event_attributes.backtrace.frames").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.remove("_ingest._value.imageOffset").is_none() {
                return Err(TransformError::FieldNotFound { path: "_ingest._value.imageOffset".into() });
                }
                Ok(())
                })();
                let left = event.remove("_ingest._value");
                match key {
                // An entry the body renamed AWAY is gone from the
                // object, which is how a foreach lifts fields up.
                Some(key) => {
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("json.event_attributes.backtrace.frames", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("json.event_attributes.backtrace.frames").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes.backtrace.frames", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("_ingest._value.imageUUID", "_ingest._value.image_uuid")?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                if event.has_value("json.event_attributes.backtrace.frames") {
                event.rename("json.event_attributes.backtrace.frames", "jamf_compliance_reporter.log.event_attributes.backtrace.frames")?;
                }
                if event.has_value("json.event_attributes.category") {
                event.rename("json.event_attributes.category", "jamf_compliance_reporter.log.event_attributes.category")?;
                }
                if event.has_value("json.event_attributes.eventMessage") {
                event.rename("json.event_attributes.eventMessage", "jamf_compliance_reporter.log.event_attributes.event.message")?;
                }
                if event.has_value("json.event_attributes.eventType") {
                event.rename("json.event_attributes.eventType", "jamf_compliance_reporter.log.event_attributes.event.type")?;
                }
                if event.has_value("json.event_attributes.formatString") {
                event.rename("json.event_attributes.formatString", "jamf_compliance_reporter.log.event_attributes.format_string")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.machTimestamp") {
                if let Some(val) = event.get("json.event_attributes.machTimestamp") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.machTimestamp".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.mach_timestamp", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.event_attributes.messageType") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("event.type", json!("info"))?;
                Ok(())
                })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.parentActivityIdentifier") {
                if let Some(val) = event.get("json.event_attributes.parentActivityIdentifier") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.parentActivityIdentifier".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.processID") {
                if let Some(val) = event.get("json.event_attributes.processID") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.processID".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.process.id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.event_attributes.processImagePath") {
                event.rename("json.event_attributes.processImagePath", "jamf_compliance_reporter.log.event_attributes.process.image.path")?;
                }
                if event.has_value("json.event_attributes.processImageUUID") {
                event.rename("json.event_attributes.processImageUUID", "jamf_compliance_reporter.log.event_attributes.process.image.uuid")?;
                }
                if event.has_value("json.event_attributes.senderImagePath") {
                event.rename("json.event_attributes.senderImagePath", "jamf_compliance_reporter.log.event_attributes.sender.image.path")?;
                }
                if event.has_value("json.event_attributes.senderImageUUID") {
                event.rename("json.event_attributes.senderImageUUID", "jamf_compliance_reporter.log.event_attributes.sender.image.uuid")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_attributes.senderProgramCounter") {
                if let Some(val) = event.get("json.event_attributes.senderProgramCounter") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.senderProgramCounter".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.sender.program_counter", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                if event.has_value("json.event_attributes.source") {
                event.rename("json.event_attributes.source", "jamf_compliance_reporter.log.event_attributes.source")?;
                }
                if event.has_value("json.event_attributes.subsystem") {
                event.rename("json.event_attributes.subsystem", "jamf_compliance_reporter.log.event_attributes.subsystem")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.threadID") {
                if let Some(val) = event.get("json.event_attributes.threadID") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.threadID".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.thread_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.event_attributes.timestamp") && event.get_i64("json.event_attributes.timestamp") != Some(0) };
                if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.timestamp") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSSZ"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.timestamp", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.timestamp".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                }
                if event.has_value("json.event_attributes.timezoneName") {
                event.rename("json.event_attributes.timezoneName", "jamf_compliance_reporter.log.event_attributes.timezone_name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.event_attributes.traceID") {
                if let Some(val) = event.get("json.event_attributes.traceID") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.event_attributes.traceID".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.trace_id", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_unified_log_event"
            }

            let _cond = { event.get_str("event.action") == Some("xprotect_definitions_version_info") };
            if _cond {
                // Begin nested pipeline: "pipeline_xprotect_definitions_version_info"
                if event.has_value("json.event_attributes.BuildAliasOf") {
                event.rename("json.event_attributes.BuildAliasOf", "jamf_compliance_reporter.log.event_attributes.build_alias_of")?;
                }
                if event.has_value("json.event_attributes.BuildVersion") {
                event.rename("json.event_attributes.BuildVersion", "jamf_compliance_reporter.log.event_attributes.build_version")?;
                }
                if event.has_value("json.event_attributes.CFBundleShortVersionString") {
                event.rename("json.event_attributes.CFBundleShortVersionString", "jamf_compliance_reporter.log.event_attributes.cf_bundle_short_version_string")?;
                }
                if event.has_value("json.event_attributes.CFBundleVersion") {
                event.rename("json.event_attributes.CFBundleVersion", "jamf_compliance_reporter.log.event_attributes.cf_bundle_version")?;
                }
                if event.has_value("json.event_attributes.ProjectName") {
                event.rename("json.event_attributes.ProjectName", "jamf_compliance_reporter.log.event_attributes.project_name")?;
                }
                if event.has_value("json.event_attributes.SourceVersion") {
                event.rename("json.event_attributes.SourceVersion", "jamf_compliance_reporter.log.event_attributes.source_version")?;
                }
                // End nested pipeline: "pipeline_xprotect_definitions_version_info"
            }

            let _cond = { event.get_str("event.action") == Some("xprotect_event_log") };
            if _cond {
                // Begin nested pipeline: "pipeline_xprotect_event_log"
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.activity_identifier", json!(event.get("_ingest._value.activityIdentifier").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.activity_identifier") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.activity_identifier") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.activity_identifier".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.activity_identifier", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                foreach_array(event, "_ingest._value.backtrace.frames", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset", json!(event.get("_ingest._value.imageOffset").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                Ok(())
                })?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                foreach_array(event, "_ingest._value.backtrace.frames", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_uuid", json!(event.get("_ingest._value.imageUUID").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.category", json!(event.get("_ingest._value.category").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.event.message", json!(event.get("_ingest._value.eventMessage").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.event.type", json!(event.get("_ingest._value.eventType").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.format_string", json!(event.get("_ingest._value.formatString").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("_ingest._value.machTimestamp") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.machTimestamp".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.mach_timestamp", converted)?;
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("_ingest._value.machTimestamp");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("event.type", json!("info"))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", json!(event.get("_ingest._value.parentActivityIdentifier").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.parent_activity_identifier".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.process.id", json!(event.get("_ingest._value.processID").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.process.id") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.process.id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.process.id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.process.id", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.process.image.path", json!(event.get("_ingest._value.processImagePath").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.process.image.uuid", json!(event.get("_ingest._value.processImageUUID").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.image.path", json!(event.get("_ingest._value.senderImagePath").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.image.uuid", json!(event.get("_ingest._value.senderImageUUID").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.program_counter", json!(event.get("_ingest._value.senderProgramCounter").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.sender.program_counter") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.sender.program_counter") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.sender.program_counter".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.sender.program_counter", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.source", json!(event.get("_ingest._value.source").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.subsystem", json!(event.get("_ingest._value.subsystem").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.thread_id", json!(event.get("_ingest._value.threadID").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.thread_id") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.thread_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.thread_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.thread_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("json.event_attributes").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                if let Some(date_str) = event.get_as_string("_ingest._value.timestamp") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSSZ"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.timestamp", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.timestamp".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.remove("_ingest._value.timestamp");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(value) = left { fields.insert(key, value); }
                }
                None => list.push(left.unwrap_or(Value::Null)),
                }
                }
                match enclosing {
                Some(previous) => { event.set("_ingest._value", previous)?; }
                None => { event.remove("_ingest"); }
                }
                if let Some(previous) = enclosing_key {
                event.set("_ingest._key", previous)?;
                }
                event.set("json.event_attributes", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
                }
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.timezone_name", json!(event.get("_ingest._value.timezone_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.timezoneName", json!(event.get("_ingest._value.timezoneName").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                let _cond = { event.get("json.event_attributes").is_some_and(|v| v.is_array()) };
                if _cond {
                foreach_array(event, "json.event_attributes", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("jamf_compliance_reporter.log.event_attributes.trace_id", json!(event.get("_ingest._value.traceID").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
                })?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("jamf_compliance_reporter.log.event_attributes.trace_id") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.trace_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.trace_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.trace_id", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_xprotect_event_log"
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
