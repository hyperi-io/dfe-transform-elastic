// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `tcp` pipeline.
pub struct Tcp;

impl Transform for Tcp {
    fn name(&self) -> &str {
        "tcp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.remove("upstream_service_time").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "upstream_service_time".into(),
                });
            }
            if event.remove("method").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "method".into(),
                });
            }
            if event.remove("user_agent").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "user_agent".into(),
                });
            }
            if event.remove("path").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "path".into(),
                });
            }
            if event.remove("response_code").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "response_code".into(),
                });
            }

            event.rename("bytes_received", "destination.bytes")?;

            if let Some(val) = event.get("destination.bytes") {
                let converted =
                    convert_value(val, "long").map_err(|message| TransformError::ParseError {
                        path: "destination.bytes".into(),
                        message,
                    })?;
                event.set("destination.bytes", converted)?;
            }

            event.rename_over("bytes_sent", "source.bytes")?;

            if let Some(val) = event.get("source.bytes") {
                let converted =
                    convert_value(val, "long").map_err(|message| TransformError::ParseError {
                        path: "source.bytes".into(),
                        message,
                    })?;
                event.set("source.bytes", converted)?;
            }

            event.set("envoyproxy.log.proxy_type", json!("tcp"))?;

            event.append_unique("event.type", json!("connection"))?;

            event.set("network.transport", json!("tcp"))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
