// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `system` pipeline.
pub struct System;

impl Transform for System {
    fn name(&self) -> &str {
        "system"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event.has_value("_temp_.unparsed.message")
                    && event.get_str("_temp_.unparsed.message") != Some("")
            };
            if _cond {
                if let Some(input) = event.get_string("_temp_.unparsed.message") {
                    // Grok pattern: ^(?:%{PROG:syslog_program}|-)?\\s(?:%{POSINT:syslog_pid}|-)?\\s(?:%{WORD:tag}|-)?\\s([-]+\\s)?%{GREEDYDATA:_temp_.unparsed.system_structured_brief}\\s?$
                    // Grok pattern: ^%{GREEDYDATA:message}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^(?:%{PROG:syslog_program}|-)?\\s(?:%{POSINT:syslog_pid}|-)?\\s(?:%{WORD:tag}|-)?\\s([-]+\\s)?%{GREEDYDATA:_temp_.unparsed.system_structured_brief}\\s?$"
                            ),
                            cached_grok!("^%{GREEDYDATA:message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.unparsed.system_structured_brief")
                    && event.get_str("_temp_.unparsed.system_structured_brief") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.unparsed.system_structured_brief")
                    {
                        // Grok pattern: ^%{WORD:_temp_.negotiation.type} negotiation %{GREEDYDATA:_temp_.negotiation.message}$
                        // Grok pattern: ^(%{SYSLOGHOST:syslog_hostname}\\s)?((?P<_temp__tag_brief>(?:(?!FW)[A-Za-z_]+))(\\s\\(pid=%{DATA:syslog_pid}\\))?(:\\s))?%{GREEDYDATA:_temp_.message_brief}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:_temp_.negotiation.type} negotiation %{GREEDYDATA:_temp_.negotiation.message}$"
                                ),
                                cached_grok_mapped!(
                                    "^(%{SYSLOGHOST:syslog_hostname}\\s)?((?P<_temp__tag_brief>(?:(?!FW)[A-Za-z_]+))(\\s\\(pid=%{DATA:syslog_pid}\\))?(:\\s))?%{GREEDYDATA:_temp_.message_brief}$",
                                    [("_temp__tag_brief", "_temp_.tag_brief")]
                                ),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_system_structured_brief",
                    )?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("syslog_program")
                    && (!event.has_value("juniper.srx.process")
                        || event.get_str("juniper.srx.process") == Some("-"))
            };
            if _cond {
                event.set(
                    "juniper.srx.process",
                    json!(
                        event
                            .get("syslog_program")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.tag_brief")
                    && (!event.has_value("juniper.srx.tag")
                        || event.get_str("juniper.srx.tag") == Some("-"))
            };
            if _cond {
                event.set(
                    "juniper.srx.tag",
                    json!(
                        event
                            .get("_temp_.tag_brief")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.negotiation.message")
                    && event
                        .get_str("_temp_.negotiation.message")
                        .is_some_and(|s| s.starts_with("failed"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.negotiation.message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("failed with error: ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(". ") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.negotiation.err_msg", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(". ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("message", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.negotiation.message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "dissect_neg_failed")?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.negotiation.message")
                    && event
                        .get_str("_temp_.negotiation.message")
                        .is_some_and(|s| s.starts_with("success"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.negotiation.message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("successfully completed. ")
                            else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("message", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.negotiation.message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "dissect_neg_success")?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.negotiation") };
            if _cond {
                event.rename("_temp_.negotiation", "juniper.srx.negotiation")?;
            }

            let _cond = {
                event.has_value("_temp_.message_brief")
                    && event
                        .get_str("_temp_.message_brief")
                        .is_some_and(|s| s.starts_with("FW:"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.message_brief") {
                        // Grok pattern: ^FW:\\s%{NOTSPACE:_temp_.fw.interface_name}\\s%{NOTSPACE:_temp_.fw.filter_action}\\s%{NOTSPACE:_temp_.fw.packet_protocol}\\s%{NOTSPACE:_temp_.fw.src_addr}\\s%{NOTSPACE:_temp_.fw.dst_addr}\\s%{NOTSPACE:_temp_.fw.src_port}\\s%{NOTSPACE:_temp_.fw.dst_port}\\s(\\(%{NOTSPACE:_temp_.fw.packets_num} packets\\))?\\s?$
                        if !cached_grok!("^FW:\\s%{NOTSPACE:_temp_.fw.interface_name}\\s%{NOTSPACE:_temp_.fw.filter_action}\\s%{NOTSPACE:_temp_.fw.packet_protocol}\\s%{NOTSPACE:_temp_.fw.src_addr}\\s%{NOTSPACE:_temp_.fw.dst_addr}\\s%{NOTSPACE:_temp_.fw.src_port}\\s%{NOTSPACE:_temp_.fw.dst_port}\\s(\\(%{NOTSPACE:_temp_.fw.packets_num} packets\\))?\\s?$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_message_brief")?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.fw") };
            if _cond {
                event.rename("_temp_.fw", "juniper.srx.firewall")?;
            }

            let _cond = { event.has_value("juniper.srx.firewall.interface_name") };
            if _cond {
                event.rename(
                    "juniper.srx.firewall.interface_name",
                    "juniper.srx.interface_name",
                )?;
            }

            let _cond = {
                event.has_value("_temp_.message_brief")
                    && event
                        .get_str("_temp_.message_brief")
                        .is_some_and(|s| s.starts_with("rtslib_dfwsm_get_async_cb:"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.message_brief") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) =
                                remaining.strip_prefix("rtslib_dfwsm_get_async_cb:u_data:")
                            else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" k_usr_d:") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.rtslib_dfwsm.u_data", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" k_usr_d:") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp_.rtslib_dfwsm.k_usr_d", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.message_brief".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "dissect_rtslib_dfwsmr")?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.rtslib_dfwsm") };
            if _cond {
                event.rename("_temp_.rtslib_dfwsm", "juniper.srx.rtslib_dfwsm")?;
            }

            let _cond = {
                event.has_value("_temp_.tag_brief")
                    && event.get_str("_temp_.tag_brief") == Some("ip_mon_reth_scan")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.message_brief") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("interface ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" trigger ") else {
                                break 'dissect false;
                            };
                            captured.push((
                                "_temp_.ip_mon_reth_scan.interface_name",
                                &remaining[..pos],
                            ));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" trigger ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp_.ip_mon_reth_scan.trigger", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.message_brief".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_ip_mon_reth_scan",
                    )?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.ip_mon_reth_scan") };
            if _cond {
                event.rename("_temp_.ip_mon_reth_scan", "juniper.srx.ip_mon_reth_scan")?;
            }

            let _cond = { event.has_value("juniper.srx.ip_mon_reth_scan.interface_name") };
            if _cond {
                event.rename(
                    "juniper.srx.ip_mon_reth_scan.interface_name",
                    "juniper.srx.interface_name",
                )?;
            }

            let _cond = {
                event.has_value("_temp_.tag_brief")
                    && event.get_str("_temp_.tag_brief") == Some("dpdk_eth_devstart")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.message_brief") {
                        // Grok pattern: ^port %{POSINT:_temp_.dpdk.port_number} (has already been started|ifd %{DATA:_temp_.dpdk.interface_name}), (new\\s)?dpdk_port_state=%{POSINT:_temp_.dpdk.port_state} dpdk_swt_port_state %{POSINT:_temp_.dpdk.swt_port_state}$
                        if !cached_grok!("^port %{POSINT:_temp_.dpdk.port_number} (has already been started|ifd %{DATA:_temp_.dpdk.interface_name}), (new\\s)?dpdk_port_state=%{POSINT:_temp_.dpdk.port_state} dpdk_swt_port_state %{POSINT:_temp_.dpdk.swt_port_state}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_dpdk_eth_devstart")?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.dpdk.port_number") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_temp_.dpdk.port_number") {
                        if let Some(val) = event.get("_temp_.dpdk.port_number") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_temp_.dpdk.port_number".into(),
                                    message,
                                }
                            })?;
                            event.set("_temp_.dpdk.port_number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_dpdk_port_number_to_int",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.dpdk.port_state") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_temp_.dpdk.port_state") {
                        if let Some(val) = event.get("_temp_.dpdk.port_state") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_temp_.dpdk.port_state".into(),
                                    message,
                                }
                            })?;
                            event.set("_temp_.dpdk.port_state", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_dpdk_port_state_to_int",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.dpdk.swt_port_state") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_temp_.dpdk.swt_port_state") {
                        if let Some(val) = event.get("_temp_.dpdk.swt_port_state") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_temp_.dpdk.swt_port_state".into(),
                                    message,
                                }
                            })?;
                            event.set("_temp_.dpdk.swt_port_state", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_swt_port_state_to_int",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_temp_.dpdk") };
            if _cond {
                event.rename("_temp_.dpdk", "juniper.srx.dpdk")?;
            }

            let _cond = { event.has_value("juniper.srx.dpdk.interface_name") };
            if _cond {
                event.rename(
                    "juniper.srx.dpdk.interface_name",
                    "juniper.srx.interface_name",
                )?;
            }

            let _cond = {
                event.has_value("_temp_.unparsed.system_structured_brief")
                    && event.get_str("juniper.srx.tag") == Some("RTLOG_CONN_ERROR")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.unparsed.system_structured_brief")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix(" Connection error ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured
                                .push(("_temp_.rtlog_conn_error.stream_name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp_.rtlog_conn_error.err_msg", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.unparsed.system_structured_brief".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_tag_rtlog_conn_err",
                    )?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.rtlog_conn_error")
                    && event.get_str("juniper.srx.tag") == Some("RTLOG_CONN_ERROR")
            };
            if _cond {
                event.rename("_temp_.rtlog_conn_error", "juniper.srx.rtlog_conn_error")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("juniper.srx.rtlog_conn_error.err_msg") {
                    if let Some(input) = event.get_string("juniper.srx.rtlog_conn_error.err_msg") {
                        // Grok pattern: ^(status: %{DATA:juniper.srx.rtlog_conn_error.status}, )?Error code: major %{NUMBER:juniper.srx.rtlog_conn_error.major} minor %{NUMBER:juniper.srx.rtlog_conn_error.minor} code %{NUMBER:juniper.srx.rtlog_conn_error.code}, description:%{DATA:juniper.srx.rtlog_conn_error.description}$
                        if !cached_grok!("^(status: %{DATA:juniper.srx.rtlog_conn_error.status}, )?Error code: major %{NUMBER:juniper.srx.rtlog_conn_error.major} minor %{NUMBER:juniper.srx.rtlog_conn_error.minor} code %{NUMBER:juniper.srx.rtlog_conn_error.code}, description:%{DATA:juniper.srx.rtlog_conn_error.description}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("juniper.srx.rtlog_conn_error.status") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.rtlog_conn_error.status") {
                        if let Some(val) = event.get("juniper.srx.rtlog_conn_error.status") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.rtlog_conn_error.status".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.rtlog_conn_error.status", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_rtlog_conn_error_status_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.rtlog_conn_error.major") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.rtlog_conn_error.major") {
                        if let Some(val) = event.get("juniper.srx.rtlog_conn_error.major") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.rtlog_conn_error.major".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.rtlog_conn_error.major", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_rtlog_conn_error_major_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.rtlog_conn_error.minor") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.rtlog_conn_error.minor") {
                        if let Some(val) = event.get("juniper.srx.rtlog_conn_error.minor") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.rtlog_conn_error.minor".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.rtlog_conn_error.minor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_rtlog_conn_error_minor_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.rtlog_conn_error.code") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.rtlog_conn_error.code") {
                        if let Some(val) = event.get("juniper.srx.rtlog_conn_error.code") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.rtlog_conn_error.code".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.rtlog_conn_error.code", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_rtlog_conn_error_code_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.unparsed.system_structured_brief")
                    && event.get_str("juniper.srx.tag") == Some("PING_TEST_COMPLETED")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.unparsed.system_structured_brief")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix(" pingCtlOwnerIndex = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(", pingCtlTestName = ") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.ping_test.owner", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(", pingCtlTestName = ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp_.ping_test.name", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.unparsed.system_structured_brief".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "dissect_tag_ping_test")?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.ping_test")
                    && event.get_str("juniper.srx.tag") == Some("PING_TEST_COMPLETED")
            };
            if _cond {
                event.rename("_temp_.ping_test", "juniper.srx.ping_test")?;
            }

            let _cond = {
                event.has_value("_temp_.unparsed.system_structured_brief")
                    && event.get_str("juniper.srx.tag") == Some("KERN_ARP_ADDR_CHANGE")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp_.unparsed.system_structured_brief")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix(" arp info overwritten for ")
                            else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" from ") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.kern_arp_addr_change.ip", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" from ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" to ") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.kern_arp_addr_change.mac1", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" to ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp_.kern_arp_addr_change.mac2", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.unparsed.system_structured_brief".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_tag_kern_arp_addr",
                    )?;
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
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.kern_arp_addr_change")
                    && event.get_str("juniper.srx.tag") == Some("KERN_ARP_ADDR_CHANGE")
            };
            if _cond {
                event.rename(
                    "_temp_.kern_arp_addr_change",
                    "juniper.srx.kern_arp_addr_change",
                )?;
            }

            let _cond = { event.has_value("juniper.srx.kern_arp_addr_change.ip") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.kern_arp_addr_change.ip") {
                        if let Some(val) = event.get("juniper.srx.kern_arp_addr_change.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.kern_arp_addr_change.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.kern_arp_addr_change.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_kern_arp_ip_to_ip",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("message") && event.get_str("message") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        if let Some(kv_str) = event.get_string("message") {
                            for pair in cached_regex!(",\\s(?=[a-zA-Z0-9\\_\\-\\s]+:)")
                                .split(&kv_str)
                                .into_iter()
                            {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once(":") else {
                                    return Err(TransformError::ParseError {
                                        path: "message".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let value = value.trim_matches(|c| "\"".contains(c));
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("juniper.srx.system.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.system") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.juniper.srx.system = ctx.juniper.srx.system.entrySet().stream().collect(Collectors.toMap(e -> e.getKey().replace(' ', '_').replace('-', '_').toLowerCase(), e -> e.getValue().trim()));
                guarded_replace(
                    event,
                    &GuardedReplace::new(
                        "juniper.srx.system.entrySet().stream().collect(Collectors.toMap(e -> e.getKey()",
                        "juniper.srx.system",
                        " ",
                        "_",
                    ),
                );
            }

            let _cond = { event.has_value("juniper.srx.system.aux_spi") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.system.aux_spi") {
                        if let Some(val) = event.get("juniper.srx.system.aux_spi") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.system.aux_spi".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.system.aux_spi", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_aux_spi_to_int")?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.system.ike_version") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.system.ike_version") {
                        if let Some(val) = event.get("juniper.srx.system.ike_version") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.system.ike_version".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.system.ike_version", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ike_version_to_int",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.system.local_gateway") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.system.local_gateway") {
                        if let Some(val) = event.get("juniper.srx.system.local_gateway") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.system.local_gateway".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.system.local_gateway", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_local_gateway_to_ip",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.system.remote_gateway") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.system.remote_gateway") {
                        if let Some(val) = event.get("juniper.srx.system.remote_gateway") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.system.remote_gateway".into(),
                                    message,
                                }
                            })?;
                            event.set("juniper.srx.system.remote_gateway", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_remote_gateway_to_ip",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.system") };
            if _cond {
                // Painless script
                // Source: ctx?.juniper?.srx?.system.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx?.juniper?.srx?.system.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));"#
                    ),
                    cached_params!(
                        "{\"values\":[\"None\",\"UNKNOWN\",\"N/A\",\"-\",\"Not-Available\"]}"
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.message_brief") && !event.has_value("message") };
            if _cond {
                if let Some(v) = event.get("_temp_.message_brief").cloned() {
                    event.set("message", v)?;
                }
            }

            event.set("juniper.srx.log_type", json!("system"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            let _cond = { event.get_str("juniper.srx.tag") == Some("SSHD_LOGIN_FAILED") };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("juniper.srx.firewall.filter_action") == Some("A") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("juniper.srx.firewall.filter_action") == Some("D") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_str("juniper.srx.firewall.filter_action") == Some("R") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.has_value("juniper.srx.ike_negotiation") };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            let _cond = { event.has_value("juniper.srx.rtslib_dfwsm") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("juniper.srx.rtlog_conn_error") };
            if _cond {
                event.append("event.type", json!("error"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.has_value("juniper.srx.ping_test") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("juniper.srx.ping_test") };
            if _cond {
                event.append("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("juniper.srx.tag") == Some("SSHD_LOGIN_FAILED") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.has_value("juniper.srx.remote_address") };
            if _cond {
                if event.has_value("juniper.srx.remote_address") {
                    event.rename("juniper.srx.remote_address", "destination.ip")?;
                }
            }

            let _cond = {
                !event.has_value("destination.ip")
                    && event.has_value("juniper.srx.destination_address")
            };
            if _cond {
                if event.has_value("juniper.srx.destination_address") {
                    event.rename("juniper.srx.destination_address", "destination.ip")?;
                }
            }

            let _cond = {
                !event.has_value("destination.ip")
                    && event.has_value("juniper.srx.firewall.dst_addr")
            };
            if _cond {
                if event.has_value("juniper.srx.firewall.dst_addr") {
                    event.rename("juniper.srx.firewall.dst_addr", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.set(
                    "server.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("juniper.srx.nat_remote_address") };
            if _cond {
                if event.has_value("juniper.srx.nat_remote_address") {
                    event.rename("juniper.srx.nat_remote_address", "destination.nat.ip")?;
                }
            }

            let _cond = {
                !event.has_value("destination.nat.ip")
                    && event.has_value("juniper.srx.nat_destination_address")
            };
            if _cond {
                if event.has_value("juniper.srx.nat_destination_address") {
                    event.rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.destination_port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.destination_port") {
                        if let Some(val) = event.get("juniper.srx.destination_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.destination_port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destination_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                !event.has_value("destination.port")
                    && event.has_value("juniper.srx.firewall.dst_port")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.firewall.dst_port") {
                        if let Some(val) = event.get("juniper.srx.firewall.dst_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.firewall.dst_port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_firewall_destination_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.port") };
            if _cond {
                event.set(
                    "server.port",
                    json!(
                        event
                            .get("destination.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("server.port") {
                        if let Some(val) = event.get("server.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.port".into(),
                                    message,
                                }
                            })?;
                            event.set("server.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_server_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.nat_destination_port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.nat_destination_port") {
                        if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.nat_destination_port".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_nat_destination_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.nat.port") };
            if _cond {
                event.set(
                    "server.nat.port",
                    json!(
                        event
                            .get("destination.nat.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.nat.port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("server.nat.port") {
                        if let Some(val) = event.get("server.nat.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.nat.port".into(),
                                    message,
                                }
                            })?;
                            event.set("server.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_server_nat_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.inbound_bytes") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.inbound_bytes") {
                        if let Some(val) = event.get("juniper.srx.inbound_bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.inbound_bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_inbound_bytes_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.bytes") };
            if _cond {
                event.set(
                    "server.bytes",
                    json!(
                        event
                            .get("destination.bytes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.bytes") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("server.bytes") {
                        if let Some(val) = event.get("server.bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("server.bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_server_bytes_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.inbound_packets") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.inbound_packets") {
                        if let Some(val) = event.get("juniper.srx.inbound_packets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.inbound_packets".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_inbound_packets_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.packets") };
            if _cond {
                event.set(
                    "server.packets",
                    json!(
                        event
                            .get("destination.packets")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.packets") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("server.packets") {
                        if let Some(val) = event.get("server.packets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.packets".into(),
                                    message,
                                }
                            })?;
                            event.set("server.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_server_packets_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.local_address") };
            if _cond {
                if event.has_value("juniper.srx.local_address") {
                    event.rename("juniper.srx.local_address", "source.ip")?;
                }
            }

            let _cond =
                { !event.has_value("source.ip") && event.has_value("juniper.srx.source_address") };
            if _cond {
                if event.has_value("juniper.srx.source_address") {
                    event.rename("juniper.srx.source_address", "source.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.firewall.src_addr") };
            if _cond {
                if event.has_value("juniper.srx.firewall.src_addr") {
                    event.rename("juniper.srx.firewall.src_addr", "source.ip")?;
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.set(
                    "client.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("juniper.srx.nat_local_address") };
            if _cond {
                if event.has_value("juniper.srx.nat_local_address") {
                    event.rename("juniper.srx.nat_local_address", "source.nat.ip")?;
                }
            }

            let _cond = {
                !event.has_value("source.nat.ip")
                    && event.has_value("juniper.srx.nat_source_address")
            };
            if _cond {
                if event.has_value("juniper.srx.nat_source_address") {
                    event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.sourceip") };
            if _cond {
                if event.has_value("juniper.srx.sourceip") {
                    event.rename("juniper.srx.sourceip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.source_port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.source_port") {
                        if let Some(val) = event.get("juniper.srx.source_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.source_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.firewall.src_port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.firewall.src_port") {
                        if let Some(val) = event.get("juniper.srx.firewall.src_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.firewall.src_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_firewall_src_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("source.port") };
            if _cond {
                event.set(
                    "client.port",
                    json!(
                        event
                            .get("source.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("client.port") {
                        if let Some(val) = event.get("client.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.port".into(),
                                    message,
                                }
                            })?;
                            event.set("client.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.nat_source_port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.nat_source_port") {
                        if let Some(val) = event.get("juniper.srx.nat_source_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.nat_source_port".into(),
                                    message,
                                }
                            })?;
                            event.set("source.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_nat_source_port_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("source.nat.port") };
            if _cond {
                event.set(
                    "client.nat.port",
                    json!(
                        event
                            .get("source.nat.port")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.nat.port") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("client.nat.port") {
                        if let Some(val) = event.get("client.nat.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.nat.port".into(),
                                    message,
                                }
                            })?;
                            event.set("client.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_nat_port",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.outbound_bytes") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.outbound_bytes") {
                        if let Some(val) = event.get("juniper.srx.outbound_bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.outbound_bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("source.bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_outbounds_bytes_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("source.bytes") };
            if _cond {
                event.set(
                    "client.bytes",
                    json!(
                        event
                            .get("source.bytes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.bytes") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("client.bytes") {
                        if let Some(val) = event.get("client.bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("client.bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_bytes_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.outbound_packets") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.outbound_packets") {
                        if let Some(val) = event.get("juniper.srx.outbound_packets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.outbound_packets".into(),
                                    message,
                                }
                            })?;
                            event.set("source.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_outbound_packets_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.firewall.packets_num") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("juniper.srx.firewall.packets_num") {
                        if let Some(val) = event.get("juniper.srx.firewall.packets_num") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.firewall.packets_num".into(),
                                    message,
                                }
                            })?;
                            event.set("source.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_firewall_packets_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("source.packets") };
            if _cond {
                event.set(
                    "client.packets",
                    json!(
                        event
                            .get("source.packets")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.packets") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("client.packets") {
                        if let Some(val) = event.get("client.packets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.packets".into(),
                                    message,
                                }
                            })?;
                            event.set("client.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_packets_to_long",
                    )?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.username") };
            if _cond {
                if event.has_value("juniper.srx.username") {
                    event.rename("juniper.srx.username", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.system.local_gateway") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("juniper.srx.system.local_gateway")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("juniper.srx.system.remote_gateway") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("juniper.srx.system.remote_gateway")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("juniper.srx.interface_name") {
                event.rename(
                    "juniper.srx.interface_name",
                    "observer.ingress.interface.name",
                )?;
            }

            let _cond = { event.has_value("syslog_hostname") && !event.has_value("observer.name") };
            if _cond {
                if event.has_value("syslog_hostname") {
                    event.rename("syslog_hostname", "observer.name")?;
                }
            }

            if let Some(v) = event
                .get("observer.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("juniper.srx.rulebase_name") };
            if _cond {
                if event.has_value("juniper.srx.rulebase_name") {
                    event.rename("juniper.srx.rulebase_name", "rule.name")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.rule_name") };
            if _cond {
                if event.has_value("juniper.srx.rule_name") {
                    event.rename("juniper.srx.rule_name", "rule.id")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.protocol_name") };
            if _cond {
                if event.has_value("juniper.srx.protocol_name") {
                    event.rename("juniper.srx.protocol_name", "network.protocol")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.firewall.packet_protocol") };
            if _cond {
                if event.has_value("juniper.srx.firewall.packet_protocol") {
                    event.rename("juniper.srx.firewall.packet_protocol", "network.transport")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.message") };
            if _cond {
                if event.has_value("juniper.srx.message") {
                    event.rename("juniper.srx.message", "message")?;
                }
            }

            let _cond = {
                event.has_value("juniper.srx.process")
                    && ["-", "N/A", "UNKNOWN", "None"]
                        .contains(&event.get_str("juniper.srx.process").unwrap_or(""))
            };
            if _cond {
                if event.remove("juniper.srx.process").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "juniper.srx.process".into(),
                    });
                }
            }

            event.remove("syslog_program");
            event.remove("syslog_hostname");
            event.remove("tag");
            event.remove("juniper.srx.destination_port");
            event.remove("juniper.srx.nat_destination_port");
            event.remove("juniper.srx.outbound_bytes");
            event.remove("juniper.srx.outbound_packets");
            event.remove("juniper.srx.source_port");
            event.remove("juniper.srx.nat_source_port");
            event.remove("juniper.srx.inbound_bytes");
            event.remove("juniper.srx.inbound_packets");
            event.remove("juniper.srx.firewall");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
