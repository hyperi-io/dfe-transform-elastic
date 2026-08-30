// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_file_collection` pipeline.
pub struct PipelineEventFileCollection;

impl Transform for PipelineEventFileCollection {
    fn name(&self) -> &str {
        "pipeline_event_file_collection"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("A crash or diagnostic file is detected being created"))?;

            if event.has_value("jamf_protect.telemetry.event.file_collection.file.path") {
                event.rename("jamf_protect.telemetry.event.file_collection.file.path", "log.file.path")?;
            }

        if event.has_value("jamf_protect.telemetry.event.file_collection.contents") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.file_collection.contents") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.file_collection.contents".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.log_entries", converted)?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
