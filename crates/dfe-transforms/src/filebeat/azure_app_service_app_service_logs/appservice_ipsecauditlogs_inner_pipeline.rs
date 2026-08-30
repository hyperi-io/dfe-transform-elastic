// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `appservice_ipsecauditlogs_inner_pipeline` pipeline.
pub struct AppserviceIpsecauditlogsInnerPipeline;

impl Transform for AppserviceIpsecauditlogsInnerPipeline {
    fn name(&self) -> &str {
        "appservice_ipsecauditlogs_inner_pipeline"
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

                event.rename("azure.app_service.properties.CIp", "azure.app_service.properties.client_ip")?;

                event.rename("azure.app_service.properties.CsHost", "azure.app_service.properties.cs_host")?;

                event.rename("azure.app_service.properties.Result", "azure.app_service.properties.result")?;

                event.rename("azure.app_service.properties.Details", "azure.app_service.properties.details")?;

                event.rename("azure.app_service.properties.ServiceEndpoint", "azure.app_service.properties.service_endpoint")?;

                event.rename("azure.app_service.properties.XForwardedFor", "azure.app_service.properties.xforwarded_for")?;

                event.rename("azure.app_service.properties.XForwardedHost", "azure.app_service.properties.xforwarded_host")?;

                event.rename("azure.app_service.properties.XAzureFDID", "azure.app_service.properties.xazurefdid")?;

                event.rename("azure.app_service.properties.XFDHealthProbe", "azure.app_service.properties.xfdhealth_probe")?;

                if event.has_value("azure.app_service.properties.Type") {
                    event.rename("azure.app_service.properties.Type", "azure.app_service.properties.type")?;
                }

                if event.has_value("azure.app_service.properties.TimeGenerated") {
                    event.rename("azure.app_service.properties.TimeGenerated", "azure.app_service.properties.time_generated")?;
                }

                if event.has_value("azure.app_service.properties.SourceSystem") {
                    event.rename("azure.app_service.properties.SourceSystem", "azure.app_service.properties.source_system")?;
                }

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
