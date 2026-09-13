// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_alarm` pipeline.
pub struct PipelineAlarm;

impl Transform for PipelineAlarm {
    fn name(&self) -> &str {
        "pipeline_alarm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body} %{SPACE}\\(%{DATA:watchguard_firebox.log.policy_name}\\)$
                    if !cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body} %{SPACE}\\(%{DATA:watchguard_firebox.log.policy_name}\\)$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0168", "3000-0169"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{DATA} Traffic detected from %{IP:watchguard_firebox.log.source_ip} to %{IP:watchguard_firebox.log.destination_ip}.$
                    if !cached_grok!("^%{DATA} Traffic detected from %{IP:watchguard_firebox.log.source_ip} to %{IP:watchguard_firebox.log.destination_ip}.$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0152"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IPv4 source route attack from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" detected.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" detected.") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0153", "3000-0154", "3000-0155", "3000-0156", "3000-0157", "3000-0162", "3000-0163", "3000-0164", "3000-0165", "3000-0166"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" against ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" against ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" detected. ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" detected. ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.packets_count", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0158", "3000-0159"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" against ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" against ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" detected.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" detected.") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0160"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("DDOS against server ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" detected.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" detected.") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0161"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("DDOS from client ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" detected.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" detected.") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0167"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Policy Name: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Source IP Address: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.pcy_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Source IP Address: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Source Port: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Source Port: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Destination IP Address: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Destination IP Address: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Destination Port: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Destination Port: ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.destination_port", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0170"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("The total number of current sessions (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") has reached the high water mark (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.current_session", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") has reached the high water mark (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(").") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.limit", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(").") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0171"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("The number of connections (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") has reached the configured limit (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.current_connection", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") has reached the configured limit (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(").") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.limit", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(").") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0172"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Traffic detected from ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Traffic detected from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on port ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on port ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(".") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["020B-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" tunnel '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" tunnel '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' local ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' local ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" remote ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" remote ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" under gateway '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.remote", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" under gateway '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' is ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.status", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(".") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.current_connection") {
                if let Some(val) = event.get("watchguard_firebox.log.current_connection") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.current_connection".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.current_connection", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_current_connection_to_long")?;
                        if event.remove("watchguard_firebox.log.current_connection").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.current_connection".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.limit") {
                if let Some(val) = event.get("watchguard_firebox.log.limit") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.limit".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.limit", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_value2_to_long")?;
                        if event.remove("watchguard_firebox.log.limit").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.limit".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("watchguard_firebox.log.source_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.source_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.source_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.source_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.source_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_to_ip")?;
                        if event.remove("watchguard_firebox.log.source_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.source_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.packets_count") {
                if let Some(val) = event.get("watchguard_firebox.log.packets_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.packets_count".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.packets_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_gap_to_long")?;
                        if event.remove("watchguard_firebox.log.packets_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.packets_count".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("watchguard_firebox.log.destination_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.destination_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.destination_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.destination_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.destination_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_to_ip")?;
                        if event.remove("watchguard_firebox.log.destination_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.destination_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.source_port") {
                if let Some(val) = event.get("watchguard_firebox.log.source_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.source_port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.source_port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                        if event.remove("watchguard_firebox.log.source_port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.source_port".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.destination_port") {
                if let Some(val) = event.get("watchguard_firebox.log.destination_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.destination_port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.destination_port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                        if event.remove("watchguard_firebox.log.destination_port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.destination_port".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.current_session") {
                if let Some(val) = event.get("watchguard_firebox.log.current_session") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.current_session".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.current_session", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_current_session_to_long")?;
                        if event.remove("watchguard_firebox.log.current_session").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.current_session".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.port") {
                if let Some(val) = event.get("watchguard_firebox.log.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_port_to_long")?;
                        if event.remove("watchguard_firebox.log.port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.port".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.body").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.pcy_name") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("watchguard_firebox.log.pcy_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.policy_name") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("watchguard_firebox.log.policy_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.destination_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.source_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("watchguard_firebox.log.destination_ip");
                event.remove("watchguard_firebox.log.destination_port");
                event.remove("watchguard_firebox.log.policy_name");
                event.remove("watchguard_firebox.log.pcy_name");
                event.remove("watchguard_firebox.log.source_ip");
                event.remove("watchguard_firebox.log.source_port");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
