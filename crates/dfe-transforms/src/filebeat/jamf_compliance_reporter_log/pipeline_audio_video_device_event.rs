// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_audio_video_device_event` pipeline.
pub struct PipelineAudioVideoDeviceEvent;

impl Transform for PipelineAudioVideoDeviceEvent {
    fn name(&self) -> &str {
        "pipeline_audio_video_device_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
