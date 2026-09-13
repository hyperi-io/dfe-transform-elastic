// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_log_collection` pipeline.
pub struct PipelineEventLogCollection;

impl Transform for PipelineEventLogCollection {
    fn name(&self) -> &str {
        "pipeline_event_log_collection"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("New entries have been collected from a log file"))?;

            if event.has_value("jamf_protect.telemetry.event.log_collection.path") {
                event.rename("jamf_protect.telemetry.event.log_collection.path", "log.file.path")?;
            }

        if event.has_value("jamf_protect.telemetry.event.log_collection.texts") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.log_collection.texts") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.log_collection.texts".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.log_entries", converted)?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
