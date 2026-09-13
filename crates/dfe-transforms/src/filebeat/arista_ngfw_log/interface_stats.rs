// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `interface_stats` pipeline.
pub struct InterfaceStats;

impl Transform for InterfaceStats {
    fn name(&self) -> &str {
        "interface_stats"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("arista.interfaceId") {
                    event.rename("arista.interfaceId", "arista.interface.id")?;
                }

                if event.has_value("arista.rxBytes") {
                    event.rename("arista.rxBytes", "arista.received.bytes")?;
                }

                if event.has_value("arista.rxRate") {
                    event.rename("arista.rxRate", "arista.received.rate")?;
                }

            if event.has_value("arista.received.rate") {
                if let Some(val) = event.get("arista.received.rate") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "arista.received.rate".into(),
                            message,
                        })?;
                    event.set("arista.received.rate", converted)?;
                }
            }

                if event.has_value("arista.txBytes") {
                    event.rename("arista.txBytes", "arista.transmitted.bytes")?;
                }

                if event.has_value("arista.txRate") {
                    event.rename("arista.txRate", "arista.transmitted.rate")?;
                }

            if event.has_value("arista.transmitted.rate") {
                if let Some(val) = event.get("arista.transmitted.rate") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "arista.transmitted.rate".into(),
                            message,
                        })?;
                    event.set("arista.transmitted.rate", converted)?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
