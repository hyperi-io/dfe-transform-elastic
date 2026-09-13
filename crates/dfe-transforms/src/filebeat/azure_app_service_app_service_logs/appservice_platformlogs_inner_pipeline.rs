// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `appservice_platformlogs_inner_pipeline` pipeline.
pub struct AppservicePlatformlogsInnerPipeline;

impl Transform for AppservicePlatformlogsInnerPipeline {
    fn name(&self) -> &str {
        "appservice_platformlogs_inner_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("azure.app_service.resourceId") {
                    event.rename("azure.app_service.resourceId", "azure.resource.id")?;
                }

                event.rename("azure.app_service.operationName", "azure.app_service.operation_name")?;

                event.rename("azure.app_service.EventStampType", "azure.app_service.event_stamp_type")?;

                event.rename("azure.app_service.EventPrimaryStampName", "azure.app_service.event_primary_stamp_name")?;

                event.rename("azure.app_service.EventStampName", "azure.app_service.event_stamp_name")?;

                event.rename("azure.app_service.Host", "azure.app_service.host")?;

                event.rename("azure.app_service.EventIpAddress", "azure.app_service.event_ip_address")?;

                event.rename("azure.app_service.properties_raw", "azure.app_service.log")?;

                event.remove("azure.app_service.properties");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("{} {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
