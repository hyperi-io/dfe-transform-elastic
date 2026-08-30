// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event` pipeline.
pub struct PipelineEvent;

impl Transform for PipelineEvent {
    fn name(&self) -> &str {
        "pipeline_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-002F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^Feature key does not support the feature %{GREEDYDATA:watchguard_firebox.log.feature_name}.$
                    // Grok pattern: ^No valid %{DATA:watchguard_firebox.log.feature_name} feature$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Feature key does not support the feature %{GREEDYDATA:watchguard_firebox.log.feature_name}.$"),
                            cached_grok!("^No valid %{DATA:watchguard_firebox.log.feature_name} feature$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-00C9"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.probe_method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Load Balance Server ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Load Balance Server ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" port ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" port ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-00CB"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.probe_method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Load Balance Server ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Load Balance Server ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-012C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^ARP spoofing attack detected, ip=%{IP:watchguard_firebox.log.ip_address}, mac=%{MAC:watchguard_firebox.log.mac_address}, interface=%{NUMBER:watchguard_firebox.log.interface_id}$
                    let _ = cached_grok!("^ARP spoofing attack detected, ip=%{IP:watchguard_firebox.log.ip_address}, mac=%{MAC:watchguard_firebox.log.mac_address}, interface=%{NUMBER:watchguard_firebox.log.interface_id}$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0174"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("SD-WAN action ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.action_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from interface ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.update", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from interface ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.new_interface", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3001-1001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Temporarily blocking host ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (reason = ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (reason = ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3001-1002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("The Temporary Blocked Sites list is full (capacity=") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("). The oldest entry ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.blocked_site_limit", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("). The oldest entry ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" was removed.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" was removed.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0F01-0015"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("APT threat notified. Details='Policy Name: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Reason: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.policy_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Reason: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Task_UUID: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Task_UUID: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Source IP: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.task_uuid", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Source IP: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Source Port: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Source Port: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Destination IP: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Destination IP: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Destination Port: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Destination Port: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Proxy Type: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Proxy Type: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Proxy Host: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.proxy_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Proxy Host: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Path: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.proxy_host", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Path: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.path", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0F01-0016"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("APT safe result from file submission. Details='Policy Name: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Reason: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.policy_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Reason: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Message: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Message: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Task_UUID: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.message", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Task_UUID: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" MD5: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.task_uuid", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" MD5: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Source IP: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.md5", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Source IP: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Source Port: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Source Port: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Destination IP: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Destination IP: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Destination Port: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Destination Port: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Proxy Type: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Proxy Type: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Proxy Host: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.proxy_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Proxy Host: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Path: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.proxy_host", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Path: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.path", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1B04-00CE"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Ruleset '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' lookup failed") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ruleset_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' lookup failed") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1C02-00CD"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cannot get the rule from ruleset '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ruleset_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0074", "3100-0072", "0900-000A", "0203-0025", "3100-0071", "3100-0073", "0900-0008", "3100-0009", "3100-000A", "3100-000B", "3100-000D", "3100-0029", "3100-002B", "3100-002C", "3100-0046", "3100-0047", "3100-0069", "3100-006A", "3100-006C", "3100-006D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.dev_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")] ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")] ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("_tmp_msg", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0900-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Enforced PPPoE static IP address: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is replaced with ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.negotiation_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is replaced with ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.static_ip", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0071", "3100-0073"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(", from ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.new_ip", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0072", "3100-0074"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" IP address ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IP address ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" did not change during cluster failover") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" did not change during cluster failover") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0900-0009"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.physical_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")] PPPoE session[") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")] PPPoE session[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] is established, acquired IP address ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.session_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] is established, acquired IP address ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", peer ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", peer ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.peer_address", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0010"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Failed to add bridge ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" VLAN ID ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" VLAN ID ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.vlan_id", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0900-0009"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.physical_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")] PPPoE session[") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")] PPPoE session[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] is established, acquired IP address ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.session_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] is established, acquired IP address ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", peer ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", peer ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.peer_address", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["4900-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[Link Monitor] No response received on ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.target", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["6800-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" scan completed") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.scan_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" scan completed") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["6800-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" scan - ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.scan_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" scan - ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" started") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.scan_stage", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" started") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0039"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[Cluster] Management interface setting is changed: interface from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IPv4 address from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.new_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IPv4 address from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IPv4 mask from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.new_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IPv4 mask from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_mask", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", IPv6 CIDR from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.new_mask", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", IPv6 CIDR from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_ipv6", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.new_ipv6", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3A00-000E", "3A00-000F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Virtual Router with cluster ID ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.cluster_id", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-0278"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster disabled. Nonmaster member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" will be reset to factory-default settings.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" will be reset to factory-default settings.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-0279"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Non-master member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" will be reset to factory-default settings due to a critical cluster configuration change.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" will be reset to factory-default settings due to a critical cluster configuration change.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-0280"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" was unable to issue a device discovery message.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" was unable to issue a device discovery message.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-0282", "3900-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-025A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster enabled on member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-025B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster disabled on cluster master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-027A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Non-master cluster member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" was removed from cluster, and will be reset to factory-default settings.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" was removed from cluster, and will be reset to factory-default settings.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-027E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Failed to reset cluster member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to factory-default settings.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to factory-default settings.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" detected loss of heartbeat from member ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" detected loss of heartbeat from member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", cluster channel is up.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", cluster channel is up.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed over to member ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed over to member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", which has a greater Weighted Average Index.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", which has a greater Weighted Average Index.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0010"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" changed role to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" changed role to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.role", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0011"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Monitored interface ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" link is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" link is ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.link_state", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0012"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^Member %{DATA:watchguard_firebox.log.member_id} took over as master from member %{DATA:watchguard_firebox.log.member_id}.$
                    let _ = cached_grok!("^Member %{DATA:watchguard_firebox.log.member_id} took over as master from member %{DATA:watchguard_firebox.log.member_id}.$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0015"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" initiated failover by administrator request.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" initiated failover by administrator request.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0016"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cannot initiate failover from master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to member ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" due to higher Weighted Average Index on current master or backup master is unreachable.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" due to higher Weighted Average Index on current master or backup master is unreachable.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0019"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster failover due to interface ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" link ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" link ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" event.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.link_state", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" event.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-0058"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" changed role from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" changed role from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.role", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.updated_role", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-000C", "3900-000D", "3900-000E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Full state synchronization from master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to backup master ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to backup master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3900-000F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Master ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed-over to member ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.master_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed-over to member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" due to a link-down event on interface ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" due to a link-down event on interface ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0102-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" added feature key '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" added feature key '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.feature_key", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0102-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" removed feature key '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" removed feature key '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.feature_key", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0105-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Moved ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from position ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.policy_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from position ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.old_policy_position", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.new_policy_position", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Authentication of ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" user [") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" user [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] from ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" was accepted") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" was accepted") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Authentication of ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" user [") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" user [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] from ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" was rejected, ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" was rejected, ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reason", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0006"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is unlocked ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is unlocked ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.unlocked_by", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is locked out ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is locked out ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" after ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.lockout_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" after ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" login failures") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.failure_count", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" login failures") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Authentication of BOVPN TLS client [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.client_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" was rejected, ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" was rejected, ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reason", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-000C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Authentication error. ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.error", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_name", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-000D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Authentication of user [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] failed. Both primary and secondary servers are unavailable.") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] failed. Both primary and secondary servers are unavailable.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-000E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Authentication of firewall user [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] failed. RADIUS authentication method MSCHAP_V1 is not supported.") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] failed. RADIUS authentication method MSCHAP_V1 is not supported.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0010"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Firebox connected to the SSO agent at ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" successfully.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" successfully.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["1100-0012"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Firebox failed to connect to the SSO agent at ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(". Reason: ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(". Reason: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3D04-0001", "3D04-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Log Server at ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Log Server at ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.ip_address", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3E00-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{DATA:watchguard_firebox.log.user_type} %{WORD:watchguard_firebox.log.user_name}(?:@%{DATA:watchguard_firebox.log.authentication_server})? from %{IP:watchguard_firebox.log.ip_address} logged in(?: %{IP:watchguard_firebox.log.virtual_ip} %{GREEDYDATA:watchguard_firebox.log.message})?$
                    let _ = cached_grok!("^%{DATA:watchguard_firebox.log.user_type} %{WORD:watchguard_firebox.log.user_name}(?:@%{DATA:watchguard_firebox.log.authentication_server})? from %{IP:watchguard_firebox.log.ip_address} logged in(?: %{IP:watchguard_firebox.log.virtual_ip} %{GREEDYDATA:watchguard_firebox.log.message})?$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3E00-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{DATA:watchguard_firebox.log.user_type} %{WORD:watchguard_firebox.log.user_name}(?:@%{DATA:watchguard_firebox.log.authentication_server})? from %{IP:watchguard_firebox.log.ip_address} log in attempt was rejected(?: %{IP:watchguard_firebox.log.virtual_ip} %{GREEDYDATA:watchguard_firebox.log.message})?.$
                    let _ = cached_grok!("^%{DATA:watchguard_firebox.log.user_type} %{WORD:watchguard_firebox.log.user_name}(?:@%{DATA:watchguard_firebox.log.authentication_server})? from %{IP:watchguard_firebox.log.ip_address} log in attempt was rejected(?: %{IP:watchguard_firebox.log.virtual_ip} %{GREEDYDATA:watchguard_firebox.log.message})?.$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3E00-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{DATA:watchguard_firebox.log.user_type} %{WORD:watchguard_firebox.log.user_name}(?:@%{DATA:watchguard_firebox.log.authentication_server})? from %{IP:watchguard_firebox.log.ip_address} logged out(?: %{IP:watchguard_firebox.log.virtual_ip} %{GREEDYDATA:watchguard_firebox.log.message})?$
                    let _ = cached_grok!("^%{DATA:watchguard_firebox.log.user_type} %{WORD:watchguard_firebox.log.user_name}(?:@%{DATA:watchguard_firebox.log.authentication_server})? from %{IP:watchguard_firebox.log.ip_address} logged out(?: %{IP:watchguard_firebox.log.virtual_ip} %{GREEDYDATA:watchguard_firebox.log.message})?$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3E00-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^Updated the value of %{DATA:watchguard_firebox.log.property_name} from %{DATA:watchguard_firebox.log.previous_system_time} %{DATA:watchguard_firebox.log.unit} to %{DATA:watchguard_firebox.log.new_system_time} %{DATA:watchguard_firebox.log.unit}$
                    let _ = cached_grok!("^Updated the value of %{DATA:watchguard_firebox.log.property_name} from %{DATA:watchguard_firebox.log.previous_system_time} %{DATA:watchguard_firebox.log.unit} to %{DATA:watchguard_firebox.log.new_system_time} %{DATA:watchguard_firebox.log.unit}$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["4001-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^CA certificate updated successfully to version %{DATA:watchguard_firebox.log.new_ca_certificate_version}.$
                    let _ = cached_grok!("^CA certificate updated successfully to version %{DATA:watchguard_firebox.log.new_ca_certificate_version}.$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["4001-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^CA certificate update failed. Current CA certificate version: %{DATA:watchguard_firebox.log.current_ca_certificate_version}.$
                    let _ = cached_grok!("^CA certificate update failed. Current CA certificate version: %{DATA:watchguard_firebox.log.current_ca_certificate_version}.$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["4001-0003", "4001-0004", "4001-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Certificate (subject=") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.certificate_subject", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") is ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["4101-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("RapidDeploy configuration from a USB drive was not applied: ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.message", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["4100-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("RapidDeploy package was not applied: ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reason", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5000-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{DATA:watchguard_firebox.log.ui_type} %{NOTSPACE:_tmp_user} from %{IP:watchguard_firebox.log.ip_address} log in attempt was rejected - %{GREEDYDATA:watchguard_firebox.log.message}.$
                    let _ = cached_grok!("^%{DATA:watchguard_firebox.log.ui_type} %{NOTSPACE:_tmp_user} from %{IP:watchguard_firebox.log.ip_address} log in attempt was rejected - %{GREEDYDATA:watchguard_firebox.log.message}.$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0000"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System boot up at ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.bootup_time", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("LIVESECURITY' feature expired (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") prior to package release date (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.feature_expiration_date", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") prior to package release date (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.package_release_time", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0006"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System upgrade to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" successful, ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.software_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" successful, ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reboot_status", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System is automatically rebooting at ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reboot_hour", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reboot_second", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System time changed from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.previous_system_time", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.new_system_time", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-000B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Device ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" restore from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.restore_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" restore from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" image initiated, ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.image_source", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" image initiated, ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reboot_option", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0021"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Device ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" restore from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.restore_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" restore from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" succeeded") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.image_source", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" succeeded") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0016"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^LIVESECURITY' feature will expire on %{GREEDYDATA:watchguard_firebox.log.feature_expiration_date}.$
                    let _ = cached_grok!("^LIVESECURITY' feature will expire on %{GREEDYDATA:watchguard_firebox.log.feature_expiration_date}.$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-001A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System upgrade failed: ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reason", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0025"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System integrity check failed. ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reason", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0026"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System integrity check could not complete because of an error. ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.reason", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0101-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Management user ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.operation", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.subsystem", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0207-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("'") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" IPSec tunnel is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IPSec tunnel is ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(". local:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(". local:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" remote:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_mask_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" remote:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" inSA:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.remote_mask_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" inSA:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" outSA:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.in_spi", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" outSA:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" role:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.out_spi", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" role:") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.negotiation_role", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["7800-0000"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("VPN (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") connection by user ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.vpn_connection_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") connection by user ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed to meet TDR Host Sensor Enforcement requirement: ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed to meet TDR Host Sensor Enforcement requirement: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.reason", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["7800-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("VPN (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") connection by user ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.vpn_connection_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") connection by user ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" met all TDR Host Sensor Enforcement requirements.") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" met all TDR Host Sensor Enforcement requirements.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["7001-0001", "7001-0002", "7001-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Mobile device ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(": user ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.device_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": user ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["7001-0009"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Mobile device ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(": session for user ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.device_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": session for user ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" is recreated.") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" is recreated.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["7002-0000"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Mobile device ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(": device authorization agreement (version ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.device_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": device authorization agreement (version ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(") is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.version_number", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(") is ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" by user ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" by user ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" on ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" on ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_response_time", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["7001-0000", "7001-0004", "7001-0005", "7001-0006", "7001-0007", "7001-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Mobile device ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.device_id", &remaining[..pos]));
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_user") {
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\%{USERNAME:watchguard_firebox.log.user_name}$
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.user_name}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.user_name}@%{HOSTNAME:watchguard_firebox.log.user_domain}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.user_name}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.user_name}@%{HOSTNAME:watchguard_firebox.log.user_domain}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.user_name}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.blocked_site_limit") {
                if let Some(val) = event.get("watchguard_firebox.log.blocked_site_limit") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.blocked_site_limit".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.blocked_site_limit", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_limit_to_long")?;
                        if event.remove("watchguard_firebox.log.blocked_site_limit").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.blocked_site_limit".into() });
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
                event.set("_ingest.on_failure_processor_tag", "convert_dst_ip_to_ip")?;
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
            if event.has_value("watchguard_firebox.log.failure_count") {
                if let Some(val) = event.get("watchguard_firebox.log.failure_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.failure_count".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.failure_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_failure_count_to_long")?;
                        if event.remove("watchguard_firebox.log.failure_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.failure_count".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("watchguard_firebox.log.ip_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.ip_address") {
                if let Some(val) = event.get("watchguard_firebox.log.ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.ip_address".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ipaddr_to_ip")?;
                        if event.remove("watchguard_firebox.log.ip_address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.ip_address".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.negotiation_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.negotiation_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.negotiation_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.negotiation_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.negotiation_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_nego_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.negotiation_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.negotiation_ip".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.new_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.new_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.new_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.new_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.new_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_new_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.new_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.new_ip".into() });
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
            if event.has_value("watchguard_firebox.log.new_mask") {
                if let Some(val) = event.get("watchguard_firebox.log.new_mask") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.new_mask".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.new_mask", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_new_mask_to_long")?;
                        if event.remove("watchguard_firebox.log.new_mask").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.new_mask".into() });
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
            if event.has_value("watchguard_firebox.log.old_policy_position") {
                if let Some(val) = event.get("watchguard_firebox.log.old_policy_position") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.old_policy_position".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.old_policy_position", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_old_policy_position_to_long")?;
                        if event.remove("watchguard_firebox.log.old_policy_position").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.old_policy_position".into() });
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
            if event.has_value("watchguard_firebox.log.new_policy_position") {
                if let Some(val) = event.get("watchguard_firebox.log.new_policy_position") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.new_policy_position".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.new_policy_position", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_new_policy_position_to_long")?;
                        if event.remove("watchguard_firebox.log.new_policy_position").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.new_policy_position".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("watchguard_firebox.log.peer_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.peer_address") {
                if let Some(val) = event.get("watchguard_firebox.log.peer_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.peer_address".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.peer_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_peer_addr_to_ip")?;
                        if event.remove("watchguard_firebox.log.peer_address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.peer_address".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.previous_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.previous_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.previous_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.previous_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.previous_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_pre_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.previous_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.previous_ip".into() });
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
            if event.has_value("watchguard_firebox.log.previous_mask") {
                if let Some(val) = event.get("watchguard_firebox.log.previous_mask") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.previous_mask".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.previous_mask", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_pre_mask_to_long")?;
                        if event.remove("watchguard_firebox.log.previous_mask").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.previous_mask".into() });
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
            if event.has_value("watchguard_firebox.log.reboot_hour") {
                if let Some(val) = event.get("watchguard_firebox.log.reboot_hour") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.reboot_hour".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.reboot_hour", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_hour_to_long")?;
                        if event.remove("watchguard_firebox.log.reboot_hour").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.reboot_hour".into() });
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
            if event.has_value("watchguard_firebox.log.reboot_second") {
                if let Some(val) = event.get("watchguard_firebox.log.reboot_second") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.reboot_second".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.reboot_second", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_second_to_long")?;
                        if event.remove("watchguard_firebox.log.reboot_second").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.reboot_second".into() });
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
                event.set("_ingest.on_failure_processor_tag", "convert_src_ip_to_ip")?;
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
                event.set("_ingest.on_failure_processor_tag", "convert_src_port_to_long")?;
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

            let _cond = { event.get_str("watchguard_firebox.log.static_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.static_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.static_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.static_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.static_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_static_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.static_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.static_ip".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.virtual_ip_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.virtual_ip_address") {
                if let Some(val) = event.get("watchguard_firebox.log.virtual_ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.virtual_ip_address".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.virtual_ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_virtual_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.virtual_ip_address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.virtual_ip_address".into() });
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

            if let Some(v) = event.get("watchguard_firebox.log.body").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.message").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.operation").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.update").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.authentication_server").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.authentication_server") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.authentication_server").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.certificate_subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.client_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.user_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.user_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.current_ca_certificate_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.x509.version_number", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.new_ca_certificate_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.x509.version_number", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.peer_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.virtual_ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.dev_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.device_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("device.id", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.interface_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.id", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.interface_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.alias", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if event.has_value("watchguard_firebox.log.mac_address") {
                gsub_field(event, "watchguard_firebox.log.mac_address", "watchguard_firebox.log.mac_address", cached_regex!(":"), "-")?;
            }

            if event.has_value("watchguard_firebox.log.mac_address") {
                map_strings(event, "watchguard_firebox.log.mac_address", "watchguard_firebox.log.mac_address", str::to_uppercase)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.mac_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.mac", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.policy_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.peer_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.peer_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.previous_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.previous_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.virtual_ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.virtual_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.destination_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.source_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.user_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.user_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.client_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.client_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.negotiation_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.negotiation_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.new_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.new_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.static_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.static_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.proxy_host") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.proxy_host").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("watchguard_firebox.log.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.package_release_time") && event.get_str("watchguard_firebox.log.package_release_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("watchguard_firebox.log.package_release_time") {
                    match parse_date_out(&date_str, &["EEE MMM dd HH:mm:ss yyyy"], None, None) {
                        Some(parsed) => event.set("watchguard_firebox.log.package_release_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "watchguard_firebox.log.package_release_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_set_package_release_time")?;
                        if event.remove("watchguard_firebox.log.package_release_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.package_release_time".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.feature_expiration_date") && event.get_str("watchguard_firebox.log.feature_expiration_date") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("watchguard_firebox.log.feature_expiration_date") {
                    match parse_date_out(&date_str, &["EEE MMM dd HH:mm:ss yyyy", "EEE., MMM d, HH:mm:ss z yyyy"], None, None) {
                        Some(parsed) => event.set("watchguard_firebox.log.feature_expiration_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "watchguard_firebox.log.feature_expiration_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_set_feature_expiration_date")?;
                        if event.remove("watchguard_firebox.log.feature_expiration_date").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.feature_expiration_date".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_response_time") && event.get_str("watchguard_firebox.log.user_response_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("watchguard_firebox.log.user_response_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss Z", "yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("watchguard_firebox.log.user_response_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "watchguard_firebox.log.user_response_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_set_user_response_time")?;
                        if event.remove("watchguard_firebox.log.user_response_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.user_response_time".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.bootup_time") && event.get_str("watchguard_firebox.log.bootup_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("watchguard_firebox.log.bootup_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss Z", "yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("watchguard_firebox.log.bootup_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "watchguard_firebox.log.bootup_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_set_bootup_time")?;
                        if event.remove("watchguard_firebox.log.bootup_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.bootup_time".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("_tmp_msg");
                event.remove("_tmp_user");

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("watchguard_firebox.log.action");
                event.remove("watchguard_firebox.log.authentication_server");
                event.remove("watchguard_firebox.log.certificate_subject");
                event.remove("watchguard_firebox.log.client_name");
                event.remove("watchguard_firebox.log.current_ca_certificate_version");
                event.remove("watchguard_firebox.log.destination_ip");
                event.remove("watchguard_firebox.log.destination_port");
                event.remove("watchguard_firebox.log.dev_name");
                event.remove("watchguard_firebox.log.device_id");
                event.remove("watchguard_firebox.log.domain");
                event.remove("watchguard_firebox.log.interface_id");
                event.remove("watchguard_firebox.log.interface_name");
                event.remove("watchguard_firebox.log.ip_address");
                event.remove("watchguard_firebox.log.mac_address");
                event.remove("watchguard_firebox.log.md5");
                event.remove("watchguard_firebox.log.message");
                event.remove("watchguard_firebox.log.new_ca_certificate_version");
                event.remove("watchguard_firebox.log.operation");
                event.remove("watchguard_firebox.log.path");
                event.remove("watchguard_firebox.log.peer_address");
                event.remove("watchguard_firebox.log.policy_name");
                event.remove("watchguard_firebox.log.port");
                event.remove("watchguard_firebox.log.reason");
                event.remove("watchguard_firebox.log.source_ip");
                event.remove("watchguard_firebox.log.source_port");
                event.remove("watchguard_firebox.log.update");
                event.remove("watchguard_firebox.log.user_domain");
                event.remove("watchguard_firebox.log.user_name");
                event.remove("watchguard_firebox.log.virtual_ip_address");
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
