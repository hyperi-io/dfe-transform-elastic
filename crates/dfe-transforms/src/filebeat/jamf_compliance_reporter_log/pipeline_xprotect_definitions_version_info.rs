// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_xprotect_definitions_version_info` pipeline.
pub struct PipelineXprotectDefinitionsVersionInfo;

impl Transform for PipelineXprotectDefinitionsVersionInfo {
    fn name(&self) -> &str {
        "pipeline_xprotect_definitions_version_info"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
