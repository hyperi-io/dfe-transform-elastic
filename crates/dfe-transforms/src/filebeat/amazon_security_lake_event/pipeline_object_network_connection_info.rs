// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_network_connection_info` pipeline.
pub struct PipelineObjectNetworkConnectionInfo;

impl Transform for PipelineObjectNetworkConnectionInfo {
    fn name(&self) -> &str {
        "pipeline_object_network_connection_info"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("ocsf.connection_info.boundary_id") {
                if let Some(val) = event.get("ocsf.connection_info.boundary_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.connection_info.boundary_id".into(),
                            message,
                        })?;
                    event.set("ocsf.connection_info.boundary_id", converted)?;
                }
            }

            if event.has_value("ocsf.connection_info.direction_id") {
                if let Some(val) = event.get("ocsf.connection_info.direction_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.connection_info.direction_id".into(),
                            message,
                        })?;
                    event.set("ocsf.connection_info.direction_id", converted)?;
                }
            }

            if event.has_value("ocsf.connection_info.protocol_ver_id") {
                if let Some(val) = event.get("ocsf.connection_info.protocol_ver_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.connection_info.protocol_ver_id".into(),
                            message,
                        })?;
                    event.set("ocsf.connection_info.protocol_ver_id", converted)?;
                }
            }

            if event.has_value("ocsf.connection_info.protocol_num") {
                if let Some(val) = event.get("ocsf.connection_info.protocol_num") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.connection_info.protocol_num".into(),
                            message,
                        })?;
                    event.set("ocsf.connection_info.protocol_num", converted)?;
                }
            }

            let _cond = { event.get_str("ocsf.connection_info.tcp_flags") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.connection_info.tcp_flags") {
                if let Some(val) = event.get("ocsf.connection_info.tcp_flags") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.connection_info.tcp_flags".into(),
                            message,
                        })?;
                    event.set("ocsf.connection_info.tcp_flags", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_connection_info_tcp_flags_to_long")?;
                        event.remove("ocsf.connection_info.tcp_flags");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("ocsf.connection_info.protocol_ver") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.connection_info.protocol_ver") {
                map_strings(event, "ocsf.connection_info.protocol_ver", "network.type", str::to_lowercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "lowercase")?;
                event.set("_ingest.on_failure_processor_tag", "lowercase_ocsf_connection_info_protocol_ver")?;
                        event.remove("ocsf.connection_info.protocol_ver");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("ocsf.connection_info.protocol_name") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.connection_info.protocol_name") {
                map_strings(event, "ocsf.connection_info.protocol_name", "network.transport", str::to_lowercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "lowercase")?;
                event.set("_ingest.on_failure_processor_tag", "lowercase_ocsf_connection_info_protocol_name")?;
                        event.remove("ocsf.connection_info.protocol_name");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.connection_info.protocol_num").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.iana_number", v)?;
            }

            let _cond = { event.has_value("ocsf.connection_info.boundary") && event.get_str("ocsf.connection_info.boundary") == Some("Internal") };
            if _cond {
                event.append_unique("network.direction", json!("internal"))?;
            }

            let _cond = { event.has_value("ocsf.connection_info.boundary") && event.get_str("ocsf.connection_info.boundary") == Some("External") };
            if _cond {
                event.append_unique("network.direction", json!("external"))?;
            }

            let _cond = { event.has_value("ocsf.connection_info.direction") && event.get_str("ocsf.connection_info.direction") == Some("Inbound") };
            if _cond {
                event.append_unique("network.direction", json!("inbound"))?;
            }

            let _cond = { event.has_value("ocsf.connection_info.direction") && event.get_str("ocsf.connection_info.direction") == Some("Outbound") };
            if _cond {
                event.append_unique("network.direction", json!("outbound"))?;
            }

            let _cond = { (event.has_value("ocsf.connection_info.direction") && event.get_str("ocsf.connection_info.direction") == Some("Unknown")) || (event.has_value("ocsf.connection_info.boundary") && event.get_str("ocsf.connection_info.boundary") == Some("Unknown")) };
            if _cond {
                event.append_unique("network.direction", json!("unknown"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
