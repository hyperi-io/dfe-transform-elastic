// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dns_and_ssli_feature` pipeline.
pub struct DnsAndSsliFeature;

impl Transform for DnsAndSsliFeature {
    fn name(&self) -> &str {
        "dns_and_ssli_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} User%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.user} - Domain%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.domain} - Category%{SPACE}:%{SPACE}%{INT:citrix_adc.log.category} Action%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.action} - Reason%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.reason}$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} User%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.user} - Domain%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.domain} - Category%{SPACE}:%{SPACE}%{INT:citrix_adc.log.category} Action%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.action} - Reason%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.reason}$"),
                            cached_grok!("%{GREEDYDATA:citrix_adc.log.message}"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("citrix_adc.log.destination.ip") && event.get_str("citrix_adc.log.destination.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.destination.ip") {
                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.destination.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.destination.port") {
                if let Some(val) = event.get("citrix_adc.log.destination.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.destination.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.source.ip") && event.get_str("citrix_adc.log.source.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.source.ip") {
                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.source.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.source.port") {
                if let Some(val) = event.get("citrix_adc.log.source.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.source.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.source.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.category") {
                if let Some(val) = event.get("citrix_adc.log.category") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.category".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.category", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_category_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
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
