// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_resources_data_json` pipeline.
pub struct PipelineResourcesDataJson;

impl Transform for PipelineResourcesDataJson {
    fn name(&self) -> &str {
        "pipeline_resources_data_json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.rename("_ingest._value", "_tmp_resource")?;

        let _cond = { event.get("_tmp_resource.data").is_some_and(|v| v.is_string()) };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            parse_json_field(event, "_tmp_resource.data", "_tmp_resource.data")?;
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "json")?;
                    event.rename("_tmp_resource.data", "_tmp_resource.data.value")?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("_tmp_resource.data") && !(event.get("_tmp_resource.data").is_some_and(|v| v.is_object())) };
        if _cond {
            event.rename("_tmp_resource.data", "_tmp_resource.data.value")?;
        }

        if let Some(v) = event.get("_tmp_resource").cloned() {
            event.set("_ingest._value", v)?;
        }

            if event.remove("_tmp_resource").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_resource".into() });
            }

        Ok(TransformResult::Continue)
    }
}
