// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `tcp_and_acl_feature` pipeline.
pub struct TcpAndAclFeature;

impl Transform for TcpAndAclFeature {
    fn name(&self) -> &str {
        "tcp_and_acl_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - End Time %{DATA:_tmp.end_time}(?: %{DATA:citrix_adc.log.end_time_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.original_destination.ip}:%{INT:citrix_adc.log.original_destination.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.translated_destination.ip}:%{INT:citrix_adc.log.translated_destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long} - Closure%{SPACE}Reason %{GREEDYDATA:citrix_adc.log.closure_reason}$
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip} --> Destination %{IP:citrix_adc.log.destination.ip} - Protocol %{WORD:citrix_adc.log.protocol} - Type %{INT:citrix_adc.log.type} - Code %{INT:citrix_adc.log.code} - Time%{SPACE}Stamp %{DATA:citrix_adc.log.timestamp}%{SPACE}\\(ms\\) - Hitcount %{INT:citrix_adc.log.hit.count:int} - Hit%{SPACE}Rule %{GREEDYDATA:citrix_adc.log.hit.rule} - Action %{WORD:citrix_adc.log.action} - Data%{SPACE}$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    if !extract_first_match(
                        &[
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$"),
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$"),
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - End Time %{DATA:_tmp.end_time}(?: %{DATA:citrix_adc.log.end_time_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$"),
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.original_destination.ip}:%{INT:citrix_adc.log.original_destination.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.translated_destination.ip}:%{INT:citrix_adc.log.translated_destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long} - Closure%{SPACE}Reason %{GREEDYDATA:citrix_adc.log.closure_reason}$"),
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip} --> Destination %{IP:citrix_adc.log.destination.ip} - Protocol %{WORD:citrix_adc.log.protocol} - Type %{INT:citrix_adc.log.type} - Code %{INT:citrix_adc.log.code} - Time%{SPACE}Stamp %{DATA:citrix_adc.log.timestamp}%{SPACE}\\(ms\\) - Hitcount %{INT:citrix_adc.log.hit.count:int} - Hit%{SPACE}Rule %{GREEDYDATA:citrix_adc.log.hit.rule} - Action %{WORD:citrix_adc.log.action} - Data%{SPACE}$"),
                            cached_grok!("%{GREEDYDATA:citrix_adc.log.message}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("citrix_adc.log.timestamp") && event.get_str("citrix_adc.log.timestamp") != Some("") && !event.has_value("_conf.custom_date_format") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("citrix_adc.log.timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS", "ISO8601", "MM/dd/yyyy:HH:mm:ss", "MMM d HH:mm:ss", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMMM d HH:mm:ss", "MMMM  d HH:mm:ss", "MMMM dd HH:mm:ss", "yyyy MMM d HH:mm:ss", "yyyy MMM  d HH:mm:ss", "yyyy MMM dd HH:mm:ss", "yyyy MMMM d HH:mm:ss", "yyyy MMMM  d HH:mm:ss", "yyyy MMMM dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("citrix_adc.log.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "citrix_adc.log.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_bytes_received") {
                if let Some(val) = event.get("citrix_adc.log.total_bytes_received") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_bytes_received".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_bytes_received", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_received_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.total_bytes_received").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            let _cond = { event.has_value("citrix_adc.log.original_destination.ip") && event.get_str("citrix_adc.log.original_destination.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.original_destination.ip") {
                if let Some(val) = event.get("citrix_adc.log.original_destination.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.original_destination.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.original_destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_original_destination_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("citrix_adc.log.translated_destination.ip") && event.get_str("citrix_adc.log.translated_destination.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.translated_destination.ip") {
                if let Some(val) = event.get("citrix_adc.log.translated_destination.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.translated_destination.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.translated_destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_translated_destination_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.translated_destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.original_destination.port") {
                if let Some(val) = event.get("citrix_adc.log.original_destination.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.original_destination.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.original_destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_original_destination_port_to_long")?;
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
            if event.has_value("citrix_adc.log.translated_destination.port") {
                if let Some(val) = event.get("citrix_adc.log.translated_destination.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.translated_destination.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.translated_destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_translated_destination_port_to_long")?;
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

            if let Some(v) = event.get("citrix_adc.log.translated_destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.closure_reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.vserver.ip") && event.get_str("citrix_adc.log.vserver.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.vserver.ip") {
                if let Some(val) = event.get("citrix_adc.log.vserver.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.vserver.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.vserver.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vserver_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.vserver.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.vserver.port") {
                if let Some(val) = event.get("citrix_adc.log.vserver.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.vserver.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.vserver.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vserver_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.vserver.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_bytes_send") {
                if let Some(val) = event.get("citrix_adc.log.total_bytes_send") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_bytes_send".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_bytes_send", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_send_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.total_bytes_send").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            let _cond = { event.has_value("citrix_adc.log.nat.ip") && event.get_str("citrix_adc.log.nat.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.nat.ip") {
                if let Some(val) = event.get("citrix_adc.log.nat.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.nat.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.nat.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_nat_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.nat.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.nat.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.nat.port") {
                if let Some(val) = event.get("citrix_adc.log.nat.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.nat.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.nat.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_nat_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.nat.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.nat.port", v)?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.type") {
                if let Some(val) = event.get("citrix_adc.log.type") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.type".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.type", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_type_to_string")?;
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
            if event.has_value("citrix_adc.log.code") {
                if let Some(val) = event.get("citrix_adc.log.code") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.code".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.code", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_code_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.code", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.hit.count") {
                if let Some(val) = event.get("citrix_adc.log.hit.count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.hit.count".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.hit.count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_hit_count_to_long")?;
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
