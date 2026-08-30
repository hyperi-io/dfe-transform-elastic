// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `appservice_auditlogs_inner_pipeline` pipeline.
pub struct AppserviceAuditlogsInnerPipeline;

impl Transform for AppserviceAuditlogsInnerPipeline {
    fn name(&self) -> &str {
        "appservice_auditlogs_inner_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("azure.app_service.ResourceId") {
                    event.rename("azure.app_service.ResourceId", "azure.resource.id")?;
                }

                event.rename("azure.app_service.Category", "azure.app_service.category")?;

                event.rename("azure.app_service.OperationName", "azure.app_service.operation_name")?;

                if event.has_value("azure.app_service.Properties") {
                    event.rename("azure.app_service.Properties", "azure.app_service.properties")?;
                }

                event.rename("azure.app_service.properties.Protocol", "azure.app_service.properties.protocol")?;

                event.rename("azure.app_service.properties.User", "azure.app_service.properties.user")?;

                event.rename("azure.app_service.properties.UserAddress", "azure.app_service.properties.client_ip")?;

                event.rename("azure.app_service.properties.UserDisplayName", "azure.app_service.properties.user_display_name")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.app_service.properties.client_ip") {
                    // Grok pattern: %{IPORHOST:azure.app_service.properties.client_ip}:%{POSINT:azure.app_service.properties.client_port:long}
                    let _ = cached_grok!("%{IPORHOST:azure.app_service.properties.client_ip}:%{POSINT:azure.app_service.properties.client_port:long}").extract_into(&input, event)?;
                }
                Ok(())
            })();

            if event.has_value("azure.app_service.properties.client_ip") {
                if let Some(val) = event.get("azure.app_service.properties.client_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "azure.app_service.properties.client_ip".into(),
                            message,
                        })?;
                    event.set("azure.app_service.properties.client_ip", converted)?;
                }
            }

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
