// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ci_feature` pipeline.
pub struct CiFeature;

impl Transform for CiFeature {
    fn name(&self) -> &str {
        "ci_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Domain %{DATA:citrix_adc.log.domain} - Content-Type %{DATA:citrix_adc.log.content_type} - ICAP%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port} - Mode %{WORD:citrix_adc.log.mode} - Service %{WORD:citrix_adc.log.service} - Response %{INT:citrix_adc.log.response.code} - Action %{WORD:citrix_adc.log.action}$
                    // Grok pattern: ^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.bytes.sent} - Bytes%{SPACE}Received %{NUMBER:citrix_adc.log.bytes.received} - Origin%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port}$
                    // Grok pattern: ^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Request%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.request.bytes_sent} - Response%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.response.bytes_sent} - Origin%{SPACE}Server %{IP:citrix_adc.log.origin_server.ip}:%{INT:citrix_adc.log.origin_server.port}$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Domain %{DATA:citrix_adc.log.domain} - Content-Type %{DATA:citrix_adc.log.content_type} - ICAP%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port} - Mode %{WORD:citrix_adc.log.mode} - Service %{WORD:citrix_adc.log.service} - Response %{INT:citrix_adc.log.response.code} - Action %{WORD:citrix_adc.log.action}$"),
                            cached_grok!("^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.bytes.sent} - Bytes%{SPACE}Received %{NUMBER:citrix_adc.log.bytes.received} - Origin%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port}$"),
                            cached_grok!("^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Request%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.request.bytes_sent} - Response%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.response.bytes_sent} - Origin%{SPACE}Server %{IP:citrix_adc.log.origin_server.ip}:%{INT:citrix_adc.log.origin_server.port}$"),
                            cached_grok!("%{GREEDYDATA:citrix_adc.log.message}"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.bytes.received") {
                if let Some(val) = event.get("citrix_adc.log.bytes.received") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.bytes.received".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.bytes.received", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bytes_received_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.bytes.received").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.bytes", v)?;
            }

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.response.code") {
                if let Some(val) = event.get("citrix_adc.log.response.code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.response.code".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.response.code", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_response_code_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.response.code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.status_code", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.icap_server.ip") && event.get_str("citrix_adc.log.icap_server.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.icap_server.ip") {
                if let Some(val) = event.get("citrix_adc.log.icap_server.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.icap_server.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.icap_server.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_icap_server_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.icap_server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.ip", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.origin_server.ip") && event.get_str("citrix_adc.log.origin_server.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.origin_server.ip") {
                if let Some(val) = event.get("citrix_adc.log.origin_server.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.origin_server.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.origin_server.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_origin_server_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.origin_server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.icap_server.port") {
                if let Some(val) = event.get("citrix_adc.log.icap_server.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.icap_server.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.icap_server.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_icap_server_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.icap_server.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.origin_server.port") {
                if let Some(val) = event.get("citrix_adc.log.origin_server.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.origin_server.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.origin_server.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_origin_server_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.origin_server.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.bytes.sent") {
                if let Some(val) = event.get("citrix_adc.log.bytes.sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.bytes.sent".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.bytes.sent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bytes_sent_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.bytes.sent").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.bytes", v)?;
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

            if let Some(v) = event.get("citrix_adc.log.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.request.bytes_sent") {
                if let Some(val) = event.get("citrix_adc.log.request.bytes_sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.request.bytes_sent".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.request.bytes_sent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_request_bytes_sent_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.response.bytes_sent") {
                if let Some(val) = event.get("citrix_adc.log.response.bytes_sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.response.bytes_sent".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.response.bytes_sent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_response_bytes_sent_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.id") {
                if let Some(val) = event.get("citrix_adc.log.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.id".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_id_to_string")?;
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
