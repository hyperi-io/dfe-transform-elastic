// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `appservice_httplogs_inner_pipeline` pipeline.
pub struct AppserviceHttplogsInnerPipeline;

impl Transform for AppserviceHttplogsInnerPipeline {
    fn name(&self) -> &str {
        "appservice_httplogs_inner_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("azure.app_service.resourceId") {
                    event.rename("azure.app_service.resourceId", "azure.resource.id")?;
                }

                event.rename("azure.app_service.properties.CIp", "azure.app_service.properties.client_ip")?;

                if event.has_value("azure.app_service.properties.Protocol") {
                    event.rename("azure.app_service.properties.Protocol", "azure.app_service.properties.protocol")?;
                }

                event.rename("azure.app_service.properties.ComputerName", "azure.app_service.properties.computer_name")?;

                event.rename("azure.app_service.properties.Cookie", "azure.app_service.properties.cookie")?;

                event.rename("azure.app_service.properties.CsBytes", "azure.app_service.properties.cs_bytes")?;

            if event.has_value("azure.app_service.properties.cs_bytes") {
                if let Some(val) = event.get("azure.app_service.properties.cs_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "azure.app_service.properties.cs_bytes".into(),
                            message,
                        })?;
                    event.set("azure.app_service.properties.cs_bytes", converted)?;
                }
            }

                event.rename("azure.app_service.properties.CsHost", "azure.app_service.properties.cs_host")?;

                event.rename("azure.app_service.properties.CsMethod", "azure.app_service.properties.cs_method")?;

                event.rename("azure.app_service.properties.CsUriQuery", "azure.app_service.properties.cs_uri_query")?;

                event.rename("azure.app_service.properties.CsUriStem", "azure.app_service.properties.cs_uri_stem")?;

                event.rename("azure.app_service.properties.CsUsername", "azure.app_service.properties.cs_username")?;

                event.rename("azure.app_service.properties.Referer", "azure.app_service.properties.referer")?;

                event.rename("azure.app_service.properties.Result", "azure.app_service.properties.result")?;

                event.rename("azure.app_service.properties.SPort", "azure.app_service.properties.s_port")?;

                event.rename("azure.app_service.properties.ScBytes", "azure.app_service.properties.sc_bytes")?;

            if event.has_value("azure.app_service.properties.sc_bytes") {
                if let Some(val) = event.get("azure.app_service.properties.sc_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "azure.app_service.properties.sc_bytes".into(),
                            message,
                        })?;
                    event.set("azure.app_service.properties.sc_bytes", converted)?;
                }
            }

                event.rename("azure.app_service.properties.ScStatus", "azure.app_service.properties.sc_status")?;

            if event.has_value("azure.app_service.properties.sc_status") {
                if let Some(val) = event.get("azure.app_service.properties.sc_status") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "azure.app_service.properties.sc_status".into(),
                            message,
                        })?;
                    event.set("azure.app_service.properties.sc_status", converted)?;
                }
            }

                if event.has_value("azure.app_service.properties.ScSubStatus") {
                    event.rename("azure.app_service.properties.ScSubStatus", "azure.app_service.properties.sc_substatus")?;
                }

                if event.has_value("azure.app_service.properties.ScWin32Status") {
                    event.rename("azure.app_service.properties.ScWin32Status", "azure.app_service.properties.sc_win32status")?;
                }

                event.rename("azure.app_service.properties.TimeTaken", "azure.app_service.properties.time_taken")?;

                event.rename("azure.app_service.properties.UserAgent", "azure.app_service.properties.user_agent")?;

                if event.has_value("azure.app_service.EventIpAddress") {
                    event.rename("azure.app_service.EventIpAddress", "azure.app_service.event_ip_address")?;
                }

                if event.has_value("azure.app_service.EventPrimaryStampName") {
                    event.rename("azure.app_service.EventPrimaryStampName", "azure.app_service.event_primary_stamp_name")?;
                }

                if event.has_value("azure.app_service.EventStampName") {
                    event.rename("azure.app_service.EventStampName", "azure.app_service.event_stamp_name")?;
                }

                if event.has_value("azure.app_service.EventStampType") {
                    event.rename("azure.app_service.EventStampType", "azure.app_service.event_stamp_type")?;
                }

                if event.has_value("azure.app_service.Host") {
                    event.rename("azure.app_service.Host", "azure.app_service.host")?;
                }

                event.remove("azure.app_service.EventTime");

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
